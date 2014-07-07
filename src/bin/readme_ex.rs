#![feature(phase)]

#[phase(plugin, link)]
extern crate route_macros;
extern crate web_dispatcher;
extern crate debug;

use std::collections::HashMap;
use web_dispatcher::{Dispatcher, WebParams, Response, Request} ;

#[route = "/some/*/strange/:age/route"]
pub fn default(p: &Request, _: ()) -> Box<Response> {
    println!("The name is: {} and the age is {}",
             p.params().to_string("name"),
             p.params().to_int("age"))
    (box ()()) as Box<Response>
}

fn main() {
    // Get routes
    let routes = routes!();
    println!("{:?}", routes.as_slice());
    // Create and fill the webparams
    let mut params = HashMap::new();
    params.insert("name".to_string(), "Paul".to_string());

    // Create the web_dispatcher and initialize it with routes
    let mut dispatcher = Dispatcher::<()>::new(routes.as_slice());

    // Dispatch and store the result
    dispatcher.run("/some/really/strange/42/route", params);
}