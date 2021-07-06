// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{self, Read};
use std::mem;
use std::os::raw::{c_char, c_void};
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;
use std::rc::Rc;

use super::mruby_ffi::*;

/// A `type` wrapper around a `Rc<RefCell<Mruby>>`. Created with `Mruby::new()`.
pub type MrubyType = Rc<RefCell<Mruby>>;

/// A safe `struct` for the mruby API. The `struct` only contains creation and desctruction
/// methods. Creating an `Mruby` returns a `MrubyType` (`Rc<RefCell<Mruby>>`) which implements
/// `MrubyImpl` where the rest of the implemented API is found.
///
/// # Examples
///
/// ```
/// use mrusty::{Mruby, MrubyImpl};
///
/// let mruby = Mruby::new();
/// let result = mruby.run("2 + 2 == 5").unwrap();
///
/// assert_eq!(result.to_bool().unwrap(), false);
/// ```
pub struct Mruby {
    pub mrb: *const MrState,
    ctx: *const MrContext,
    filename: Option<String>,
    classes: HashMap<TypeId, (*const MrClass, MrDataType, String)>,
    methods: HashMap<TypeId, HashMap<u32, Rc<dyn Fn(MrubyType, Value) -> Value>>>,
    class_methods: HashMap<TypeId, HashMap<u32, Rc<dyn Fn(MrubyType, Value) -> Value>>>,
    mruby_methods: HashMap<String, HashMap<u32, Rc<dyn Fn(MrubyType, Value) -> Value>>>,
    mruby_class_methods: HashMap<String, HashMap<u32, Rc<dyn Fn(MrubyType, Value) -> Value>>>,
    files: HashMap<String, Vec<fn(MrubyType)>>,
    required: HashSet<String>,
}

impl Mruby {
    /// Creates an mruby state and context stored in a `MrubyType` (`Rc<RefCell<Mruby>>`).
    ///
    /// # Example
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// let mruby = Mruby::new();
    /// ```
    pub fn new() -> MrubyType {
        unsafe {
            let mrb = mrb_open();

            let mruby = Rc::new(RefCell::new(Mruby {
                mrb: mrb,
                ctx: mrbc_context_new(mrb),
                filename: None,
                classes: HashMap::new(),
                methods: HashMap::new(),
                class_methods: HashMap::new(),
                mruby_methods: HashMap::new(),
                mruby_class_methods: HashMap::new(),
                files: HashMap::new(),
                required: HashSet::new(),
            }));

            let kernel_str = CString::new("Kernel").unwrap();
            let kernel = mrb_module_get(mrb, kernel_str.as_ptr());

            extern "C" fn require(mrb: *const MrState, _slf: MrValue) -> MrValue {
                unsafe {
                    let ptr = mrb_ext_get_ud(mrb);
                    let mruby: MrubyType = mem::transmute(ptr);

                    let mut name = std::mem::MaybeUninit::<*const c_char>::uninit();

                    let sig_str = CString::new("z").unwrap();

                    mrb_get_args(
                        mrb,
                        sig_str.as_ptr(),
                        name.as_mut_ptr() as *const *const c_char,
                    );

                    let name = name.assume_init();
                    let name = CStr::from_ptr(name).to_str().unwrap();

                    let already_required = { mruby.borrow().required.contains(name) };

                    let result = if already_required {
                        mruby.bool(false)
                    } else {
                        let reqs = {
                            let borrow = mruby.borrow();

                            borrow.files.get(name).map(|reqs| reqs.clone())
                        };

                        match reqs {
                            Some(reqs) => {
                                {
                                    mruby.borrow_mut().required.insert(name.to_owned());
                                }

                                for req in reqs {
                                    req(mruby.clone());
                                }

                                mruby.bool(true)
                            }
                            None => {
                                let filename = {
                                    let borrow = mruby.borrow();

                                    borrow.filename.clone()
                                };

                                let execute =
                                    |path: &Path, name: String, filename: Option<String>| {
                                        {
                                            mruby.borrow_mut().required.insert(name);
                                        }

                                        let result = mruby.execute(path);

                                        match filename {
                                            Some(filename) => mruby.filename(&filename),
                                            None => mruby.borrow_mut().filename = None,
                                        }

                                        match result {
                                            Err(err) => {
                                                Mruby::raise(
                                                    mrb,
                                                    "RuntimeError",
                                                    &format!("{}", err),
                                                );
                                            }
                                            _ => (),
                                        }

                                        mruby.bool(true)
                                    };

                                let path = Path::new(name);
                                let rb = name.to_owned() + ".rb";
                                let rb = Path::new(&rb);
                                let mrbb = name.to_owned() + ".mrb";
                                let mrbb = Path::new(&mrbb);

                                if rb.is_file() {
                                    execute(rb, name.to_owned(), filename)
                                } else if mrbb.is_file() {
                                    execute(mrbb, name.to_owned(), filename)
                                } else if path.is_file() {
                                    execute(path, name.to_owned(), filename)
                                } else {
                                    Mruby::raise(
                                        mrb,
                                        "RuntimeError",
                                        &format!("cannot load {}.rb or {}.mrb", name, name),
                                    );

                                    mruby.nil()
                                }
                            }
                        }
                    };

                    mem::forget(mruby);

                    result.value
                }
            }

            let require_str = CString::new("require").unwrap();

            mrb_define_module_function(mrb, kernel, require_str.as_ptr(), require, 1 << 12);

            let ptr: *const u8 = mem::transmute(mruby);
            mrb_ext_set_ud(mrb, ptr);

            let mruby: MrubyType = mem::transmute(ptr);

            mruby.run_unchecked(
                "
              class RustPanic < Exception
                def initialize(message)
                  super message
                end
              end
            ",
            );

            // Bad magic
            // mruby.run_unchecked("GC.disable");

            mruby
        }
    }

    fn raise(mrb: *const MrState, eclass: &str, message: &str) -> MrValue {
        unsafe {
            let eclass_str = CString::new(eclass).unwrap();
            let message_str = CString::new(message).unwrap();

            mrb_ext_raise(mrb, eclass_str.as_ptr(), message_str.as_ptr());

            MrValue::nil()
        }
    }

    fn close(&self) {
        unsafe {
            mrbc_context_free(self.mrb, self.ctx);
            mrb_close(self.mrb);
        }
    }
}

/// An `enum` containing all possbile types of errors.
#[derive(Debug)]
pub enum MrubyError {
    /// type cast error
    Cast(String),
    /// undefined type error
    Undef,
    /// mruby runtime error
    Runtime(String),
    /// unrecognized file type error
    Filetype,
    /// Rust `Io` error
    Io(io::Error),
}

impl fmt::Display for MrubyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MrubyError::Cast(ref expected) => {
                write!(f, "Cast error: expected {}", expected)
            }
            MrubyError::Undef => {
                write!(f, "Undefined error: type is not defined")
            }
            MrubyError::Runtime(ref err) => {
                write!(f, "Runtime error: {}", err)
            }
            MrubyError::Filetype => {
                write!(
                    f,
                    "Filetype error: script needs a compatible (.rb, .mrb) extension"
                )
            }
            MrubyError::Io(ref err) => err.fmt(f),
        }
    }
}

