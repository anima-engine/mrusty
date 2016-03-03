# mrusty. mruby safe bindings for Rust
[![Build Status](https://travis-ci.org/anima-engine/mrusty.svg?branch=master)]
(https://travis-ci.org/anima-engine/mrusty)
[![Coverage Status]
(https://coveralls.io/repos/github/anima-engine/mrusty/badge.svg?branch=master)]
(https://coveralls.io/github/anima-engine/mrusty?branch=master)

mrusty lets you reflect Rust `struct`s and `enum`s in mruby and run them.
It does this in a safely neat way while also bringing spec testing and a REPL
to the table.


## Requirements
- [mruby](https://github.com/mruby/mruby)

mrusty requires mruby compiled with `-fPIC`. To compile and install mruby 1.2.0:
- make sure you have [Bison](https://www.gnu.org/software/bison/)
& [Ruby](https://www.ruby-lang.org/) installed
- download the [source](https://github.com/mruby/mruby/archive/1.2.0.zip)
- unzip and `cd` to `mruby-1.2.0/`
- add the following lines to `build_config.rb` as in the `# C compiler settings`
example:
```ruby
conf.cc do |cc|
    cc.flags << '-fPIC'
end
```
- run `./minirake`

## [Documentation](http://anima-engine.github.io/mrusty/)

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
}

// Cont should not flood the current namespace. We will add it with require.
impl MRubyFile for Cont {
    fn require(mruby: MRubyType) {
        mruby.def_class::<Cont>("Container");

        // Converts mruby types automatically & safely.
        // slf is always Value in initialize().
        mruby.def_method::<Cont, _>("initialize", mrfn!(|_mruby, slf: Value, v: i32| {
            let cont = Cont { value: v };

            slf.init(cont)
        }));
        mruby.def_method::<Cont, _>("value", mrfn!(|mruby, slf: Cont| {
            mruby.fixnum(slf.value)
        }));
    }
}

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
