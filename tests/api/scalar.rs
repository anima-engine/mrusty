// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use mrusty::*;

use api::Vector;

#[derive(Clone, Debug, PartialEq)]
pub struct Scalar {
    pub value: f32
}

impl Scalar {
    pub fn new(value: f32) -> Scalar {
        Scalar {
            value: value
        }
    }
}

mrclass!(Scalar, {
    def!("initialize", |v: f64| {
        Scalar::new(v as f32)
    });

    def!("value", |mruby, slf: Scalar| {
        mruby.float(slf.value as f64)
    });

    def!("*", |mruby, slf: Scalar, vector: Vector| {
        mruby.obj((*slf).clone() * (*vector).clone())
    });

    def!("panic", |_slf: Scalar| {
        panic!("I always panic.");
    });

    def!("raise", |mruby, _slf: Scalar| {
        mruby.raise("RuntimeError", "Except me.")
    });
});
