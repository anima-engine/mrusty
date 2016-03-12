// mrusty. mruby safe bindings for Rust
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

use std::fmt::Display;

#[cfg(feature = "gnu-readline")]
use rl_sys::readline;
#[cfg(feature = "gnu-readline")]
use rl_sys::history::{listmgmt, mgmt};
#[cfg(feature = "gnu-readline")]
use rl_sys::ReadlineError;

/// A `trait` which performs very basic readline utility.
pub trait ReadLine<Error: Display> {
    /// A function that reads one line from the terminal.
    fn read(&self, prompt: &str) -> Result<Option<String>, Error>;

    /// A function that add one line to the readline history.
    fn add(&self, line: &str);
}

/// A `struct` that implements `ReadLine` with very basic gnureadline functionality.
#[cfg(feature = "gnu-readline")]
pub struct GnuReadLine;

#[cfg(feature = "gnu-readline")]
impl ReadLine<ReadlineError> for GnuReadLine {
    fn read(&self, prompt: &str) -> Result<Option<String>, ReadlineError> {
        readline::readline(prompt)
    }

    fn add(&self, line: &str) {
        listmgmt::add(line).unwrap();
    }
}

#[cfg(feature = "gnu-readline")]
impl Drop for GnuReadLine {
    fn drop(&mut self) {
        mgmt::cleanup();
    }
}
