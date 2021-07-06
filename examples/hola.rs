extern crate mrusty;

use mrusty::{Mruby, MrubyError, MrubyImpl};

fn main() {
    let mruby = Mruby::new();
    mruby.filename("script.rb");

    let result = mruby.run("1.nope");

    match result {
        Err(MrubyError::Runtime(err)) => {
            assert_eq!(err, "undefined method \'nope\' (NoMethodError)");
            println!("OK");
        }
        _ => assert!(false),
    }
}
