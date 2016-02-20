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

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;
use std::mem;
use std::rc::Rc;

use super::mruby_ffi::*;

/// A safe `struct` for the mruby API.
///
/// # Examples
///
/// ```
/// # use mrusty::MRuby;
/// # use mrusty::run;
/// let mruby = MRuby::new();
/// let result = run(&mruby, "2 + 2 == 5").unwrap();
///
/// assert_eq!(result.to_bool().unwrap(), false);
/// ```
pub struct MRuby {
    mrb: *mut MRState,
    ctx: *mut MRContext,
    classes: Box<HashMap<String, (*mut MRClass, MRDataType)>>,
    methods: Box<HashMap<String, Box<HashMap<u32, Box<Fn(&Rc<RefCell<MRuby>>, Value) -> Value>>>>>
}

impl MRuby {
    /// Creates an mruby state and context stored in a `struct`.
    ///
    /// # Example
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// let mruby = MRuby::new();
    /// ```
    pub fn new() -> Rc<RefCell<MRuby>> {
        unsafe {
            let mrb = mrb_open();

            let mruby = Rc::new(RefCell::new(
                MRuby {
                    mrb: mrb,
                    ctx: mrbc_context_new(mrb),
                    classes: Box::new(HashMap::new()),
                    methods: Box::new(HashMap::new())
                }
            ));

            mrb_ext_set_ud(mrb, mem::transmute::<&Rc<RefCell<MRuby>>, *const u8>(&mruby));

            mruby
        }
    }

    fn close(&self) {
        unsafe {
            mrb_close(self.mrb);
        }
    }
}

/// Runs mruby `script` on a state and context and returns a `Value` in an `Ok`
/// or an `Err` containing an mruby exception.
///
/// # Examples
///
/// ```
/// # use mrusty::MRuby;
/// # use mrusty::run;
/// let mruby = MRuby::new();
/// let result = run(&mruby, "true").unwrap();
///
/// assert_eq!(result.to_bool().unwrap(), true);
/// ```
///
/// ```
/// # use mrusty::MRuby;
/// # use mrusty::run;
/// let mruby = MRuby::new();
/// let result = run(&mruby, "'' + 1");
///
/// assert_eq!(result, Err("TypeError: expected String"));
/// ```
pub fn run<'a>(mruby: &'a Rc<RefCell<MRuby>>, script: &str) -> Result<Value, &'a str> {
    unsafe {
        let value = mrb_load_string_cxt(mruby.borrow().mrb, CString::new(script).unwrap().as_ptr(), mruby.borrow().ctx);
        let exc = mrb_ext_get_exc(mruby.borrow().mrb);

        match exc.typ {
            MRType::MRB_TT_STRING => Err(exc.to_str(mruby.borrow().mrb).unwrap()),
            _                     => Ok(Value::new(mruby.clone(), value))
        }
    }
}

/// Defines Rust type `T` as an mruby `Class` named `name`.
///
/// # Examples
///
/// ```
/// # use mrusty::MRuby;
/// # use mrusty::def_class;
/// let mut mruby = MRuby::new();
///
/// struct Cont {
///     value: i32
/// }
///
/// def_class::<Cont>(&mruby, "Container");
/// ```
pub fn def_class<T>(mruby: &Rc<RefCell<MRuby>>, name: &str) {
    unsafe {
        let name = name.to_string();

        let c_name = CString::new(name.clone()).unwrap();
        let object = CString::new("Object").unwrap();
        let object = mrb_class_get(mruby.borrow().mrb, object.as_ptr());

        let class = mrb_define_class(mruby.borrow().mrb, c_name.as_ptr(), object);

        mrb_ext_set_instance_tt(class, MRType::MRB_TT_DATA);

        extern "C" fn free<T>(_mrb: *mut MRState, ptr: *const u8) {
            unsafe {
                Box::from_raw(ptr as *mut T);
            }
        }

        let data_type = MRDataType { name: c_name.as_ptr(), free: free::<T> };

        mruby.borrow_mut().classes.insert(name.clone(), (class, data_type));
        mruby.borrow_mut().methods.insert(name, Box::new(HashMap::new()));
    }
}