impl Error for MrubyError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        match *self {
            MrubyError::Cast(_) => "mruby value cast error",
            MrubyError::Undef => "mruby undefined error",
            MrubyError::Runtime(_) => "mruby runtime error",
            MrubyError::Filetype => "filetype mistmatch",
            MrubyError::Io(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for MrubyError {
    fn from(err: io::Error) -> MrubyError {
        MrubyError::Io(err)
    }
}

/// A `trait` useful for organising Rust types into dynamic mruby files.
///
/// # Examples
///
/// ```
/// # use mrusty::Mruby;
/// # use mrusty::MrubyFile;
/// # use mrusty::MrubyImpl;
/// # use mrusty::MrubyType;
/// struct Cont {
///     value: i32
/// }
///
/// impl MrubyFile for Cont {
///     fn require(mruby: MrubyType) {
///         mruby.def_class_for::<Cont>("Container");
///     }
/// }
///
/// let mruby = Mruby::new();
///
/// mruby.def_file::<Cont>("cont");
/// ```
pub trait MrubyFile {
    fn require(mruby: MrubyType);
}

/// A `trait` used on `MrubyType` which implements mruby functionality.
pub trait MrubyImpl {
    /// Adds a filename to the mruby context.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyError;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    /// mruby.filename("script.rb");
    ///
    /// let result = mruby.run("1.nope");
    ///
    /// match result {
    ///     Err(MrubyError::Runtime(err)) => {
    ///         assert_eq!(err, "undefined method 'nope' (NoMethodError)");
    /// },
    ///     _ => assert!(false)
    /// }
    /// ```

    fn filename(&self, filename: &str);

    /// Runs mruby `script` on a state and context and returns a `Value` in an `Ok`
    /// or an `Err` containing an mruby `Exception`'s message.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    /// let result = mruby.run("true").unwrap();
    ///
    /// assert_eq!(result.to_bool().unwrap(), true);
    /// ```
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyError;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    /// let result = mruby.run("'' + 1");
    ///
    /// match result {
    ///     Err(MrubyError::Runtime(err)) => {
    ///         assert_eq!(err, "Integer cannot be converted to String (TypeError)");
    /// },
    ///     _ => assert!(false)
    /// }
    /// ```

    fn run(&self, script: &str) -> Result<Value, MrubyError>;

    /// Runs mruby `script` on a state and context and returns a `Value`. If an mruby Exception is
    /// raised, mruby will be left to handle it.
    ///
    /// The method is unsafe because running it within a Rust context will interrupt drops,
    /// potentially leading to memory leaks.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    /// let result = unsafe { mruby.run_unchecked("true") };
    ///
    /// assert_eq!(result.to_bool().unwrap(), true);
    /// ```
    ///
    /// ```
    /// # #[macro_use] extern crate mrusty;
    /// use mrusty::{Mruby, MrubyImpl};
    ///
    /// # fn main() {
    /// let mruby = Mruby::new();
    ///
    /// struct Cont;
    ///
    /// mruby.def_class_for::<Cont>("Container");
    /// mruby.def_class_method_for::<Cont, _>("raise", mrfn!(|mruby, _slf: Value| {
    ///     unsafe { mruby.run_unchecked("fail 'surprize'") }
    /// }));
    ///
    /// let result = mruby.run("
    ///   begin
    ///     Container.raise
    ///   rescue => e
    ///     e.message
    ///   end
    /// ").unwrap();
    ///
    /// assert_eq!(result.to_str().unwrap(), "surprize");
    /// # }
    /// ```

    unsafe fn run_unchecked(&self, script: &str) -> Value;

    /// Runs mruby compiled (.mrb) `script` on a state and context and returns a `Value` in an `Ok`
    /// or an `Err` containing an mruby `Exception`'s message.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mruby = Mruby::new();
    /// let result = mruby.runb(include_bytes!("script.mrb")).unwrap();
    /// ```

    fn runb(&self, script: &[u8]) -> Result<Value, MrubyError>;

    /// Runs mruby (compiled (.mrb) or not (.rb)) `script` on a state and context and returns a
    /// `Value` in an `Ok` or an `Err` containing an mruby `Exception`'s message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// # use std::path::Path;
    /// let mruby = Mruby::new();
    /// let result = mruby.execute(&Path::new("script.rb")).unwrap();
    /// ```

    fn execute(&self, script: &Path) -> Result<Value, MrubyError>;

    /// Returns whether the mruby `Class` or `Module` named `name` is defined.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    /// let object = mruby.is_defined("Object");
    /// let objekt = mruby.is_defined("Objekt");
    ///
    /// assert!(object);
    /// assert!(!objekt);
    /// ```

    fn is_defined(&self, name: &str) -> bool;

    /// Returns whether the mruby `Class` or `Module` named `name` is defined under `outer` `Class`
    /// or `Module`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// let module = mruby.def_module("Just");
    /// mruby.def_module_under("Mine", &module);
    ///
    /// assert!(mruby.is_defined_under("Mine", &module));
    /// ```

    fn is_defined_under<T: ClassLike>(&self, name: &str, outer: &T) -> bool;

    /// Returns the mruby `Class` named `name` in a `Some` or `None` if it is not defined.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    /// let object = mruby.get_class("Object");
    /// let objekt = mruby.get_class("Objekt");
    ///
    /// assert_eq!(object.unwrap().to_str(), "Object");
    /// assert!(objekt.is_err());
    /// ```

    fn get_class(&self, name: &str) -> Result<Class, MrubyError>;

    /// Returns the mruby `Class` named `name` under `outer` `Class` or `Module` in a `Some` or
    /// `None` if it is not defined.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// struct Cont;
    ///
    /// let module = mruby.def_module("Mine");
    /// mruby.def_class_under_for::<Cont, _>("Container", &module);
    ///
    /// let result = mruby.get_class_under("Container", &module).unwrap();
    ///
    /// assert_eq!(result.to_str(), "Mine::Container");
    /// ```

    fn get_class_under<T: ClassLike>(&self, name: &str, outer: &T) -> Result<Class, MrubyError>;

    /// Returns the mruby `Module` named `name` in a `Some` or `None` if it is not defined.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    /// let kernel = mruby.get_module("Kernel");
    /// let kernet = mruby.get_module("Kernet");
    ///
    /// assert_eq!(kernel.unwrap().to_str(), "Kernel");
    /// assert!(kernet.is_err());
    /// ```

    fn get_module(&self, name: &str) -> Result<Module, MrubyError>;

    /// Returns the mruby `Module` named `name` under `outer` `Class` or `Module` in a `Some` or
    /// `None` if it is not defined.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// let module = mruby.def_module("Just");
    /// mruby.def_module_under("Mine", &module);
    ///
    /// let result = mruby.get_module_under("Mine", &module).unwrap();
    ///
    /// assert_eq!(result.to_str(), "Just::Mine");
    /// ```

    fn get_module_under<T: ClassLike>(&self, name: &str, outer: &T) -> Result<Module, MrubyError>;

    /// Defines a dynamic file that can be `require`d containing the Rust type `T` and runs its
    /// `MrubyFile`-inherited `require` method.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate mrusty;
    /// use mrusty::{Mruby, MrubyFile, MrubyImpl, MrubyType};
    ///
    /// # fn main() {
    /// let mruby = Mruby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// };
    ///
    /// impl MrubyFile for Cont {
    ///     fn require(mruby: MrubyType) {
    ///         mruby.def_class_for::<Cont>("Container");
    ///         mruby.def_method_for::<Cont, _>("initialize", mrfn!(|_mruby, slf: Value, v: i32| {
    ///             let cont = Cont { value: v };
    ///
    ///             slf.init(cont)
    ///         }));
    ///         mruby.def_method_for::<Cont, _>("value", mrfn!(|mruby, slf: (&Cont)| {
    ///             mruby.fixnum(slf.value)
    ///         }));
    ///     }
    /// }
    ///
    /// mruby.def_file::<Cont>("cont");
    ///
    /// let result = mruby.run("
    ///     require 'cont'
    ///
    ///     Container.new(3).value
    /// ").unwrap();
    ///
    /// assert_eq!(result.to_i32().unwrap(), 3);
    /// # }
    /// ```

    fn def_file<T: MrubyFile>(&self, name: &str);

    /// Defines an mruby `Class` named `name`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// mruby.def_class("Container");
    ///
    /// assert!(mruby.is_defined("Container"));
    /// ```
    fn def_class(&self, name: &str) -> Class;

    /// Defines an mruby `Class` named `name` under `outer` `Class` or `Module`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// let module = mruby.def_module("Mine");
    /// mruby.def_class_under("Container", &module);
    ///
    /// assert!(mruby.is_defined_under("Container", &module));
    /// ```
    fn def_class_under<U: ClassLike>(&self, name: &str, outer: &U) -> Class;

    /// Defines Rust type `T` as an mruby `Class` named `name`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// }
    ///
    /// mruby.def_class_for::<Cont>("Container");
    ///
    /// assert!(mruby.is_defined("Container"));
    /// ```
    fn def_class_for<T: Any>(&self, name: &str) -> Class;

    /// Defines Rust type `T` as an mruby `Class` named `name` under `outer` `Class` or `Module`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// struct Cont;
    ///
    /// let module = mruby.def_module("Mine");
    /// mruby.def_class_under_for::<Cont, _>("Container", &module);
    ///
    /// assert!(mruby.is_defined_under("Container", &module));
    /// ```
    fn def_class_under_for<T: Any, U: ClassLike>(&self, name: &str, outer: &U) -> Class;

    /// Defines an mruby `Module` named `name`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// mruby.def_module("Container");
    ///
    /// assert!(mruby.is_defined("Container"));
    /// ```
    fn def_module(&self, name: &str) -> Module;

    /// Defines an mruby `Module` named `name` under `outer` `Class` or `Module`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// let module = mruby.def_module("Just");
    /// mruby.def_module_under("Mine", &module);
    ///
    /// assert!(mruby.is_defined_under("Mine", &module));
    /// ```
    fn def_module_under<T: ClassLike>(&self, name: &str, outer: &T) -> Module;

    /// Defines an mruby method named `name` on `Class` `class`. The closure to be run when the
    /// `name` method is called should be passed through the `mrfn!` macro.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate mrusty;
    /// use mrusty::{Mruby, MrubyImpl};
    ///
    /// # fn main() {
    /// let mruby = Mruby::new();
    ///
    /// let class = mruby.def_class("Container");
    /// mruby.def_method(class, "value", mrfn!(|mruby, slf: Value| {
    ///     mruby.fixnum(3)
    /// }));
    ///
    /// let result = mruby.run("Container.new.value").unwrap();
    ///
    /// assert_eq!(result.to_i32().unwrap(), 3);
    /// # }
    /// ```
    fn def_method<F>(&self, class: Class, name: &str, method: F)
    where
        F: Fn(MrubyType, Value) -> Value + 'static;

    /// Defines an mruby class method named `name` on `Class` `class`. The closure to be run when
    /// the `name` method is called should be passed through the `mrfn!` macro.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate mrusty;
    /// use mrusty::{Mruby, MrubyImpl};
    ///
    /// # fn main() {
    /// let mruby = Mruby::new();
    ///
    /// let class = mruby.def_class("Container");
    /// mruby.def_class_method(class, "hi", mrfn!(|mruby, _slf: Value, v: i32| {
    ///     mruby.fixnum(v)
    /// }));
    ///
    /// let result = mruby.run("Container.hi 3").unwrap();
    ///
    /// assert_eq!(result.to_i32().unwrap(), 3);
    /// # }
    /// ```
    fn def_class_method<F>(&self, class: Class, name: &str, method: F)
    where
        F: Fn(MrubyType, Value) -> Value + 'static;

    /// Defines an mruby method named `name` on the mruby `Class` reflecting type `T`. The closure
    /// to be run when the `name` method is called should be passed through the `mrfn!` macro.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate mrusty;
    /// use mrusty::{Mruby, MrubyImpl};
    ///
    /// # fn main() {
    /// let mruby = Mruby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// };
    ///
    /// mruby.def_class_for::<Cont>("Container");
    /// mruby.def_method_for::<Cont, _>("initialize", mrfn!(|_mruby, slf: Value, v: i32| {
    ///     let cont = Cont { value: v };
    ///
    ///     slf.init(cont)
    /// }));
    /// mruby.def_method_for::<Cont, _>("value", mrfn!(|mruby, slf: (&Cont)| {
    ///     mruby.fixnum(slf.value)
    /// }));
    ///
    /// let result = mruby.run("Container.new(3).value").unwrap();
    ///
    /// assert_eq!(result.to_i32().unwrap(), 3);
    /// # }
    /// ```
    fn def_method_for<T: Any, F>(&self, name: &str, method: F)
    where
        F: Fn(MrubyType, Value) -> Value + 'static;

    /// Defines an mruby class method named `name` on the mruby `Class` reflecting type `T`. The
    /// closure to be run when the `name` method is called should be passed through the `mrfn!`
    /// macro.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate mrusty;
    /// use mrusty::{Mruby, MrubyImpl};
    ///
    /// # fn main() {
    /// let mruby = Mruby::new();
    ///
    /// struct Cont;
    ///
    /// mruby.def_class_for::<Cont>("Container");
    /// mruby.def_class_method_for::<Cont, _>("hi", mrfn!(|mruby, _slf: Value, v: i32| {
    ///     mruby.fixnum(v)
    /// }));
    ///
    /// let result = mruby.run("Container.hi 3").unwrap();
    ///
    /// assert_eq!(result.to_i32().unwrap(), 3);
    /// # }
    /// ```
    fn def_class_method_for<T: Any, F>(&self, name: &str, method: F)
    where
        F: Fn(MrubyType, Value) -> Value + 'static;

    /// Return the mruby name of a previously defined Rust type `T` with `def_class`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::{Mruby, MrubyImpl};
    ///
    /// let mruby = Mruby::new();
    ///
    /// struct Cont;
    ///
    /// mruby.def_class_for::<Cont>("Container");
    ///
    /// assert_eq!(mruby.class_name_for::<Cont>().unwrap(), "Container");
    /// ```
    fn class_name_for<T: Any>(&self) -> Result<String, MrubyError>;

    /// Creates mruby `Value` `nil`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// struct Cont;
    ///
    /// mruby.def_class_for::<Cont>("Container");
    /// mruby.def_method_for::<Cont, _>("nil", |mruby, _slf| mruby.nil());
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
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
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
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
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
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
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
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// let s = mruby.string("hi");
    ///
    /// assert_eq!(s.to_str().unwrap(), "hi");
    /// ```

    fn string(&self, value: &str) -> Value;

    /// Creates mruby `Value` of `Class` `Symbol`.
    ///
    /// # Examples
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// let s = mruby.symbol("hi");
    ///
    /// assert_eq!(s.to_str().unwrap(), "hi");
    /// ```

    fn symbol(&self, value: &str) -> Value;

    /// Creates mruby `Value` of `Class` `name` containing a Rust object of type `T`.
    ///
    /// *Note:* `T` must be defined on the current `Mruby` with `def_class`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// }
    ///
    /// mruby.def_class_for::<Cont>("Container");
    ///
    /// let value = mruby.obj(Cont { value: 3 });
    /// ```

    fn obj<T: Any>(&self, obj: T) -> Value;

    /// Creates mruby `Value` of `Class` `name` containing a Rust `Option` of type `T`.
    ///
    /// *Note:* `T` must be defined on the current `Mruby` with `def_class`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// }
    ///
    /// mruby.def_class_for::<Cont>("Container");
    ///
    /// let none = mruby.option::<Cont>(None);
    /// let some = mruby.option(Some(Cont { value: 3 }));
    ///
    /// let some = some.to_obj::<Cont>().unwrap();
    /// let some = some.borrow();
    ///
    /// assert_eq!(none.call("nil?", vec![]).unwrap().to_bool().unwrap(), true);
    /// assert_eq!(some.value, 3);
    /// ```

    fn option<T: Any>(&self, obj: Option<T>) -> Value;

    /// Creates mruby `Value` of `Class` `Array`.
    ///
    /// # Examples
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// let array = mruby.array(vec![
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

    fn array(&self, value: Vec<Value>) -> Value;
}

fn get_class<F>(mruby: &MrubyType, name: &str, class: Result<Class, MrubyError>, get: F) -> Class
where
    F: Fn(*const MrState, *const c_char, *const MrClass) -> *const MrClass,
{
    unsafe {
        let class = if let Ok(class) = class {
            class
        } else {
            let name = name.to_owned();

            let c_name = CString::new(name.clone()).unwrap();
            let object = CString::new("Object").unwrap();
            let object = mrb_class_get(mruby.borrow().mrb, object.as_ptr());

            let class = get(mruby.borrow().mrb, c_name.as_ptr(), object);

            Class::new(mruby.clone(), class)
        };

        mruby
            .borrow_mut()
            .mruby_methods
            .insert(class.to_str().to_owned(), HashMap::new());
        mruby
            .borrow_mut()
            .mruby_class_methods
            .insert(class.to_str().to_owned(), HashMap::new());

        class
    }
}

fn get_class_for<T: Any, F>(mruby: &MrubyType, name: &str, get: F) -> Class
where
    F: Fn(*const MrState, *const c_char, *const MrClass) -> *const MrClass,
{
    let class = unsafe {
        let name = name.to_owned();

        let c_name = CString::new(name.clone()).unwrap();
        let object = CString::new("Object").unwrap();
        let object = mrb_class_get(mruby.borrow().mrb, object.as_ptr());

        let class = get(mruby.borrow().mrb, c_name.as_ptr(), object);

        mrb_ext_set_instance_tt(class, MrType::MRB_TT_DATA);

        extern "C" fn free<T>(_mrb: *const MrState, ptr: *const u8) {
            unsafe {
                mem::transmute::<*const u8, Rc<RefCell<T>>>(ptr);
            }
        }

        let data_type = MrDataType {
            name: c_name.as_ptr(),
            free: free::<T>,
        };

        mruby
            .borrow_mut()
            .classes
            .insert(TypeId::of::<T>(), (class, data_type, name));
        mruby
            .borrow_mut()
            .methods
            .insert(TypeId::of::<T>(), HashMap::new());
        mruby
            .borrow_mut()
            .class_methods
            .insert(TypeId::of::<T>(), HashMap::new());

        Class::new(mruby.clone(), class)
    };

    mruby.def_method_for::<T, _>("dup", |_mruby, slf| slf.clone());

    class
}

macro_rules! insert_method {
    ( $mruby:expr, $name:expr, $method:expr, $methods:ident, $key:expr ) => {{
        let sym = unsafe {
            let name_str = CString::new($name).unwrap();

            mrb_intern($mruby.borrow().mrb, name_str.as_ptr(), $name.len())
        };

        let mut borrow = $mruby.borrow_mut();

        let methods = match borrow.$methods.get_mut($key) {
            Some(methods) => methods,
            None => panic!("Class not found."),
        };

        methods.insert(sym, Rc::new($method));
    }};
}

macro_rules! callback {
    ( $name:ident, $methods:ident, $key:expr ) => {
        extern "C" fn $name<T: Any>(mrb: *const MrState, slf: MrValue) -> MrValue {
            unsafe {
                let ptr = mrb_ext_get_ud(mrb);
                let mruby: MrubyType = mem::transmute(ptr);

                let result = {
                    let value = Value::new(mruby.clone(), slf);

                    let method = {
                        let borrow = mruby.borrow();

                        borrow.$methods.get($key).map(|methods| {
                            let sym = mrb_ext_get_mid(mrb);

                            methods.get(&sym).map(|method| method.clone())
                        })
                    };

                    if let Some(Some(method)) = method {
                        match panic::catch_unwind(AssertUnwindSafe(|| {
                            method(mruby.clone(), value).value
                        })) {
                            Ok(value) => value,
                            Err(error) => {
                                let message = match error.downcast_ref::<&'static str>() {
                                    Some(s) => *s,
                                    None => match error.downcast_ref::<String>() {
                                        Some(s) => &s[..],
                                        None => "",
                                    },
                                };

                                Mruby::raise(mrb, "RustPanic", message)
                            }
                        }
                    } else {
                        Mruby::raise(mrb, "TypeError", "Class not found.")
                    }
                };

                mem::forget(mruby);

                result
            }
        }
    };
}

macro_rules! mruby_callback {
    ( $value:expr, class ) => {
        $value.class().to_str()
    };
    ( $value:expr, to_class ) => {
        $value.to_class().unwrap().to_str()
    };
    ( $name:ident, $methods:ident, $conv:tt ) => {
        extern "C" fn $name(mrb: *const MrState, slf: MrValue) -> MrValue {
            unsafe {
                let ptr = mrb_ext_get_ud(mrb);
                let mruby: MrubyType = mem::transmute(ptr);

                let result = {
                    let value = Value::new(mruby.clone(), slf);

                    let method = {
                        let borrow = mruby.borrow();

                        borrow
                            .$methods
                            .get(mruby_callback!(value, $conv))
                            .map(|methods| {
                                let sym = mrb_ext_get_mid(mrb);

                                methods.get(&sym).map(|method| method.clone())
                            })
                    };

                    if let Some(Some(method)) = method {
                        match panic::catch_unwind(AssertUnwindSafe(|| {
                            method(mruby.clone(), value).value
                        })) {
                            Ok(value) => value,
                            Err(error) => {
                                let message = match error.downcast_ref::<&'static str>() {
                                    Some(s) => *s,
                                    None => match error.downcast_ref::<String>() {
                                        Some(s) => &s[..],
                                        None => "",
                                    },
                                };

                                Mruby::raise(mrb, "RustPanic", message)
                            }
                        }
                    } else {
                        Mruby::raise(mrb, "TypeError", "Class not found.")
                    }
                };

                mem::forget(mruby);

                result
            }
        }
    };
}

