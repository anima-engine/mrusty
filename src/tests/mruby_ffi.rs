// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ffi::{CStr, CString};

use super::*;

#[test]
fn open_close() {
    unsafe {
        let mrb = mrb_open();

        mrb_close(mrb);
    }
}

#[test]
fn ud() {
    use std::mem;

    unsafe {
        let mrb = mrb_open();

        let n = &1;

        mrb_ext_set_ud(mrb, mem::transmute::<&i32, *const u8>(n));
        let n: &i32 = mem::transmute(mrb_ext_get_ud(mrb));

        assert_eq!(*n, 1);

        mrb_close(mrb);
    }
}

#[test]
fn exec_bin_context() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let bin = [
            // struct rite_binary_header
            69u8, 84u8, 73u8, 82u8, // Binary identifier "RITE"
            48u8, 48u8, 48u8, 54u8, // Binary format version "0006"
            204u8, 148u8,           // Binary CRC
            0u8, 0u8, 0u8, 69u8,    // Binary size
            77u8, 65u8, 84u8, 90u8, // Compiler name "MATZ"
            48u8, 48u8, 48u8, 48u8, // Compiler version "0000"

            // struct rite_section_irep_header
            73u8, 82u8, 69u8, 80u8, // section_ident "IREP"
            0u8, 0u8, 0u8, 39u8,    // section_size
            48u8, 48u8, 48u8, 50u8, // rite_version "0002"

            // irep record
            0u8, 0u8, 0u8, 46u8,    // record size
            0u8, 1u8,               // number of local variable
            0u8, 2u8,               // number of register variable
            0u8, 0u8,               // number of child irep

            // ISEQ BLOCK
            0u8, 0u8, 0u8, 5u8,     // number of iseq
            // skip_padding target(nothing)
            8u8, 1u8, 55u8, 1u8, 103u8, // mrb_code * number of iseq

            // POOL BLOCK
            0u8, 0u8, 0u8, 0u8,     // number of pool

            // SYMS BLOCK
            0u8, 0u8, 0u8, 0u8, // syms length

            69u8, 78u8, 68u8, 0u8, // RITE_BINARY_EOF "END\0"
            0u8, 0u8, 0u8, 8u8];

        let result = mrb_ext_load_irep_cxt_suppress_alignment(mrb, bin.as_ptr(), context);

        assert_eq!(result.to_i32().unwrap(), 2);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn symbol_to_string() {
    unsafe {
        let mrb = mrb_open();

        let result = MrValue::symbol(mrb, "symbol");

        assert_eq!(result.to_str(mrb).unwrap(), "symbol");

        mrb_close(mrb);
    }
}

#[test]
fn define_method() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let new_class_str = CString::new("Mine").unwrap();
        let new_class = mrb_define_class(mrb, new_class_str.as_ptr(), obj_class);

        extern "C" fn job(_mrb: *const MrState, _slf: MrValue) -> MrValue {
            unsafe {
                MrValue::fixnum(2)
            }
        }

        let job_str = CString::new("job").unwrap();

        mrb_define_method(mrb, new_class, job_str.as_ptr(), job, 0);

        let code = "Mine.new.job";

        assert_eq!(mrb_ext_load_nstring_cxt_nothrow(mrb, code.as_ptr(), code.len(), context).to_i32().unwrap(), 2);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn class_defined() {
    unsafe {
        let mrb = mrb_open();

        let obj_str = CString::new("Object").unwrap();
        let kernel_str = CString::new("Kernel").unwrap();

        assert_eq!(mrb_class_defined(mrb, obj_str.as_ptr()), true);
        assert_eq!(mrb_class_defined(mrb, kernel_str.as_ptr()), true);

        mrb_close(mrb);
    }
}

#[test]
fn class_name() {
    unsafe {
        let mrb = mrb_open();

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let new_class_str = CString::new("Mine").unwrap();
        let new_class = mrb_define_class(mrb, new_class_str.as_ptr(), obj_class);

        let kernel_str = CString::new("Kernel").unwrap();
        let kernel = mrb_module_get(mrb, kernel_str.as_ptr());

        let name = mrb_class_name(mrb, new_class);

        assert_eq!(CStr::from_ptr(name).to_str().unwrap(), "Mine");

        let name = mrb_class_name(mrb, kernel);

        assert_eq!(CStr::from_ptr(name).to_str().unwrap(), "Kernel");

        mrb_close(mrb);
    }
}