/// Defines Rust type `T` as an mruby `Class` named `name`.
///
/// # Examples
///
/// ```
/// # use mrusty::MRuby;
/// # use mrusty::run;
/// # use mrusty::def_class;
/// # use mrusty::def_method;
/// # use mrusty::fixnum;
/// let mut mruby = MRuby::new();
///
/// struct Cont;
///
/// def_class::<Cont>(&mruby, "Container");
/// def_method(&mruby, "Container", "hi", |mruby, _slf| fixnum(&mruby, 2));
///
/// let result = run(&mruby, "Container.new.hi").unwrap();
///
/// assert_eq!(result.to_i32().unwrap(), 2);
/// ```
pub fn def_method<F>(mruby: &Rc<RefCell<MRuby>>, class: &str, name: &str, method: F)
    where F: Fn(&Rc<RefCell<MRuby>>, Value) -> Value + 'static {
    {
        let sym = unsafe {
            mrb_intern_cstr(mruby.borrow().mrb, CString::new(name.clone()).unwrap().as_ptr())
        };

        let mut borrow = mruby.borrow_mut();

        let methods = match borrow.methods.get_mut(class) {
            Some(methods) => methods,
            None       => panic!("{} class not found.", class)
        };

        methods.insert(sym, Box::new(method));
    }

    extern "C" fn call_method(mrb: *mut MRState, slf: MRValue) -> MRValue {
        unsafe {
            let ptr = mrb_ext_get_ud(mrb);
            let mruby = mem::transmute::<*const u8, &Rc<RefCell<MRuby>>>(ptr);

            let value = Value::new(mruby.clone(), slf);

            let class = mrb_funcall(mrb, slf, CString::new("class").unwrap().as_ptr(), 0);
            let class = mrb_funcall(mrb, class, CString::new("to_s").unwrap().as_ptr(), 0);
            let class = class.to_str(mrb).unwrap();

            let borrow = mruby.borrow();

            let methods = match borrow.methods.get(class) {
                Some(methods) => methods,
                None          => panic!("{} class not found.", class)
            };

            let sym = mrb_ext_get_mid(mrb);

            let method = match methods.get(&sym) {
                Some(method) => method,
                None         => panic!("Method not found.")
            };

            method(mruby, value).value
        }
    }

    let borrow = mruby.borrow();

    let class = match borrow.classes.get(class) {
        Some(class) => class,
        None       => panic!("{} class not found.", class)
    };

    unsafe {
        mrb_define_method(mruby.borrow().mrb, class.0, CString::new(name).unwrap().as_ptr(), call_method, 1 << 12);
    }
}

/// Creates mruby `Value` of `Class` `Fixnum`.
///
/// # Examples
///
/// ```
/// # use mrusty::MRuby;
/// # use mrusty::fixnum;
/// let mut mruby = MRuby::new();
///
/// let fixn = fixnum(&mruby, 2);
///
/// assert_eq!(fixn.to_i32().unwrap(), 2);
/// ```
pub fn fixnum(mruby: &Rc<RefCell<MRuby>>, value: i32) -> Value {
    unsafe {
        Value::new(mruby.clone(), MRValue::fixnum(value))
    }
}

/// Creates mruby `Value` of `Class` `name` containing a Rust object of type `T`.
///
/// # Examples
///
/// ```
/// # use mrusty::MRuby;
/// # use mrusty::def_class;
/// # use mrusty::obj;
/// let mut mruby = MRuby::new();
///
/// struct Cont {
///     value: i32
/// }
///
/// def_class::<Cont>(&mruby, "Container");
///
/// let value = obj(&mruby, Cont { value: 3 }, "Container");
/// ```
pub fn obj<T>(mruby: &Rc<RefCell<MRuby>>, obj: T, name: &str) -> Value {
    let borrow = mruby.borrow();

    let class = match borrow.classes.get(name) {
        Some(class) => class,
        None       => panic!("{} class not found.", name)
    };

    let boxed = Box::into_raw(Box::new(obj));

    unsafe {
        Value::new(mruby.clone(), MRValue::obj(mruby.borrow().mrb, class.0 as *mut MRClass, boxed, &class.1))
    }
}

