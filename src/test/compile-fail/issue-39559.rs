// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait Dim {
    fn dim() -> usize;
}

enum Dim3 {}

impl Dim for Dim3 {
    fn dim() -> usize {
        3
    }
}

pub struct Vector<T, D: Dim> {
    entries: [T; D::dim()]
    //~^ ERROR cannot use an outer type parameter in this context
}

fn main() {
    let array: [usize; Dim3::dim()]
    //~^ ERROR calls in constants are limited to constant functions
        = [0; Dim3::dim()];
        //~^ ERROR calls in constants are limited to constant functions
}
