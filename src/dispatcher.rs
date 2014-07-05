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
use std::collections::HashMap;

use method::{Method, Get};
use response::{Resp, RoutingError};
use tools::{RoutesFnType, UnusedProducer, Producer};

/// The web dispatcher
pub struct Dispatcher<T, P = UnusedProducer, U = ()> {
    routes: HashMap<(Vec<String>, Method), RoutesFnType<T, U>>,
    producer: P
}

impl<T, P: Producer<U> + Default = UnusedProducer, U = ()> Dispatcher<T, P, U> {
    pub fn new(routes: Vec<(RoutesFnType<T, U>, &str, &str)>) -> Dispatcher<T, P, U> {
        Dispatcher {
            routes: routes.move_iter().fold(HashMap::new(), |mut h, (f, r, m)| {
                let r_ = split_route(r);
                h.insert((r_, from_str(m).unwrap()), f); h
            }),
            producer: Default::default()
        }
    }

    pub fn set_producer(&mut self, param_producer: P) {
        self.producer = param_producer
    }

    pub fn run_with_method(&mut self,
                           route: &str,
                           web_params: HashMap<String, String>,
                           method: Method)
                           -> Resp<T> {
        match self.find_simple_hash_route(route, &web_params, method) {
            Some(r) => r,
            None    => {
                match self.find_complex_route(route, &web_params, method) {
                    Some(r) => r,
                    None    => RoutingError(format!("route: {}, don't exist", route))
                }
            }
        }
    }

    pub fn run(&mut self,
               route: &str,
               web_params: HashMap<String, String>)
               -> Resp<T> {
        self.run_with_method(route, web_params, Get)
    }

    fn find_simple_hash_route(&mut self,
                             route: &str,
                             web_params: &HashMap<String, String>,
                             method: Method)
                             -> Option<Resp<T>> {
        let r_ = split_route(route);
        match self.routes.find(&(r_, method)) {
            Some(&f) => Some(f(web_params.clone(), self.producer.get_new())),
            None     => None
        }
    }

    fn find_complex_route(&mut self,
                          route: &str,
                          web_params: &HashMap<String, String>,
                          method: Method)
                          -> Option<Resp<T>> {
        let r_ = split_route(route);
        for (&(ref r, m), f) in self.routes.iter() {
            println!("{}", r);
        }
        None
    }
}

fn split_route(route: &str) -> Vec<String> {
    let r_: Vec<&str> = route.split('/').collect();
    let mut r_: Vec<String> = r_.iter().map(|r| r.to_string()).collect();
    if r_.last().is_some() && r_.last().unwrap() == &"".to_string() { r_.pop(); }
    r_
}

