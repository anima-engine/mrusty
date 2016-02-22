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

use std::any::Any;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;
use std::mem;
use std::rc::Rc;

pub use super::mruby_ffi::*;

/// Not meant to be called directly.
#[doc(hidden)]
#[macro_export]
macro_rules! init {
    () => ();
    ( $name:ident, bool )   => (let $name = uninitialized::<bool>(););
    ( $name:ident, i32 )    => (let $name = uninitialized::<i32>(););
    ( $name:ident, f64 )    => (let $name = uninitialized::<f64>(););
    ( $name:ident, str )    => (let $name = uninitialized::<*const c_char>(););
    ( $name:ident, Value )  => (let $name = uninitialized::<MRValue>(););
    ( $name:ident : $t:tt ) => (init!($name, $t));
    ( $name:ident : $t:tt, $($names:ident : $ts:tt),+ ) => {
        init!($name, $t);
        init!($( $names : $ts ),*);
    };
}

/// Not meant to be called directly.
#[doc(hidden)]
#[macro_export]
macro_rules! sig {
    () => ("");
    ( bool )   => ("b");
    ( i32 )    => ("i");
    ( f64 )    => ("f");
    ( str )    => ("z");
    ( Value )  => ("o");
    ( $t:tt, $( $ts:tt ),+ ) => (concat!(sig!($t), sig!($( $ts ),*)));
}

/// Not meant to be called directly.
#[doc(hidden)]
#[macro_export]
macro_rules! args {
    () => ();
    ( $name:ident, bool )   => (&$name as *const bool);
    ( $name:ident, i32 )    => (&$name as *const i32);
    ( $name:ident, f64 )    => (&$name as *const f64);
    ( $name:ident, str )    => (&$name as *const *const c_char);
    ( $name:ident, Value )  => (&$name as *const MRValue);
    ( $name:ident : $t:tt ) => (args!($name, $t));
    ( $mrb:expr, $sig:expr, $name:ident : $t:tt) => {
        mrb_get_args($mrb, $sig, args!($name, $t));
    };
    ( $mrb:expr, $sig:expr, $name:ident : $t:tt, $($names:ident : $ts:tt),+ ) => {
        mrb_get_args($mrb, $sig, args!($name, $t), args!($( $names : $ts ),*));
    };
}

/// Not meant to be called directly.
#[doc(hidden)]
#[macro_export]
macro_rules! conv {
    ( $mruby:expr )                      => ();
    ( $mruby:expr, $name:ident, str )    => (let $name = CStr::from_ptr($name).to_str().unwrap(););
    ( $mruby:expr, $name:ident, Value )  => (let $name = Value::new($mruby.clone(), $name););
    ( $mruby:expr, $name:ident, $_t:ty ) => ();
    ( $mruby:expr, $name:ident : $t:tt ) => (conv!($mruby, $name, $t));
    ( $mruby:expr, $name:ident : $t:tt, $($names:ident : $ts:tt),+ ) => {
        conv!($mruby, $name, $t);
        conv!($mruby, $( $names : $ts ),*);
    };
}