#[allow(unused_mut)]
impl MrubyImpl for MrubyType {
    fn filename(&self, filename: &str) {
        self.borrow_mut().filename = Some(filename.to_owned());

        unsafe {
            let filename_str = CString::new(filename).unwrap();

            mrbc_filename(self.borrow().mrb, self.borrow().ctx, filename_str.as_ptr());
        }
    }

    fn run(&self, script: &str) -> Result<Value, MrubyError> {
        extern "C" fn run_protected(mrb: *const MrState, data: MrValue) -> MrValue {
            unsafe {
                let ptr = data.to_ptr().unwrap();
                let args = *mem::transmute::<*const u8, *const [*const u8; 3]>(ptr);

                let script_len: &i32 = mem::transmute(args[1]);
                let ctx: *const MrContext = mem::transmute(args[2]);

                let result = mrb_load_nstring_cxt(mrb, args[0], *script_len, ctx);

                mrb_ext_raise_current(mrb);

                result
            }
        }

        unsafe {
            let (mrb, ctx) = {
                let borrow = self.borrow();

                (borrow.mrb, borrow.ctx)
            };

            let script_ptr = script.as_ptr();
            let script_len = script.len();
            let script_len_ptr: *const u8 = mem::transmute(&script_len);
            let ctx_ptr: *const u8 = mem::transmute(ctx);

            let args = [script_ptr, script_len_ptr, ctx_ptr];
            let args_ptr: *const u8 = mem::transmute(&args);
            let data = MrValue::ptr(mrb, args_ptr);

            let mut state = std::mem::MaybeUninit::<bool>::uninit();

            let value = mrb_protect(mrb, run_protected, data, state.as_mut_ptr() as *const bool);

            if state.assume_init() {
                let str = mrb_ext_exc_str(mrb, value).to_str(mrb).unwrap();

                Err(MrubyError::Runtime(str.to_owned()))
            } else {
                Ok(Value::new(self.clone(), value))
            }
        }
    }

