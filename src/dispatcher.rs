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

pub struct ZipWhile<I, J> {
    i: I,
    j: J
}

impl<A, B, I: Iterator<A>, J: Iterator<B>> Iterator<(Option<A>, Option<B>)> for ZipWhile<I, J> {
    fn next(&mut self) -> Option<(Option<A>, Option<B>)> {
        let i_next = self.i.next();
        let j_next = self.j.next();
        if i_next.is_none() && j_next.is_none() {
            None
        } else {
            Some((i_next, j_next))
        }
    }
}

pub fn zip_while<A, B, I: Iterator<A>, J: Iterator<B>>(i: I, j: J) -> ZipWhile<I, J> {
    ZipWhile { i: i, j: j }
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
        let mut new_params = HashMap::new();
        let mut result = None;

        // for all stored routes
        for (&(ref r, m), f) in self.routes.iter() {
            if m == method { // if method match
                // check a stored route with the current route
                let res = zip_while(r.iter(), r_.iter()).advance(|(a, b)| {
                    if a.is_none() && b.is_none() { // both are None -> route match
                        true
                    } else if a.is_none() || b.is_none() { // one of two is None -> route don't match
                        false
                    } else {
                        let a = a.unwrap();
                        let b = b.unwrap();
                        if a == b { // if the route fragments match
                            true
                        } else {
                            if a.len() > 0 && a.as_bytes()[0] == ':' as u8 { // this is a custom var
                                let mut var = String::from_str(a.as_slice());
                                var.shift_char();
                                new_params.insert(var, b.clone());
                                true
                            } else if b.as_slice() == "*" { // glob matching
                                true
                            } else { // route don't exist
                                false
                            }
                        }
                    }
                });
                match res {
                    true => {
                        new_params.extend(web_params.clone().move_iter());
                        let f_result: Resp<T> = (*f)(new_params, self.producer.get_new());
                        result = Some(f_result);
                        break
                    },
                    false => new_params.clear()
                }
            }
        }
        result
    }
}

fn split_route(route: &str) -> Vec<String> {
    let r_: Vec<&str> = route.split('/').collect();
    let mut r_: Vec<String> = r_.iter().map(|r| r.to_string()).collect();
    if r_.last().is_some() && r_.last().unwrap() == &"".to_string() { r_.pop(); }
    r_
}