/// A `macro` useful for defining Rust closures for mruby.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate mrusty;
/// use mrusty::*;
///
/// # fn main() {
/// let mruby = MRuby::new();
///
/// struct Cont;
///
/// mruby.def_class::<Cont>("Container");
/// mruby.def_method::<Cont, _>("hi", mrfn!(|mruby, slf, a: i32, b: i32| {
///     mruby.fixnum(a + b)
/// }));
///
/// let result = mruby.run("Container.new.hi 1, 2").unwrap();
///
/// assert_eq!(result.to_i32().unwrap(), 3);
/// # }
/// ```
///
/// ```
/// # #[macro_use] extern crate mrusty;
/// use mrusty::*;
///
/// # fn main() {
/// let mruby = MRuby::new();
///
/// struct Cont;
///
/// mruby.def_class::<Cont>("Container");
/// mruby.def_method::<Cont, _>("hi", mrfn!(|mruby, slf, a: str, b: str| {
///     mruby.string(&(a.to_string() + b))
/// }));
///
/// let result = mruby.run("Container.new.hi 'a', 'b'").unwrap();
///
/// assert_eq!(result.to_str().unwrap(), "ab");
/// # }
/// ```
///
/// ```
/// # #[macro_use] extern crate mrusty;
/// use mrusty::*;
///
/// # fn main() {
/// let mruby = MRuby::new();
///
/// struct Cont {
///     value: i32
/// };
///
/// mruby.def_class::<Cont>("Container");
/// mruby.def_method::<Cont, _>("gt", mrfn!(|mruby, slf, o: Value| {
///    let slf = slf.to_obj::<Cont>().unwrap();
///    let o = o.to_obj::<Cont>().unwrap();
///
///    mruby.bool(slf.value > o.value)
/// }));
///
/// let a = mruby.obj::<Cont>(Cont { value: 3 });
/// let b = mruby.obj::<Cont>(Cont { value: 2 });
///
/// let result = a.call("gt", vec![b]);
///
/// assert_eq!(result.to_bool().unwrap(), true);
/// # }
/// ```
#[macro_export]
macro_rules! mrfn {
    ( |$mruby:ident, $slf:ident| $block:expr ) => {
        |$mruby, $slf| $block
    };
    ( |$mruby:ident, $slf:ident, $( $name:ident : $t:tt ),*| $block:expr ) => {
        |$mruby, $slf| {
            use std::ffi::CStr;
            use std::ffi::CString;
            use std::mem::uninitialized;
            use std::os::raw::c_char;

            unsafe {
                init!($( $name : $t ),*);

                let mrb = $mruby.borrow().mrb;
                let sig = CString::new(sig!($( $t ),*)).unwrap().as_ptr();

                args!(mrb, sig, $( $name : $t ),*);
                conv!($mruby, $( $name : $t ),*);

                $block
            }
        }
    };
}

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
    pub mrb: *mut MRState,
    ctx: *mut MRContext,
    classes: Box<HashMap<TypeId, (*mut MRClass, MRDataType)>>,
    methods: Box<HashMap<TypeId, Box<HashMap<u32, Box<Fn(Rc<RefCell<MRuby>>, Value) -> Value>>>>>
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
    fn def_class<T: Any>(&self, name: &str);

    /// Defines Rust type `T` as an mruby `Class` named `name`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate mrusty;
    /// use mrusty::*;
    ///
    /// # fn main() {
    /// let mruby = MRuby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// };
    ///
    /// mruby.def_class::<Cont>("Container");
    /// mruby.def_method::<Cont, _>("initialize", mrfn!(|mruby, slf, v: i32| {
    ///     let cont = Cont { value: v };
    ///
    ///     slf.init(cont)
    /// }));
    /// mruby.def_method::<Cont, _>("value", mrfn!(|mruby, slf| {
    ///     let cont = slf.to_obj::<Cont>().unwrap();
    ///
    ///     mruby.fixnum(cont.value)
    /// }));
    ///
    /// let result = mruby.run("Container.new(3).value").unwrap();
    ///
    /// assert_eq!(result.to_i32().unwrap(), 3);
    /// # }
    /// ```
    fn def_method<T: Any, F>(&self, name: &str, method: F)
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
    /// mruby.def_method::<Cont, _>("nil", |mruby, _slf| mruby.nil());
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
    /// let value = mruby.obj(Cont { value: 3 });
    /// ```
    fn obj<T: Any>(&self, obj: T) -> Value;

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

    fn def_class<T: Any>(&self, name: &str) {
        unsafe {
            let c_name = CString::new(name.to_string()).unwrap();
            let object = CString::new("Object").unwrap();
            let object = mrb_class_get(self.borrow().mrb, object.as_ptr());

            let class = mrb_define_class(self.borrow().mrb, c_name.as_ptr(), object);

            mrb_ext_set_instance_tt(class, MRType::MRB_TT_DATA);

            extern "C" fn free<T>(_mrb: *mut MRState, ptr: *const u8) {
                unsafe {
                    mem::transmute::<*const u8, Rc<T>>(ptr);
                }
            }

            let data_type = MRDataType { name: c_name.as_ptr(), free: free::<T> };

            self.borrow_mut().classes.insert(TypeId::of::<T>(), (class, data_type));
            self.borrow_mut().methods.insert(TypeId::of::<T>(), Box::new(HashMap::new()));
        }
    }

    fn def_method<T: Any, F>(&self, name: &str, method: F)
        where F: Fn(Rc<RefCell<MRuby>>, Value) -> Value + 'static {
        {
            let sym = unsafe {
                mrb_intern_cstr(self.borrow().mrb, CString::new(name.clone()).unwrap().as_ptr())
            };

            let mut borrow = self.borrow_mut();

            let methods = match borrow.methods.get_mut(&TypeId::of::<T>()) {
                Some(methods) => methods,
                None          => panic!("Class not found.")
            };

            methods.insert(sym, Box::new(method));
        }

        extern "C" fn call_method<T: Any>(mrb: *mut MRState, slf: MRValue) -> MRValue {
            unsafe {
                let ptr = mrb_ext_get_ud(mrb);
                let mruby = mem::transmute::<*const u8, Rc<RefCell<MRuby>>>(ptr);

                let result = {
                    let value = Value::new(mruby.clone(), slf);

                    let borrow = mruby.borrow();

                    let methods = match borrow.methods.get(&TypeId::of::<T>()) {
                        Some(methods) => methods,
                        None          => panic!("Class not found.")
                    };

                    let sym = mrb_ext_get_mid(mrb);

                    let method = match methods.get(&sym) {
                        Some(method) => method,
                        None         => panic!("Method not found.")
                    };

                    method(mruby.clone(), value).value
                };

                mem::forget(mruby);

                result
            }
        }

        let borrow = self.borrow();

        let class = match borrow.classes.get(&TypeId::of::<T>()) {
            Some(class) => class,
            None       => panic!("Class not found.")
        };

        unsafe {
            mrb_define_method(self.borrow().mrb, class.0, CString::new(name).unwrap().as_ptr(), call_method::<T>, 1 << 12);
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

    fn obj<T: Any>(&self, obj: T) -> Value {
        let borrow = self.borrow();

        let class = match borrow.classes.get(&TypeId::of::<T>()) {
            Some(class) => class,
            None       => panic!("Class not found.")
        };

        unsafe {
            Value::new(self.clone(), MRValue::obj(self.borrow().mrb, class.0 as *mut MRClass, obj, &class.1))
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
    /// Not meant to be called directly.
    #[doc(hidden)]
    pub fn new(mruby: Rc<RefCell<MRuby>>, value: MRValue) -> Value {
        Value {
            mruby: mruby,
            value: value
        }
    }

    /// Initializes the `self` mruby object passed to `initialize` with a Rust object of type `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate mrusty;
    /// use mrusty::*;
    ///
    /// # fn main() {
    /// let mruby = MRuby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// };
    ///
    /// mruby.def_class::<Cont>("Container");
    /// mruby.def_method::<Cont, _>("initialize", mrfn!(|mruby, slf, v: i32| {
    ///     let cont = Cont { value: v };
    ///
    ///     slf.init(cont)
    /// }));
    ///
    /// let result = mruby.run("Container.new 3").unwrap();
    ///
    /// assert_eq!(result.to_obj::<Cont>().unwrap().value, 3);
    /// # }
    /// ```
    pub fn init<T: Any>(self, obj: T) -> Value {
        unsafe {
            let rc = Rc::new(obj);
            let ptr = mem::transmute::<Rc<T>, *const u8>(rc);

            let borrow = self.mruby.borrow();

            let class = match borrow.classes.get(&TypeId::of::<T>()) {
                Some(class) => class,
                None       => panic!("Class not found.")
            };

            let data_type = &class.1;

            mrb_ext_data_init(&self.value as *const MRValue, ptr, data_type as *const MRDataType);
        }

        self
    }

    /// Calls method `name` on a `Value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::MRuby;
    /// # use mrusty::MRubyImpl;
    /// let mruby = MRuby::new();
    ///
    /// let one = mruby.fixnum(1);
    /// let result = one.call("+", vec![mruby.fixnum(2)]);
    ///
    /// assert_eq!(result.to_i32().unwrap(), 3);
    /// ```
    pub fn call(&self, name: &str, args: Vec<Value>) -> Value {
        unsafe {
            let c_name = CString::new(name).unwrap().as_ptr();
            let sym = mrb_intern_cstr(self.mruby.borrow().mrb, c_name);

            let args: Vec<MRValue> = args.iter().map(|value| value.value).collect();

            let result = mrb_funcall_argv(self.mruby.borrow().mrb, self.value, sym,
                args.len() as i32, args.as_ptr());

            Value::new(self.mruby.clone(), result)
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
    /// let value = mruby.obj(Cont { value: 3 });
    /// let cont = value.to_obj::<Cont>().unwrap();
    ///
    /// assert_eq!(cont.value, 3);
    /// ```
    pub fn to_obj<T: Any>(&self) -> Result<Rc<T>, &str> {
        unsafe {
            let borrow = self.mruby.borrow();

            let class = match borrow.classes.get(&TypeId::of::<T>()) {
                Some(class) => class,
                None       => panic!("Class not found.")
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
        let result = self.call("==", vec![other.clone()]);

        result.to_bool().unwrap()
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value {{ {:?} }}", self.value)
    }
}
