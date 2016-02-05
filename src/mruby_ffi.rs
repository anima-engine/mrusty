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

extern crate libc;
extern crate byteorder;

use std::io::Cursor;

use self::libc::c_char;

use self::byteorder::{LittleEndian, ReadBytesExt};

pub enum MRState {}
pub enum MRContext {}
pub enum MRParser {}

pub enum MRProc {}

#[repr(C)]
pub struct MRValue {
    value: [u8; 8],
    typ: MRType
}

impl MRValue {
    pub unsafe fn to_i32(&self) -> Result<i32, &str> {
        match self.typ {
            MRType::MRB_TT_FIXNUM => {
                let mut rdr = Cursor::new(self.value);

                Ok(rdr.read_i32::<LittleEndian>().unwrap())
            },
            _ => Err("Value must be Fixnum.")
        }
    }
}

#[repr(C)]
pub enum MRType {
    MRB_TT_FALSE,
    MRB_TT_FREE,
    MRB_TT_TRUE,
    MRB_TT_FIXNUM,
    MRB_TT_SYMBOL,
    MRB_TT_UNDEF,
    MRB_TT_FLOAT,
    MRB_TT_CPTR,
    MRB_TT_OBJECT,
    MRB_TT_CLASS,
    MRB_TT_MODULE,
    MRB_TT_ICLASS,
    MRB_TT_SCLASS,
    MRB_TT_PROC,
    MRB_TT_ARRAY,
    MRB_TT_HASH,
    MRB_TT_STRING,
    MRB_TT_RANGE,
    MRB_TT_EXCEPTION,
    MRB_TT_FILE,
    MRB_TT_ENV,
    MRB_TT_DATA,
    MRB_TT_FIBER,
    MRB_TT_MAXDEFINE
}

#[link(name = "mruby")]
extern "C" {
    pub fn mrb_open() -> *mut MRState;
    pub fn mrb_close(mrb: *mut MRState);

    pub fn mrbc_context_new(mrb: *mut MRState) -> *mut MRContext;

    pub fn mrbc_filename(mrb: *mut MRState, context: *mut MRContext, filename: *const u8) -> &[c_char];

    pub fn mrb_parse_string(mrb: *mut MRState, code: *const u8, context: *mut MRContext) -> *mut MRParser;
    pub fn mrb_generate_code(mrb: *mut MRState, parser: *mut MRParser) -> *mut MRProc;

    pub fn mrb_load_string_cxt(mrb: *mut MRState, code: *const u8, context: *mut MRContext) -> MRValue;
}
