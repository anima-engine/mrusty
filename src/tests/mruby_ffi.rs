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
        let n = mem::transmute::<*const u8, &i32>(mrb_ext_get_ud(mrb));

        assert_eq!(*n, 1);

        mrb_close(mrb);
    }
}

#[test]
fn exec_context() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        mrbc_filename(mrb, context, CString::new("script.rb").unwrap().as_ptr());

        let code = "'' + 0";

        mrb_load_nstring_cxt(mrb, code.as_ptr(), code.len() as i32,  context);

        assert_eq!(mrb_ext_get_exc(mrb).to_str(mrb).unwrap(),
                   "script.rb:1: expected String (TypeError)");

        mrb_close(mrb);
    }
}

#[test]
fn exec_bin_context() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let bin = [82u8, 73u8, 84u8, 69u8, 48u8, 48u8, 48u8, 51u8, 107u8, 70u8, 0u8, 0u8, 0u8,
                   72u8, 77u8, 65u8, 84u8, 90u8, 48u8, 48u8, 48u8, 48u8, 73u8, 82u8, 69u8, 80u8,
                   0u8, 0u8, 0u8, 42u8, 48u8, 48u8, 48u8, 48u8, 0u8, 0u8, 0u8, 34u8, 0u8, 1u8, 0u8,
                   2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 192u8, 0u8, 131u8, 0u8, 0u8, 0u8, 74u8,
                   0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 69u8, 78u8, 68u8, 0u8, 0u8, 0u8, 0u8,
                   8u8];

        let result = mrb_load_irep_cxt(mrb, bin.as_ptr(), context);

        assert_eq!(result.to_i32().unwrap(), 2);

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

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let new_class = mrb_define_class(mrb, CString::new("Mine").unwrap().as_ptr(), obj_class);

        extern "C" fn job(_mrb: *const MrState, _slf: MrValue) -> MrValue {
            unsafe {
                MrValue::fixnum(2)
            }
        }

        mrb_define_method(mrb, new_class, CString::new("job").unwrap().as_ptr(), job, 0);

        let code = "Mine.new.job";

        assert_eq!(mrb_load_nstring_cxt(mrb, code.as_ptr(), code.len() as i32, context).to_i32().unwrap(), 2);

        mrb_close(mrb);
    }
}

#[test]
fn class_name() {
    unsafe {
        let mrb = mrb_open();

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let new_class = mrb_define_class(mrb, CString::new("Mine").unwrap().as_ptr(), obj_class);

        let name = mrb_class_name(mrb, new_class);

        assert_eq!(CStr::from_ptr(name).to_str().unwrap(), "Mine");

        mrb_close(mrb);
    }
}

#[test]
fn define_class_method() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let new_class = mrb_define_class(mrb, CString::new("Mine").unwrap().as_ptr(), obj_class);

        extern "C" fn job(_mrb: *const MrState, _slf: MrValue) -> MrValue {
            unsafe {
                MrValue::fixnum(2)
            }
        }

        mrb_define_class_method(mrb, new_class, CString::new("job").unwrap().as_ptr(), job, 0);

        let code = "Mine.job";

        assert_eq!(mrb_load_nstring_cxt(mrb, code.as_ptr(), code.len() as i32, context)
                   .to_i32().unwrap(), 2);

        mrb_close(mrb);
    }
}

#[test]
fn define_module_function() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let kernel_mod = mrb_module_get(mrb, CString::new("Kernel").unwrap().as_ptr());

        extern "C" fn hi(mrb: *const MrState, _slf: MrValue) -> MrValue {
            unsafe {
                MrValue::string(mrb, "hi")
            }
        }

        mrb_define_module_function(mrb, kernel_mod, CString::new("hi").unwrap().as_ptr(), hi, 0);

        let code = "hi";

        assert_eq!(mrb_load_nstring_cxt(mrb, code.as_ptr(), code.len() as i32, context)
                   .to_str(mrb).unwrap(), "hi");

        mrb_close(mrb);
    }
}

