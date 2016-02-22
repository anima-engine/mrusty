// mrusty. mruby bindings for Rust
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

use std::cell::RefCell;
use std::rc::Rc;

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

    pub fn to_mruby(mruby: Rc<RefCell<MRuby>>) {
        mruby.def_class::<Scalar>("Scalar");

        mruby.def_method::<Scalar, _>("initialize", mrfn!(|mruby, slf, v: f64| {
            let scalar = Scalar::new(v as f32);

            slf.init(scalar)
        }));

        mruby.def_method::<Scalar, _>("value", mrfn!(|mruby, slf| {
            mruby.float(slf.to_obj::<Scalar>().unwrap().value as f64)
        }));

        mruby.def_method::<Scalar, _>("*", mrfn!(|mruby, slf, vector: Value| {
            let scalar = slf.to_obj::<Scalar>().unwrap();
            let vector = vector.to_obj::<Vector>().unwrap();

            mruby.obj((*scalar).clone() * (*vector).clone())
        }));
    }
}