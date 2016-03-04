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

use std::any::Any;
use std::ffi::CStr;
use std::mem;
use std::os::raw::c_char;
use std::rc::Rc;

use super::MRubyError;

pub enum MRState {}
pub enum MRContext {}

pub enum MRClass {}
pub enum MRData {}

pub type MRFunc = extern "C" fn(*const MRState, MRValue) -> MRValue;

#[repr(C)]
pub struct MRDataType {
    pub name: *const c_char,
    pub free: extern "C" fn(*const MRState, *const u8)
}

/// Not meant to be called directly.
#[doc(hidden)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MRValue {
    pub value: u64,
    pub typ: MRType,
}

impl MRValue {
    #[inline]
    pub unsafe fn nil() -> MRValue {
        mrb_ext_nil()
    }

    #[inline]
    pub unsafe fn bool(value: bool) -> MRValue {
        if value {
            mrb_ext_true()
        } else {
            mrb_ext_false()
        }
    }

    #[inline]
    pub unsafe fn fixnum(value: i32) -> MRValue {
        mrb_ext_cint_to_fixnum(value)
    }

    #[inline]
    pub unsafe fn float(mrb: *const MRState, value: f64) -> MRValue {
        mrb_ext_cdouble_to_float(mrb, value)
    }

    #[inline]
    pub unsafe fn string(mrb: *const MRState, value: &str) -> MRValue {
        mrb_str_new(mrb, value.as_ptr(), value.len())
    }

    #[inline]
    pub unsafe fn obj<T: Any>(mrb: *const MRState, class: *const MRClass,
                              obj: T, typ: &MRDataType) -> MRValue {
        let rc = Rc::new(obj);
        let ptr = mem::transmute::<Rc<T>, *const u8>(rc);
        let data = mrb_data_object_alloc(mrb, class, ptr, typ as *const MRDataType);

        mrb_ext_data_value(data)
    }

    #[inline]
    pub unsafe fn array(mrb: *const MRState, value: Vec<MRValue>) -> MRValue {
        let array = mrb_ary_new_capa(mrb, value.len() as i32);

        for (i, value) in value.iter().enumerate() {
            mrb_ary_set(mrb, array, i as i32, *value);
        }

        array
    }

