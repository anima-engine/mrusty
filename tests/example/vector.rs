// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use mrusty::MrubyImpl;

use super::Scalar;

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
}

mrusty_class!(Vector, {
    def!("initialize", |x: f64, y: f64, z: f64| {
        Vector::new(x as f32, y as f32, z as f32)
    });

    def_self!("from_a", |slf: Value, array: (Vec<Value>)| {
        slf.call_unchecked("new", array)
    });

    def!("x", |mruby, slf: (&Vector)| {
        mruby.float(slf.x as f64)
    });

    def!("y", |mruby, slf: (&Vector)| {
        mruby.float(slf.y as f64)
    });

    def!("z", |mruby, slf: (&Vector)| {
        mruby.float(slf.z as f64)
    });

    def!("to_a", |mruby, slf: (&Vector)| {
        mruby.array(vec![
            mruby.float(slf.x as f64),
            mruby.float(slf.y as f64),
            mruby.float(slf.z as f64)
        ])
    });
});

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
