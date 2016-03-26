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

//! # mrusty. mruby safe bindings for Rust
//!
//! mrusty lets you:
//!
//! * run Ruby 1.9 files with a very restricted API (without having to install Ruby)
//! * reflect Rust `struct`s and `enum`s in mruby and run them
//!
//! It does all this in a safely neat way while also bringing spec testing and a
//! REPL to the table.

#![feature(recover, std_panic)]

#[cfg(feature = "gnu-readline")]
extern crate rl_sys;

mod macros;
mod mruby;
mod mruby_ffi;
mod read_line;
mod spec;

mod repl;

/// Not meant to be called directly.
#[doc(hidden)]
pub use mruby_ffi::MrValue;
/// Not meant to be called directly.
#[doc(hidden)]
pub use mruby_ffi::mrb_get_args;

pub use mruby::Mruby;
pub use mruby::MrubyError;
pub use mruby::MrubyFile;
pub use mruby::MrubyImpl;
pub use mruby::MrubyType;
pub use mruby::Value;
pub use read_line::ReadLine;
pub use repl::Repl;
pub use spec::Spec;

#[cfg(feature = "gnu-readline")]
pub use read_line::GnuReadLine;