#[test]
fn class_value() {
    unsafe {
        let mrb = mrb_open();

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let obj_class = mrb_ext_class_value(obj_class);
        let to_s_str = CString::new("to_s").unwrap();
        let args = &[];

        let sym = mrb_intern(mrb, to_s_str.as_ptr(), 4usize);

        let result = mrb_funcall_argv(mrb, obj_class, sym, 0, args.as_ptr());

        assert_eq!(result.to_str(mrb).unwrap(), "Object");

        mrb_close(mrb);
    }
}

#[test]
fn value_class() {
    unsafe {
        let mrb = mrb_open();

        let nil = MrValue::nil();
        let nil_class = mrb_ext_class(mrb, nil);

        let name = mrb_class_name(mrb, nil_class);

        assert_eq!(CStr::from_ptr(name).to_str().unwrap(), "NilClass");

        mrb_close(mrb);
    }
}

#[test]
fn value_to_class() {
    unsafe {
        let mrb = mrb_open();

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let obj_class_value = mrb_ext_class_value(obj_class);

        assert_eq!(obj_class_value.to_class().unwrap(), obj_class);

        mrb_close(mrb);
    }
}

#[test]
fn define_module() {
    unsafe {
        let mrb = mrb_open();

        let mod_str = CString::new("MyMod").unwrap();

        mrb_define_module(mrb, mod_str.as_ptr());

        assert_eq!(mrb_class_defined(mrb, mod_str.as_ptr()), true);

        mrb_close(mrb);
    }
}

#[test]
fn defined_under() {
    unsafe {
        let mrb = mrb_open();

        let kernel_str = CString::new("Kernel").unwrap();
        let kernel = mrb_module_get(mrb, kernel_str.as_ptr());
        let name_str = CString::new("Mine").unwrap();
        let name = name_str.as_ptr();

        mrb_define_module_under(mrb, kernel, name);

        assert!(mrb_ext_class_defined_under(mrb, kernel, name));

        mrb_close(mrb);
    }
}

#[test]
fn class_under() {
    unsafe {
        let mrb = mrb_open();

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let name_str = CString::new("Mine").unwrap();
        let name = name_str.as_ptr();

        mrb_define_class_under(mrb, obj_class, name, obj_class);
        let new_class = mrb_class_get_under(mrb, obj_class, name);

        let name = mrb_class_name(mrb, new_class);

        assert_eq!(CStr::from_ptr(name).to_str().unwrap(), "Mine");

        mrb_close(mrb);
    }
}

#[test]
fn module_under() {
    unsafe {
        let mrb = mrb_open();

        let kernel_str = CString::new("Kernel").unwrap();
        let kernel = mrb_module_get(mrb, kernel_str.as_ptr());
        let name_str = CString::new("Mine").unwrap();
        let name = name_str.as_ptr();

        mrb_define_module_under(mrb, kernel, name);
        let new_module = mrb_module_get_under(mrb, kernel, name);

        let name = mrb_class_name(mrb, new_module);

        assert_eq!(CStr::from_ptr(name).to_str().unwrap(), "Kernel::Mine");

        mrb_close(mrb);
    }
}