    unsafe fn run_unchecked(&self, script: &str) -> Value {
        let (mrb, ctx) = {
            let borrow = self.borrow();

            (borrow.mrb, borrow.ctx)
        };

        let value = mrb_load_nstring_cxt(mrb, script.as_ptr(), script.len() as i32, ctx);

        Value::new(self.clone(), value)
    }

    fn runb(&self, script: &[u8]) -> Result<Value, MrubyError> {
        extern "C" fn runb_protected(mrb: *const MrState, data: MrValue) -> MrValue {
            unsafe {
                let ptr = data.to_ptr().unwrap();
                let args = *mem::transmute::<*const u8, *const [*const u8; 2]>(ptr);

                let ctx: *const MrContext = mem::transmute(args[1]);

                let result = mrb_load_irep_cxt(mrb, args[0], ctx);

                mrb_ext_raise_current(mrb);

                result
            }
        }

        unsafe {
            let (mrb, ctx) = {
                let borrow = self.borrow();

                (borrow.mrb, borrow.ctx)
            };

            let script_ptr = script.as_ptr();
            let ctx_ptr: *const u8 = mem::transmute(ctx);

            let args = [script_ptr, ctx_ptr];
            let args_ptr: *const u8 = mem::transmute(&args);
            let data = MrValue::ptr(mrb, args_ptr);

            let mut state = std::mem::MaybeUninit::<bool>::uninit();

            let value = mrb_protect(mrb, runb_protected, data, state.as_mut_ptr() as *const bool);

            if state.assume_init() {
                let str = mrb_ext_exc_str(mrb, value).to_str(mrb).unwrap();

                Err(MrubyError::Runtime(str.to_owned()))
            } else {
                Ok(Value::new(self.clone(), value))
            }
        }
    }

