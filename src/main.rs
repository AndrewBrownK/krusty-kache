extern crate iron;
extern crate router;

use iron::{Iron, Request, Response};
use iron::status;
use router::{Router};
use std::sync::{Arc, Mutex};
use std::io::Read;

fn main() {
    // The EVMap is the primary store of data.
    // It is designed for concurrent use cases, so we have distinct read and write handles.
    // Read handles can't cross thread boundaries, but read handle factories can.
    let (rh, wh) = evmap::new();
    let read_handle_factory = rh.factory();
    let write_handle_post = Arc::new(Mutex::new(wh));
    let write_handle_delete = write_handle_post.clone();

    // This is the handler for posting data to the cache.
    // TODO figure out how the fuck to partially apply higher order functions in Rust so this
    //  can be organized outside of main()
    let post_handler = Box::new(move |req: &mut Request| {
        let key = match extract_key(req) {
            None => return Ok(Response::with(status::BadRequest)),
            Some(k) => String::from(k)
        };
        let mut body = String::new();
        match req.body.read_to_string(&mut body) {
            Ok(size) => {
                println!("Posting entry {} of size {}", key, size);
            },
            Err(e) => {
                println!("We only accept UTF-8 entries: {} -> {}", key, e);
                return Ok(Response::with(status::BadRequest))
            },
        }

        let resp = match write_handle_post.lock() {
            Ok(mut write_handle) => {
                write_handle.update(key.clone(), body);
                write_handle.refresh();
                Ok(Response::with((status::Ok, "")))
            },
            Err(err) => {
                println!("Our write handle mutex is poisoned! Why did you panic? {}", err);
                Ok(Response::with((status::InternalServerError, key.clone())))
            }
        };

        resp
    });

    // This is the handler for getting data from the cache.
    // TODO figure out how the fuck to partially apply higher order functions in Rust so this
    //  can be organized outside of main()
    let get_handler = Box::new(move |req: &mut Request| {
        let rh = read_handle_factory.handle();
        let key = match extract_key(req) {
            None => return Ok(Response::with(status::BadRequest)),
            Some(k) => String::from(k)
        };

        let resp = match rh.get_one(&key) {
            None => Ok(Response::with(status::NoContent)),
            Some(value) => Ok(Response::with((status::Ok, (*value).as_str())))
        };

        resp
    });

    // This is the handler for deleting data from the cache.
    // TODO figure out how the fuck to partially apply higher order functions in Rust so this
    //  can be organized outside of main()
    let delete_handler = Box::new(move |req: &mut Request| {
        let key = match extract_key(req) {
            None => return Ok(Response::with(status::BadRequest)),
            Some(k) => String::from(k)
        };

        let resp = match write_handle_delete.lock() {
            Ok(mut write_handle) => {
                write_handle.empty(key.clone());
                write_handle.refresh();
                Ok(Response::with((status::Ok, "")))
            },
            Err(err) => {
                println!("Our write handle mutex is poisoned! Why did you panic? {}", err);
                Ok(Response::with((status::InternalServerError, key.clone())))
            }
        };

        resp
    });

    // Here we define a single route that works with four methods (GET, POST, PUT, DELETE).
    // POST and PUT are synonymous in this case, although PUT may be more semantically correct.
    let mut routes = Router::new();
    routes
        .get("/cache/:key", get_handler, "cache_get")
        .post("/cache/:key", post_handler.clone(), "cache_post")
        .put("/cache/:key", post_handler, "cache_put")
        .delete("/cache/:key", delete_handler, "cache_delete");

    // Here we actually start the server.
    match Iron::new(routes).http("localhost:3000") {
        Err(e) => println!("Iron server had trouble initializing =( {}", e),
        _ => println!("Iron server is running!")
    }
}



// This is a handy little function to extract the key from the request.
fn extract_key<'a>(req: &'a Request) -> Option<&'a str> {
    req.extensions.get::<Router>().and_then(|it| { it.find("key")})
}
