# mrusty. mruby safe bindings for Rust
[![Build Status](https://travis-ci.org/dragostis/mrusty.svg?branch=master)](https://travis-ci.org/dragostis/mrusty)
[![Coverage Status](https://coveralls.io/repos/github/dragostis/mrusty/badge.svg?branch=master)](https://coveralls.io/github/dragostis/mrusty?branch=master)


## Requirements
- [mruby](https://github.com/mruby/mruby)

mrusty requires mruby compiled with fPIC. To compile and install mruby 1.2.0:
- make sure you have [Bison](https://www.gnu.org/software/bison/) & [Ruby](https://www.ruby-lang.org/) installed
- download the [source](https://github.com/mruby/mruby/archive/1.2.0.zip)
- unzip and `cd` to `mruby-1.2.0/`
- add the following lines to `build_config.rb` as in the `# C compiler settings` example:
```ruby
conf.cc do |cc|
    cc.flags << '-fPIC'
end
```
- run `./minirake`

## Example
A very simple example of a Container `struct` which will be passed to mruby and
which is perfectly callable.
```rust
// mrfn!
#[macro_use]
extern crate mrusty;

// Needs some undocumented, hidden calls.
use mrusty::*;

let mruby = MRuby::new();

struct Cont {
    value: i32
};

mruby.def_class::<Cont>("Container");

// Converts mruby types automatically & safely.
mruby.def_method::<Cont, _>("initialize", mrfn!(|mruby, slf: Value, v: i32| { // slf is always Value in initialize().
    let cont = Cont { value: v };

    slf.init(cont)
}));
mruby.def_method::<Cont, _>("value", mrfn!(|mruby, slf: Cont| {
    mruby.fixnum(slf.value)
}));

let result = mruby.run("Container.new(3).value").unwrap(); // Returns Value.

println!("{}", result.to_i32().unwrap()); // Prints "3".
```
