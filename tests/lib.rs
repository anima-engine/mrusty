// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_use]
extern crate mrusty;

use std::path::Path;

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

            mruby.def_class_for::<Cont>("Container");

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

#[test]
fn api_execute_binary() {
    let mruby = Mruby::new();

    Scalar::require(mruby.clone());

    let result = mruby.execute(Path::new("tests/compiled.mrb")).unwrap();

    assert_eq!(*result.to_obj::<Scalar>().unwrap(), Scalar::new(2.0));
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

    it 'raises exception on #raise' do
      expect { zero.raise }.to raise_error RuntimeError, 'Except me.'
    end
  end
");
