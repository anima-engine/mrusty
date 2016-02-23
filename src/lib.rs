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

//! # mrusty. mruby safe bindings for Rust
//!
//! ## Requirements
//! * [mruby](https://github.com/mruby/mruby)
//!
//! mrusty requires mruby compiled with fPIC. To compile and install mruby 1.2.0:
//!
//! * download the [source](https://github.com/mruby/mruby/archive/1.2.0.zip)
//! * unzip and `cd` to `mruby/`
//! * add the following lines to `build_config.rb` as in the `# C compiler settings` example:
//!
//! ```ruby
//! conf.cc do |cc|
//!     cc.flags << '-fPIC'
//! end
//! ```
//! * run `./minirake`

mod mruby;
mod mruby_ffi;

/// Not meant to be called directly.
#[doc(hidden)]
pub use mruby_ffi::MRValue;
/// Not meant to be called directly.
#[doc(hidden)]
pub use mruby_ffi::mrb_get_args;

pub use mruby::MRuby;
pub use mruby::MRubyImpl;
pub use mruby::Value;
