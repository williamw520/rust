// xfail-fast

// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern mod extra;

use extra::getopts::{optopt, getopts};

pub fn main() {
    let args = ~[];
    let opts = ~[optopt("b")];

    match getopts(args, opts) {
        Ok(ref m)  =>
            assert!(!m.opt_present("b")),
        Err(ref f) => fail2!("{:?}", (*f).clone().to_err_msg())
    };

}
