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

#![feature(phase)]

#[phase(plugin, link)]
extern crate route_macros;
extern crate web_dispatcher;

use std::collections::HashMap;

use web_dispatcher::{Dispatcher, WebParams, Resp, Filled, Producer, Get};

mod foo;

#[deriving(Default)]
pub struct StringProducer;

impl Producer<String> for StringProducer {
    fn get_new(&self) -> String {
        String::from_str("This is a string from the custom producer")
    }
}


#[method = "POST"]
#[route = "/hello/main"]
pub fn hello_route(_: HashMap<String, String>, u: String) -> Resp<String> {
    println!("param u contains: {}", u);
    Filled("hello from root mod !".to_string())
}

#[route = "/hello/:my_var/main/"]
pub fn hello_route2(p: HashMap<String, String>, _: String) -> Resp<String> {
    println!(":my_var is {}", p.to_string("my_var"));
    Filled(format!("Your name is: {}, and your age is: {} !",
           p.to_string("name").unwrap(),
           p.to_int("age").unwrap()))
}

pub fn add_route(_: HashMap<String, String>, _: String) -> Resp<String> {
    println!("Hand added route !");
    Filled("Hand added route!".to_string())
}

fn main() {
    let mut params = HashMap::new();
    params.insert("name".to_string(), "Paul".to_string());
    params.insert("age".to_string(), "42".to_string());
    let mut dispatcher = Dispatcher::<String, StringProducer, String>::new(routes!());
    dispatcher.add_route(add_route, "/add/route", Get);
    let return_value = dispatcher.run_with_method("/hello/main", params.clone(), web_dispatcher::Post);
    dispatcher.run("/hello/blah/main/", params.clone());
    dispatcher.run("/add/route", params.clone());
    dispatcher.run("/hello/*/bar/", params.clone());
    println!("{}", return_value.unwrap());
}
