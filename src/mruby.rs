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
/// use mrusty::*;
/// let mruby = MRuby::new();
/// let result = mruby.run("2 + 2 == 5").unwrap();
///
/// assert_eq!(result.to_bool().unwrap(), false);
/// ```
pub struct MRuby {
    mrb: *mut MRState,
    ctx: *mut MRContext,
    classes: Box<HashMap<String, (*mut MRClass, MRDataType)>>,
    methods: Box<HashMap<String, Box<HashMap<u32, Box<Fn(Rc<RefCell<MRuby>>, Value) -> Value>>>>>
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

            mrb_ext_set_ud(mrb, mem::transmute::<Rc<RefCell<MRuby>>, *const u8>(mruby.clone()));

            mruby
        }
    }

    fn close(&self) {
        unsafe {
            mrb_close(self.mrb);
        }
    }
}

pub trait MRubyImpl {
    /// Runs mruby `script` on a state and context and returns a `Value` in an `Ok`
    /// or an `Err` containing an mruby exception.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    /// let result = mruby.run("true").unwrap();
    ///
    /// assert_eq!(result.to_bool().unwrap(), true);
    /// ```
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    /// let result = mruby.run("'' + 1");
    ///
    /// assert_eq!(result, Err("TypeError: expected String"));
    /// ```
    fn run<'a>(&'a self, script: &str) -> Result<Value, &'a str>;

    /// Defines Rust type `T` as an mruby `Class` named `name`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// }
    ///
    /// mruby.def_class::<Cont>("Container");
    /// ```
    fn def_class<T>(&self, name: &str);

    /// Defines Rust type `T` as an mruby `Class` named `name`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    ///
    /// struct Cont;
    ///
    /// mruby.def_class::<Cont>("Container");
    /// mruby.def_method("Container", "hi", |mruby, _slf| mruby.fixnum(2));
    ///
    /// let result = mruby.run("Container.new.hi").unwrap();
    ///
    /// assert_eq!(result.to_i32().unwrap(), 2);
    /// ```
    fn def_method<F>(&self, class: &str, name: &str, method: F)
        where F: Fn(Rc<RefCell<MRuby>>, Value) -> Value + 'static;

    /// Creates mruby `Value` `nil`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    ///
    /// struct Cont;
    ///
    /// mruby.def_class::<Cont>("Container");
    /// mruby.def_method("Container", "nil", |mruby, _slf| mruby.nil());
    ///
    /// let result = mruby.run("Container.new.nil.nil?").unwrap();
    ///
    /// assert_eq!(result.to_bool().unwrap(), true);
    /// ```
    fn nil(&self) -> Value;

    /// Creates mruby `Value` containing `true` or `false`.
    ///
    /// # Examples
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    ///
    /// let b = mruby.bool(true);
    ///
    /// assert_eq!(b.to_bool().unwrap(), true);
    /// ```
    fn bool(&self, value: bool) -> Value;

    /// Creates mruby `Value` of `Class` `Fixnum`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    ///
    /// let fixn = mruby.fixnum(2);
    ///
    /// assert_eq!(fixn.to_i32().unwrap(), 2);
    /// ```
    fn fixnum(&self, value: i32) -> Value;

    /// Creates mruby `Value` of `Class` `Float`.
    ///
    /// # Examples
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    ///
    /// let fl = mruby.float(2.3);
    ///
    /// assert_eq!(fl.to_f64().unwrap(), 2.3);
    /// ```
    fn float(&self, value: f64) -> Value;

    /// Creates mruby `Value` of `Class` `String`.
    ///
    /// # Examples
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    ///
    /// let s = mruby.string("hi");
    ///
    /// assert_eq!(s.to_str().unwrap(), "hi");
    /// ```
    fn string(&self, value: &str) -> Value;

    /// Creates mruby `Value` of `Class` `name` containing a Rust object of type `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// }
    ///
    /// mruby.def_class::<Cont>("Container");
    ///
    /// let value = mruby.obj(Cont { value: 3 }, "Container");
    /// ```
    fn obj<T>(&self, obj: T, name: &str) -> Value;

    /// Creates mruby `Value` of `Class` `Array`.
    ///
    /// # Examples
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    ///
    /// let array = mruby.array(&vec![
    ///     mruby.fixnum(1),
    ///     mruby.fixnum(2),
    ///     mruby.fixnum(3)
    /// ]);
    ///
    /// assert_eq!(array.to_vec().unwrap(), vec![
    ///     mruby.fixnum(1),
    ///     mruby.fixnum(2),
    ///     mruby.fixnum(3)
    /// ]);
    /// ```
    fn array(&self, value: &Vec<Value>) -> Value;
}

