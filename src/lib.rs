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

#![crate_id = "github.com/JeremyLetang/web_dispatcher#web_dispatcher:0.0.1"]
#![desc = "web dispatcher for Rust"]
#![license = "mit"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![allow(dead_code)]
#![allow(visible_private_types)]
#![allow(missing_doc)]
#![allow(unused_variable)]
#![feature(macro_registrar, managed_boxes, quote)]
#![feature(macro_rules)]

extern crate syntax;

use std::local_data;

use syntax::ast;
use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast::{Ident, TokenTree, Expr, Name, ExprVec, MetaItem, MetaNameValue, LitStr, Item,
                  ItemFn};
use syntax::ext::base::{ExtCtxt, MacResult, SyntaxExtension, BasicMacroExpander, NormalTT,
                        ItemModifier, MacExpr};

// Store routes in a local data
static routes: local_data::Key<Vec<(Ident, ~str)>> = &local_data::Key;

#[macro_registrar]
pub fn registrar(register: |Name, SyntaxExtension|) {
    register(token::intern("route"), ItemModifier(expand_route));
    register(token::intern("get_routes"),
             NormalTT(~BasicMacroExpander {
                expander: expand_get_routes,
                span: None,
             },
             None));
}

fn expand_get_routes(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> ~MacResult {
    let v = local_data::get(routes, |d| {
        match d {
            Some(v) =>  v.clone(),
            None         => Vec::new()
        }
    });
    let v = v.iter().map(|&(f, ref s)| {
        quote_expr!(&*cx, ($f, $s))
    }).collect();
    let v = create_slice_expr(v, sp);
    MacExpr::new(quote_expr!(cx, $v))
}

fn create_slice_expr(vec: Vec<@Expr>, sp: Span) -> @Expr {
    @Expr {
        id: ast::DUMMY_NODE_ID,
        node: ExprVec(vec),
        span: sp
    }
}

pub fn get_attr_value(cx: &mut ExtCtxt, sp: Span, meta_item: @MetaItem, item: @Item) {
    match meta_item.node {
        MetaNameValue(ref s, ref l) => {
            match l.node {
                LitStr(ref s, _) => {
                    let mut v = local_data::get(routes, |d| {
                        match d {
                            Some(v) =>  v.clone(),
                            None => Vec::new()
                        }
                    });
                    for m in cx.mod_path.iter() {
                        println!("mod path: {}", token::get_ident(item.ident).get().to_owned());
                    }
                    v.push((item.ident, s.get().to_owned()));
                    local_data::set(routes, v);
                },
                _ => cx.span_err(sp, "route attribute can only use literal str")
            }
        }
        _ => cx.span_err(sp, "route attribute must be on the form: \
            #[route = \"my/route/\"] pub fn my_route()")
    }
}

fn expand_route(cx: &mut ExtCtxt, sp: Span, meta_item: @MetaItem, item: @Item) -> @Item {
    match item.node {
        ItemFn(ref decl, ref style, ref abi, ref generics, block) => {
            let function_name = token::get_ident(item.ident).get().to_owned();
            get_attr_value(cx, sp, meta_item, item);
            item
        },
        _ => {
            cx.span_err(sp, "route attribute can only be used on functions");
            item
        }
    }
}
