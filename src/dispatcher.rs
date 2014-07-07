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
use response::{Response, Request};
use tools::{RoutesFnType, UnusedProducer, Producer};


static RE_VAR: Regex = regex!(":[0-9a-zA-Z-_]+");

pub struct RouteDatas<U> {
    var_names: Vec<String>,
    regex: Regex,
    f: RoutesFnType<U>
}

/// The web dispatcher
pub struct Dispatcher<P = UnusedProducer, U = ()> {
    routes: HashMap<(String, Method), RouteDatas<U>>,
    producer: P
}

impl<P: Producer<U> + Default = UnusedProducer, U = ()> Dispatcher<P, U> {
    pub fn new(routes: &[(RoutesFnType<U>, &str, &str, Vec<&str>, &str)]) -> Dispatcher<P, U> {
        Dispatcher {
            routes: routes.iter().fold(HashMap::new(), |mut h, &(f, r, m, ref vars, matcher)| {
                let d = RouteDatas {
                    var_names: vars.iter().map(|v| v.to_string()).collect(),
                    regex: Regex::new(matcher).unwrap(),
                    f: f
                };
                h.insert((r.to_string(), from_str(m).unwrap()), d); h
            }),
            producer: Default::default()
        }
    }

    // pub fn new_with_producer(routes: Vec<(RoutesFnType<U>, &str, &str, Vec<&'static str>, &str)>,
    //                          producer: P) -> Dispatcher<P, U> {
    //     let mut d = Dispatcher::new(routes);
    //     d.producer = producer;
    //     d
    // }

    pub fn set_producer(&mut self, param_producer: P) {
        self.producer = param_producer
    }

    pub fn add(&mut self,
               func: RoutesFnType<U>,
               route: &str,
               method: Method) {
        let clean_route = remove_trailling_slash(route);
        self.routes.insert((clean_route.to_string(), method),
                           RouteDatas {
                               var_names: create_vars_regex_vec(clean_route.as_slice()),
                               regex: create_match_regex(clean_route.as_slice()),
                               f: func
                           });
    }

    pub fn run_with_method(&mut self,
                           route: &str,
                           web_params: HashMap<String, String>,
                           method: Method)
                           -> Result<Box<Response>, String> {
        match self.find_simple_hash_route(route, &web_params, method) {
            Some(r) => Ok(r),
            None    => {
                match self.find_complex_route(route, &web_params, method) {
                    Some(r) => Ok(r),
                    None    => Err(format!("route: {}, don't exist", route))
                }
            }
        }
    }

    pub fn run(&mut self,
               route: &str,
               web_params: HashMap<String, String>)
               -> Result<Box<Response>, String> {
        self.run_with_method(route, web_params, Get)
    }

    fn find_simple_hash_route(&mut self,
                             route: &str,
                             web_params: &HashMap<String, String>,
                             method: Method)
                             -> Option<Box<Response>> {
        match self.routes.find(&(route.to_string(), method)) {
            Some(f) => Some((f.f)(web_params as &Request, self.producer.get_new())),
            None     => None
        }
    }

    fn find_complex_route(&mut self,
                          route: &str,
                          web_params: &HashMap<String, String>,
                          method: Method)
                          -> Option<Box<Response>> {
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
                    result = Some((d.f)(&new_params, self.producer.get_new()));
                    break;
                }
            }
        }

        result
    }
}

fn remove_trailling_slash(route: &str) -> String {
    let mut r = route.to_string();
    let mut len = r.len() - 1u;
    while r.len() > 0 && r.as_bytes()[len] == '/' as u8 {
        r.pop_char();
        len -= 1;
    }
    r
}

// create a Vec which contains all the captures names for a route
fn create_vars_regex_vec(route: &str) -> Vec<String> {
    let vars_regex = create_vars_regex(route);
    let re = Regex::new(vars_regex.as_slice()).unwrap();
    match re.captures(route) {
        Some(c) => {
            let mut cap_i = c.iter();
            cap_i.next();
            cap_i.map(|x| {x.to_string()}).collect()
        },
        None => Vec::new()
    }
}

// create the regex to captures the vars in the route
fn create_vars_regex(r: &str) -> String {
    let mut var_reg: String = String::from_char(1, '^');
    var_reg = var_reg.append(RE_VAR.replace_all(r, ":([0-9a-zA-Z-_]+)").as_slice());
    var_reg = var_reg.replace("*", "\\*");
    var_reg.push_str("/??$");
    var_reg
}

// crete the matching regex to recognize the routes
fn create_match_regex(r: &str) -> Regex {
    let mut match_reg: String = String::from_char(1, '^');
    match_reg = match_reg.append(RE_VAR.replace_all(r, "([0-9a-zA-Z-_]+)").as_slice());
    match_reg = match_reg.replace("*", "[0-9a-zA-Z-_]*");
    match_reg.push_str("/??$");
    Regex::new(match_reg.as_slice()).unwrap()
}

impl<U, P> Show for Dispatcher<U, P> {
     fn fmt(&self, f: &mut Formatter) ->  Result<(), FormatError> {
        let mut to_write = String::from_str("Dispatcher {\n");
        for (&(ref r, m), _) in self.routes.iter() {
            to_write.push_str(format!("  {} {}\n", m, r).as_slice());
        }
        write!(f, "{}}}\n", to_write)
     }
}
