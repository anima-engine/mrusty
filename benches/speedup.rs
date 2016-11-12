#![feature(test)]

extern crate test;

#[macro_use]
extern crate mrusty;

use test::Bencher;

use mrusty::{Mruby, MrubyImpl};

#[bench]
fn fib_rust(b: &mut Bencher) {
    fn fib(n: i32) -> i32 {
        if n <= 2 {
            1
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }

    b.iter(|| {
        let n = test::black_box(20);

        fib(n)
    });
}

#[bench]
fn fib_rusty(b: &mut Bencher) {
    let mruby = Mruby::new();

    fn fib(n: i32) -> i32 {
        if n <= 2 {
            1
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }

    mruby_class!(mruby, "Fib", {
        def_self!("fib", |mruby, _slf: Value, n: i32| {
            mruby.fixnum(fib(n))
        });
    });

    b.iter(|| {
        mruby.run("Fib.fib 20").unwrap()
    });
}

#[bench]
fn fib_ruby(b: &mut Bencher) {
    let mruby = Mruby::new();

    mruby.run("
      def fib(n)
        if n <= 2
          1
        else
          fib(n - 1) + fib(n - 2)
        end
      end
    ").unwrap();

    b.iter(|| {
        mruby.run("fib 20").unwrap()
    });
}
