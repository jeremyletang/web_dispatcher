web_dispatcher
==============

Experiment with macros and web routes dispatch

Here is a simple a example of what am i doing:

```Rust

#[route = "/"] // the route
pub fn default(_: HashMap<~str, ~str>, db: ~Any) -> RespResult {
    // You can write some stuff here
}

#[method = "POST"] // explicit method (default is GET)
#[route = "/some/custom/route"] // again the route
pub fn other_route(_: HashMap<~str, ~str>, db: ~Any) -> RespResult {
    // Write other stuff here
}

fn main() {
    let routes = get_routes!();
}

```