#[test]
fn raise_exc() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let new_class = mrb_define_class(mrb, CString::new("Mine").unwrap().as_ptr(), obj_class);

        extern "C" fn job(mrb: *const MrState, _slf: MrValue) -> MrValue {
            unsafe {
                mrb_ext_raise(mrb, CString::new("RuntimeError").unwrap().as_ptr(),
                              CString::new("excepting").unwrap().as_ptr());

                MrValue::nil()
            }
        }

        mrb_define_class_method(mrb, new_class, CString::new("job").unwrap().as_ptr(), job, 0);

        let code = "Mine.job";

        mrb_load_nstring_cxt(mrb, code.as_ptr(), code.len() as i32, context);

        assert_eq!(mrb_ext_get_exc(mrb).to_str(mrb).unwrap(), "RuntimeError: excepting");

        mrb_close(mrb);
    }
}

#[test]
pub fn args() {
    use std::mem::uninitialized;

    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        extern "C" fn add(mrb: *const MrState, _slf: MrValue) -> MrValue {
            unsafe {
                let a = uninitialized::<MrValue>();
                let b = uninitialized::<MrValue>();

                mrb_get_args(mrb, CString::new("oo").unwrap().as_ptr(), &a as *const MrValue,
                             &b as *const MrValue);

                let args = &[b];
                let sym = mrb_intern(mrb, "+".as_ptr(), 1usize);

                mrb_funcall_argv(mrb, a, sym, 1, args.as_ptr())
            }
        }

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let new_class = mrb_define_class(mrb, CString::new("Mine").unwrap().as_ptr(), obj_class);

        mrb_define_method(mrb, new_class, CString::new("add").unwrap().as_ptr(), add,
                          (2 & 0x1f) << 18);

        let code = "Mine.new.add 1, 1";

        assert_eq!(mrb_load_nstring_cxt(mrb, code.as_ptr(), code.len() as i32, context)
                   .to_i32().unwrap(), 2);

        mrb_close(mrb);
    }
}

#[test]
pub fn str_args() {
    use std::ffi::CStr;
    use std::mem::uninitialized;
    use std::os::raw::c_char;

    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        extern "C" fn add(mrb: *const MrState, _slf: MrValue) -> MrValue {
            unsafe {
                let a = uninitialized::<*const c_char>();
                let b = uninitialized::<*const c_char>();

                mrb_get_args(mrb, CString::new("zz").unwrap().as_ptr(), &a as *const *const c_char,
                             &b as *const *const c_char);

                let a = CStr::from_ptr(a).to_str().unwrap();
                let b = CStr::from_ptr(b).to_str().unwrap();

                let args = &[MrValue::string(mrb, b)];
                let sym = mrb_intern(mrb, "+".as_ptr(), 1usize);

                mrb_funcall_argv(mrb, MrValue::string(mrb, a), sym, 1, args.as_ptr())
            }
        }

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let new_class = mrb_define_class(mrb, CString::new("Mine").unwrap().as_ptr(), obj_class);

        mrb_define_method(mrb, new_class, CString::new("add").unwrap().as_ptr(), add,
                          (2 & 0x1f) << 18);

        let code = "Mine.new.add 'a', 'b'";

        assert_eq!(mrb_load_nstring_cxt(mrb, code.as_ptr(), code.len() as i32, context)
                   .to_str(mrb).unwrap(), "ab");

        mrb_close(mrb);
    }
}

#[test]
pub fn array_args() {
    use std::mem::uninitialized;

    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        extern "C" fn add(mrb: *const MrState, _slf: MrValue) -> MrValue {
            unsafe {
                let array = uninitialized::<MrValue>();

                mrb_get_args(mrb, CString::new("A").unwrap().as_ptr(), &array as *const MrValue);

                let vec = array.to_vec(mrb).unwrap();

                let args = &[vec[1]];
                let sym = mrb_intern(mrb, "+".as_ptr(), 1usize);

                mrb_funcall_argv(mrb, vec[0], sym, 1, args.as_ptr())
            }
        }

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let new_class = mrb_define_class(mrb, CString::new("Mine").unwrap().as_ptr(), obj_class);

        mrb_define_method(mrb, new_class, CString::new("add").unwrap().as_ptr(), add,
                          (2 & 0x1f) << 18);

        let code = "Mine.new.add [1, 1]";

        assert_eq!(mrb_load_nstring_cxt(mrb, code.as_ptr(), code.len() as i32, context)
                   .to_i32().unwrap(), 2);

        mrb_close(mrb);
    }
}