    fn execute(&self, script: &Path) -> Result<Value, MrubyError> {
        match script.extension() {
            Some(ext) => {
                self.filename(script.file_name().unwrap().to_str().unwrap());

                // This should be mut but got warning...
                let mut file = File::open(script)?;

                match ext.to_str().unwrap() {
                    "rb" => {
                        let mut script = String::new();
                        file.read_to_string(&mut script)?;

                        self.run(&script)
                    }
                    "mrb" => {
                        let mut script = Vec::new();
                        file.read_to_end(&mut script)?;

                        self.runb(&script)
                    }
                    _ => Err(MrubyError::Filetype),
                }
            }
            None => Err(MrubyError::Filetype),
        }
    }

    fn is_defined(&self, name: &str) -> bool {
        unsafe {
            let name_str = CString::new(name).unwrap();

            mrb_class_defined(self.borrow().mrb, name_str.as_ptr())
        }
    }

    fn is_defined_under<T: ClassLike>(&self, name: &str, outer: &T) -> bool {
        unsafe {
            let name_str = CString::new(name).unwrap();

            mrb_ext_class_defined_under(self.borrow().mrb, outer.class(), name_str.as_ptr())
        }
    }

    fn get_class(&self, name: &str) -> Result<Class, MrubyError> {
        unsafe {
            let name_str = CString::new(name).unwrap();

            if mrb_class_defined(self.borrow().mrb, name_str.as_ptr()) {
                let class = mrb_class_get(self.borrow().mrb, name_str.as_ptr());

                Ok(Class::new(self.clone(), class))
            } else {
                Err(MrubyError::Undef)
            }
        }
    }

    fn get_class_under<T: ClassLike>(&self, name: &str, outer: &T) -> Result<Class, MrubyError> {
        unsafe {
            let name_str = CString::new(name).unwrap();

            if mrb_ext_class_defined_under(self.borrow().mrb, outer.class(), name_str.as_ptr()) {
                let class =
                    mrb_class_get_under(self.borrow().mrb, outer.class(), name_str.as_ptr());

                Ok(Class::new(self.clone(), class))
            } else {
                Err(MrubyError::Undef)
            }
        }
    }

    fn get_module(&self, name: &str) -> Result<Module, MrubyError> {
        unsafe {
            let name_str = CString::new(name).unwrap();

            if mrb_class_defined(self.borrow().mrb, name_str.as_ptr()) {
                let class = mrb_module_get(self.borrow().mrb, name_str.as_ptr());

                Ok(Module::new(self.clone(), class))
            } else {
                Err(MrubyError::Undef)
            }
        }
    }

    fn get_module_under<T: ClassLike>(&self, name: &str, outer: &T) -> Result<Module, MrubyError> {
        unsafe {
            let name_str = CString::new(name).unwrap();

            if mrb_ext_class_defined_under(self.borrow().mrb, outer.class(), name_str.as_ptr()) {
                let class =
                    mrb_module_get_under(self.borrow().mrb, outer.class(), name_str.as_ptr());

                Ok(Module::new(self.clone(), class))
            } else {
                Err(MrubyError::Undef)
            }
        }
    }

    fn def_file<T: MrubyFile>(&self, name: &str) {
        let mut borrow = self.borrow_mut();

        if borrow.files.contains_key(name) {
            let mut file = borrow.files.get_mut(name).unwrap();

            file.push(T::require);
        } else {
            borrow.files.insert(name.to_owned(), vec![T::require]);
        }
    }

    fn def_class(&self, name: &str) -> Class {
        get_class(
            self,
            name,
            self.get_class(name),
            |mrb: *const MrState, name: *const c_char, object: *const MrClass| unsafe {
                mrb_define_class(mrb, name, object)
            },
        )
    }

    fn def_class_under<U: ClassLike>(&self, name: &str, outer: &U) -> Class {
        get_class(
            self,
            name,
            self.get_class_under(name, outer),
            |mrb: *const MrState, name: *const c_char, object: *const MrClass| unsafe {
                mrb_define_class_under(mrb, outer.class(), name, object)
            },
        )
    }

    fn def_class_for<T: Any>(&self, name: &str) -> Class {
        get_class_for::<T, _>(
            self,
            name,
            |mrb: *const MrState, name: *const c_char, object: *const MrClass| unsafe {
                mrb_define_class(mrb, name, object)
            },
        )
    }

    fn def_class_under_for<T: Any, U: ClassLike>(&self, name: &str, outer: &U) -> Class {
        get_class_for::<T, _>(
            self,
            name,
            |mrb: *const MrState, name: *const c_char, object: *const MrClass| unsafe {
                mrb_define_class_under(mrb, outer.class(), name, object)
            },
        )
    }

    fn def_module(&self, name: &str) -> Module {
        unsafe {
            let name_str = CString::new(name).unwrap();

            let module = mrb_define_module(self.borrow().mrb, name_str.as_ptr());

            Module::new(self.clone(), module)
        }
    }

    fn def_module_under<T: ClassLike>(&self, name: &str, outer: &T) -> Module {
        unsafe {
            let name_str = CString::new(name).unwrap();

            let module =
                mrb_define_module_under(self.borrow().mrb, outer.class(), name_str.as_ptr());

            Module::new(self.clone(), module)
        }
    }

    fn def_method<F>(&self, class: Class, name: &str, method: F)
    where
        F: Fn(MrubyType, Value) -> Value + 'static,
    {
        insert_method!(self, name, method, mruby_methods, class.to_str());

        mruby_callback!(call_mruby_method, mruby_methods, class);

        unsafe {
            let name_str = CString::new(name).unwrap();

            mrb_define_method(
                self.borrow().mrb,
                class.class,
                name_str.as_ptr(),
                call_mruby_method,
                1 << 12,
            );
        }
    }

    fn def_class_method<F>(&self, class: Class, name: &str, method: F)
    where
        F: Fn(MrubyType, Value) -> Value + 'static,
    {
        insert_method!(self, name, method, mruby_class_methods, class.to_str());

        mruby_callback!(call_mruby_class_method, mruby_class_methods, to_class);

        unsafe {
            let name_str = CString::new(name).unwrap();

            mrb_define_class_method(
                self.borrow().mrb,
                class.class,
                name_str.as_ptr(),
                call_mruby_class_method,
                1 << 12,
            );
        }
    }

    fn def_method_for<T: Any, F>(&self, name: &str, method: F)
    where
        F: Fn(MrubyType, Value) -> Value + 'static,
    {
        insert_method!(self, name, method, methods, &TypeId::of::<T>());

        callback!(call_method, methods, &TypeId::of::<T>());

        let borrow = self.borrow();

        let class = match borrow.classes.get(&TypeId::of::<T>()) {
            Some(class) => class,
            None => panic!("Class not found."),
        };

        unsafe {
            let name_str = CString::new(name).unwrap();

            mrb_define_method(
                borrow.mrb,
                class.0,
                name_str.as_ptr(),
                call_method::<T>,
                1 << 12,
            );
        }
    }

    fn def_class_method_for<T: Any, F>(&self, name: &str, method: F)
    where
        F: Fn(MrubyType, Value) -> Value + 'static,
    {
        insert_method!(self, name, method, class_methods, &TypeId::of::<T>());

        callback!(call_class_method, class_methods, &TypeId::of::<T>());

        let borrow = self.borrow();

        let class = match borrow.classes.get(&TypeId::of::<T>()) {
            Some(class) => class,
            None => panic!("Class not found."),
        };

        unsafe {
            let name_str = CString::new(name).unwrap();

            mrb_define_class_method(
                borrow.mrb,
                class.0,
                name_str.as_ptr(),
                call_class_method::<T>,
                1 << 12,
            );
        }
    }

