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

use web_dispatcher::{Dispatcher, WebParams, Resp, Filled};

mod foo;

#[method = "POST"]
#[route = "/hello/main/POST"]
pub fn hello_route(_: HashMap<StrBuf, StrBuf>, _: Box<Any>) -> Resp<StrBuf> {
    Filled("hello from root mod !".to_strbuf())
}

#[route = "/hello/main"]
pub fn hello_route2(p: HashMap<StrBuf, StrBuf>, _: Box<Any>) -> Resp<StrBuf> {
    Filled(format_strbuf!("Your name is: {}, and your age is: {} !",
           p.to_string("name").unwrap(),
           p.to_int("age").unwrap()))
}

fn main() {
    let mut params = HashMap::new();
    params.insert("Paul".to_strbuf(), "Paul".to_strbuf());
    params.insert("age".to_strbuf(), "42".to_strbuf());
    let mut dispatcher = Dispatcher::<StrBuf>::new(routes!());
    let return_value = dispatcher.run("/hello/main", params);
    println!("{}", return_value.unwrap())
}
