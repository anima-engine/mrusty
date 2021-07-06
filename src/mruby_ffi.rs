// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::any::Any;
use std::cell::RefCell;
use std::ffi::CStr;
use std::mem;
use std::os::raw::c_char;
use std::rc::Rc;

use super::MrubyError;

pub enum MrState {}
pub enum MrContext {}

pub enum MrClass {}
pub enum MrData {}

pub type MrFunc = extern "C" fn(*const MrState, MrValue) -> MrValue;

#[repr(C)]
pub struct MrDataType {
    pub name: *const c_char,
    pub free: extern "C" fn(*const MrState, *const u8),
}

/// Not meant to be called directly.
#[doc(hidden)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MrValue {
    pub value: u64,
    pub typ: MrType,
}

impl MrValue {
    pub unsafe fn nil() -> MrValue {
        mrb_ext_nil()
    }

    pub unsafe fn bool(value: bool) -> MrValue {
        if value {
            mrb_ext_true()
        } else {
            mrb_ext_false()
        }
    }

    pub unsafe fn fixnum(value: i32) -> MrValue {
        mrb_ext_cint_to_fixnum(value)
    }

    pub unsafe fn float(mrb: *const MrState, value: f64) -> MrValue {
        mrb_ext_cdouble_to_float(mrb, value)
    }

    pub unsafe fn string(mrb: *const MrState, value: &str) -> MrValue {
        mrb_str_new(mrb, value.as_ptr(), value.len())
    }

    pub unsafe fn symbol(mrb: *const MrState, value: &str) -> MrValue {
        mrb_ext_sym_new(mrb, value.as_ptr(), value.len())
    }

    pub unsafe fn obj<T: Any>(
        mrb: *const MrState,
        class: *const MrClass,
        obj: T,
        typ: &MrDataType,
    ) -> MrValue {
        let rc = Rc::new(RefCell::new(obj));
        let ptr: *const u8 = mem::transmute(rc);
        let data = mrb_data_object_alloc(mrb, class, ptr, typ as *const MrDataType);

        mrb_ext_data_value(data)
    }

    pub unsafe fn array(mrb: *const MrState, value: Vec<MrValue>) -> MrValue {
        let array = mrb_ary_new_capa(mrb, value.len() as i32);

        for (i, value) in value.iter().enumerate() {
            mrb_ary_set(mrb, array, i as i32, *value);
        }

        array
    }

    pub unsafe fn ptr(mrb: *const MrState, value: *const u8) -> MrValue {
        mrb_ext_set_ptr(mrb, value)
    }

