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

#[macro_use]
extern crate mrusty;

use mrusty::*;

mod api;

use api::{Scalar, Vector};

#[test]
fn api_init() {
    let mruby = Mruby::new();

    Scalar::require(mruby.clone());
    Vector::require(mruby.clone());

    let scalar = mruby.run("Scalar.new 2.3").unwrap();
    let vector = mruby.run("Vector.new 1.0, 2.0, 3.0").unwrap();

    assert_eq!(*scalar.to_obj::<Scalar>().unwrap(), Scalar::new(2.3));
    assert_eq!(*vector.to_obj::<Vector>().unwrap(), Vector::new(1.0, 2.0, 3.0));
}

#[test]
fn api_getters() {
    let mruby = Mruby::new();

    Scalar::require(mruby.clone());
    Vector::require(mruby.clone());

    let scalar = mruby.run("Scalar.new 2.3").unwrap();
    let vector = mruby.run("Vector.new 1.0, 2.0, 3.0").unwrap();

    assert_eq!(scalar.to_obj::<Scalar>().unwrap().value, 2.3);

    assert_eq!(vector.to_obj::<Vector>().unwrap().x, 1.0);
    assert_eq!(vector.to_obj::<Vector>().unwrap().y, 2.0);
    assert_eq!(vector.to_obj::<Vector>().unwrap().z, 3.0);
}

#[test]
fn api_mul() {
    let mruby = Mruby::new();

    Scalar::require(mruby.clone());
    Vector::require(mruby.clone());

    let vector = mruby.run("Scalar.new(2.0) * Vector.new(1.0, 2.0, 3.0)").unwrap();

    assert_eq!(*vector.to_obj::<Vector>().unwrap(), Vector::new(2.0, 4.0, 6.0));
}

#[test]
fn api_array() {
    let mruby = Mruby::new();

    Scalar::require(mruby.clone());
    Vector::require(mruby.clone());

    let result = mruby.run("Vector.new(1.0, 2.0, 3.0).to_a.last").unwrap();

    assert_eq!(result.to_f64().unwrap(), 3.0);
}

#[test]
fn api_vec() {
    let mruby = Mruby::new();

    Scalar::require(mruby.clone());
    Vector::require(mruby.clone());

    let result = mruby.run("Vector.from_a [1.0, 2.0, 3.0]").unwrap();

    assert_eq!(*result.to_obj::<Vector>().unwrap(), Vector::new(1.0, 2.0, 3.0));
}

#[test]
fn api_require() {
    let mruby = Mruby::new();

    mruby.def_file::<Vector>("math");

    let result = mruby.run("
        require 'math'

        Vector.new(1.0, 2.0, 3.0)
    ").unwrap();

    assert_eq!(*result.to_obj::<Vector>().unwrap(), Vector::new(1.0, 2.0, 3.0));
}

#[test]
fn api_require_file() {
    use std::fs::File;
    use std::io::Write;

    let mruby = Mruby::new();

    let mut file = File::create("/tmp/some.rb").unwrap();

    file.write_all(b"class Some; end").unwrap();

    mruby.run("
        require '/tmp/some'

        Some.new
    ").unwrap();
}

#[test]
fn api_dup() {
    static mut DROPPED: bool = false;

    struct Cont {
        value: i32
    }

    impl Drop for Cont {
        fn drop(&mut self) {
            unsafe {
                DROPPED = true;
            }
        }
    }

    unsafe {
        {
            let mruby = Mruby::new();

            mruby.def_class::<Cont>("Container");

            {
                let orig = Cont { value: 3 };

                {
                    let obj = mruby.obj(orig);
                    let dup = obj.call("dup", vec![]).unwrap().to_obj::<Cont>().unwrap();

                    assert_eq!(dup.value, 3);

                    assert_eq!(DROPPED, false);
                }

                assert_eq!(DROPPED, false);
            }

            assert_eq!(DROPPED, false);
        }

        assert_eq!(DROPPED, true);
    }
}

describe!(Scalar, "
  context 'when zero' do
    let(:zero) { Scalar.new 0 }

    it 'returns 0 on #value' do
      expect(zero.value).to eql 0
    end

    it 'raises exception on #panic' do
      expect { zero.panic }.to raise_error RustPanic, 'I always panic.'
    end
  end
");
