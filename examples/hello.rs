#![feature(globs)]
extern crate iron;

use iron::prelude::*;

use iron::response::modifiers::{Status, Body};
use iron::status;

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::new()
           .set(Status(status::Ok))
           .set(Body("Hello world!")))
}

fn main() {
    Iron::new(hello_world).listen("localhost:3000").unwrap();
    println!("On 3000");
}

