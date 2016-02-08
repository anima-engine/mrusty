use super::*;

#[test]
fn test_open_close() {
    unsafe {
        let mrb = mrb_open();

        mrb_close(mrb);
    }
}

#[test]
fn test_exec_context() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        mrbc_filename(mrb, context, "script.rb\0".as_ptr());

        let code = "'' + 0\0".as_ptr();

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

        let code = "1 + 1\0".as_ptr();
        let parser = mrb_parse_string(mrb, code, context);
        let prc = mrb_generate_code(mrb, parser);

        let result = mrb_run(mrb, prc, mrb_top_self(mrb));

        assert_eq!(result.to_i32().unwrap(), 2);

        mrb_close(mrb);
    }
}

#[test]
fn test_class_defined() {
    unsafe {
        let mrb = mrb_open();

        let obj_class = "Object\0".as_ptr();

        assert_eq!(mrb_class_defined(mrb, obj_class), 1);

        mrb_close(mrb);
    }
}

#[test]
fn test_define_class() {
    unsafe {
        let mrb = mrb_open();

        let obj_class = "Object\0".as_ptr();
        let new_class = "Mine\0".as_ptr();

        let obj_class = mrb_class_get(mrb, "Object\0".as_ptr());

        mrb_define_class(mrb, new_class, obj_class);

        assert_eq!(mrb_class_defined(mrb, new_class), 1);

        mrb_close(mrb);
    }
}

#[test]
fn test_define_module() {
    unsafe {
        let mrb = mrb_open();

        let new_module = "Mine\0".as_ptr();

        mrb_define_module(mrb, new_module);

        mrb_module_get(mrb, new_module);

        mrb_close(mrb);
    }
}

#[test]
fn test_include_module() {
    unsafe {
        let mrb = mrb_open();

        let kernel = "Kernel\0".as_ptr();
        let new_module = "Mine\0".as_ptr();

        let new_module = mrb_define_module(mrb, new_module);
        let kernel = mrb_module_get(mrb, kernel);

        mrb_include_module(mrb, kernel, new_module);

        mrb_close(mrb);
    }
}

#[test]
fn test_prepend_module() {
    unsafe {
        let mrb = mrb_open();

        let kernel = "Kernel\0".as_ptr();
        let new_module = "Mine\0".as_ptr();

        let new_module = mrb_define_module(mrb, new_module);
        let kernel = mrb_module_get(mrb, kernel);

        mrb_prepend_module(mrb, kernel, new_module);

        mrb_close(mrb);
    }
}

#[test]
fn test_define_method() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_class = "Object\0".as_ptr();
        let new_class = "Mine\0".as_ptr();

        let obj_class = mrb_class_get(mrb, "Object\0".as_ptr());
        let new_class = mrb_define_class(mrb, new_class, obj_class);

        extern "C" fn job(mrb: *mut MRState, slf: MRValue) -> MRValue {
            unsafe {
                MRValue::fixnum(2)
            }
        }

        mrb_define_method(mrb, new_class, "job\0".as_ptr(), job, 0);

        let code = "Mine.new.job\0".as_ptr();

        assert_eq!(mrb_load_string_cxt(mrb, code, context).to_i32().unwrap(), 2);

        mrb_close(mrb);
    }
}

#[test]
fn test_define_class_method() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_class = "Object\0".as_ptr();
        let new_class = "Mine\0".as_ptr();

        let obj_class = mrb_class_get(mrb, "Object\0".as_ptr());
        let new_class = mrb_define_class(mrb, new_class, obj_class);

        extern "C" fn job(mrb: *mut MRState, slf: MRValue) -> MRValue {
            unsafe {
                MRValue::fixnum(2)
            }
        }

        mrb_define_class_method(mrb, new_class, "job\0".as_ptr(), job, 0);

        let code = "Mine.job\0".as_ptr();

        assert_eq!(mrb_load_string_cxt(mrb, code, context).to_i32().unwrap(), 2);

        mrb_close(mrb);
    }
}

#[test]
fn test_define_module_function() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let new_module = "Mine\0".as_ptr();
        let new_module = mrb_define_module(mrb, new_module);

        extern "C" fn job(mrb: *mut MRState, slf: MRValue) -> MRValue {
            unsafe {
                MRValue::fixnum(2)
            }
        }

        mrb_define_module_function(mrb, new_module, "job\0".as_ptr(), job, 0);

        let code = "Mine.job\0".as_ptr();

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

        let obj_class = "Object\0".as_ptr();
        let obj_class = mrb_class_get(mrb, "Object\0".as_ptr());

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

        mrb_funcall_with_block(mrb, MRValue::fixnum(5), mrb_intern_cstr(mrb, "times\0".as_ptr()), 0, ptr::null() as *const MRValue, prc);

        mrb_close(mrb);
    }
}

#[test]
fn test_nil() {
    unsafe {
        let mrb = mrb_open();

        let nil = MRValue::nil();
        let result = mrb_funcall(mrb, nil, "to_s\0".as_ptr(), 0);

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

        let string_value = MRValue::str(mrb, "qwerty\0");
        assert_eq!(string_value.to_str(mrb).unwrap(), "qwerty");

        mrb_close(mrb);
    }
}

#[test]
fn test_proc() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let code = "1 + 1\0".as_ptr();
        let parser = mrb_parse_string(mrb, code, context);
        let prc = mrb_generate_code(mrb, parser);

        let result = mrb_run(mrb, MRValue::prc(mrb, prc).to_prc().unwrap(), mrb_top_self(mrb));

        assert_eq!(result.to_i32().unwrap(), 2);

        mrb_close(mrb);
    }
}

#[test]
fn test_object() {
    unsafe {
        let mrb = mrb_open();

        #[derive(Clone, Copy, Debug, PartialEq)]
        struct Container {
            value: i32
        }

        let container = Container { value: 42 };
        let obj = MRValue::obj::<Container>(mrb, &container);

        assert_eq!(obj.to_obj::<Container>().unwrap(), container);

        mrb_close(mrb);
    }
}
