// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::thread::Thread;

pub fn main() {
    let mut result = Thread::scoped(child);
    println!("1");
    Thread::yield_now();
    println!("2");
    Thread::yield_now();
    println!("3");
    result.join();
}

fn child() {
    println!("4"); Thread::yield_now(); println!("5"); Thread::yield_now(); println!("6");
}
