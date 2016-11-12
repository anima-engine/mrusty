#![feature(test)]

extern crate test;

#[macro_use]
extern crate mrusty;

use test::Bencher;

use mrusty::{Mruby, MrubyImpl};

#[bench]
fn convert_fixnum(b: &mut Bencher) {
    let mruby = Mruby::new();

    b.iter(|| {
        let one = mruby.fixnum(1);

        one.to_i32().unwrap()
    });
}

#[bench]
fn convert_string(b: &mut Bencher) {
    let mruby = Mruby::new();

    b.iter(|| {
        let string = mruby.string("hi");

        string.to_str().unwrap()
    });
}

#[bench]
fn convert_obj(b: &mut Bencher) {
    struct Cont;

    let mruby = Mruby::new();

    mruby.def_class_for::<Cont>("Container");

    b.iter(|| {
        let obj = mruby.obj(Cont);

        obj.to_obj::<Cont>().unwrap()
    });
}
