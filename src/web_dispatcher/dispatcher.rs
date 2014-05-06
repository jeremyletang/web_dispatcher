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

//! The web dispatcher

#![allow(unused_variable)]

use std::default::Default;
use collections::HashMap;

use response::{Resp, RoutingError};
use tools::{RoutesFnType, DummyProducer, Producer};

pub struct Dispatcher<T, U = DummyProducer> {
    routes: HashMap<~str, (RoutesFnType<T>, &'static str)>,
    producer: U
}

impl<T, U: Producer + Default = DummyProducer> Dispatcher<T, U> {
    pub fn new(routes: Vec<(RoutesFnType<T>, &'static str, &'static str)>) -> Dispatcher<T, U> {
        Dispatcher {
            routes: routes.move_iter().fold(HashMap::new(), |mut h, (f, r, m)| {
                h.insert(r.to_owned(), (f, m)); h
            }),
            producer: Default::default()
        }
    }

    pub fn set_producer(&mut self, param_producer: U) {
        self.producer = param_producer
    }

    pub fn run(&mut self,
               route: &str,
               web_params: HashMap<~str, ~str>)
               -> Resp<T> {
        match self.routes.find(&route.to_owned()) {
            Some(&(f, m)) => f(web_params, self.producer.get_new()),
            None => RoutingError(format!("route: {}, don't exist", route))
        }
    }
}