impl MRubyImpl for Rc<RefCell<MRuby>> {
    fn run<'a>(&'a self, script: &str) -> Result<Value, &'a str> {
        unsafe {
            let value = mrb_load_string_cxt(self.borrow().mrb, CString::new(script).unwrap().as_ptr(), self.borrow().ctx);
            let exc = mrb_ext_get_exc(self.borrow().mrb);

            match exc.typ {
                MRType::MRB_TT_STRING => Err(exc.to_str(self.borrow().mrb).unwrap()),
                _                     => Ok(Value::new(self.clone(), value))
            }
        }
    }

    fn def_class<T>(&self, name: &str) {
        unsafe {
            let name = name.to_string();

            let c_name = CString::new(name.clone()).unwrap();
            let object = CString::new("Object").unwrap();
            let object = mrb_class_get(self.borrow().mrb, object.as_ptr());

            let class = mrb_define_class(self.borrow().mrb, c_name.as_ptr(), object);

            mrb_ext_set_instance_tt(class, MRType::MRB_TT_DATA);

            extern "C" fn free<T>(_mrb: *mut MRState, ptr: *const u8) {
                unsafe {
                    Box::from_raw(ptr as *mut T);
                }
            }

            let data_type = MRDataType { name: c_name.as_ptr(), free: free::<T> };

            self.borrow_mut().classes.insert(name.clone(), (class, data_type));
            self.borrow_mut().methods.insert(name, Box::new(HashMap::new()));
        }
    }

    fn def_method<F>(&self, class: &str, name: &str, method: F)
        where F: Fn(Rc<RefCell<MRuby>>, Value) -> Value + 'static {
        {
            let sym = unsafe {
                mrb_intern_cstr(self.borrow().mrb, CString::new(name.clone()).unwrap().as_ptr())
            };

            let mut borrow = self.borrow_mut();

            let methods = match borrow.methods.get_mut(class) {
                Some(methods) => methods,
                None       => panic!("{} class not found.", class)
            };

            methods.insert(sym, Box::new(method));
        }

        extern "C" fn call_method(mrb: *mut MRState, slf: MRValue) -> MRValue {
            unsafe {
                let ptr = mrb_ext_get_ud(mrb);
                let mruby = mem::transmute::<*const u8, Rc<RefCell<MRuby>>>(ptr);

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

                method(mruby.clone(), value).value
            }
        }

        let borrow = self.borrow();

        let class = match borrow.classes.get(class) {
            Some(class) => class,
            None       => panic!("{} class not found.", class)
        };

        unsafe {
            mrb_define_method(self.borrow().mrb, class.0, CString::new(name).unwrap().as_ptr(), call_method, 1 << 12);
        }
    }

    fn nil(&self) -> Value {
        unsafe {
            Value::new(self.clone(), MRValue::nil())
        }
    }

    fn bool(&self, value: bool) -> Value {
        unsafe {
            Value::new(self.clone(), MRValue::bool(value))
        }
    }

    fn fixnum(&self, value: i32) -> Value {
        unsafe {
            Value::new(self.clone(), MRValue::fixnum(value))
        }
    }

    fn float(&self, value: f64) -> Value {
        unsafe {
            Value::new(self.clone(), MRValue::float(self.borrow().mrb, value))
        }
    }

    fn string(&self, value: &str) -> Value {
        unsafe {
            Value::new(self.clone(), MRValue::string(self.borrow().mrb, value))
        }
    }

    fn obj<T>(&self, obj: T, name: &str) -> Value {
        let borrow = self.borrow();

        let class = match borrow.classes.get(name) {
            Some(class) => class,
            None       => panic!("{} class not found.", name)
        };

        let boxed = Box::into_raw(Box::new(obj));

        unsafe {
            Value::new(self.clone(), MRValue::obj(self.borrow().mrb, class.0 as *mut MRClass, boxed, &class.1))
        }
    }

    fn array(&self, value: &Vec<Value>) -> Value {
        let array: Vec<MRValue> = value.iter().map(|value| {
            value.value
        }).collect();

        unsafe {
            Value::new(self.clone(), MRValue::array(self.borrow().mrb, &array))
        }
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
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    /// let result = mruby.run("
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
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    /// let result = mruby.run("
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
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    /// let result = mruby.run("
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
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    /// let result = mruby.run("
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
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// }
    ///
    /// mruby.def_class::<Cont>("Container");
    ///
    /// let value = mruby.obj(Cont { value: 3 }, "Container");
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

    /// Casts mruby `Value` of `Class` `Array` to Rust type `Vec<Value>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    /// let result = mruby.run("
    ///   [1, 2, 3].map(&:to_s)
    /// ").unwrap();
    ///
    /// assert_eq!(result.to_vec().unwrap(), vec![
    ///     mruby.string("1"),
    ///     mruby.string("2"),
    ///     mruby.string("3")
    /// ]);
    /// ```
    pub fn to_vec(&self) -> Result<Vec<Value>, &str> {
        unsafe {
            self.value.to_vec(self.mruby.borrow().mrb).map(|vec| {
                vec.iter().map(|mrvalue| {
                    Value::new(self.mruby.clone(), *mrvalue)
                }).collect()
            })
        }
    }
}

use std::fmt;

impl PartialEq<Value> for Value {
    fn eq(&self, other: &Value) -> bool {
        unsafe {
            let call = CString::new("==").unwrap().as_ptr();
            let result = mrb_funcall(self.mruby.borrow().mrb, self.value, call, 1, other.value);

            result.to_bool().unwrap()
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value {{ {:?} }}", self.value)
    }
}
