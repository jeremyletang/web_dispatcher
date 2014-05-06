web_dispatcher
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

Here is a simple a example of what am i doing:

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
pub fn default(p: HashMap<~str, ~str>, _: ~Any) -> Resp<~str> {
    Filled(format!("The name is: {}", p.to_string()))
}

fn main() {
    // Create a fill the webparams
    let mut params = HashMap::new();
    params.insert("name".to_owned(), "Paul".to_owned());

    // Create the web_dispatcher initialized with routes
    let mut dispatcher = Dispatcher::<~str>::new(routes!());

    // Dispatch and store the result
    let return_value = dispatcher.run("/some/route", HashMap::new()).unwrap();

    println!("{}", return_value)
}

```


limits
======

For the moment you can use only on prototype for your all programm when you use `libroute_macros`.

The web dispatcher is really naive for the moment, and can only handle route using this kinds
of functons: `fn(HashMap<~str, ~str>, ~Any) -> Resp<T>`.