    fn class_name_for<T: Any>(&self) -> Result<String, MrubyError> {
        let borrow = self.borrow();

        match borrow.classes.get(&TypeId::of::<T>()) {
            Some(class) => Ok(class.2.clone()),
            None => Err(MrubyError::Undef),
        }
    }

    fn nil(&self) -> Value {
        unsafe { Value::new(self.clone(), MrValue::nil()) }
    }

    fn bool(&self, value: bool) -> Value {
        unsafe { Value::new(self.clone(), MrValue::bool(value)) }
    }

    fn fixnum(&self, value: i32) -> Value {
        unsafe { Value::new(self.clone(), MrValue::fixnum(value)) }
    }

    fn float(&self, value: f64) -> Value {
        unsafe { Value::new(self.clone(), MrValue::float(self.borrow().mrb, value)) }
    }

    fn string(&self, value: &str) -> Value {
        unsafe { Value::new(self.clone(), MrValue::string(self.borrow().mrb, value)) }
    }

    fn symbol(&self, value: &str) -> Value {
        unsafe { Value::new(self.clone(), MrValue::symbol(self.borrow().mrb, value)) }
    }

    fn obj<T: Any>(&self, obj: T) -> Value {
        let borrow = self.borrow();

        let class = match borrow.classes.get(&TypeId::of::<T>()) {
            Some(class) => class,
            None => panic!("Class not found."),
        };

        unsafe {
            Value::new(
                self.clone(),
                MrValue::obj(borrow.mrb, class.0 as *const MrClass, obj, &class.1),
            )
        }
    }

    fn option<T: Any>(&self, obj: Option<T>) -> Value {
        match obj {
            Some(obj) => self.obj(obj),
            None => self.nil(),
        }
    }

    fn array(&self, value: Vec<Value>) -> Value {
        let array: Vec<MrValue> = value.iter().map(|value| value.value).collect();

        unsafe { Value::new(self.clone(), MrValue::array(self.borrow().mrb, array)) }
    }
}

impl Drop for Mruby {
    fn drop(&mut self) {
        self.close();
    }
}

/// A `struct` that wraps around any mruby variable.
///
/// `Values` are created from the `Mruby` instance:
///
/// * [`nil`](../mrusty/trait.MrubyImpl.html#tymethod.nil)
/// * [`bool`](../mrusty/trait.MrubyImpl.html#tymethod.bool)
/// * [`fixnum`](../mrusty/trait.MrubyImpl.html#tymethod.fixnum)
/// * [`float`](../mrusty/trait.MrubyImpl.html#tymethod.float)
/// * [`string`](../mrusty/trait.MrubyImpl.html#tymethod.string)
/// * [`obj`](../mrusty/trait.MrubyImpl.html#tymethod.obj)
/// * [`option`](../mrusty/trait.MrubyImpl.html#tymethod.option)
/// * [`array`](../mrusty/trait.MrubyImpl.html#tymethod.array)
///
/// # Examples
///
/// ```
/// # use mrusty::Mruby;
/// # use mrusty::MrubyImpl;
/// let mruby = Mruby::new();
/// let result = mruby.run("true").unwrap(); // Value
///
/// // Values need to be unwrapped in order to make sure they have the right mruby type.
/// assert_eq!(result.to_bool().unwrap(), true);
/// ```
pub struct Value {
    mruby: MrubyType,
    value: MrValue,
}

impl Value {
    /// Not meant to be called directly.
    #[doc(hidden)]
    pub fn new(mruby: MrubyType, value: MrValue) -> Value {
        Value {
            mruby: mruby,
            value: value,
        }
    }

    /// Initializes the `self` mruby object passed to `initialize` with a Rust object of type `T`.
    ///
    /// *Note:* `T` must be defined on the current `Mruby` with `def_class`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate mrusty;
    /// use mrusty::{Mruby, MrubyImpl};
    ///
    /// # fn main() {
    /// let mruby = Mruby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// };
    ///
    /// mruby.def_class_for::<Cont>("Container");
    /// mruby.def_method_for::<Cont, _>("initialize", mrfn!(|_mruby, slf: Value, v: i32| {
    ///     let cont = Cont { value: v };
    ///
    ///     slf.init(cont) // Return the same slf value.
    /// }));
    ///
    /// let result = mruby.run("Container.new 3").unwrap();
    /// let result = result.to_obj::<Cont>().unwrap();
    /// let result = result.borrow();
    ///
    /// assert_eq!(result.value, 3);
    /// # }
    /// ```
    pub fn init<T: Any>(self, obj: T) -> Value {
        unsafe {
            let rc = Rc::new(RefCell::new(obj));
            let ptr: *const u8 = mem::transmute(rc);

            let borrow = self.mruby.borrow();

            let class = match borrow.classes.get(&TypeId::of::<T>()) {
                Some(class) => class,
                None => panic!("Class not found."),
            };

            let data_type = &class.1;

            mrb_ext_data_init(
                &self.value as *const MrValue,
                ptr,
                data_type as *const MrDataType,
            );
        }

        self
    }

    /// Calls method `name` on a `Value` passing `args`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// let one = mruby.fixnum(1);
    /// let result = one.call("+", vec![mruby.fixnum(2)]).unwrap();
    ///
    /// assert_eq!(result.to_i32().unwrap(), 3);
    /// ```
    pub fn call(&self, name: &str, args: Vec<Value>) -> Result<Value, MrubyError> {
        extern "C" fn call_protected(mrb: *const MrState, data: MrValue) -> MrValue {
            unsafe {
                let ptr = data.to_ptr().unwrap();
                let args = *mem::transmute::<*const u8, *const [*const u8; 4]>(ptr);

                let value: MrValue = mem::transmute_copy(&*args[0]);
                let sym: &u32 = mem::transmute(args[1]);
                let argc: &i32 = mem::transmute(args[2]);
                let argv: *const MrValue = mem::transmute(args[3]);

                let result = mrb_funcall_argv(mrb, value, *sym, *argc, argv);

                mrb_ext_raise_current(mrb);

                result
            }
        }

        unsafe {
            let mrb = self.mruby.borrow().mrb;

            let name_str = CString::new(name).unwrap();
            let sym = mrb_intern(mrb, name_str.as_ptr(), name.len());

            let args: Vec<MrValue> = args.iter().map(|value| value.value).collect();

            let value_ptr: *const u8 = mem::transmute(&self.value);
            let sym_ptr: *const u8 = mem::transmute(&sym);
            let argc = args.len();
            let argc_ptr: *const u8 = mem::transmute(&argc);
            let argv_ptr: *const u8 = mem::transmute(args.as_ptr());

            let args = [value_ptr, sym_ptr, argc_ptr, argv_ptr];
            let args_ptr: *const u8 = mem::transmute(&args);
            let data = MrValue::ptr(mrb, args_ptr);

            let mut state = std::mem::MaybeUninit::<bool>::uninit();

            let value = mrb_protect(mrb, call_protected, data, state.as_mut_ptr() as *const bool);

            if state.assume_init() {
                let str = mrb_ext_exc_str(mrb, value).to_str(mrb).unwrap();

                Err(MrubyError::Runtime(str.to_owned()))
            } else {
                Ok(Value::new(self.mruby.clone(), value))
            }
        }
    }

    /// Calls method `name` on a `Value` passing `args`. If call fails, mruby will be left to
    /// handle the exception.
    ///
    /// The method is unsafe because running it within a Rust context will interrupt drops,
    /// potentially leading to memory leaks.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// let one = mruby.fixnum(1);
    /// let result = unsafe { one.call_unchecked("+", vec![mruby.fixnum(2)]) };
    ///
    /// assert_eq!(result.to_i32().unwrap(), 3);
    /// ```
    pub unsafe fn call_unchecked(&self, name: &str, args: Vec<Value>) -> Value {
        let name_str = CString::new(name).unwrap();

        let sym = mrb_intern(self.mruby.borrow().mrb, name_str.as_ptr(), name.len());

        let args: Vec<MrValue> = args.iter().map(|value| value.value).collect();

        let result = mrb_funcall_argv(
            self.mruby.borrow().mrb,
            self.value,
            sym,
            args.len() as i32,
            args.as_ptr(),
        );

        Value::new(self.mruby.clone(), result)
    }

