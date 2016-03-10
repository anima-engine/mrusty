// mrfn!
#[macro_use]
extern crate mrusty;

// Needs some undocumented, hidden calls.
use mrusty::*;

fn main() {
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

    let result = mruby.run("
        require 'cont'

        Container.new(3).value
    ").unwrap(); // Returns Value.

    println!("{}", result.to_i32().unwrap()); // Prints "3".
}
