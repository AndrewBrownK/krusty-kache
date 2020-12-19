extern crate iron;
extern crate router;

use iron::{Iron, Request, Response, Handler};
use iron::status;
use router::{Router};
use std::sync::{Arc, Mutex};
use std::io::Read;
use evmap::{WriteHandle, ReadHandleFactory};
use thread_local::ThreadLocal;

/// Main function, the entry point for our program.
fn main() {
    // The EVMap is the primary store of data.
    // It is designed for concurrent use cases, so we have distinct read and write handles.
    let (rh, wh) = evmap::new();

    // Read handles can't cross thread boundaries, but read handle factories can.
    let read_handle_factory = rh.factory();

    // Write handle (shared mutex references to it)
    let write_handle_post = Arc::new(Mutex::new(wh));
    let write_handle_put = write_handle_post.clone();
    let write_handle_delete = write_handle_post.clone();

    // Here we define a single route that works with four methods (GET, POST, PUT, DELETE).
    let mut routes = Router::new();
    routes
        .get("/cache/:key", create_get_handler(read_handle_factory), "cache_get")
        .post("/cache/:key", create_put_handler(write_handle_post), "cache_post")
        .put("/cache/:key", create_put_handler(write_handle_put), "cache_put")
        .delete("/cache/:key", create_delete_handler(write_handle_delete), "cache_delete");

    // Here we actually start the server.
    // By default, Iron uses 8 times the number of CPU threads.
    match Iron::new(routes).http("localhost:3000") {
        Err(e) => println!("Iron server had trouble initializing =( {}", e),
        _ => println!("Iron server is running!")
    }
}


/// Extract a key from a request, and then produce a response.
fn use_key<'a>(f: impl Fn(String) -> Response + Sync + Send + 'a) -> Box<dyn Handler + 'a> {
    use_key_and_value(move |k, _| f(k))
}

/// Extract a key and value (body) from a request, and then produce a response.
fn use_key_and_value<'a>(f: impl Fn(String, String) -> Response + Sync + Send + 'a) -> Box<dyn Handler + 'a> {
    Box::new(move |req: &mut Request| {
        let key = req
            .extensions.get::<Router>()
            .and_then(|it| { it.find("key")});
        let key = match key {
            None => return Ok(Response::with(status::BadRequest)),
            Some(k) => String::from(k)
        };
        let mut body = String::new();
        match req.body.read_to_string(&mut body) {
            Ok(size) => {
                if size > 0 {
                    println!("Request with key {} has value size of {}", key, size);
                }
            },
            Err(e) => {
                println!("We only accept UTF-8 entries: {} -> {}", key, e);
                return Ok(Response::with(status::BadRequest))
            },
        }
        Ok(f(key, body))
    })
}

/// This is the PUT/POST handler logic.
fn create_put_handler(write_handle: Arc<Mutex<WriteHandle<String, String>>>) -> Box<dyn Handler> {
    use_key_and_value(move |key, value| {
        match write_handle.lock() {
            Ok(mut write_handle) => {
                write_handle.update(key, value);
                write_handle.refresh();
                Response::with((status::Ok, ""))
            },
            Err(err) => {
                println!("Our write handle mutex is poisoned! Why did you panic? {}", err);
                Response::with((status::InternalServerError, key))
            }
        }
    })
}

/// This is the DELETE handler logic.
fn create_delete_handler(write_handle: Arc<Mutex<WriteHandle<String, String>>>) -> Box<dyn Handler> {
    use_key(move |key| {
        match write_handle.lock() {
            Ok(mut write_handle) => {
                write_handle.empty(key);
                write_handle.refresh();
                Response::with((status::Ok, ""))
            },
            Err(err) => {
                println!("Our write handle mutex is poisoned! Why did you panic? {}", err);
                Response::with((status::InternalServerError, key))
            }
        }
    })
}

/// This is the GET handler logic.
/// ReadHandles are not Sync or Send, and so we cannot give them to this handler which will be
/// multithreaded. Instead, the ReadHandleFactory is Sync and Send, and we can use it acquire a
/// ReadHandle in each thread separately.
fn create_get_handler(read_handle_factory: ReadHandleFactory<String, String>) -> Box<dyn Handler> {
    // The drawback with ReadHandleFactory is that using it to acquire a new ReadHandle puts us
    // through a synchronized lock. Therefore we don't want to get a fresh read handle once per
    // request, because that will make every concurrent request go through the same lock. Instead
    // of getting a fresh read handle once per request, we want a fresh read handle once per thread.
    // That is why we are using ThreadLocal to lazily initialize a single ReadHandle per thread.
    let read_handle = ThreadLocal::new();
    use_key(move |key| {
        let read_handle = read_handle.get_or(|| read_handle_factory.handle());
        match read_handle.get_one(&key) {
            None => Response::with(status::NoContent),
            Some(value) => Response::with((status::Ok, (*value).as_str()))
        }
    })
}
