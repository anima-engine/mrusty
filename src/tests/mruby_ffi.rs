use std::ffi::CString;

use super::*;

#[test]
fn test_open_close() {
    unsafe {
        let mrb = mrb_open();

        mrb_close(mrb);
    }
}

#[test]
fn test_ud() {
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
fn test_exec_context() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        mrbc_filename(mrb, context, CString::new("script.rb").unwrap().as_ptr());

        let code = CString::new("'' + 0").unwrap().as_ptr();

        mrb_load_string_cxt(mrb, code, context);

        assert_eq!(mrb_ext_get_exc(mrb).to_str(mrb).unwrap(), "script.rb:1: expected String (TypeError)");

        mrb_close(mrb);
    }
}

#[test]
fn test_create_run_proc() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let code = CString::new("1 + 1").unwrap().as_ptr();
        let parser = mrb_parse_string(mrb, code, context);
        let prc = mrb_generate_code(mrb, parser);

        let result = mrb_run(mrb, prc, mrb_top_self(mrb));

        assert_eq!(result.to_i32().unwrap(), 2);
        assert_eq!(mrb_ext_get_exc(mrb).to_bool().unwrap(), false);

        mrb_close(mrb);
    }
}

#[test]
fn test_class_defined() {
    unsafe {
        let mrb = mrb_open();

        let obj_class = CString::new("Object").unwrap().as_ptr();

        assert_eq!(mrb_class_defined(mrb, obj_class), 1);

        mrb_close(mrb);
    }
}

#[test]
fn test_define_class() {
    unsafe {
        let mrb = mrb_open();

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        mrb_define_class(mrb, CString::new("Mine").unwrap().as_ptr(), obj_class);

        assert_eq!(mrb_class_defined(mrb, CString::new("Mine").unwrap().as_ptr()), 1);

        mrb_close(mrb);
    }
}

#[test]
fn test_define_module() {
    unsafe {
        let mrb = mrb_open();

        mrb_define_module(mrb, CString::new("Mine").unwrap().as_ptr());
        mrb_module_get(mrb, CString::new("Mine").unwrap().as_ptr());

        mrb_close(mrb);
    }
}

#[test]
fn test_include_module() {
    unsafe {
        let mrb = mrb_open();

        let new_module = mrb_define_module(mrb, CString::new("Mine").unwrap().as_ptr());
        let kernel = mrb_module_get(mrb, CString::new("Kernel").unwrap().as_ptr());

        mrb_include_module(mrb, kernel, new_module);

        mrb_close(mrb);
    }
}

#[test]
fn test_prepend_module() {
    unsafe {
        let mrb = mrb_open();

        let new_module = mrb_define_module(mrb, CString::new("Mine").unwrap().as_ptr());
        let kernel = mrb_module_get(mrb, CString::new("Kernel").unwrap().as_ptr());

        mrb_prepend_module(mrb, kernel, new_module);

        mrb_close(mrb);
    }
}

#[test]
fn test_define_method() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let new_class = mrb_define_class(mrb, CString::new("Mine").unwrap().as_ptr(), obj_class);

        extern "C" fn job(mrb: *mut MRState, slf: MRValue) -> MRValue {
            unsafe {
                MRValue::fixnum(2)
            }
        }

        mrb_define_method(mrb, new_class, CString::new("job").unwrap().as_ptr(), job, 0);

        let code = CString::new("Mine.new.job").unwrap().as_ptr();

        assert_eq!(mrb_load_string_cxt(mrb, code, context).to_i32().unwrap(), 2);

        mrb_close(mrb);
    }
}

#[test]
fn test_define_class_method() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let new_class = mrb_define_class(mrb, CString::new("Mine").unwrap().as_ptr(), obj_class);

        extern "C" fn job(mrb: *mut MRState, slf: MRValue) -> MRValue {
            unsafe {
                MRValue::fixnum(2)
            }
        }

        mrb_define_class_method(mrb, new_class, CString::new("job").unwrap().as_ptr(), job, 0);

        let code = CString::new("Mine.job").unwrap().as_ptr();

        assert_eq!(mrb_load_string_cxt(mrb, code, context).to_i32().unwrap(), 2);

        mrb_close(mrb);
    }
}

