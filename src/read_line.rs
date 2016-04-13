// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
///  Requires `gnu-readline` build feature.
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
