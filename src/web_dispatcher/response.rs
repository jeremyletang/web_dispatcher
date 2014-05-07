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

use std::default::Default;

/// Responses returned by the web dispatcher
pub enum Resp<T> {
    /// The route is valid and the functon has filled the response with data
    Filled(T),
    /// The route is valid but the function don't returned nothing
    NoResp,
    /// The route is valid but an error has occured inside the user function
    InternalError(~str),
    /// The route is not valid this error is returned by the web dispatcher
    RoutingError(~str)
}

impl<T> Resp<T> {
    pub fn is_success(&self) -> bool {
        match *self {
            Filled(_)
            | NoResp => true,
            _        => false
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            Filled(t) => t,
            _         => fail!()
        }
    }
}

impl<T: Default> Default for Resp<T> {
    fn default() -> Resp<T> {
        Filled(Default::default())
    }
}
