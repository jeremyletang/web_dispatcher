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
use std::fmt::{Show, Formatter, FormatError};

use regex::Regex;

use method::{Method, Get};
use response::{Resp, RoutingError};
use tools::{RoutesFnType, UnusedProducer, Producer};

pub struct RouteDatas<T, U> {
    var_names: Vec<&'static str>,
    regex: Regex,
    f: RoutesFnType<T, U>
}

/// The web dispatcher
pub struct Dispatcher<T, P = UnusedProducer, U = ()> {
    routes: HashMap<(String, Method), RouteDatas<T, U>>,
    producer: P
}

impl<T, P: Producer<U> + Default = UnusedProducer, U = ()> Dispatcher<T, P, U> {
    pub fn new(routes: Vec<(RoutesFnType<T, U>, &str, &str, Vec<&'static str>, &str)>) -> Dispatcher<T, P, U> {
        Dispatcher {
            routes: routes.move_iter().fold(HashMap::new(), |mut h, (f, r, m, vars, matcher)| {
                // let re = Regex::new(vars).unwrap();
                // let vars: Vec<String> = match re.captures(r) {
                //     Some(c) => {
                //         let mut cap_i = c.iter();
                //         cap_i.next();
                //         cap_i.map(|x| String::from_str(x)).collect()
                //     },
                //     None => Vec::new()
                // };
                let d = RouteDatas {
                    var_names: vars,
                    regex: Regex::new(matcher).unwrap(),
                    f: f
                };
                h.insert((r.to_string(), from_str(m).unwrap()), d); h
            }),
            producer: Default::default()
        }
    }

    pub fn new_with_producer(routes: Vec<(RoutesFnType<T, U>, &str, &str, Vec<&'static str>, &str)>,
                             producer: P) -> Dispatcher<T, P, U> {
        let mut d = Dispatcher::new(routes);
        d.producer = producer;
        d
    }

    pub fn set_producer(&mut self, param_producer: P) {
        self.producer = param_producer
    }

    // pub fn add_route(&mut self,
    //                  func: RoutesFnType<T, U>,
    //                  route_name: &str,
    //                  method: Method) {

    //     self.routes.insert((split_route(route_name), method), func);
    // }

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
        // let r_ = split_route(route);
        match self.routes.find(&(route.to_string(), method)) {
            Some(f) => Some((f.f)(web_params.clone(), self.producer.get_new())),
            None     => None
        }
    }

    fn find_complex_route(&mut self,
                          route: &str,
                          web_params: &HashMap<String, String>,
                          method: Method)
                          -> Option<Resp<T>> {
        let mut result = None;
        for (&(_, m), d) in self.routes.iter() {
            if m == method {
                if d.regex.is_match(route) {
                    let mut new_params: HashMap<String, String> = HashMap::new();
                    if d.var_names.len() > 0 {
                        let c = d.regex.captures(route).unwrap();
                        let mut i = c.iter();
                        i.next();
                        d.var_names.iter().zip(i).advance(|(a, b)| {
                            new_params.insert(a.to_string(), b.to_string());
                            true
                        });
                    }
                    new_params.extend(web_params.clone().move_iter());
                    result = Some((d.f)(new_params, self.producer.get_new()));
                    break;
                }
            }
        }

        result
    }
}

impl<T, U, P> Show for Dispatcher<T, U, P> {
     fn fmt(&self, f: &mut Formatter) ->  Result<(), FormatError> {
        let mut to_write = String::from_str("Dispatcher {\n");
        for (&(ref r, m), _) in self.routes.iter() {
            to_write.push_str(format!("    {} {}\n", m, r).as_slice());
        }
        write!(f, "{}}}\n", to_write)
     }
}
