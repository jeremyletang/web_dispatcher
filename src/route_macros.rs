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

//! A syntax extension to handle route attributes

#![crate_name = "route_macros"]
#![desc = "route macros for Rust"]
#![license = "mit"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![experimental]
#![allow(missing_doc, unused_variable)]
#![feature(plugin_registrar, managed_boxes, quote, phase)]

#[phase(plugin, link)]
extern crate regex_macros;
extern crate syntax;
extern crate rustc;
extern crate url;
extern crate regex;

use std::local_data;
use std::gc::{GC, Gc};
use std::ascii::OwnedStrAsciiExt;

use rustc::plugin::registry::Registry;

use syntax::ast;
use syntax::ast::Path;
use syntax::parse::token;
use syntax::ast::ExprPath;
use syntax::codemap::Span;
use syntax::ast::PathSegment;
use syntax::owned_slice::OwnedSlice;
use syntax::ast::{Ident,
                  TokenTree,
                  Expr,
                  ExprVec,
                  MetaItem,
                  MetaNameValue,
                  LitStr,
                  Item,
                  ItemFn};
use syntax::ext::base::{ExtCtxt,
                        MacResult,
                        ItemModifier,
                        MacExpr};

use regex::Regex;

static RE_VAR: Regex = regex!(":[0-9a-zA-Z-_]+");

// Store routes in a local data (vector of path ident, associated route, method as a string)
static routes: local_data::Key<Vec<(Vec<Ident>, String, String)>> = &local_data::Key;

fn local_data_get_or_init() -> Vec<(Vec<Ident>, String, String)> {
    match routes.get() {
        Some(v) =>  v.clone(),
        None => Vec::new()
    }
}

#[doc(hidden)]
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(token::intern("route"), ItemModifier(expand_route));
    reg.register_syntax_extension(token::intern("method"), ItemModifier(expand_method));
    reg.register_macro("routes", expand_get_routes);
}

