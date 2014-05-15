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

#![allow(visible_private_types)]

use std::default::Default;
use collections::HashMap;

use method::{Method, Get};
use response::{Resp, RoutingError};
use tools::{RoutesFnType, DummyProducer, Producer};

/// The web dispatcher
pub struct Dispatcher<T, U = DummyProducer> {
    routes: HashMap<(Vec<StrBuf>, Method), RoutesFnType<T>>,
    producer: U
}

impl<T, U: Producer + Default = DummyProducer> Dispatcher<T, U> {
    pub fn new(routes: Vec<(RoutesFnType<T>, &str, &str)>) -> Dispatcher<T, U> {
        Dispatcher {
            routes: routes.move_iter().fold(HashMap::new(), |mut h, (f, r, m)| {
                let r_ = split_route(r);
                h.insert((r_, from_str(m).unwrap()), f); h
            }),
            producer: Default::default()
        }
    }

    pub fn set_producer(&mut self, param_producer: U) {
        self.producer = param_producer
    }

    pub fn run_for_method(&mut self,
                          route: &str,
                          web_params: HashMap<StrBuf, StrBuf>,
                          method: Method)
                          -> Resp<T> {
        match self.simple_hash_find_route(route, &web_params, method) {
            Some(r) => r,
            None    => {
                match self.complex_regex_find_route(route, &web_params, method) {
                    Some(r) => r,
                    None    => RoutingError(format_strbuf!("route: {}, don't exist", route))
                }
            }
        }
    }

    pub fn run(&mut self,
               route: &str,
               web_params: HashMap<StrBuf, StrBuf>)
               -> Resp<T> {
        self.run_for_method(route, web_params, Get)
    }

    fn simple_hash_find_route(&mut self,
                             route: &str,
                             web_params: &HashMap<StrBuf, StrBuf>,
                             method: Method)
                             -> Option<Resp<T>> {
        let r_ = split_route(route);
        match self.routes.find(&(r_, method)) {
            Some(&f) => Some(f(web_params.clone(), self.producer.get_new())),
            None     => None
        }
    }

    fn complex_regex_find_route(&mut self,
                             route: &str,
                             web_params: &HashMap<StrBuf, StrBuf>,
                             method: Method)
                             -> Option<Resp<T>> {
        let r_ = split_route(route);
        match self.routes.find(&(r_, method)) {
            Some(&f) => Some(f(web_params.clone(), self.producer.get_new())),
            None     => None
        }
    }
}

fn split_route(route: &str) -> Vec<StrBuf> {
    let r_: Vec<&str> = route.split('/').collect();
    let mut r_: Vec<StrBuf> = r_.iter().map(|r| r.to_strbuf()).collect();
    if r_.last().is_some() && r_.last().unwrap() == &"".to_strbuf() { r_.pop(); }
    r_
}

