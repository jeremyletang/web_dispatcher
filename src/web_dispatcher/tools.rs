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

//! Some tools types for routes and responses

use std::any::Any;

use collections::HashMap;
use response::Resp;

/// Function signature for a route
///
/// * `web_params` - the web parametre transmitted using GET or POST method +
/// maybe one or more urls fragment. e.g for url `/home/{my_var}/account`,
/// the url fragment int the place `{my_var}` will be inserted in the web_params
/// at the field `my_var`
///
/// * `user_params` - a custom user parameter
///
/// * `return` - Resp<T> the custom return value of the function
pub type RoutesFnType<T> = fn(web_params: HashMap<~str, ~str>, user_param: Box<Any>) -> Resp<T>;

/// Retrieve a given type from web params easily
///
///# example
///
///```Rust
/// let age: int = web_params.to_int("age").unwrap();
///```
pub trait WebParams {
    fn to_int(&self, &str)    -> Option<int>;
    fn to_i8(&self, &str)     -> Option<i8>;
    fn to_i16(&self, &str)    -> Option<i16>;
    fn to_i32(&self, &str)    -> Option<i32>;
    fn to_i64(&self, &str)    -> Option<i64>;
    fn to_uint(&self, &str)   -> Option<uint>;
    fn to_u8(&self, &str)     -> Option<u8>;
    fn to_u16(&self, &str)    -> Option<u16>;
    fn to_u32(&self, &str)    -> Option<u32>;
    fn to_u64(&self, &str)    -> Option<u64>;
    fn to_f32(&self, &str)    -> Option<f32>;
    fn to_f64(&self, &str)    -> Option<f64>;
    fn to_bool(&self, &str)   -> Option<bool>;
    fn to_string(&self, &str) -> Option<~str>;
}

macro_rules! to_type(
    ($p:expr) => (
        match self.find(&$p.to_owned()) {
            Some(pp) => from_str(*pp),
            None => None
        }
    )
)

impl WebParams for HashMap<~str, ~str> {
    fn to_int(&self, param_name: &str)    -> Option<int>  { to_type!(param_name) }
    fn to_i8(&self, param_name: &str)     -> Option<i8>   { to_type!(param_name) }
    fn to_i16(&self, param_name: &str)    -> Option<i16>  { to_type!(param_name) }
    fn to_i32(&self, param_name: &str)    -> Option<i32>  { to_type!(param_name) }
    fn to_i64(&self, param_name: &str)    -> Option<i64>  { to_type!(param_name) }
    fn to_uint(&self, param_name: &str)   -> Option<uint> { to_type!(param_name) }
    fn to_u8(&self, param_name: &str)     -> Option<u8>   { to_type!(param_name) }
    fn to_u16(&self, param_name: &str)    -> Option<u16>  { to_type!(param_name) }
    fn to_u32(&self, param_name: &str)    -> Option<u32>  { to_type!(param_name) }
    fn to_u64(&self, param_name: &str)    -> Option<u64>  { to_type!(param_name) }
    fn to_f32(&self, param_name: &str)    -> Option<f32>  { to_type!(param_name) }
    fn to_f64(&self, param_name: &str)    -> Option<f64>  { to_type!(param_name) }
    fn to_bool(&self, param_name: &str)   -> Option<bool> { to_type!(param_name) }
    fn to_string(&self, param_name: &str) -> Option<~str> { to_type!(param_name) }
}

/// The trait which should be implemented by structs who can product the user_params
pub trait Producer {
    /// Return a new instance of the user_params
    fn get_new(&self) -> Box<Any>;
}

#[doc(hidden)]
pub struct Dummy;

#[doc(hidden)]
#[deriving(Default)]
pub struct DummyProducer;

impl Producer for DummyProducer {
    fn get_new(&self) -> Box<Any> {
        box Dummy as Box<Any>
    }
}
