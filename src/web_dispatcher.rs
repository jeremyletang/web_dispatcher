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

//! A web dispatcher library for Rust

#![crate_name = "web_dispatcher"]
#![desc = "web dispatcher for Rust"]
#![license = "mit"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![experimental]
#![allow(missing_doc)]
#![feature(macro_rules)]
#![feature(default_type_params, phase)]

#[phase(plugin, link)]
extern crate regex_macros;
extern crate regex;
extern crate http;

pub use dispatcher::Dispatcher;
pub use tools::{WebParams, RoutesFnType, Producer};
pub use method::{Method, Get, Post, Head, Delete, Put, Connect};
pub use response::{Response, Request};

mod tools;
mod response;
mod method;
mod dispatcher;
