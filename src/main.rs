extern crate futures;
extern crate hyper;
extern crate phone_adventures;
extern crate url;

use phone_adventures::web;
use hyper::rt::Future;
use hyper::service;
use hyper::Server;
use hyper::rt;

fn main() {
    let addr = ([0, 0, 0, 0], 3000).into();

    let server = Server::bind(&addr)
        .serve(|| service::service_fn(web::handle))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    rt::run(server);
}
