# mrusty. mruby safe bindings for Rust
[![Build Status](https://travis-ci.org/anima-engine/mrusty.svg?branch=master)](https://travis-ci.org/anima-engine/mrusty)
[![Coverage Status](https://coveralls.io/repos/github/anima-engine/mrusty/badge.svg?branch=master)](https://coveralls.io/github/anima-engine/mrusty?branch=master)
[![Cargo Crate](http://meritbadge.herokuapp.com/mrusty)](https://crates.io/crates/mrusty)

mrusty lets you:

* run Ruby 1.9 files with a very restricted API (without having to install Ruby)
* reflect Rust `struct`s and `enum`s in mruby and run them

It does all this in a safely neat way, while also bringing spec testing and a
REPL to the table.

## [Documentation](http://anima-engine.github.io/mrusty/)

## Example
A very simple example of a Container `struct` which will be passed to mruby and
which is perfectly callable.
```rust
// mrusty_class!
#[macro_use]
extern crate mrusty;

use mrusty::{Mruby, MrubyImpl};

let mruby = Mruby::new();

struct Cont {
    value: i32
}

// Cont should not flood the current namespace. We will add it with require.
mrusty_class!(Cont, "Container", {
    // Converts mruby types automatically & safely.
    def!("initialize", |v: i32| {
        Cont { value: v }
    });

    // Converts slf to Cont.
    def!("value", |mruby, slf: (&Cont)| {
        mruby.fixnum(slf.value)
    });
});

// Add file to the context, making it requirable.
mruby.def_file::<Cont>("cont");

// Add spec testing.
describe!(Cont, "
  context 'when containing 1' do
    it 'returns 1 when calling #value' do
      expect(Container.new(1).value).to eql 1
    end
  end
");

let result = mruby.run("
  require 'cont'

  Container.new(3).value
").unwrap(); // Returns Value.

println!("{}", result.to_i32().unwrap()); // Prints "3".
```
