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

extern crate collections;
extern crate web_dispatcher;
#[phase(syntax, link)]
extern crate route_macros;

use std::any::Any;
use collections::HashMap;

use web_dispatcher::response::Resp;
use web_dispatcher::Dispatcher;

mod foo;

#[method = "POST"]
#[route = "/hello/main/POST"]
pub fn hello_route(_: HashMap<~str, ~str>, _: ~Any) -> Resp<~str> {
    println!("hello from root mod !");
    Resp::no()
}

#[route = "/hello/main"]
pub fn hello_route2(_: HashMap<~str, ~str>, _: ~Any) -> Resp<~str> {
    println!("hello from root mod too !");
    Resp::no()
}

fn main() {
    let mut dispatcher = Dispatcher::<~str>::new(routes!());
    dispatcher.run("/hello/main", HashMap::new());
    let r = routes!();
    println!("{:?}", r);
}