    /// Returns whether the instance variable `name` is defined on a `Value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// mruby.def_class("Container");
    ///
    /// let cont = mruby.run("Container.new").unwrap();
    ///
    /// assert!(!cont.has_var("value"));
    /// ```

    pub fn has_var(&self, name: &str) -> bool {
        unsafe {
            let name_str = CString::new(name).unwrap();

            let sym = mrb_intern(self.mruby.borrow().mrb, name_str.as_ptr(), name.len());

            mrb_iv_defined(self.mruby.borrow().mrb, self.value, sym)
        }
    }

    /// Returns the value of the instance variable `name` in a `Some` or `None` if it is not
    /// defined.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// mruby.def_class("Container");
    ///
    /// let cont = mruby.run("Container.new").unwrap();
    ///
    /// cont.set_var("value", mruby.fixnum(2));
    ///
    /// assert_eq!(cont.get_var("value").unwrap().to_i32().unwrap(), 2);
    /// assert!(cont.get_var("valup").is_none());
    /// ```

    pub fn get_var(&self, name: &str) -> Option<Value> {
        unsafe {
            let name_str = CString::new(name).unwrap();

            let sym = mrb_intern(self.mruby.borrow().mrb, name_str.as_ptr(), name.len());

            if mrb_iv_defined(self.mruby.borrow().mrb, self.value, sym) {
                Some(Value::new(
                    self.mruby.clone(),
                    mrb_iv_get(self.mruby.borrow().mrb, self.value, sym),
                ))
            } else {
                None
            }
        }
    }

    /// Sets the value of the instance variable `name` to `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// mruby.def_class("Container");
    ///
    /// let cont = mruby.run("Container.new").unwrap();
    ///
    /// cont.set_var("value", mruby.fixnum(2));
    ///
    /// assert!(cont.has_var("value"));
    /// assert_eq!(cont.get_var("value").unwrap().to_i32().unwrap(), 2);
    /// ```
    /// <br/>
    ///
    /// Method panics if called on non-objects.
    ///
    /// ```should_panic
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// let one = mruby.fixnum(1);
    ///
    /// one.set_var("value", mruby.fixnum(2)); // panics because Fixnum cannot have instance vars
    /// ```

    pub fn set_var(&self, name: &str, value: Value) {
        match self.value.typ {
            MrType::MRB_TT_OBJECT
            | MrType::MRB_TT_CLASS
            | MrType::MRB_TT_MODULE
            | MrType::MRB_TT_SCLASS
            | MrType::MRB_TT_HASH
            | MrType::MRB_TT_DATA
            | MrType::MRB_TT_EXCEPTION => unsafe {
                let name_str = CString::new(name).unwrap();

                let sym = mrb_intern(self.mruby.borrow().mrb, name_str.as_ptr(), name.len());

                mrb_iv_set(self.mruby.borrow().mrb, self.value, sym, value.value)
            },
            _ => panic!("Cannot set instance variable on non-object."),
        }
    }

    /// Returns the `Class` of an mruby `Value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// let one = mruby.run("1").unwrap();
    /// assert_eq!(one.class().to_str(), "Integer");
    /// ```

    pub fn class(&self) -> Class {
        unsafe {
            let class = mrb_ext_class(self.mruby.borrow().mrb, self.value);

            Class::new(self.mruby.clone(), class)
        }
    }

    /// Casts a `Value` and returns a `bool` in an `Ok` or an `Err` if the types mismatch.
    ///
    /// # Example
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
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

    pub fn to_bool(&self) -> Result<bool, MrubyError> {
        unsafe { self.value.to_bool() }
    }

    /// Casts a `Value` and returns an `i32` in an `Ok` or an `Err` if the types mismatch.
    ///
    /// # Example
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
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

    pub fn to_i32(&self) -> Result<i32, MrubyError> {
        unsafe { self.value.to_i32() }
    }

    /// Casts a `Value` and returns an `f64` in an `Ok` or an `Err` if the types mismatch.
    ///
    /// # Example
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    /// let result = mruby.run("
    ///   3 / 2.0
    /// ").unwrap();
    ///
    /// assert_eq!(result.to_f64().unwrap(), 1.5);
    /// ```

    pub fn to_f64(&self) -> Result<f64, MrubyError> {
        unsafe { self.value.to_f64() }
    }

    /// Casts a `Value` and returns a `&str` in an `Ok` or an `Err` if the types mismatch.
    ///
    /// # Example
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    /// let result = mruby.run("
    ///   [1, 2, 3].map(&:to_s).join
    /// ").unwrap();
    ///
    /// assert_eq!(result.to_str().unwrap(), "123");
    /// ```
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    /// let result = mruby.run(":symbol").unwrap();
    ///
    /// assert_eq!(result.to_str().unwrap(), "symbol");
    /// ```

    pub fn to_str<'a>(&self) -> Result<&'a str, MrubyError> {
        unsafe { self.value.to_str(self.mruby.borrow().mrb) }
    }

    /// Casts mruby `Value` of `Class` `name` to Rust type `Rc<T>`.
    ///
    /// *Note:* `T` must be defined on the current `Mruby` with `def_class`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// }
    ///
    /// mruby.def_class_for::<Cont>("Container");
    ///
    /// let value = mruby.obj(Cont { value: 3 });
    /// let cont = value.to_obj::<Cont>().unwrap();
    /// let cont = cont.borrow();
    ///
    /// assert_eq!(cont.value, 3);
    /// ```

    pub fn to_obj<T: Any>(&self) -> Result<Rc<RefCell<T>>, MrubyError> {
        unsafe {
            let borrow = self.mruby.borrow();

            let class = match borrow.classes.get(&TypeId::of::<T>()) {
                Some(class) => class,
                None => return Err(MrubyError::Undef),
            };

            let self_class = self.class();

            if self_class.to_str() != class.2 {
                return Err(MrubyError::Undef);
            }

            self.value.to_obj::<T>(borrow.mrb, &class.1)
        }
    }

    /// Casts mruby `Value` of `Class` `name` to Rust `Option` of `Rc<T>`.
    ///
    /// *Note:* `T` must be defined on the current `Mruby` with `def_class`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// struct Cont {
    ///     value: i32
    /// }
    ///
    /// mruby.def_class_for::<Cont>("Container");
    ///
    /// let value = mruby.obj(Cont { value: 3 });
    /// let cont = value.to_option::<Cont>().unwrap().unwrap();
    /// let cont = cont.borrow();
    ///
    /// assert_eq!(cont.value, 3);
    /// assert!(mruby.nil().to_option::<Cont>().unwrap().is_none());
    /// ```

    pub fn to_option<T: Any>(&self) -> Result<Option<Rc<RefCell<T>>>, MrubyError> {
        if self.value.typ == MrType::MRB_TT_DATA {
            self.to_obj::<T>().map(|obj| Some(obj))
        } else {
            Ok(None)
        }
    }

    /// Casts mruby `Value` of `Class` `Array` to Rust type `Vec<Value>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
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

    pub fn to_vec(&self) -> Result<Vec<Value>, MrubyError> {
        unsafe {
            self.value.to_vec(self.mruby.borrow().mrb).map(|vec| {
                vec.iter()
                    .map(|mrvalue| Value::new(self.mruby.clone(), *mrvalue))
                    .collect()
            })
        }
    }

    /// Casts mruby `Value` of `Class` `Class` to Rust type `Class`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    /// let result = mruby.run("Object").unwrap();
    ///
    /// assert_eq!(result.to_class().unwrap().to_str(), "Object");
    /// ```

    pub fn to_class(&self) -> Result<Class, MrubyError> {
        unsafe {
            let class = self.value.to_class()?;

            Ok(Class::new(self.mruby.clone(), class))
        }
    }

    /// Casts mruby `Value` of `Class` `Module` to Rust type `Module`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    /// let result = mruby.run("Kernel").unwrap();
    ///
    /// assert_eq!(result.to_module().unwrap().to_str(), "Kernel");
    /// ```

    pub fn to_module(&self) -> Result<Module, MrubyError> {
        unsafe {
            let module = self.value.to_module()?;

            Ok(Module::new(self.mruby.clone(), module))
        }
    }
}

use std::fmt;

