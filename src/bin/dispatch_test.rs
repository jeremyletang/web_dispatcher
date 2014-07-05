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

use std::any::Any;
use std::collections::HashMap;

use web_dispatcher::{Dispatcher, WebParams, Resp, Filled};

mod foo;

// #[method = "POST"]
#[route = "/hello/main"]
pub fn hello_route(_: HashMap<String, String>, _: Box<Any>) -> Resp<String> {
    Filled("hello from root mod !".to_string())
}

#[route = "/hello/{my_var}/main/"]
pub fn hello_route2(p: HashMap<String, String>, _: Box<Any>) -> Resp<String> {
    Filled(format!("Your name is: {}, and your age is: {} !",
           p.to_string("name").unwrap(),
           p.to_int("age").unwrap()))
}

fn main() {
    let mut params = HashMap::new();
    params.insert("Paul".to_string(), "Paul".to_string());
    params.insert("age".to_string(), "42".to_string());
    let mut dispatcher = Dispatcher::<String>::new(routes!());
    let return_value = dispatcher.run("/hello/main", params);
    println!("{}", return_value.unwrap())
}
