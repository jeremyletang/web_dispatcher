#![feature(phase)]

#[phase(plugin, link)]
extern crate route_macros;
extern crate web_dispatcher;

use std::collections::HashMap;
use web_dispatcher::{Dispatcher, WebParams, Resp, Filled} ;

#[route = "/some/*/strange/:age/route"]
pub fn default(p: HashMap<String, String>, _: ()) -> Resp<String> {
    Filled(format!("The name is: {} and the age is {}",
                   p.to_string("name"),
                   p.to_int("age")))
}

fn main() {
    // Create and fill the webparams
    let mut params = HashMap::new();
    params.insert("name".to_string(), "Paul".to_string());

    // Create the web_dispatcher and initialize it with routes
    let mut dispatcher = Dispatcher::<String>::new(routes!());

    // Dispatch and store the result
    let return_value = dispatcher.run("/some/really/strange/42/route", params);

    // print the response
    println!("{}", return_value.unwrap())
}