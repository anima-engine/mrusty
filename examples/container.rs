// mrclass!
#[macro_use]
extern crate mrusty;

// Needs some undocumented, hidden calls.
use mrusty::*;

fn main() {
    let mruby = Mruby::new();

    struct Cont {
        value: i32
    }

    // Cont should not flood the current namespace. We will add it with require.
    mrclass!(Cont, "Container", {
        // Converts mruby types automatically & safely.
        def!("initialize", |v: i32| {
            Cont { value: v }
        });

        // Converts slf to Cont.
        def!("value", |mruby, slf: Cont| {
            mruby.fixnum(slf.value)
        });
    });

    // Add file to the context, making it requirable.
    mruby.def_file::<Cont>("cont");

    let result = mruby.run("
      require 'cont'

      Container.new(3).value
    ").unwrap(); // Returns Value.

    println!("{}", result.to_i32().unwrap()); // Prints "3".
}