#[test]
fn include_module() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let code = "module Increment; def inc; self + 1; end; end";

        mrb_ext_load_nstring_cxt_nothrow(mrb, code.as_ptr(), code.len(), context);

        let fixnum_str = CString::new("Fixnum").unwrap();
        let fixnum = mrb_class_get(mrb, fixnum_str.as_ptr());
        let increment_str = CString::new("Increment").unwrap();
        let increment = mrb_module_get(mrb, increment_str.as_ptr());

        mrb_include_module(mrb, fixnum, increment);

        let code = "1.inc";

        assert_eq!(mrb_ext_load_nstring_cxt_nothrow(mrb, code.as_ptr(), code.len(), context)
                   .to_i32().unwrap(), 2);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn define_class_method() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let new_class_str = CString::new("Mine").unwrap();
        let new_class = mrb_define_class(mrb, new_class_str.as_ptr(), obj_class);

        extern "C" fn job(_mrb: *const MrState, _slf: MrValue) -> MrValue {
            unsafe {
                MrValue::fixnum(2)
            }
        }

        let job_str = CString::new("job").unwrap();

        mrb_define_class_method(mrb, new_class, job_str.as_ptr(), job, 0);

        let code = "Mine.job";

        assert_eq!(mrb_ext_load_nstring_cxt_nothrow(mrb, code.as_ptr(), code.len(), context)
                   .to_i32().unwrap(), 2);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn define_constant() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let kernel_str = CString::new("Kernel").unwrap();
        let kernel = mrb_module_get(mrb, kernel_str.as_ptr());

        let one = MrValue::fixnum(1);
        let one_str = CString::new("ONE").unwrap();

        mrb_define_const(mrb, obj_class, one_str.as_ptr(), one);
        mrb_define_const(mrb, kernel, one_str.as_ptr(), one);

        let code = "Object::ONE";

        assert_eq!(mrb_ext_load_nstring_cxt_nothrow(mrb, code.as_ptr(), code.len(), context)
                   .to_i32().unwrap(), 1);

        let code = "Kernel::ONE";

        assert_eq!(mrb_ext_load_nstring_cxt_nothrow(mrb, code.as_ptr(), code.len(), context)
                   .to_i32().unwrap(), 1);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn define_module_function() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let kernel_str = CString::new("Kernel").unwrap();
        let kernel = mrb_module_get(mrb, kernel_str.as_ptr());

        extern "C" fn hi(mrb: *const MrState, _slf: MrValue) -> MrValue {
            unsafe {
                MrValue::string(mrb, "hi")
            }
        }

        let hi_str = CString::new("hi").unwrap();

        mrb_define_module_function(mrb, kernel, hi_str.as_ptr(), hi, 0);

        let code = "hi";

        assert_eq!(mrb_ext_load_nstring_cxt_nothrow(mrb, code.as_ptr(), code.len(), context)
                   .to_str(mrb).unwrap(), "hi");

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn protect() {
    use std::mem::MaybeUninit;

    unsafe {
        let mrb = mrb_open();
        let ctx = mrbc_context_new(mrb);

        extern "C" fn run_protected(mrb: *const MrState, data: MrValue) -> MrValue {
            unsafe {
                let ptr = data.to_ptr().unwrap();
                let args = *mem::transmute::<*const u8, *const [*const u8; 3]>(ptr);

                let script = args[0];
                let script_len: &usize = mem::transmute(args[1]);
                let ctx: *const MrContext = mem::transmute(args[2]);

                let result = mrb_ext_load_nstring_cxt_nothrow(mrb, script, *script_len, ctx);

                mrb_ext_raise_current(mrb);

                result
            }
        }

        let script = "false 'surprize'";
        let script_ptr = script.as_ptr();
        let script_len = script.len();
        let script_len_ptr: *const u8 = mem::transmute(&script_len);
        let ctx_ptr: *const u8 = mem::transmute(ctx);

        let args = [script_ptr, script_len_ptr, ctx_ptr];
        let args_ptr: *const u8 = mem::transmute(&args);
        let data = MrValue::ptr(mrb, args_ptr);

        let state = MaybeUninit::<bool>::zeroed().assume_init();

        let exc = mrb_protect(mrb, run_protected, data, &state as *const bool);

        assert_eq!(state, true);

        let args = &[];

        let class_str = CString::new("class").unwrap();
        let class_sym = mrb_intern(mrb, class_str.as_ptr(), 5usize);
        let to_s_str = CString::new("to_s").unwrap();
        let to_s_sym = mrb_intern(mrb, to_s_str.as_ptr(), 4usize);

        let class = mrb_funcall_argv(mrb, exc, class_sym, 0, args.as_ptr());
        let result = mrb_funcall_argv(mrb, class, to_s_sym, 0, args.as_ptr());

        assert_eq!(result.to_str(mrb).unwrap(), "SyntaxError");

        mrbc_context_free(mrb, ctx);
        mrb_close(mrb);
    }
}

