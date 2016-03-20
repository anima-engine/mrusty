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
//! ## Requirements
//! * [mruby](https://github.com/mruby/mruby)
//!
//! mrusty requires mruby compiled with `fPIC`. To compile and install mruby 1.2.0:
//!
//! * make sure you have [Bison](https://www.gnu.org/software/bison/)
//! & [Ruby](https://www.ruby-lang.org/) installed
//! * download the [source](https://github.com/mruby/mruby/archive/1.2.0.zip)
//! * unzip and `cd` to `mruby-1.2.0/`
//! * add the following lines to `build_config.rb` as in the `# C compiler settings`
//! example (make sure you add it after the comment):
//! ```ruby
//! conf.cc do |cc|
//!   cc.flags << '-fPIC'
//! end
//! ```
//! * run `./minirake`
//! * copy header files from `include` to `/usr/loca/include`
//! * copy `build/host/lib/libmruby.a` to `/usr/local/lib`
//!
//! *Note:* On OSX you can install it from homebrew with `brew install mruby`.

#![feature(recover, std_panic)]

#[cfg(feature = "gnu-readline")]
extern crate rl_sys;

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