#[test]
fn funcall_argv() {
    unsafe {
        let mrb = mrb_open();

        let one = MrValue::fixnum(1);
        let args = &[MrValue::fixnum(2)];

        let sym = mrb_intern(mrb, "+".as_ptr(), 1usize);

        let result = mrb_funcall_argv(mrb, one, sym, 1, args.as_ptr());

        assert_eq!(result.to_i32().unwrap(), 3);

        mrb_close(mrb);
    }
}

#[test]
fn nil() {
    unsafe {
        let mrb = mrb_open();

        let nil = MrValue::nil();

        let args: &[MrValue] = &[];

        let sym = mrb_intern(mrb, "to_s".as_ptr(), 4usize);

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
    use std::mem;
    use std::rc::Rc;

    unsafe {
        struct Cont {
            value: i32
        }

        let mrb = mrb_open();

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let cont_class = mrb_define_class(mrb, CString::new("Cont").unwrap().as_ptr(), obj_class);

        mrb_ext_set_instance_tt(cont_class, MrType::MRB_TT_DATA);

        extern "C" fn free(_mrb: *const MrState, ptr: *const u8) {
            unsafe {
                mem::transmute::<*const u8, Rc<Cont>>(ptr);
            }
        }

        let data_type = MrDataType { name: CString::new("Cont").unwrap().as_ptr(), free: free };

        let obj = Cont { value: 3 };
        let obj = MrValue::obj(mrb, cont_class, obj, &data_type);
        let obj: Rc<Cont> = obj.to_obj(mrb, &data_type).unwrap();

        assert_eq!(obj.value, 3);

        mrb_close(mrb);
    }
}

#[test]
fn obj_init() {
    use std::mem;
    use std::rc::Rc;

    unsafe {
        struct Cont {
            value: i32
        }

        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let cont_class = mrb_define_class(mrb, CString::new("Cont").unwrap().as_ptr(), obj_class);

        mrb_ext_set_instance_tt(cont_class, MrType::MRB_TT_DATA);

        extern "C" fn free(_mrb: *const MrState, ptr: *const u8) {
            unsafe {
                mem::transmute::<*const u8, Rc<Cont>>(ptr);
            }
        }

        extern "C" fn init(mrb: *const MrState, slf: MrValue) -> MrValue {
            unsafe {
                let cont = Cont { value: 3 };
                let rc = Rc::new(cont);
                let ptr = mem::transmute::<Rc<Cont>, *const u8>(rc);

                let data_type = mem::transmute::<*const u8,
                                                 *const MrDataType>(mrb_ext_get_ud(mrb));

                mrb_ext_data_init(&slf as *const MrValue, ptr, data_type);

                slf
            }
        }

        extern "C" fn value(mrb: *const MrState, slf: MrValue) -> MrValue {
            unsafe {
                let data_type = mem::transmute::<*const u8, &MrDataType>(mrb_ext_get_ud(mrb));

                let cont = slf.to_obj::<Cont>(mrb, data_type);

                MrValue::fixnum(cont.unwrap().value)
            }
        }

        let data_type = &MrDataType { name: CString::new("Cont").unwrap().as_ptr(), free: free };

        mrb_ext_set_ud(mrb, mem::transmute::<&MrDataType, *const u8>(data_type));

        mrb_define_method(mrb, cont_class, CString::new("initialize").unwrap().as_ptr(), init,
                          1 << 12);
        mrb_define_method(mrb, cont_class, CString::new("value").unwrap().as_ptr(), value,
                          1 << 12);

        let code = "Cont.new.value";
        let val = mrb_load_nstring_cxt(mrb, code.as_ptr(), code.len() as i32, context)
                  .to_i32().unwrap();

        assert_eq!(val, 3);

        mrb_close(mrb);
    }
}

#[test]
fn obj_scoping() {
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

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let cont_class = mrb_define_class(mrb, CString::new("Cont").unwrap().as_ptr(), obj_class);

        mrb_ext_set_instance_tt(cont_class, MrType::MRB_TT_DATA);

        extern "C" fn free(_mrb: *const MrState, ptr: *const u8) {
            unsafe {
                mem::transmute::<*const u8, Rc<Cont>>(ptr);
            }
        }

        let data_type = MrDataType { name: CString::new("Cont").unwrap().as_ptr(), free: free };

        {
            let orig = Cont { value: 3 };

            {
                let obj = MrValue::obj(mrb, cont_class, orig, &data_type);
                let obj: Rc<Cont> = obj.to_obj(mrb, &data_type).unwrap();

                assert_eq!(obj.value, 3);

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
