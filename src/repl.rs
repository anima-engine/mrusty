// mrusty. mruby bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use rl_sys::readline;
use rl_sys::history::{listmgmt, mgmt};

use super::mruby::*;

/// A `struct` that exposes an `MRuby` to a REPL. Requires `repl` build feature.
///
/// # Examples
///
/// ```no-run
/// # use mrusty::MRuby;
/// # use mrusty::Repl;
/// let mruby = MRuby::new();
/// let repl = Repl::new(mruby);
///
/// repl.start();
/// ```
#[cfg(feature = "repl")]
pub struct Repl {
    mruby: MRubyType
}

impl Repl {
    /// Creates a new `Repl`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::Repl;
    /// let mruby = MRuby::new();
    /// let repl = Repl::new(mruby);
    /// ```
    pub fn new(mruby: MRubyType) -> Repl {
        mruby.filename("repl");

        Repl {
            mruby: mruby
        }
    }

    /// Starts a `Repl`.
    ///
    /// Use `'\'` for multiline statements.
    ///
    /// # Examples
    ///
    /// ```no-run
    /// # use mrusty::MRuby;
    /// # use mrusty::Repl;
    /// let mruby = MRuby::new();
    /// let repl = Repl::new(mruby);
    ///
    /// repl.start();
    /// ```
    pub fn start(&self) {
        let mut command = String::new();

        loop {
            let head = if command.is_empty() {
                "mrusty> "
            } else {
                "mrusty* "
            };

            let input = match readline::readline(head) {
                Ok(Some(s)) => s,
                Ok(None) => break,
                Err(e) => {
                    println!("{}", e);

                    break
                }
            };

            if input.ends_with("\\") {
                let trimmed = input.trim_right_matches("\\");

                command = command + trimmed + "\n";
                listmgmt::add(&trimmed).unwrap();

                continue
            } else {
                listmgmt::add(&input).unwrap();
            }

            if command == "" {
                command = input;
            } else {
                command = command + &input;
            }

            match self.mruby.run(&command) {
                Ok(value) => {
                    let result = value.call("to_s", vec![]).unwrap().to_str().unwrap();

                    println!("{}", result);
                },
                Err(message) => {
                    println!("{}", message);
                }
            }

            if !command.is_empty() {
                command = String::new();
            }
        }

        mgmt::cleanup();

        println!("");
    }
}
