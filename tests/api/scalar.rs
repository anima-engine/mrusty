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

impl MrubyFile for Scalar {
    fn require(mruby: MrubyType) {
        mruby.def_class::<Scalar>("Scalar");

        mruby.def_method::<Scalar, _>("initialize", mrfn!(|_mruby, slf: Value, v: f64| {
            let scalar = Scalar::new(v as f32);

            slf.init(scalar)
        }));

        mruby.def_method::<Scalar, _>("value", mrfn!(|mruby, slf: Scalar| {
            mruby.float(slf.value as f64)
        }));

        mruby.def_method::<Scalar, _>("*", mrfn!(|mruby, slf: Scalar, vector: Vector| {
            mruby.obj((*slf).clone() * (*vector).clone())
        }));

        mruby.def_method::<Scalar, _>("panic", mrfn!(|_mruby, _slf: Scalar| {
            panic!("I always panic.");
        }));
    }
}