#[test]
fn test_define_module_function() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let new_module = mrb_define_module(mrb, CString::new("Mine").unwrap().as_ptr());

        extern "C" fn job(mrb: *mut MRState, slf: MRValue) -> MRValue {
            unsafe {
                MRValue::fixnum(2)
            }
        }

        mrb_define_module_function(mrb, new_module, CString::new("job").unwrap().as_ptr(), job, 0);

        let code = CString::new("Mine.job").unwrap().as_ptr();

        assert_eq!(mrb_load_string_cxt(mrb, code, context).to_i32().unwrap(), 2);

        mrb_close(mrb);
    }
}

#[test]
fn test_obj_new() {
    use std::ptr;

    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());

        mrb_obj_new(mrb, obj_class, 0, ptr::null() as *const MRValue);

        mrb_close(mrb);
    }
}

#[test]
fn test_proc_new_cfunc() {
    use std::ptr;

    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        extern "C" fn job(mrb: *mut MRState, slf: MRValue) -> MRValue {
            unsafe {
                MRValue::fixnum(2)
            }
        }

        let prc = MRValue::prc(mrb, mrb_proc_new_cfunc(mrb, job));

        mrb_funcall_with_block(mrb, MRValue::fixnum(5), mrb_intern_cstr(mrb, CString::new("times").unwrap().as_ptr()), 0, ptr::null() as *const MRValue, prc);

        mrb_close(mrb);
    }
}

#[test]
pub fn test_args() {
    use std::mem::uninitialized;

    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        extern "C" fn add(mrb: *mut MRState, slf: MRValue) -> MRValue {
            unsafe {
                let a = uninitialized::<MRValue>();
                let b = uninitialized::<MRValue>();

                mrb_get_args(mrb, CString::new("oo").unwrap().as_ptr(), &a as *const MRValue, &b as *const MRValue);

                mrb_funcall(mrb, a, CString::new("+").unwrap().as_ptr(), 1, b)
            }
        }

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let new_class = mrb_define_class(mrb, CString::new("Mine").unwrap().as_ptr(), obj_class);

        mrb_define_method(mrb, new_class, CString::new("add").unwrap().as_ptr(), add, (2 & 0x1f) << 18);

        let code = CString::new("Mine.new.add 1, 1").unwrap().as_ptr();

        assert_eq!(mrb_load_string_cxt(mrb, code, context).to_i32().unwrap(), 2);

        mrb_close(mrb);
    }
}

#[test]
pub fn test_str_args() {
    use std::ffi::CStr;
    use std::mem::uninitialized;
    use std::os::raw::c_char;

    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        extern "C" fn add(mrb: *mut MRState, slf: MRValue) -> MRValue {
            unsafe {
                let a = uninitialized::<*const c_char>();
                let b = uninitialized::<*const c_char>();

                mrb_get_args(mrb, CString::new("zz").unwrap().as_ptr(), &a as *const *const c_char, &b as *const *const c_char);

                let a = CStr::from_ptr(a).to_str().unwrap();
                let b = CStr::from_ptr(b).to_str().unwrap();

                mrb_funcall(mrb, MRValue::string(mrb, a), CString::new("+").unwrap().as_ptr(), 1, MRValue::string(mrb, b))
            }
        }

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let new_class = mrb_define_class(mrb, CString::new("Mine").unwrap().as_ptr(), obj_class);

        mrb_define_method(mrb, new_class, CString::new("add").unwrap().as_ptr(), add, (2 & 0x1f) << 18);

        let code = CString::new("Mine.new.add 'a', 'b'").unwrap().as_ptr();

        assert_eq!(mrb_load_string_cxt(mrb, code, context).to_str(mrb).unwrap(), "ab");

        mrb_close(mrb);
    }
}

