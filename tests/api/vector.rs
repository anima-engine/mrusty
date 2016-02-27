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

use mrusty::*;

use api::Scalar;

#[derive(Clone, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn to_mruby(mruby: MRubyType) {
        mruby.def_class::<Vector>("Vector");

        mruby.def_method::<Vector, _>("initialize", mrfn!(|_mruby, slf: Value,
                                                           x: f64, y: f64, z: f64| {
            let vector = Vector::new(x as f32, y as f32, z as f32);

            slf.init(vector)
        }));

        mruby.def_class_method::<Vector, _>("from_a", mrfn!(|_mruby, slf: Value, array: Vec| {
            slf.call_unchecked("new", array)
        }));
        
        mruby.def_method::<Vector, _>("x", mrfn!(|mruby, slf: Vector| {
            mruby.float(slf.x as f64)
        }));

        mruby.def_method::<Vector, _>("y", mrfn!(|mruby, slf: Vector| {
            mruby.float(slf.y as f64)
        }));

        mruby.def_method::<Vector, _>("z", mrfn!(|mruby, slf: Vector| {
            mruby.float(slf.z as f64)
        }));

        mruby.def_method::<Vector, _>("to_a", mrfn!(|mruby, slf: Vector| {
            mruby.array(vec![
                mruby.float(slf.x as f64),
                mruby.float(slf.y as f64),
                mruby.float(slf.z as f64)
            ])
        }));
    }
}

use std::ops::Mul;

impl Mul<Vector> for Scalar {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        Vector {
            x: vector.x * self.value,
            y: vector.y * self.value,
            z: vector.z * self.value
        }
    }
}
