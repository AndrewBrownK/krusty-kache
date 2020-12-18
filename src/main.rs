extern crate iron;
extern crate router;

use iron::{Iron, Request, Response, Handler};
use iron::status;
use router::{Router};
use std::sync::{Arc, Mutex};
use std::io::Read;
use evmap::{WriteHandle, ReadHandleFactory};

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

    // Route handlers
    // POST and PUT are synonymous in this case, although PUT may be more semantically correct.
    let post_handler = create_put_handler(write_handle_post);
    let put_handler = create_put_handler(write_handle_put);
    let get_handler = create_get_handler(read_handle_factory);
    let delete_handler = create_delete_handler(write_handle_delete);

    // Here we define a single route that works with four methods (GET, POST, PUT, DELETE).
    let mut routes = Router::new();
    routes
        .get("/cache/:key", get_handler, "cache_get")
        .post("/cache/:key", post_handler, "cache_post")
        .put("/cache/:key", put_handler, "cache_put")
        .delete("/cache/:key", delete_handler, "cache_delete");

    // Here we actually start the server.
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
                write_handle.update(key.clone(), value);
                write_handle.refresh();
                Response::with((status::Ok, ""))
            },
            Err(err) => {
                println!("Our write handle mutex is poisoned! Why did you panic? {}", err);
                Response::with((status::InternalServerError, key.clone()))
            }
        }
    })
}

/// This is the DELETE handler logic.
fn create_delete_handler(write_handle: Arc<Mutex<WriteHandle<String, String>>>) -> Box<dyn Handler> {
    use_key(move |key| {
        match write_handle.lock() {
            Ok(mut write_handle) => {
                write_handle.empty(key.clone());
                write_handle.refresh();
                Response::with((status::Ok, ""))
            },
            Err(err) => {
                println!("Our write handle mutex is poisoned! Why did you panic? {}", err);
                Response::with((status::InternalServerError, key.clone()))
            }
        }
    })
}

/// This is the GET handler logic.
fn create_get_handler(read_handle_factory: ReadHandleFactory<String, String>) -> Box<dyn Handler> {
    use_key(move |key| {
        // todo: acquiring a read handle goes through a lock, and so does not scale concurrently.
        //  Ideally we should only acquire a read handle once per thread, not once per request.
        //  However of borrowing rules, it is not as simple as scooting rh out of this closure.
        //  So figure this out when you have the time to.
        let rh = read_handle_factory.handle();

        // For some reason the borrow checker won't let us return this expression directly
        // because of rh's lifetime. So let's just slap the result in a variable and return it
        // right after.
        let result = match rh.get_one(&key) {
            None => Response::with(status::NoContent),
            Some(value) => Response::with((status::Ok, (*value).as_str()))
        };
        result
    })
}
