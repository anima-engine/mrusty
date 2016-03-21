// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

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