#[test]
pub fn args() {
    use std::mem::MaybeUninit;

    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        extern "C" fn add(mrb: *const MrState, _slf: MrValue) -> MrValue {
            unsafe {
                let a = MaybeUninit::<MrValue>::uninit().assume_init();
                let b = MaybeUninit::<MrValue>::uninit().assume_init();

                let sig_str = CString::new("oo").unwrap();

                mrb_get_args(mrb, sig_str.as_ptr(), &a as *const MrValue,
                             &b as *const MrValue);

                let args = &[b];

                let plus_str = CString::new("+").unwrap();

                let sym = mrb_intern(mrb, plus_str.as_ptr(), 1usize);

                mrb_funcall_argv(mrb, a, sym, 1, args.as_ptr())
            }
        }

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let new_class_str = CString::new("Mine").unwrap();
        let new_class = mrb_define_class(mrb, new_class_str.as_ptr(), obj_class);

        let add_str = CString::new("add").unwrap();

        mrb_define_method(mrb, new_class, add_str.as_ptr(), add,
                          (2 & 0x1f) << 18);

        let code = "Mine.new.add 1, 1";

        assert_eq!(mrb_ext_load_nstring_cxt_nothrow(mrb, code.as_ptr(), code.len(), context)
                   .to_i32().unwrap(), 2);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
pub fn str_args() {
    use std::ffi::CStr;
    use std::mem::MaybeUninit;
    use std::os::raw::c_char;

    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        extern "C" fn add(mrb: *const MrState, _slf: MrValue) -> MrValue {
            unsafe {
                let a = MaybeUninit::<*const c_char>::uninit().assume_init();
                let b = MaybeUninit::<*const c_char>::uninit().assume_init();

                let sig_str = CString::new("zz").unwrap();

                mrb_get_args(mrb, sig_str.as_ptr(), &a as *const *const c_char,
                             &b as *const *const c_char);

                let a = CStr::from_ptr(a).to_str().unwrap();
                let b = CStr::from_ptr(b).to_str().unwrap();

                let args = &[MrValue::string(mrb, b)];

                let plus_str = CString::new("+").unwrap();

                let sym = mrb_intern(mrb, plus_str.as_ptr(), 1usize);

                mrb_funcall_argv(mrb, MrValue::string(mrb, a), sym, 1, args.as_ptr())
            }
        }

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let new_class_str = CString::new("Mine").unwrap();
        let new_class = mrb_define_class(mrb, new_class_str.as_ptr(), obj_class);

        let add_str = CString::new("add").unwrap();

        mrb_define_method(mrb, new_class, add_str.as_ptr(), add,
                          (2 & 0x1f) << 18);

        let code = "Mine.new.add 'a', 'b'";

        assert_eq!(mrb_ext_load_nstring_cxt_nothrow(mrb, code.as_ptr(), code.len(), context)
                   .to_str(mrb).unwrap(), "ab");

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
pub fn array_args() {
    use std::mem::MaybeUninit;

    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        extern "C" fn add(mrb: *const MrState, _slf: MrValue) -> MrValue {
            unsafe {
                let array = MaybeUninit::<MrValue>::uninit().assume_init();

                let a_str = CString::new("A").unwrap();

                mrb_get_args(mrb, a_str.as_ptr(), &array as *const MrValue);

                let vec = array.to_vec(mrb).unwrap();

                let args = &[vec[1]];

                let plus_str = CString::new("+").unwrap();

                let sym = mrb_intern(mrb, plus_str.as_ptr(), 1usize);

                mrb_funcall_argv(mrb, vec[0], sym, 1, args.as_ptr())
            }
        }

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let new_class_str = CString::new("Mine").unwrap();
        let new_class = mrb_define_class(mrb, new_class_str.as_ptr(), obj_class);

        let add_str = CString::new("add").unwrap();

        mrb_define_method(mrb, new_class, add_str.as_ptr(), add,
                          (2 & 0x1f) << 18);

        let code = "Mine.new.add [1, 1]";

        assert_eq!(mrb_ext_load_nstring_cxt_nothrow(mrb, code.as_ptr(), code.len(), context)
                   .to_i32().unwrap(), 2);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn funcall_argv() {
    unsafe {
        let mrb = mrb_open();

        let one = MrValue::fixnum(1);
        let args = &[MrValue::fixnum(2)];

        let plus_str = CString::new("+").unwrap();

        let sym = mrb_intern(mrb, plus_str.as_ptr(), 1usize);

        let result = mrb_funcall_argv(mrb, one, sym, 1, args.as_ptr());

        assert_eq!(result.to_i32().unwrap(), 3);

        mrb_close(mrb);
    }
}

#[test]
fn iv() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let new_str = CString::new("Mine").unwrap();

        mrb_define_class(mrb, new_str.as_ptr(), obj_class);

        let one = MrValue::fixnum(1);

        let code = "Mine.new";
        let obj = mrb_ext_load_nstring_cxt_nothrow(mrb, code.as_ptr(), code.len(), context);

        let value_str = CString::new("value").unwrap();

        let sym = mrb_intern(mrb, value_str.as_ptr(), 1usize);

        assert!(!mrb_iv_defined(mrb, obj, sym));

        mrb_iv_set(mrb, obj, sym, one);

        assert!(mrb_iv_defined(mrb, obj, sym));
        assert_eq!(mrb_iv_get(mrb, obj, sym).to_i32().unwrap(), 1);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn nil() {
    unsafe {
        let mrb = mrb_open();

        let nil = MrValue::nil();

        let args: &[MrValue] = &[];

        let to_s_str = CString::new("to_s").unwrap();

        let sym = mrb_intern(mrb, to_s_str.as_ptr(), 4usize);

        let result = mrb_funcall_argv(mrb, nil, sym, 0, args.as_ptr());

        assert_eq!(result.to_str(mrb).unwrap(), "");

        mrb_close(mrb);
    }
}

#[test]
fn bool_true() {
    unsafe {
        let bool_true = MrValue::bool(true);
        assert_eq!(bool_true.to_bool().unwrap(), true);
    }
}

#[test]
fn bool_false() {
    unsafe {
        let bool_false = MrValue::bool(false);
        assert_eq!(bool_false.to_bool().unwrap(), false);
    }
}

#[test]
fn fixnum() {
    unsafe {
        let number = MrValue::fixnum(-1291657);
        assert_eq!(number.to_i32().unwrap(), -1291657);
    }
}

#[test]
fn float() {
    unsafe {
        let mrb = mrb_open();

        let number = MrValue::float(mrb, -1291657.37);
        assert_eq!(number.to_f64().unwrap(), -1291657.37);

        mrb_close(mrb);
    }
}

#[test]
fn string() {
    unsafe {
        let mrb = mrb_open();

        let string_value = MrValue::string(mrb, "qwerty");
        assert_eq!(string_value.to_str(mrb).unwrap(), "qwerty");

        mrb_close(mrb);
    }
}

#[test]
fn obj() {
    use std::cell::RefCell;
    use std::mem;
    use std::rc::Rc;

    unsafe {
        struct Cont {
            value: i32
        }

        let mrb = mrb_open();

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let cont_str = CString::new("Cont").unwrap();
        let cont_class = mrb_define_class(mrb, cont_str.as_ptr(), obj_class);

        mrb_ext_set_instance_tt(cont_class, MrType::MRB_TT_DATA);

        extern "C" fn free(_mrb: *const MrState, ptr: *const u8) {
            unsafe {
                mem::transmute::<*const u8, Rc<Cont>>(ptr);
            }
        }

        let data_type = mrb_ext_data_type(cont_str.as_ptr(), free);

        let obj = Cont { value: 3 };
        let obj = MrValue::obj(mrb, cont_class, obj, &data_type);
        let obj: Rc<RefCell<Cont>> = obj.to_obj(mrb, &data_type).unwrap();
        let value = obj.borrow().value;

        assert_eq!(value, 3);

        mrb_close(mrb);
    }
}

#[test]
fn obj_init() {
    use std::cell::RefCell;
    use std::mem;
    use std::rc::Rc;

    unsafe {
        struct Cont {
            value: i32
        }

        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let cont_str = CString::new("Cont").unwrap();
        let cont_class = mrb_define_class(mrb, cont_str.as_ptr(), obj_class);

        mrb_ext_set_instance_tt(cont_class, MrType::MRB_TT_DATA);

        extern "C" fn free(_mrb: *const MrState, ptr: *const u8) {
            unsafe {
                mem::transmute::<*const u8, Rc<RefCell<Cont>>>(ptr);
            }
        }

        extern "C" fn init(mrb: *const MrState, slf: MrValue) -> MrValue {
            unsafe {
                let cont = Cont { value: 3 };
                let rc = Rc::new(RefCell::new(cont));
                let ptr: *const u8 = mem::transmute(rc);

                let data_type: *const MrDataType = mem::transmute(mrb_ext_get_ud(mrb));

                mrb_ext_data_init(&slf as *const MrValue, ptr, data_type);

                slf
            }
        }

        extern "C" fn value(mrb: *const MrState, slf: MrValue) -> MrValue {
            unsafe {
                let data_type: &MrDataType = mem::transmute(mrb_ext_get_ud(mrb));

                let cont = slf.to_obj::<Cont>(mrb, data_type).unwrap();
                let value = cont.borrow().value;

                MrValue::fixnum(value)
            }
        }

        let data_type = &mrb_ext_data_type(cont_str.as_ptr(), free);

        mrb_ext_set_ud(mrb, mem::transmute::<&MrDataType, *const u8>(data_type));

        let init_str = CString::new("initialize").unwrap();
        let value_str = CString::new("value").unwrap();

        mrb_define_method(mrb, cont_class, init_str.as_ptr(), init,
                          1 << 12);
        mrb_define_method(mrb, cont_class, value_str.as_ptr(), value,
                          1 << 12);

        let code = "Cont.new.value";
        let val = mrb_ext_load_nstring_cxt_nothrow(mrb, code.as_ptr(), code.len(), context)
                  .to_i32().unwrap();

        assert_eq!(val, 3);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn obj_scoping() {
    use std::cell::RefCell;
    use std::mem;
    use std::rc::Rc;

    unsafe {
        static mut DROPPED: bool = false;

        struct Cont {
            value: i32
        }

        impl Drop for Cont {
            fn drop(&mut self) {
                unsafe {
                    DROPPED = true;
                }
            }
        }

        let mrb = mrb_open();

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let cont_str = CString::new("Cont").unwrap();
        let cont_class = mrb_define_class(mrb, cont_str.as_ptr(), obj_class);

        mrb_ext_set_instance_tt(cont_class, MrType::MRB_TT_DATA);

        extern "C" fn free(_mrb: *const MrState, ptr: *const u8) {
            unsafe {
                mem::transmute::<*const u8, Rc<RefCell<Cont>>>(ptr);
            }
        }

        let data_type = mrb_ext_data_type(cont_str.as_ptr(), free);

        {
            let orig = Cont { value: 3 };

            {
                let obj = MrValue::obj(mrb, cont_class, orig, &data_type);
                let obj: Rc<RefCell<Cont>> = obj.to_obj(mrb, &data_type).unwrap();
                let value = obj.borrow().value;

                assert_eq!(value, 3);

                assert_eq!(DROPPED, false);
            }

            assert_eq!(DROPPED, false);
        }

        assert_eq!(DROPPED, false);

        mrb_close(mrb);

        assert_eq!(DROPPED, true);
    }
}

#[test]
fn array() {
    unsafe {
        let mrb = mrb_open();

        let vec: Vec<MrValue> = [1, 2, 3].iter().map(|v| MrValue::fixnum(*v)).collect();

        let array = MrValue::array(mrb, vec.clone());

        assert_eq!(array.to_vec(mrb).unwrap(), vec);

        mrb_close(mrb);
    }
}

#[test]
fn ptr() {
    unsafe {
        let mrb = mrb_open();

        let n = 3u8;

        let value = MrValue::ptr(mrb, &n as *const u8);

        assert_eq!(*value.to_ptr().unwrap(), 3u8);

        mrb_close(mrb);
    }
}

