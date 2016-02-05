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

mod mruby_ffi;

#[test]
fn test() {
    unsafe {
        let mrb = mruby_ffi::mrb_open();
        let context = mruby_ffi::mrbc_context_new(mrb);

        let code = "2".as_ptr();

        let value = mruby_ffi::mrb_load_string_cxt(mrb, code, context);

        assert_eq!(value.to_i32().unwrap(), 2);

        mruby_ffi::mrb_close(mrb);
    }
}
