// The MIT License (MIT)
//
// Copyright (c) 2014 Jeremy Letang
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variable)]
#![feature(phase)]

#[phase(plugin, link)]
extern crate route_macros;
extern crate web_dispatcher;
extern crate debug;

use std::collections::HashMap;

use web_dispatcher::{Dispatcher, WebParams, Producer, Get, Request, Response};

mod foo;

#[deriving(Default)]
pub struct StringProducer;

impl Producer<String> for StringProducer {
    fn get_new(&self) -> String {
        String::from_str("This is a string from the custom producer")
    }
}


#[method = "post"]
#[route = "/hello/main"]
pub fn hello_route(_: &Request, u: String) -> Box<Response> {
    println!("param u contains: {}", u);
    box () () as Box<Response>
}

#[route = "/hello/:my_var/world/*/main/"]
pub fn hello_route2(p: &Request, _: String) -> Box<Response> {
    println!("From wildcar + var: :my_var is {}",
             p.params().to_string("my_var"));
    box () () as Box<Response>
}

pub fn add_route(p: &Request, _: String) -> Box<Response> {
    println!("Hand added route, user: {}", p.params().to_string("user"));
    box () () as Box<Response>
}

fn main() {
    let mut params = HashMap::new();
    let routes = routes!();
    params.insert("name".to_string(), "Paul".to_string());
    params.insert("age".to_string(), "42".to_string());
    let mut dispatcher = Dispatcher::<String, StringProducer>::new(routes.as_slice());
    dispatcher.add(add_route,
                   "/add/*/route/:user/blah/",
                   Get);
    // let return_value = dispatcher.run_with_method("/hello/main", params.clone(), web_dispatcher::Post);
    dispatcher.run("/hello/blah/world/blahahahahaha/main/", params.clone());
    dispatcher.run("/add/blah/route/jon/blah/", params.clone());
    dispatcher.run("/hello/foo/bar/", params.clone());
    // println!("{}", return_value.unwrap());
    println!("{}", dispatcher);
    println!("{:?}", &routes!());
}