impl Clone for Value {
    // #[allow(unused_must_use)]
    fn clone(&self) -> Value {
        if self.value.typ == MrType::MRB_TT_DATA {
            unsafe {
                let ptr = mrb_ext_data_ptr(self.value);
                let rc: Rc<c_void> = mem::transmute(ptr);

                // Looks like a bad magic, but I dont know how to avoid
                //rc.clone();

                mem::forget(rc);
            }
        }

        Value::new(self.mruby.clone(), self.value.clone())
    }
}

impl PartialEq<Value> for Value {
    fn eq(&self, other: &Value) -> bool {
        let result = self.call("==", vec![other.clone()]).unwrap();

        result.to_bool().unwrap()
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value {{ {:?} }}", self.value)
    }
}

/// A `trait` which connects `Class` & `Module`.
pub trait ClassLike {
    fn class(&self) -> *const MrClass;
}

/// A `struct` that wraps around an mruby `Class`.
///
/// # Examples
///
/// ```
/// # use mrusty::Mruby;
/// # use mrusty::MrubyImpl;
/// let mruby = Mruby::new();
///
/// struct Cont;
///
/// let class = mruby.def_class_for::<Cont>("Container");
///
/// assert_eq!(class.to_str(), "Container");
/// ```
pub struct Class {
    mruby: MrubyType,
    class: *const MrClass,
    name: String,
}

impl Class {
    /// Not meant to be called directly.
    #[doc(hidden)]
    pub fn new(mruby: MrubyType, class: *const MrClass) -> Class {
        let name = unsafe {
            let name = mrb_class_name(mruby.borrow().mrb, class);

            CStr::from_ptr(name).to_str().unwrap()
        };

        Class {
            mruby: mruby,
            class: class,
            name: name.to_owned(),
        }
    }

    /// Includes a `Module` in a `Class`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// mruby.run("
    ///   module Increment
    ///     def inc
    ///       self + 1
    ///     end
    ///   end
    /// ").unwrap();
    ///
    /// let fixnum = mruby.get_class("Fixnum").unwrap();
    /// let increment = mruby.get_module("Increment").unwrap();
    ///
    /// fixnum.include(increment);
    ///
    /// let result = mruby.run("1.inc").unwrap();
    ///
    /// assert_eq!(result.to_i32().unwrap(), 2);
    /// ```
    pub fn include(&self, module: Module) {
        unsafe {
            mrb_include_module(self.mruby.borrow().mrb, self.class, module.module);
        }
    }

    /// Defines constant with name `name` and value `value` on a `Class`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// mruby.run("
    ///   class Container; end
    /// ").unwrap();
    ///
    /// let cont = mruby.get_class("Container").unwrap();
    ///
    /// cont.def_const("ONE", mruby.fixnum(1));
    ///
    /// let result = mruby.run("Container::ONE").unwrap();
    ///
    /// assert_eq!(result.to_i32().unwrap(), 1);
    /// ```
    pub fn def_const(&self, name: &str, value: Value) {
        unsafe {
            let name_str = CString::new(name).unwrap();

            mrb_define_const(
                self.mruby.borrow().mrb,
                self.class,
                name_str.as_ptr(),
                value.value,
            );
        }
    }

    /// Returns a `&str` with the mruby `Class` name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// struct Cont;
    ///
    /// let class = mruby.def_class_for::<Cont>("Container");
    ///
    /// assert_eq!(class.to_str(), "Container");
    /// ```

    pub fn to_str(&self) -> &str {
        &self.name
    }

    /// Casts `Class` to `Value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// struct Cont;
    ///
    /// let class = mruby.def_class_for::<Cont>("Container");
    /// let value = class.to_value();
    ///
    /// let name = value.call("to_s", vec![]).unwrap();
    ///
    /// assert_eq!(name.to_str().unwrap(), "Container");
    /// ```

    pub fn to_value(&self) -> Value {
        unsafe {
            let value = mrb_ext_class_value(self.class);

            Value::new(self.mruby.clone(), value)
        }
    }
}

impl ClassLike for Class {
    fn class(&self) -> *const MrClass {
        self.class
    }
}

impl Clone for Class {
    fn clone(&self) -> Class {
        Class::new(self.mruby.clone(), self.class)
    }
}

impl PartialEq<Class> for Class {
    fn eq(&self, other: &Class) -> bool {
        let result = self.to_value().call("==", vec![other.to_value()]).unwrap();

        result.to_bool().unwrap()
    }
}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Class {{ {:?} }}", self.to_str())
    }
}

/// A `struct` that wraps around an mruby `Module`.
///
/// # Examples
///
/// ```
/// # use mrusty::Mruby;
/// # use mrusty::MrubyImpl;
/// let mruby = Mruby::new();
///
/// let module = mruby.def_module("Container");
///
/// assert_eq!(module.to_str(), "Container");
/// ```
pub struct Module {
    mruby: MrubyType,
    module: *const MrClass,
    name: String,
}

impl Module {
    /// Not meant to be called directly.
    #[doc(hidden)]
    pub fn new(mruby: MrubyType, module: *const MrClass) -> Module {
        let name = unsafe {
            let name = mrb_class_name(mruby.borrow().mrb, module);

            CStr::from_ptr(name).to_str().unwrap()
        };

        Module {
            mruby: mruby,
            module: module,
            name: name.to_owned(),
        }
    }

    /// Includes a `Module` in a `Module`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// mruby.run("
    ///   module Increment
    ///     def inc
    ///       self + 1
    ///     end
    ///   end
    ///
    ///   module Increment2; end
    /// ").unwrap();
    ///
    /// let fixnum = mruby.get_class("Fixnum").unwrap();
    /// let increment = mruby.get_module("Increment").unwrap();
    /// let increment2 = mruby.get_module("Increment2").unwrap();
    ///
    /// increment2.include(increment);
    /// fixnum.include(increment2);
    ///
    /// let result = mruby.run("1.inc").unwrap();
    ///
    /// assert_eq!(result.to_i32().unwrap(), 2);
    /// ```
    pub fn include(&self, module: Module) {
        unsafe {
            mrb_include_module(self.mruby.borrow().mrb, self.module, module.module);
        }
    }

    /// Defines constant with name `name` and value `value` on a `Module`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// mruby.run("
    ///   module Container; end
    /// ").unwrap();
    ///
    /// let cont = mruby.get_module("Container").unwrap();
    ///
    /// cont.def_const("ONE", mruby.fixnum(1));
    ///
    /// let result = mruby.run("Container::ONE").unwrap();
    ///
    /// assert_eq!(result.to_i32().unwrap(), 1);
    /// ```
    pub fn def_const(&self, name: &str, value: Value) {
        unsafe {
            let name_str = CString::new(name).unwrap();

            mrb_define_const(
                self.mruby.borrow().mrb,
                self.module,
                name_str.as_ptr(),
                value.value,
            );
        }
    }

    /// Returns a `&str` with the mruby `Module` name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// let module = mruby.def_module("Container");
    ///
    /// assert_eq!(module.to_str(), "Container");
    /// ```

    pub fn to_str(&self) -> &str {
        &self.name
    }

    /// Casts `Module` to `Value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mrusty::Mruby;
    /// # use mrusty::MrubyImpl;
    /// let mruby = Mruby::new();
    ///
    /// let module = mruby.def_module("Container");
    /// let value = module.to_value();
    ///
    /// let name = value.call("to_s", vec![]).unwrap();
    ///
    /// assert_eq!(name.to_str().unwrap(), "Container");
    /// ```

    pub fn to_value(&self) -> Value {
        unsafe {
            let value = mrb_ext_module_value(self.module);

            Value::new(self.mruby.clone(), value)
        }
    }
}

impl ClassLike for Module {
    fn class(&self) -> *const MrClass {
        self.module
    }
}

impl Clone for Module {
    fn clone(&self) -> Module {
        Module::new(self.mruby.clone(), self.module)
    }
}

impl PartialEq<Module> for Module {
    fn eq(&self, other: &Module) -> bool {
        let result = self.to_value().call("==", vec![other.to_value()]).unwrap();

        result.to_bool().unwrap()
    }
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Module {{ {:?} }}", self.to_str())
    }
}