fn expand_get_routes(cx: &mut ExtCtxt, sp: Span, _: &[TokenTree]) -> Box<MacResult> {
    let v = local_data_get_or_init();
    let v = v.iter().map(|&(ref f, ref r, ref m)| {
        let func_path = create_func_path_expr(f, sp);
        let _route = remove_trailling_slash(r.as_slice());
        let route = _route.as_slice();
        let method = m.as_slice();
        let vars = create_vars_regex_vec(cx, route, sp);
        let m_r = create_match_regex(route);
        let match_reg = m_r.as_slice();
        quote_expr!(&*cx, ($func_path, $route, $method, $vars.to_owned(), $match_reg))
    }).collect();
    let v = create_vec_expr(v, sp);
    // MacExpr::new(quote_expr!(cx, Vec::from_slice($v.to_owned())))
    MacExpr::new(quote_expr!(cx, $v.to_owned()))
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
fn create_vars_regex_vec(cx: &mut ExtCtxt, route: &str, sp: Span) -> Gc<Expr> {
    let vars_regex = create_vars_regex(route);
    let re = Regex::new(vars_regex.as_slice()).unwrap();
    let vars = match re.captures(route) {
        Some(c) => {
            let mut cap_i = c.iter();
            cap_i.next();
            cap_i.map(|x| {
                quote_expr!(&*cx, $x)
            }).collect()
        },
        None => Vec::new()
    };
    create_vec_expr(vars, sp)
}

// create a Vec from the vector of (path, (routes, method, vars, match_regex))
// or any other expr base on a Vec
fn create_vec_expr(vec: Vec<Gc<Expr>>, sp: Span) -> Gc<Expr> {
    box(GC) Expr {
        id: ast::DUMMY_NODE_ID,
        node: ExprVec(vec),
        span: sp
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
fn create_match_regex(r: &str) -> String {
    let mut match_reg: String = String::from_char(1, '^');
    match_reg = match_reg.append(RE_VAR.replace_all(r, "([0-9a-zA-Z-_]+)").as_slice());
    match_reg = match_reg.replace("*", "[0-9a-zA-Z-_]*");
    match_reg.push_str("/??$");
    match_reg
}

// create the path expression
fn create_func_path_expr(vec_ident: &Vec<Ident>, sp: Span) -> Gc<Expr> {
    // create the list of path segment from the idents
    let segs = vec_ident.iter().fold(Vec::new(), |mut v, &i| {
        v.push(PathSegment {
            identifier: i,
            lifetimes: Vec::new(),
            types: OwnedSlice::empty(),
        });
        v
    });
    // create the complete Path from the segments
    let func_path = Path {
        span: sp,
        global: false,
        segments: segs,
    };
    box(GC) Expr {
        id: ast::DUMMY_NODE_ID,
        node: ExprPath(func_path),
        span: sp
    }
}

fn expand_route(cx: &mut ExtCtxt,
                sp: Span,
                meta_item: Gc<MetaItem>,
                item: Gc<Item>)
                -> Gc<Item> {
    match item.node {
        ItemFn(_, _, _, _, _) => {
            get_route_attr_value(cx, sp, meta_item, item);
            item
        },
        _ => {
            cx.span_err(sp, "route attribute can only be used on functions");
            item
        }
    }
}

fn get_route_attr_value(cx: &mut ExtCtxt,
                        sp: Span,
                        meta_item: Gc<MetaItem>,
                        item: Gc<Item>) {
    match meta_item.node {
        MetaNameValue(_, ref l) => {
            match l.node {
                LitStr(ref s, _) => {
                    // check if the route is a valid url::Path
                    let validate: Option<url::Path> = from_str(s.get());
                    if validate.is_some() {
                        // check if the route already exist.
                        let route_attr = s.get().to_string();
                        if !route_already_exist(&route_attr) {
                            insert_route(cx, item, route_attr);
                        } else {
                            cx.span_err(sp, "this route already exist for an other function")
                        }
                    } else { // FIXME: check routes -> {my_var} + regex + encode
                        let route_attr = s.get().to_string();
                        insert_route(cx, item, route_attr);
                        // cx.span_err(sp, "this route is not a valid encoded route")
                    }
                },
                _ => cx.span_err(sp, "route attribute can only use literal str")
            }
        }
        _ => cx.span_err(sp, "route attribute must be on the form: \
            #[route = \"my/route/\"] pub fn my_route()")
    }
}

// insert the route in the local data
fn insert_route(cx: &mut ExtCtxt,
                item: Gc<Item>,
                route_attr: String) {
    let v = local_data_get_or_init();
    // retrieve the complete path of the function
    let mut vec_ident = cx.mod_path.clone();
    // concatenate the name of the function
    vec_ident.push(item.ident);
    // insert the route and save
    let mut method = "GET".to_string();
    let mut v: Vec<(Vec<Ident>, String, String)> = v.move_iter().filter(|&(ref v_i, _, ref m)| {
        if v_i == &vec_ident {
            method = m.clone();
            false
        } else { true }
    }).collect();
    v.push((vec_ident, route_attr, method));
    routes.replace(Some(v));
}

// check if a route is already defined for an other function
fn route_already_exist(route: &String) -> bool {
    let v = local_data_get_or_init();
    v.iter().fold(false, |b, &(_, ref s, _)| { b || s == route })
}

fn expand_method(cx: &mut ExtCtxt, sp: Span, meta_item: Gc<MetaItem>, item: Gc<Item>) -> Gc<Item> {
    match item.node {
        ItemFn(_, _, _, _, _) => {
            get_method_attr_value(cx, sp, meta_item, item);
            item
        },
        _ => {
            cx.span_err(sp, "method attribute can only be used on functions");
            item
        }
    }
}

fn get_method_attr_value(cx: &mut ExtCtxt,
                         sp: Span,
                         meta_item: Gc<MetaItem>,
                         item: Gc<Item>) {
    match meta_item.node {
        MetaNameValue(_, ref l) => {
            match l.node {
                LitStr(ref s, _) => {
                    let method_attr = s.get();
                    if is_method_attribute_valid(method_attr) {
                        insert_method(cx, item, method_attr.to_string());
                    } else {
                        cx.span_err(sp, "this method attribut don't exist. Here is a list of \
                            available attribute: [GET, POST]")
                    }
                },
                _ => cx.span_err(sp, "method attribute can only use literal str")
            }
        }
        _ => cx.span_err(sp, "method attribute must be on the form: \
            #[method = \"GET\"] pub fn my_route()")
    }
}

fn is_method_attribute_valid(attr: &str) -> bool {
    match attr.to_string().into_ascii_upper().as_slice() {
        "GET"
        | "POST"
        | "HEAD"
        | "PUT"
        | "CONNECT"
        | "DELETE" => true,
        _ => false
    }
}

// insert the method in the local data
fn insert_method(cx: &mut ExtCtxt,
                 item: Gc<Item>,
                 method_attr: String) {
    let v = local_data_get_or_init();
    // retrieve the complete path of the function
    let mut vec_ident = cx.mod_path.clone();
    // concatenate the name of the function
    vec_ident.push(item.ident);
    // insert the route and save
    let mut route = "".to_string();
    let mut v: Vec<(Vec<Ident>, String, String)> = v.move_iter().filter(|&(ref v_i, ref r, _)| {
        if v_i == &vec_ident {
            route = r.clone();
            false
        } else { true }
    }).collect();
    v.push((vec_ident, route, method_attr));
    routes.replace(Some(v));
}
