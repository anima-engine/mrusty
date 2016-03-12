extern crate mrusty;

use mrusty::*;

// Should be compiled with --features gnu-readline.
fn main() {
    let mruby = MRuby::new();
    let repl = Repl::new(mruby);

    repl.start(&GnuReadLine);
}
