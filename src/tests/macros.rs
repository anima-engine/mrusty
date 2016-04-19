// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::super::*;

#[test]
fn mrusty_initialize_empty() {
    let mruby = Mruby::new();

    struct Cont;

    mrusty_class!(Cont, "Container", {
        def!("initialize", || {
            Cont
        });
    });

    Cont::require(mruby.clone());

    mruby.run("Container.new").unwrap();
}

#[test]
fn mrusty_initialize_only_values() {
    let mruby = Mruby::new();

    struct Cont;

    mrusty_class!(Cont, "Container", {
        def!("initialize", |_v: i32| {
            Cont
        });
    });

    Cont::require(mruby.clone());

    mruby.run("Container.new 3").unwrap();
}

#[test]
fn mrusty_initialize_only_mruby() {
    let mruby = Mruby::new();

    struct Cont;

    mrusty_class!(Cont, "Container", {
        def!("initialize", |_mruby| {
            Cont
        });
    });

    Cont::require(mruby.clone());

    mruby.run("Container.new").unwrap();
}

#[test]
fn mrusty_initialize_mruby_values() {
    let mruby = Mruby::new();

    struct Cont;

    mrusty_class!(Cont, "Container", {
        def!("initialize", |_mruby, _v: i32| {
            Cont
        });
    });

    Cont::require(mruby.clone());

    mruby.run("Container.new 3").unwrap();
}

#[test]
fn mrusty_instance_empty() {
    let mruby = Mruby::new();

    struct Cont;

    mrusty_class!(Cont, "Container", {
        def!("hi", |slf: Value| {
            slf
        });
    });

    Cont::require(mruby.clone());

    mruby.run("Container.new.hi").unwrap();
}

#[test]
fn mrusty_instance_only_values() {
    let mruby = Mruby::new();

    struct Cont;

    mrusty_class!(Cont, "Container", {
        def!("hi", |slf: Value, _v: i32| {
            slf
        });
    });

    Cont::require(mruby.clone());

    mruby.run("Container.new.hi 3").unwrap();
}

#[test]
fn mrusty_instance_only_mruby() {
    let mruby = Mruby::new();

    struct Cont;

    mrusty_class!(Cont, "Container", {
        def!("hi", |_mruby, slf: Value| {
            slf
        });
    });

    Cont::require(mruby.clone());

    mruby.run("Container.new.hi").unwrap();
}

#[test]
fn mrusty_instance_mruby_values() {
    let mruby = Mruby::new();

    struct Cont;

    mrusty_class!(Cont, "Container", {
        def!("hi", |mruby, _slf: Value, v: i32| {
            mruby.fixnum(v)
        });
    });

    Cont::require(mruby.clone());

    let result = mruby.run("Container.new.hi 3").unwrap();

    assert_eq!(result.to_i32().unwrap(), 3);
}

#[test]
fn mrusty_class_empty() {
    let mruby = Mruby::new();

    struct Cont;

    mrusty_class!(Cont, "Container", {
        def_self!("hi", |slf: Class| {
            slf.to_value()
        });
    });

    Cont::require(mruby.clone());

    let result = mruby.run("Container.hi == Container").unwrap();

    assert_eq!(result.to_bool().unwrap(), true);
}

#[test]
fn mrusty_class_only_values() {
    let mruby = Mruby::new();

    struct Cont;

    mrusty_class!(Cont, "Container", {
        def_self!("hi", |_slf: Class, other: Class| {
            other.to_value()
        });
    });

    Cont::require(mruby.clone());

    let result = mruby.run("Container.hi(Fixnum) == Fixnum").unwrap();

    assert_eq!(result.to_bool().unwrap(), true);
}

#[test]
fn mrusty_class_only_mruby() {
    let mruby = Mruby::new();

    struct Cont;

    mrusty_class!(Cont, "Container", {
        def_self!("hi", |_mruby, slf: Class| {
            slf.to_value()
        });
    });

    Cont::require(mruby.clone());

    let result = mruby.run("Container.hi == Container").unwrap();

    assert_eq!(result.to_bool().unwrap(), true);
}

#[test]
fn mrusty_class_mruby_values() {
    let mruby = Mruby::new();

    struct Cont;

    mrusty_class!(Cont, "Container", {
        def_self!("hi", |_mruby, _slf: Class, other: Class| {
            other.to_value()
        });
    });

    Cont::require(mruby.clone());

    let result = mruby.run("Container.hi(Fixnum) == Fixnum").unwrap();

    assert_eq!(result.to_bool().unwrap(), true);
}
