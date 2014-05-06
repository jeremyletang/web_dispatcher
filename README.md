web_dispatcher
==============

Experiment with macros and web routes dispatch

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
use web_dispatcher::response::Resp;

#[route = "/"] // the route
pub fn default(_: HashMap<~str, ~str>, _: ~Any) -> Resp {
    // You can write some stuff here
    Resp::no()
}

#[method = "POST"] // explicit method (default is GET)
#[route = "/some/custom/route"] // again the route
pub fn other_route(_: HashMap<~str, ~str>, _: ~Any) -> Resp {
    // Write other stuff here
    Resp::empty()
}

fn main() {
    let mut dispatcher = Dispatcher::new(routes!());
    dispatcher.run("/", HashMap::new());
}

```