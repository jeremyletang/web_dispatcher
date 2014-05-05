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

use std::local_data;

use syntax::ast;
use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast::{Ident, TokenTree, Expr, Name, ExprVec, MetaItem, MetaNameValue, LitStr, Item,
                  ItemFn};
use syntax::ext::base::{ExtCtxt, MacResult, SyntaxExtension, BasicMacroExpander, NormalTT,
                        ItemModifier, MacExpr};

use syntax::ast::Path;
use syntax::ast::PathSegment;
use syntax::owned_slice::OwnedSlice;
use syntax::ast::ExprPath;

// Store routes in a local data (vector of path ident, associated route)
static routes: local_data::Key<Vec<(Vec<Ident>, ~str)>> = &local_data::Key;

fn local_data_get_or_init() -> Vec<(Vec<Ident>, ~str)> {
    local_data::get(routes, |d| {
        match d {
            Some(v) =>  v.clone(),
            None => Vec::new()
        }
    })
}

#[doc(hidden)]
#[macro_registrar]
pub fn registrar(register: |Name, SyntaxExtension|) {
    register(token::intern("route"), ItemModifier(expand_route));
    register(token::intern("method"), ItemModifier(expand_method));
    register(token::intern("get_routes"),
             NormalTT(~BasicMacroExpander {
                expander: expand_get_routes,
                span: None,
             },
             None));
}

fn expand_get_routes(cx: &mut ExtCtxt, sp: Span, _: &[TokenTree]) -> ~MacResult {
    let v = local_data_get_or_init();
    let v = v.iter().map(|&(ref f, ref s)| {
        let p = create_func_path_expr(f, sp);
        quote_expr!(&*cx, ($p, $s))
    }).collect();
    let v = create_slice_expr(v, sp);
    MacExpr::new(quote_expr!(cx, $v))
}

// create the path expression
fn create_func_path_expr(vec_ident: &Vec<Ident>, sp: Span) -> @Expr {
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
    @Expr {
        id: ast::DUMMY_NODE_ID,
        node: ExprPath(func_path),
        span: sp
    }
}

// create a slice from the vector of (path / routes)
fn create_slice_expr(vec: Vec<@Expr>, sp: Span) -> @Expr {
    @Expr {
        id: ast::DUMMY_NODE_ID,
        node: ExprVec(vec),
        span: sp
    }
}

fn expand_route(cx: &mut ExtCtxt, sp: Span, meta_item: @MetaItem, item: @Item) -> @Item {
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

fn get_route_attr_value(cx: &mut ExtCtxt, sp: Span, meta_item: @MetaItem, item: @Item) {
    match meta_item.node {
        MetaNameValue(_, ref l) => {
            match l.node {
                LitStr(ref s, _) => {
                    let mut v = local_data_get_or_init();
                    // retrieve the complete path of the function
                    let mut vec_ident = cx.mod_path.clone();
                    // concatenate the name of the function
                    vec_ident.push(item.ident);
                    // check if the route already exist.
                    if !route_already_exist(s.get().to_owned()) {
                        v.push((vec_ident, s.get().to_owned()));
                        local_data::set(routes, v);
                    } else {
                        cx.span_err(sp, "this route already exist for an other function")
                    }
                },
                _ => cx.span_err(sp, "route attribute can only use literal str")
            }
        }
        _ => cx.span_err(sp, "route attribute must be on the form: \
            #[route = \"my/route/\"] pub fn my_route()")
    }
}

// check if a route is already defined for an other function
fn route_already_exist(route: ~str) -> bool {
    let v = local_data_get_or_init();
    v.iter().fold(false, |b, &(_, ref s)| { b || s == &route })
}

fn expand_method(cx: &mut ExtCtxt, sp: Span, meta_item: @MetaItem, item: @Item) -> @Item {
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

fn get_method_attr_value(cx: &mut ExtCtxt, sp: Span, meta_item: @MetaItem, item: @Item) {
    match meta_item.node {
        MetaNameValue(_, ref l) => {
            match l.node {
                LitStr(ref s, _) => {
                    if is_method_attribute_valid(s.get()) {
                        //
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
    match attr {
        "GET" | "POST" => true,
        _ => false
    }
}