#[test]
fn test_funcall_argv() {
    unsafe {
        let mrb = mrb_open();

        let one = MRValue::fixnum(1);
        let args = &[MRValue::fixnum(2)];

        let sym = mrb_intern_cstr(mrb, CString::new("+").unwrap().as_ptr());

        let result = mrb_funcall_argv(mrb, one, sym, 1, args.as_ptr());

        assert_eq!(result.to_i32().unwrap(), 3);

        mrb_close(mrb);
    }
}

#[test]
fn test_yield() {
    use std::mem::uninitialized;

    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        extern "C" fn add(mrb: *mut MRState, slf: MRValue) -> MRValue {
            unsafe {
                let a = uninitialized::<MRValue>();;
                let b = MRValue::fixnum(1);

                let prc = uninitialized::<MRValue>();;

                mrb_get_args(mrb, CString::new("o&").unwrap().as_ptr(), &a as *const MRValue, &prc as *const MRValue);
                let b = mrb_yield_argv(mrb, prc, 1, [b].as_ptr());

                mrb_funcall(mrb, a, CString::new("+").unwrap().as_ptr(), 1, b)
            }
        }

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let new_class = mrb_define_class(mrb, CString::new("Mine").unwrap().as_ptr(), obj_class);

        mrb_define_method(mrb, new_class, CString::new("add").unwrap().as_ptr(), add, (2 & 0x1f) << 18);

        let code = CString::new("Mine.new.add(1) { |n| n + 1 }").unwrap().as_ptr();

        assert_eq!(mrb_load_string_cxt(mrb, code, context).to_i32().unwrap(), 3);

        mrb_close(mrb);
    }
}

#[test]
fn test_nil() {
    unsafe {
        let mrb = mrb_open();

        let nil = MRValue::nil();
        let result = mrb_funcall(mrb, nil, CString::new("to_s").unwrap().as_ptr(), 0);

        assert_eq!(result.to_str(mrb).unwrap(), "");

        mrb_close(mrb);
    }
}

#[test]
fn test_bool_true() {
    unsafe {
        let bool_true = MRValue::bool(true);
        assert_eq!(bool_true.to_bool().unwrap(), true);
    }
}

#[test]
fn test_bool_false() {
    unsafe {
        let bool_false = MRValue::bool(false);
        assert_eq!(bool_false.to_bool().unwrap(), false);
    }
}

#[test]
fn test_fixnum() {
    unsafe {
        let number = MRValue::fixnum(-1291657);
        assert_eq!(number.to_i32().unwrap(), -1291657);
    }
}

#[test]
fn test_float() {
    unsafe {
        let mrb = mrb_open();

        let number = MRValue::float(mrb, -1291657.37);
        assert_eq!(number.to_f64().unwrap(), -1291657.37);

        mrb_close(mrb);
    }
}

#[test]
fn test_string() {
    unsafe {
        let mrb = mrb_open();

        let string_value = MRValue::string(mrb, "qwerty");
        assert_eq!(string_value.to_str(mrb).unwrap(), "qwerty");

        mrb_close(mrb);
    }
}

#[test]
fn test_proc() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let code = CString::new("1 + 1").unwrap().as_ptr();
        let parser = mrb_parse_string(mrb, code, context);
        let prc = mrb_generate_code(mrb, parser);

        let result = mrb_run(mrb, MRValue::prc(mrb, prc).to_prc().unwrap(), mrb_top_self(mrb));

        assert_eq!(result.to_i32().unwrap(), 2);

        mrb_close(mrb);
    }
}