    pub unsafe fn to_bool<'a>(&self) -> Result<bool, MrubyError> {
        match self.typ {
            MrType::MRB_TT_FALSE => Ok(false),
            MrType::MRB_TT_TRUE => Ok(true),
            _ => Err(MrubyError::Cast("TrueClass or FalseClass".to_owned())),
        }
    }

    pub unsafe fn to_i32(&self) -> Result<i32, MrubyError> {
        match self.typ {
            MrType::MRB_TT_INTEGER => Ok(mrb_ext_fixnum_to_cint(*self)),
            _ => Err(MrubyError::Cast("Fixnum".to_owned())),
        }
    }

    pub unsafe fn to_f64(&self) -> Result<f64, MrubyError> {
        match self.typ {
            MrType::MRB_TT_FLOAT => Ok(mrb_ext_float_to_cdouble(*self)),
            _ => Err(MrubyError::Cast("Float".to_owned())),
        }
    }

    pub unsafe fn to_str<'a>(&self, mrb: *const MrState) -> Result<&'a str, MrubyError> {
        match self.typ {
            MrType::MRB_TT_STRING => {
                let s = mrb_str_to_cstr(mrb, *self) as *const i8;

                Ok(CStr::from_ptr(s).to_str().unwrap().clone())
            }
            MrType::MRB_TT_SYMBOL => {
                let s = mrb_ext_sym2name(mrb, *self) as *const i8;

                Ok(CStr::from_ptr(s).to_str().unwrap().clone())
            }
            _ => Err(MrubyError::Cast("String".to_owned())),
        }
    }

    pub unsafe fn to_obj<T: Any>(
        &self,
        mrb: *const MrState,
        typ: &MrDataType,
    ) -> Result<Rc<RefCell<T>>, MrubyError> {
        match self.typ {
            MrType::MRB_TT_DATA => {
                let ptr = mrb_data_get_ptr(mrb, *self, typ as *const MrDataType) as *const u8;
                let rc: Rc<RefCell<T>> = mem::transmute(ptr);

                let result = Ok(rc.clone());

                mem::forget(rc);

                result
            }
            _ => Err(MrubyError::Cast("Data(Rust Rc<RefCell<T>>)".to_owned())),
        }
    }

    pub unsafe fn to_vec(&self, mrb: *const MrState) -> Result<Vec<MrValue>, MrubyError> {
        match self.typ {
            MrType::MRB_TT_ARRAY => {
                let len = mrb_ext_ary_len(mrb, *self) as usize;
                let mut vec = Vec::with_capacity(len);

                for i in 0..len {
                    vec.push(mrb_ary_ref(mrb, *self, i as i32));
                }

                Ok(vec)
            }
            _ => Err(MrubyError::Cast("Array".to_owned())),
        }
    }

    pub unsafe fn to_class(&self) -> Result<*const MrClass, MrubyError> {
        match self.typ {
            MrType::MRB_TT_CLASS => Ok(mrb_ext_get_class(*self)),
            _ => Err(MrubyError::Cast("Class".to_owned())),
        }
    }

    pub unsafe fn to_module(&self) -> Result<*const MrClass, MrubyError> {
        match self.typ {
            MrType::MRB_TT_MODULE => Ok(mrb_ext_get_class(*self)),
            _ => Err(MrubyError::Cast("Module".to_owned())),
        }
    }

    pub unsafe fn to_ptr(&self) -> Result<*const u8, MrubyError> {
        match self.typ {
            MrType::MRB_TT_CPTR => Ok(mrb_ext_get_ptr(*self)),
            _ => Err(MrubyError::Cast("Pointer".to_owned())),
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MrType {
    MRB_TT_FALSE = 0,
    MRB_TT_TRUE,
    MRB_TT_FLOAT,
    MRB_TT_INTEGER,
    MRB_TT_SYMBOL,
    MRB_TT_UNDEF,
    MRB_TT_CPTR,
    MRB_TT_FREE,
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
    MRB_TT_ENV,
    MRB_TT_DATA,
    MRB_TT_FIBER,
    MRB_TT_ISTRUCT,
    MRB_TT_BREAK,
    MRB_TT_MAXDEFINE,
}

extern "C" {
    pub fn mrb_open() -> *const MrState;
    pub fn mrb_close(mrb: *const MrState);

    pub fn mrb_ext_get_ud(mrb: *const MrState) -> *const u8;

    pub fn mrb_ext_set_ud(mrb: *const MrState, ud: *const u8);

    pub fn mrbc_context_new(mrb: *const MrState) -> *const MrContext;
    pub fn mrbc_context_free(mrb: *const MrState, ctx: *const MrContext);

    pub fn mrbc_filename(
        mrb: *const MrState,
        context: *const MrContext,
        filename: *const c_char,
    ) -> *const c_char;

    pub fn mrb_load_nstring_cxt(
        mrb: *const MrState,
        code: *const u8,
        len: i32,
        context: *const MrContext,
    ) -> MrValue;
    pub fn mrb_load_irep_cxt(
        mrb: *const MrState,
        code: *const u8,
        context: *const MrContext,
    ) -> MrValue;

    pub fn mrb_class_defined(mrb: *const MrState, name: *const c_char) -> bool;
    pub fn mrb_ext_class_defined_under(
        mrb: *const MrState,
        outer: *const MrClass,
        name: *const c_char,
    ) -> bool;

    pub fn mrb_class_get(mrb: *const MrState, name: *const c_char) -> *const MrClass;
    pub fn mrb_module_get(mrb: *const MrState, name: *const c_char) -> *const MrClass;
    pub fn mrb_class_get_under(
        mrb: *const MrState,
        outer: *const MrClass,
        name: *const c_char,
    ) -> *const MrClass;
    pub fn mrb_module_get_under(
        mrb: *const MrState,
        outer: *const MrClass,
        name: *const c_char,
    ) -> *const MrClass;

    pub fn mrb_define_class(
        mrb: *const MrState,
        name: *const c_char,
        sup: *const MrClass,
    ) -> *const MrClass;
    pub fn mrb_define_module(mrb: *const MrState, name: *const c_char) -> *const MrClass;
    pub fn mrb_define_class_under(
        mrb: *const MrState,
        outer: *const MrClass,
        name: *const c_char,
        sup: *const MrClass,
    ) -> *const MrClass;
    pub fn mrb_define_module_under(
        mrb: *const MrState,
        outer: *const MrClass,
        name: *const c_char,
    ) -> *const MrClass;

    pub fn mrb_include_module(mrb: *const MrState, class: *const MrClass, module: *const MrClass);

    pub fn mrb_define_const(
        mrb: *const MrState,
        class: *const MrClass,
        name: *const c_char,
        value: MrValue,
    );
    pub fn mrb_define_module_function(
        mrb: *const MrState,
        module: *const MrClass,
        name: *const c_char,
        fun: MrFunc,
        aspec: u32,
    );

    pub fn mrb_class_name(mrb: *const MrState, class: *const MrClass) -> *const c_char;
    pub fn mrb_ext_class_value(class: *const MrClass) -> MrValue;
    pub fn mrb_ext_module_value(module: *const MrClass) -> MrValue;

    pub fn mrb_define_method(
        mrb: *const MrState,
        class: *const MrClass,
        name: *const c_char,
        fun: MrFunc,
        aspec: u32,
    );
    pub fn mrb_define_class_method(
        mrb: *const MrState,
        class: *const MrClass,
        name: *const c_char,
        fun: MrFunc,
        aspec: u32,
    );

    pub fn mrb_protect(
        mrb: *const MrState,
        fun: MrFunc,
        data: MrValue,
        state: *const bool,
    ) -> MrValue;

    pub fn mrb_ext_class(mrb: *const MrState, value: MrValue) -> *const MrClass;

    pub fn mrb_get_args(mrb: *const MrState, format: *const c_char, ...);
    pub fn mrb_ext_get_mid(mrb: *const MrState) -> u32;

    pub fn mrb_intern(mrb: *const MrState, string: *const c_char, len: usize) -> u32;

    pub fn mrb_funcall_argv(
        mrb: *const MrState,
        object: MrValue,
        sym: u32,
        argc: i32,
        argv: *const MrValue,
    ) -> MrValue;

    pub fn mrb_iv_defined(mrb: *const MrState, object: MrValue, sym: u32) -> bool;

    pub fn mrb_iv_get(mrb: *const MrState, object: MrValue, sym: u32) -> MrValue;

    pub fn mrb_iv_set(mrb: *const MrState, object: MrValue, sym: u32, value: MrValue);

    pub fn mrb_ext_fixnum_to_cint(value: MrValue) -> i32;

    pub fn mrb_ext_float_to_cdouble(value: MrValue) -> f64;

    pub fn mrb_ext_nil() -> MrValue;

    pub fn mrb_ext_false() -> MrValue;

    pub fn mrb_ext_true() -> MrValue;

    pub fn mrb_ext_cint_to_fixnum(value: i32) -> MrValue;

    pub fn mrb_ext_cdouble_to_float(mrb: *const MrState, value: f64) -> MrValue;

    pub fn mrb_str_new(mrb: *const MrState, value: *const u8, len: usize) -> MrValue;

    pub fn mrb_ext_sym2name(mrb: *const MrState, value: MrValue) -> *const u8;

    pub fn mrb_ext_sym_new(mrb: *const MrState, value: *const u8, len: usize) -> MrValue;

    pub fn mrb_ext_get_ptr(value: MrValue) -> *const u8;

    pub fn mrb_ext_set_ptr(mrb: *const MrState, ptr: *const u8) -> MrValue;

    pub fn mrb_str_to_cstr(mrb: *const MrState, value: MrValue) -> *const c_char;

    pub fn mrb_data_object_alloc(
        mrb: *const MrState,
        class: *const MrClass,
        ptr: *const u8,
        typ: *const MrDataType,
    ) -> *const MrData;

    pub fn mrb_data_get_ptr(
        mrb: *const MrState,
        value: MrValue,
        typ: *const MrDataType,
    ) -> *const u8;

    pub fn mrb_ext_data_ptr(value: MrValue) -> *const u8;

    pub fn mrb_ext_data_init(value: *const MrValue, ptr: *const u8, typ: *const MrDataType);

    pub fn mrb_ext_set_instance_tt(class: *const MrClass, typ: MrType);

    pub fn mrb_ext_data_value(data: *const MrData) -> MrValue;

    pub fn mrb_ary_new_capa(mrb: *const MrState, size: i32) -> MrValue;

    pub fn mrb_ary_ref(mrb: *const MrState, array: MrValue, i: i32) -> MrValue;

    pub fn mrb_ary_set(mrb: *const MrState, array: MrValue, i: i32, value: MrValue);

    pub fn mrb_ext_ary_len(mrb: *const MrState, array: MrValue) -> i32;

    pub fn mrb_ext_raise(mrb: *const MrState, eclass: *const c_char, msg: *const c_char);

    pub fn mrb_ext_raise_current(mrb: *const MrState);

    pub fn mrb_ext_exc_str(mrb: *const MrState, exc: MrValue) -> MrValue;

    pub fn mrb_ext_get_class(class: MrValue) -> *const MrClass;
}

#[path = "tests/mruby_ffi.rs"]
#[cfg(test)]
mod tests;
