web_dispatcher [![Build Status](https://travis-ci.org/jeremyletang/web_dispatcher.svg?branch=master)](https://travis-ci.org/jeremyletang/web_dispatcher)
==============

Experiments with syntax extensions and web routes dispatch

__web_dispatcher__ consist of a set of two libraries:
* `libroute_macros` a syntax extension library to handle routes
* `libweb_dispatcher` the web_dispatcher to dispatch routes

The syntax extension provide new attributes to retrieve data at compile time:

* `#[route = "/home"]` allow users to associate a route to a function
* `#[method = "GET"]` allow users to associate a given method to access the routes


example
=======

Here is a simple example of what am i doing:

```Rust
//! simple routes example
#![feature(phase)]

extern crate collections;
extern crate web_dispatcher;
#[phase(syntax, link)]
extern crate route_macros;

use std::any::Any;
use collections::HashMap;

use web_dispatcher::Dispatcher;
use web_dispatcher::tools::WebParams;
use web_dispatcher::response::{Resp, Filled};

#[route = "/some/route"]
pub fn default(p: HashMap<StrBuf, StrBuf>, _: ~Any) -> Resp<StrBuf> {
    Filled(format_strbuf!("The name is: {}", p.to_strbuf()))
}

fn main() {
    // Create and fill the webparams
    let mut params = HashMap::new();
    params.insert("name".to_strbuf(), "Paul".to_strbuf());

    // Create the web_dispatcher and initialize it with routes
    let mut dispatcher = Dispatcher::<StrBuf>::new(routes!());

    // Dispatch and store the result
    let return_value = dispatcher.run("/some/route", HashMap::new());

    println!("{}", return_value.unwrap())
}

```


limits
======

For the moment you can use only one prototype for all your programm when you use `libroute_macros`.

The web dispatcher is really naive for the moment, and can only handle routes using this kinds
of functions: `fn(HashMap<StrBuf, StrBuf>, Box<Amy>) -> Resp<T>`.