#[test]
fn test_obj() {
    use std::mem;
    use std::rc::Rc;

    unsafe {
        struct Cont {
            value: i32
        }

        let mrb = mrb_open();

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let cont_class = mrb_define_class(mrb, CString::new("Cont").unwrap().as_ptr(), obj_class);

        mrb_ext_set_instance_tt(cont_class, MRType::MRB_TT_DATA);

        extern "C" fn free(mrb: *mut MRState, ptr: *const u8) {
            unsafe {
                mem::transmute::<*const u8, Rc<Cont>>(ptr);
            }
        }

        let data_type = MRDataType { name: CString::new("Cont").unwrap().as_ptr(), free: free };

        let obj = Cont { value: 3 };
        let obj = MRValue::obj(mrb, cont_class, obj, &data_type);
        let obj: Rc<Cont> = obj.to_obj(mrb, &data_type).unwrap();

        assert_eq!(obj.value, 3);

        mrb_close(mrb);
    }
}

#[test]
fn test_obj_init() {
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

        mrb_ext_set_instance_tt(cont_class, MRType::MRB_TT_DATA);

        extern "C" fn free(mrb: *mut MRState, ptr: *const u8) {
            unsafe {
                mem::transmute::<*const u8, Rc<Cont>>(ptr);
            }
        }

        extern "C" fn init(mrb: *mut MRState, slf: MRValue) -> MRValue {
            unsafe {
                let cont = Cont { value: 3 };
                let rc = Rc::new(cont);
                let ptr = mem::transmute::<Rc<Cont>, *const u8>(rc);

                let data_type = mem::transmute::<*const u8, *const MRDataType>(mrb_ext_get_ud(mrb));

                mrb_ext_data_init(&slf as *const MRValue, ptr, data_type);

                slf
            }
        }

        let data_type = &MRDataType { name: CString::new("Cont").unwrap().as_ptr(), free: free };

        mrb_ext_set_ud(mrb, mem::transmute::<&MRDataType, *const u8>(data_type));

        mrb_define_method(mrb, cont_class, CString::new("initialize").unwrap().as_ptr(), init, 1 << 12);

        let code = CString::new("Cont.new").unwrap().as_ptr();
        let obj = mrb_load_string_cxt(mrb, code, context).to_obj::<Cont>(mrb, data_type).unwrap();

        assert_eq!(obj.value, 3);

        mrb_close(mrb);
    }
}

#[test]
fn test_obj_scoping() {
    use std::mem;
    use std::rc::Rc;

    unsafe {
        static mut dropped: bool = false;

        struct Cont {
            value: i32
        }

        impl Drop for Cont {
            fn drop(&mut self) {
                unsafe {
                    dropped = true;
                }
            }
        }

        let mrb = mrb_open();

        let obj_class = mrb_class_get(mrb, CString::new("Object").unwrap().as_ptr());
        let cont_class = mrb_define_class(mrb, CString::new("Cont").unwrap().as_ptr(), obj_class);

        mrb_ext_set_instance_tt(cont_class, MRType::MRB_TT_DATA);

        extern "C" fn free(mrb: *mut MRState, ptr: *const u8) {
            unsafe {
                mem::transmute::<*const u8, Rc<Cont>>(ptr);
            }
        }

        let data_type = MRDataType { name: CString::new("Cont").unwrap().as_ptr(), free: free };

        {
            let orig = Cont { value: 3 };

            {
                let obj = MRValue::obj(mrb, cont_class, orig, &data_type);
                let obj: Rc<Cont> = obj.to_obj(mrb, &data_type).unwrap();

                assert_eq!(obj.value, 3);

                assert_eq!(dropped, false);
            }

            assert_eq!(dropped, false);
        }

        assert_eq!(dropped, false);

        mrb_close(mrb);

        assert_eq!(dropped, true);
    }
}

#[test]
fn test_array() {
    unsafe {
        let mrb = mrb_open();

        let vec: Vec<MRValue> = [1, 2, 3].iter().map(|v| MRValue::fixnum(*v)).collect();

        let array = MRValue::array(mrb, &vec);

        assert_eq!(array.to_vec(mrb).unwrap(), vec);

        mrb_close(mrb);
    }
}