    #[inline]
    pub unsafe fn to_bool<'a>(&self) -> Result<bool, MRubyError> {
        match self.typ {
            MRType::MRB_TT_FALSE => Ok(false),
            MRType::MRB_TT_TRUE  => Ok(true),
            _ => Err(MRubyError::Cast("TrueClass or FalseClass"))
        }
    }

    #[inline]
    pub unsafe fn to_i32(&self) -> Result<i32, MRubyError> {
        match self.typ {
            MRType::MRB_TT_FIXNUM => {
                Ok(mrb_ext_fixnum_to_cint(*self))
            },
            _ => Err(MRubyError::Cast("Fixnum"))
        }
    }

    #[inline]
    pub unsafe fn to_f64(&self) -> Result<f64, MRubyError> {
        match self.typ {
            MRType::MRB_TT_FLOAT => {
                Ok(mrb_ext_float_to_cdouble(*self))
            },
            _ => Err(MRubyError::Cast("Float"))
        }
    }

    #[inline]
    pub unsafe fn to_str<'a>(&self, mrb: *const MRState) -> Result<&'a str, MRubyError> {
        match self.typ {
            MRType::MRB_TT_STRING => {
                let s = mrb_str_to_cstr(mrb, *self) as *const i8;

                Ok(CStr::from_ptr(s).to_str().unwrap().clone())
            },
            _ => Err(MRubyError::Cast("String"))
        }
    }

    #[inline]
    pub unsafe fn to_obj<T: Any>(&self, mrb: *const MRState,
                                 typ: &MRDataType) -> Result<Rc<T>, MRubyError> {
        match self.typ {
            MRType::MRB_TT_DATA => {
                let ptr = mrb_data_get_ptr(mrb, *self, typ as *const MRDataType) as *const u8;
                let rc = mem::transmute::<*const u8, Rc<T>>(ptr);

                let result = Ok(rc.clone());

                mem::forget(rc);

                result
            },
            _ => Err(MRubyError::Cast("Data(Rust Rc)"))
        }
    }

    #[inline]
    pub unsafe fn to_vec(&self, mrb: *const MRState) -> Result<Vec<MRValue>, MRubyError> {
        match self.typ {
            MRType::MRB_TT_ARRAY => {
                let len = mrb_ext_ary_len(mrb, *self) as usize;
                let mut vec = Vec::with_capacity(len);

                for i in 0..len {
                    vec.push(mrb_ary_ref(mrb, *self, i as i32));
                }

                Ok(vec)
            },
            _ => Err(MRubyError::Cast("Array"))
        }
    }
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub fn mrb_open() -> *const MRState;
    pub fn mrb_close(mrb: *const MRState);

    #[inline]
    pub fn mrb_ext_get_ud(mrb: *const MRState) -> *const u8;
    #[inline]
    pub fn mrb_ext_set_ud(mrb: *const MRState, ud: *const u8);

    pub fn mrbc_context_new(mrb: *const MRState) -> *const MRContext;

    pub fn mrbc_filename(mrb: *const MRState, context: *const MRContext,
                         filename: *const c_char) -> *const c_char;

    pub fn mrb_load_nstring_cxt(mrb: *const MRState, code: *const u8, len: i32,
                                context: *const MRContext) -> MRValue;
    pub fn mrb_load_irep_cxt(mrb: *const MRState, code: *const u8,
                             context: *const MRContext) -> MRValue;

    pub fn mrb_class_get(mrb: *const MRState, name: *const c_char) -> *const MRClass;
    pub fn mrb_module_get(mrb: *const MRState, name: *const c_char) -> *const MRClass;

    pub fn mrb_define_class(mrb: *const MRState, name: *const c_char,
                            sup: *const MRClass) -> *const MRClass;
    pub fn mrb_define_module_function(mrb: *const MRState, module: *const MRClass,
                                      name: *const c_char, fun: MRFunc, aspec: u32);

    pub fn mrb_define_method(mrb: *const MRState, class: *const MRClass, name: *const c_char,
                             fun: MRFunc, aspec: u32);
    pub fn mrb_define_class_method(mrb: *const MRState, class: *const MRClass, name: *const c_char,
                                   fun: MRFunc, aspec: u32);

    pub fn mrb_get_args(mrb: *const MRState, format: *const c_char, ...);
    pub fn mrb_ext_get_mid(mrb: *const MRState) -> u32;

    pub fn mrb_intern(mrb: *const MRState, string: *const u8, len: usize) -> u32;

    pub fn mrb_funcall_argv(mrb: *const MRState, object: MRValue, sym: u32, argc: i32,
                            argv: *const MRValue) -> MRValue;

    #[inline]
    pub fn mrb_ext_fixnum_to_cint(value: MRValue) -> i32;
    #[inline]
    pub fn mrb_ext_float_to_cdouble(value: MRValue) -> f64;

    #[inline]
    pub fn mrb_ext_nil() -> MRValue;
    #[inline]
    pub fn mrb_ext_false() -> MRValue;
    #[inline]
    pub fn mrb_ext_true() -> MRValue;
    #[inline]
    pub fn mrb_ext_cint_to_fixnum(value: i32) -> MRValue;
    #[inline]
    pub fn mrb_ext_cdouble_to_float(mrb: *const MRState, value: f64) -> MRValue;
    #[inline]
    pub fn mrb_str_new(mrb: *const MRState, value: *const u8, len: usize) -> MRValue;

    #[inline]
    pub fn mrb_str_to_cstr(mrb: *const MRState, value: MRValue) -> *const c_char;

    #[inline]
    pub fn mrb_data_object_alloc(mrb: *const MRState, class: *const MRClass, ptr: *const u8,
                                 typ: *const MRDataType) -> *const MRData;
    #[inline]
    pub fn mrb_data_get_ptr(mrb: *const MRState, value: MRValue,
                            typ: *const MRDataType) -> *const u8;
    #[inline]
    pub fn mrb_ext_data_ptr(value: MRValue) -> *const u8;

    #[inline]
    pub fn mrb_ext_data_init(value: *const MRValue, ptr: *const u8, typ: *const MRDataType);
    #[inline]
    pub fn mrb_ext_set_instance_tt(class: *const MRClass, typ: MRType);
    #[inline]
    pub fn mrb_ext_data_value(data: *const MRData) -> MRValue;

    pub fn mrb_ary_new_capa(mrb: *const MRState, size: i32) -> MRValue;
    #[inline]
    pub fn mrb_ary_ref(mrb: *const MRState, array: MRValue, i: i32) -> MRValue;
    #[inline]
    pub fn mrb_ary_set(mrb: *const MRState, array: MRValue, i: i32, value: MRValue);
    #[inline]
    pub fn mrb_ext_ary_len(mrb: *const MRState, array: MRValue) -> i32;

    #[inline]
    pub fn mrb_ext_raise(mrb: *const MRState, msg: *const c_char);
    #[inline]
    pub fn mrb_ext_get_exc(mrb: *const MRState) -> MRValue;
}


#[path="tests/mruby_ffi.rs"]
#[cfg(test)]
mod tests;
