// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt::Display;

use super::mruby::*;
use super::read_line::ReadLine;

/// A `struct` that exposes an `Mruby` to a REPL.
///
/// # Examples
///
/// Using `GnuReadLine`, which requires the `gnu-readline` feature:
///
/// ```ignore
/// let mruby = Mruby::new();
/// let repl = Repl::new(mruby);
///
/// repl.start(&GnuReadLine);
/// ```
pub struct Repl {
    mruby: MrubyType,
    name: String,
}

impl Repl {
    /// Creates a new `Repl`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::Repl;
    /// let mruby = Mruby::new();
    /// let repl = Repl::new(mruby);
    /// ```
    pub fn new(mruby: MrubyType) -> Repl {
        Repl {
            mruby: mruby,
            name: "mrusty".to_owned(),
        }
    }

    /// Renames a `Repl`. The command line will start with `{name}> `.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::Repl;
    /// let mruby = Mruby::new();
    /// let mut repl = Repl::new(mruby);
    ///
    /// repl.rename("repl");
    /// ```
    pub fn rename(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    /// Starts a `Repl`.
    ///
    /// Use `'\'` for multiline statements.
    ///
    /// # Examples
    ///
    /// Using `GnuReadLine`, which requires the `gnu-readline` feature:
    ///
    /// ```ignore
    /// let mruby = Mruby::new();
    /// let repl = Repl::new(mruby);
    ///
    /// repl.start(&GnuReadLine);
    /// ```
    pub fn start<E: Display>(&self, read_line: &dyn ReadLine<E>) {
        let mut command = String::new();

        let single = self.name.clone() + "> ";
        let multi = self.name.clone() + "* ";

        loop {
            self.mruby.filename("repl");

            let head = if command.is_empty() { &single } else { &multi };

            let input = match read_line.read(head) {
                Ok(Some(s)) => s,
                Ok(None) => break,
                Err(e) => {
                    println!("{}", e);

                    break;
                }
            };

            if input.ends_with("\\") {
                let trimmed = input.trim_end_matches("\\");

                command = command + trimmed + "\n";
                read_line.add(&trimmed);

                continue;
            } else {
                read_line.add(&input);
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
                }
                Err(message) => {
                    println!("{}", message);
                }
            }

            if !command.is_empty() {
                command = String::new();
            }
        }

        println!("");
    }
}