impl Drop for MRuby {
    fn drop(&mut self) {
        self.close();
    }
}

#[derive(Clone)]
pub struct Value {
    mruby: Rc<RefCell<MRuby>>,
    value: MRValue
}

impl Value {
    fn new(mruby: Rc<RefCell<MRuby>>, value: MRValue) -> Value {
        Value {
            mruby: mruby,
            value: value
        }
    }

    /// Casts a `Value` and returns a `bool` in an `Ok` or an `Err` if the types mismatch.
    ///
    /// # Example
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::run;
    /// let mruby = MRuby::new();
    /// let result = run(&mruby, "
    ///   def pos(n)
    ///     n > 0
    ///   end
    ///
    ///   pos 1
    /// ").unwrap();
    ///
    /// assert_eq!(result.to_bool().unwrap(), true);
    /// ```
    pub fn to_bool(&self) -> Result<bool, &str> {
        unsafe {
            self.value.to_bool()
        }
    }

    /// Casts a `Value` and returns an `i32` in an `Ok` or an `Err` if the types mismatch.
    ///
    /// # Example
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::run;
    /// let mruby = MRuby::new();
    /// let result = run(&mruby, "
    ///   def fact(n)
    ///     n > 1 ? fact(n - 1) * n : 1
    ///   end
    ///
    ///   fact 5
    /// ").unwrap();
    ///
    /// assert_eq!(result.to_i32().unwrap(), 120);
    /// ```
    pub fn to_i32(&self) -> Result<i32, &str> {
        unsafe {
            self.value.to_i32()
        }
    }

    /// Casts a `Value` and returns an `f64` in an `Ok` or an `Err` if the types mismatch.
    ///
    /// # Example
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::run;
    /// let mruby = MRuby::new();
    /// let result = run(&mruby, "
    ///   3 / 2.0
    /// ").unwrap();
    ///
    /// assert_eq!(result.to_f64().unwrap(), 1.5);
    /// ```
    pub fn to_f64(&self) -> Result<f64, &str> {
        unsafe {
            self.value.to_f64()
        }
    }

    /// Casts a `Value` and returns a `&str` in an `Ok` or an `Err` if the types mismatch.
    ///
    /// # Example
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::run;
    /// let mruby = MRuby::new();
    /// let result = run(&mruby, "
    ///   [1, 2, 3].map(&:to_s).join
    /// ").unwrap();
    ///
    /// assert_eq!(result.to_str().unwrap(), "123");
    /// ```
    pub fn to_str<'b>(&self) -> Result<&'b str, &str> {
        unsafe {
            self.value.to_str(self.mruby.borrow().mrb)
        }
    }

    /// Casts mruby `Value` of `Class` `name` to Rust type `&T`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::def_class;
    /// # use mrusty::obj;
    /// let mut mruby = MRuby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// }
    ///
    /// def_class::<Cont>(&mruby, "Container");
    ///
    /// let value = obj(&mruby, Cont { value: 3 }, "Container");
    /// let cont: &Cont = value.to_obj("Container").unwrap();
    ///
    /// assert_eq!(cont.value, 3);
    /// ```
    pub fn to_obj<T>(&self, name: &str) -> Result<&T, &str> {
        unsafe {
            let borrow = self.mruby.borrow();

            let class = match borrow.classes.get(name) {
                Some(class) => class,
                None       => panic!("{} class not found.", name)
            };

            self.value.to_obj::<T>(self.mruby.borrow().mrb, &class.1)
        }
    }
}

use std::fmt;

impl PartialEq<Value> for Value {
    fn eq(&self, other: &Value) -> bool {
        self.value.eq(&other.value)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value {{ {:?} }}", self.value)
    }
}
