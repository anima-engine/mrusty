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

use std::collections::HashMap;
use std::ffi::CString;

use super::mruby_ffi::*;

/// A safe `struct` for the mruby API.
///
/// # Examples
///
/// ```
/// # use mrusty::MRuby;
/// let mruby = MRuby::new();
/// let result = mruby.run("2 + 2 == 5").unwrap();
///
/// assert_eq!(result.to_bool().unwrap(), false);
/// ```
#[derive(Debug)]
pub struct MRuby {
    mrb: *mut MRState,
    ctx: *mut MRContext,
    classes: Box<HashMap<String, (*mut MRClass, MRDataType)>>
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
    pub fn new() -> MRuby {
        unsafe {
            let mrb = mrb_open();

            MRuby {
                mrb: mrb,
                ctx: mrbc_context_new(mrb),
                classes: Box::new(HashMap::new())
            }
        }
    }

    fn close(&self) {
        unsafe {
            mrb_close(self.mrb);
        }
    }

    /// Runs mruby `script` on a state and context and returns a `Value` in an `Ok`
    /// or an `Err` containing an mruby exception.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// let mruby = MRuby::new();
    /// let result = mruby.run("true").unwrap();
    ///
    /// assert_eq!(result.to_bool().unwrap(), true);
    /// ```
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// let mruby = MRuby::new();
    /// let result = mruby.run("'' + 1");
    ///
    /// assert_eq!(result, Err("TypeError: expected String"));
    /// ```
    pub fn run(&self, script: &str) -> Result<Value, &str> {
        unsafe {
            let value = mrb_load_string_cxt(self.mrb, CString::new(script).unwrap().as_ptr(), self.ctx);
            let exc = mrb_ext_get_exc(self.mrb);

            match exc.typ {
                MRType::MRB_TT_STRING => Err(exc.to_str(self.mrb).unwrap()),
                _                     => Ok(Value::new(self, value))
            }
        }
    }

    /// Defines Rust type `T` as an mruby `Class` named `name`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// let mut mruby = MRuby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// }
    ///
    /// mruby.define_class::<Cont>("Container");
    /// ```
    pub fn define_class<T>(&mut self, name: &str) {
        unsafe {
            let name = name.to_string();

            let c_name = CString::new(name.clone()).unwrap();
            let object = CString::new("Object").unwrap();
            let object = mrb_class_get(self.mrb, object.as_ptr());

            let class = mrb_define_class(self.mrb, c_name.as_ptr(), object);

            mrb_ext_set_instance_tt(class, MRType::MRB_TT_DATA);

            extern "C" fn free<T>(_mrb: *mut MRState, ptr: *const u8) {
                unsafe {
                    Box::from_raw(ptr as *mut T);
                }
            }

            let data_type = MRDataType { name: c_name.as_ptr(), free: free::<T> };

            self.classes.insert(name, (class, data_type));
        }
    }

    /// Creates mruby `Value` of `Class` `name` containing a Rust object of type `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// let mut mruby = MRuby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// }
    ///
    /// mruby.define_class::<Cont>("Container");
    ///
    /// let value = mruby.obj(Cont { value: 3 }, "Container");
    /// ```
    pub fn obj<T>(&self, obj: T, name: &str) -> Value {
        let pair = match self.classes.get(name) {
            Some(pair) => pair,
            None       => panic!("{} class not found.", name)
        };

        let boxed = Box::into_raw(Box::new(obj));

        unsafe {
            Value::new(self, MRValue::obj(self.mrb, pair.0 as *mut MRClass, boxed, &pair.1))
        }
    }
}

impl Drop for MRuby {
    fn drop(&mut self) {
        self.close();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Value<'a> {
    mruby: &'a MRuby,
    value: MRValue
}

impl<'a> Value<'a> {
    fn new(mruby: &'a MRuby, value: MRValue) -> Value {
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
    /// let mruby = MRuby::new();
    /// let result = mruby.run("
    ///   [1, 2, 3].map(&:to_s).join
    /// ").unwrap();
    ///
    /// assert_eq!(result.to_str().unwrap(), "123");
    /// ```
    pub fn to_str<'b>(&self) -> Result<&'b str, &str> {
        unsafe {
            self.value.to_str(self.mruby.mrb)
        }
    }

    /// Casts mruby `Value` of `Class` `name` to Rust type `&T`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// let mut mruby = MRuby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// }
    ///
    /// mruby.define_class::<Cont>("Container");
    ///
    /// let value = mruby.obj(Cont { value: 3 }, "Container");
    /// let cont: &Cont = value.to_obj("Container").unwrap();
    ///
    /// assert_eq!(cont.value, 3);
    /// ```
    pub fn to_obj<T>(&self, name: &str) -> Result<&T, &str> {
        unsafe {
            let pair = match self.mruby.classes.get(name) {
                Some(pair) => pair,
                None       => panic!("{} class not found.", name)
            };

            self.value.to_obj::<T>(self.mruby.mrb, &pair.1)
        }
    }
}

impl<'a> PartialEq<Value<'a>> for Value<'a> {
    fn eq(&self, other: &Value) -> bool {
        self.value.eq(&other.value)
    }
}
