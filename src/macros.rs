// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Not meant to be called directly.
#[doc(hidden)]
#[macro_export]
macro_rules! init {
    () => ();
    ( $name:ident, bool )    => (let $name = uninitialized::<bool>(););
    ( $name:ident, i32 )     => (let $name = uninitialized::<i32>(););
    ( $name:ident, f64 )     => (let $name = uninitialized::<f64>(););
    ( $name:ident, str )     => (let $name = uninitialized::<*const c_char>(););
    ( $name:ident, Vec )     => (let $name = uninitialized::<MrValue>(););
    ( $name:ident, $_t:ty )  => (let $name = uninitialized::<MrValue>(););
    ( $name:ident : $t:tt )  => (init!($name, $t));
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
    ( bool )    => ("b");
    ( i32 )     => ("i");
    ( f64 )     => ("f");
    ( str )     => ("z");
    ( Vec )     => ("A");
    ( $_t:ty )  => ("o");
    ( $t:tt, $( $ts:tt ),+ ) => (concat!(sig!($t), sig!($( $ts ),*)));
}

/// Not meant to be called directly.
#[doc(hidden)]
#[macro_export]
macro_rules! args {
    () => ();
    ( $name:ident, bool )    => (&$name as *const bool);
    ( $name:ident, i32 )     => (&$name as *const i32);
    ( $name:ident, f64 )     => (&$name as *const f64);
    ( $name:ident, str )     => (&$name as *const *const c_char);
    ( $name:ident, Vec )     => (&$name as *const MrValue);
    ( $name:ident, $_t:ty )  => (&$name as *const MrValue);
    ( $name:ident : $t:tt )  => (args!($name, $t));
    ( $mrb:expr, $sig:expr, $name:ident : $t:tt) => {
        mrb_get_args($mrb, $sig, args!($name, $t));
    };
    ( $mrb:expr, $sig:expr, $name:ident : $t:tt, $($names:ident : $ts:tt),+ ) => {
        mrb_get_args($mrb, $sig, args!($name, $t), $( args!($names : $ts) ),*);
    };
}

/// Not meant to be called directly.
#[doc(hidden)]
#[macro_export]
macro_rules! args_rest {
    ( $mruby:expr, $sig:expr, $name:ident : $t:tt) => {
        {
            let mrb = $mruby.borrow().mrb;

            let args = uninitialized::<*mut MrValue>();
            let count = uninitialized::<i32>();

            mrb_get_args(mrb, $sig, args!($name, $t), &args as *const *mut MrValue,
                         &count as *const i32);

            let args = slice::from_raw_parts(args, count as usize);
            args.iter().map(|arg| { Value::new($mruby.clone(), arg.clone()) }).collect::<Vec<_>>()
         }
    };
    ( $mruby:expr, $sig:expr, $name:ident : $t:tt, $($names:ident : $ts:tt),+ ) => {
        {
            let mrb = $mruby.borrow().mrb;

            let args = uninitialized::<*mut MrValue>();
            let count = uninitialized::<i32>();

            mrb_get_args(mrb, $sig, args!($name, $t), $( args!($names : $ts) ),* ,
                         &args as *const *mut MrValue, &count as *const i32);

            let args = slice::from_raw_parts(args, count as usize);
            args.iter().map(|arg| { Value::new($mruby.clone(), arg.clone()) }).collect::<Vec<_>>()
         }
    };
}

/// Not meant to be called directly.
#[doc(hidden)]
#[macro_export]
macro_rules! conv {
    ( $mruby:expr )                       => ();
    ( $mruby:expr, $name:ident, bool )    => ();
    ( $mruby:expr, $name:ident, i32 )     => ();
    ( $mruby:expr, $name:ident, f64 )     => ();
    ( $mruby:expr, $name:ident, str )     => {
        let $name = CStr::from_ptr($name).to_str().unwrap();
    };
    ( $mruby:expr, $name:ident, Vec )     => {
        let $name = Value::new($mruby.clone(), $name).to_vec().unwrap();
    };
    ( $mruby:expr, $name:ident, Value )   => {
        let $name = Value::new($mruby.clone(), $name);
    };
    ( $mruby:expr, $name:ident, $t:ty )   => {
        let $name = Value::new($mruby.clone(), $name).to_obj::<$t>().unwrap();
    };
    ( $mruby:expr, $name:ident : $t:tt )  => (conv!($mruby, $name, $t));
    ( $mruby:expr, $name:ident : $t:tt, $($names:ident : $ts:tt),+ ) => {
        conv!($mruby, $name, $t);
        conv!($mruby, $( $names : $ts ),*);
    };
}

/// Not meant to be called directly.
#[doc(hidden)]
#[macro_export]
macro_rules! slf {
    ( $slf:ident, Value ) => ();
    ( $slf:ident, $t:ty ) => (let $slf = $slf.to_obj::<$t>().unwrap(););
}

/// A `macro` useful for defining Rust closures for mruby. Requires `use mrusty::*;`.
///
/// Types can be:
///
/// * `bool`
/// * `i32`
/// * `f64`
/// * `str` (`&str`; macro limtation)
/// * `Vec` (`Vec<Value>`; macro limtation)
/// * `T` (defined with `def_class`)
/// * `Value`
///
/// Any `panic!` call within the closure will get rescued in a `RustPanic` mruby `Exception`.
///
/// # Examples
///
/// `mrfn!` uses the usual Rust closure syntax. `mruby` does not need type information.
/// `slf` can be either `Value` or `T`.
///
/// ```
/// # #[macro_use] extern crate mrusty;
/// use mrusty::*;
///
/// # fn main() {
/// let mruby = Mruby::new();
///
/// struct Cont;
///
/// mruby.def_class_for::<Cont>("Container");
/// // slf cannot be cast to Cont because it does not define initialize().
/// mruby.def_method_for::<Cont, _>("hi", mrfn!(|mruby, _slf: Value, a: i32, b: i32| {
///     mruby.fixnum(a + b)
/// }));
///
/// let result = mruby.run("Container.new.hi 1, 2").unwrap();
///
/// assert_eq!(result.to_i32().unwrap(), 3);
/// # }
/// ```
/// <br/>
///
/// `mrfn!` is also used for class method definitions.
///
/// ```
/// # #[macro_use] extern crate mrusty;
/// use mrusty::*;
///
/// # fn main() {
/// let mruby = Mruby::new();
///
/// struct Cont;
///
/// mruby.def_class_for::<Cont>("Container");
/// mruby.def_class_method_for::<Cont, _>("hi", mrfn!(|mruby, _slf: Value, a: str, b: str| {
///     mruby.string(&(a.to_owned() + b))
/// }));
/// // slf is a Value here. (mruby Class type)
/// mruby.def_class_method_for::<Cont, _>("class_name", mrfn!(|_mruby, slf: Value| {
///     slf.call("to_s", vec![]).unwrap()
/// }));
///
/// let result = mruby.run("Container.hi 'a', 'b'").unwrap();
/// let name = mruby.run("Container.class_name").unwrap();
///
/// assert_eq!(result.to_str().unwrap(), "ab");
/// assert_eq!(name.to_str().unwrap(), "Container");
/// # }
/// ```
/// <br/>
///
/// `mrfn!` does automatic casting on all mruby classes defined with `def_class`.
///
/// ```
/// # #[macro_use] extern crate mrusty;
/// use mrusty::*;
///
/// # fn main() {
/// let mruby = Mruby::new();
///
/// struct Cont {
///     value: i32
/// };
///
/// mruby.def_class_for::<Cont>("Container");
/// mruby.def_method_for::<Cont, _>("gt", mrfn!(|mruby, slf: Cont, o: Cont| {
///    mruby.bool(slf.value > o.value)
/// }));
///
/// let a = mruby.obj::<Cont>(Cont { value: 3 });
/// let b = mruby.obj::<Cont>(Cont { value: 2 });
///
/// let result = a.call("gt", vec![b]).unwrap();
///
/// assert_eq!(result.to_bool().unwrap(), true);
/// # }
/// ```
/// <br/>
///
/// Last, optional untyped argument will match all remaining arguments, as long as it's separated
/// by a `;`.
///
/// ```
/// # #[macro_use] extern crate mrusty;
/// use mrusty::*;
///
/// # fn main() {
/// let mruby = Mruby::new();
///
/// struct Cont {
///     value: i32
/// };
///
/// mruby.def_class_for::<Cont>("Container");
/// mruby.def_method_for::<Cont, _>("initialize", mrfn!(|mruby, slf: Value; args| {
///    let cont = Cont { value: args[0].to_i32().unwrap() + args[1].to_i32().unwrap() };
///
///    slf.init(cont)
/// }));
///
/// let result = mruby.run("Container.new 1, 2, 3").unwrap();
///
/// assert_eq!(result.to_obj::<Cont>().unwrap().value, 3);
/// # }
/// ```
#[macro_export]
macro_rules! mrfn {
    ( |$mruby:ident, $slf:ident : $st:tt| $block:expr ) => {
        |$mruby, $slf| {
            slf!($slf, $st);

            $block
        }
    };
    ( |$mruby:ident, $slf:ident : $st:tt; $args:ident| $block:expr ) => {
        |$mruby, $slf| {
            use std::ffi::CString;
            use std::mem::uninitialized;
            use std::slice;

            slf!($slf, $st);

            unsafe {
                let mrb = $mruby.borrow().mrb;

                let $args = uninitialized::<*mut MrValue>();
                let count = uninitialized::<i32>();

                mrb_get_args(mrb, CString::new("*").unwrap().as_ptr(),
                             &$args as *const *mut MrValue, &count as *const i32);

                let $args = slice::from_raw_parts($args, count as usize);
                let $args = $args.iter().map(|arg| {
                    Value::new($mruby.clone(), arg.clone())
                }).collect::<Vec<_>>();

                $block
            }
        }
    };
    ( |$mruby:ident, $slf:ident : $st:tt, $( $name:ident : $t:tt ),*| $block:expr ) => {
        |$mruby, $slf| {
            #[allow(unused_imports)]
            use std::ffi::CStr;
            use std::ffi::CString;
            #[allow(unused_imports)]
            use std::mem::uninitialized;
            #[allow(unused_imports)]
            use std::os::raw::c_char;

            unsafe {
                slf!($slf, $st);

                init!($( $name : $t ),*);

                let mrb = $mruby.borrow().mrb;
                let sig = CString::new(sig!($( $t ),*)).unwrap().as_ptr();

                args!(mrb, sig, $( $name : $t ),*);
                conv!($mruby, $( $name : $t ),*);

                $block
            }
        }
    };
    ( |$mruby:ident, $slf:ident : $st:tt, $( $name:ident : $t:tt ),* ; $args:ident| $block:expr ) => {
        |$mruby, $slf| {
            #[allow(unused_imports)]
            use std::ffi::CStr;
            use std::ffi::CString;
            #[allow(unused_imports)]
            use std::mem::uninitialized;
            #[allow(unused_imports)]
            use std::os::raw::c_char;
            use std::slice;

            unsafe {
                slf!($slf, $st);

                init!($( $name : $t ),*);

                let sig = CString::new(concat!(sig!($( $t ),*), "*")).unwrap().as_ptr();

                let $args = args_rest!($mruby, sig, $( $name : $t ),*);
                conv!($mruby, $( $name : $t ),*);

                $block
            }
        }
    };
}

/// Not meant to be called directly.
#[doc(hidden)]
#[macro_export]
macro_rules! defines {
    // end recursion
    ( $mruby:expr, $name:ty, ) => ();

    // initialize
    ( $mruby:expr, $name:ty, def!("initialize", || $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>("initialize", mrfn!(|_mruby, slf: Value| {
            slf.init($block)
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def!("initialize", | $( $n:ident : $t:tt ),* | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>("initialize", mrfn!(|_mruby, slf: Value, $( $n : $t ),*| {
            slf.init($block)
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def!("initialize", | $mrb:ident | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>("initialize", mrfn!(|$mrb, slf: Value| {
            slf.init($block)
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def!("initialize", | $mrb:ident, $( $n:ident : $t:tt ),* | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>("initialize", mrfn!(|$mrb, slf: Value, $( $n : $t ),*| {
            slf.init($block)
        }));

        defines!($mruby, $name, $( $rest )*);
    };

    // instance methods
    ( $mruby:expr, $name:ty, def!($method:expr, | $slf:ident : $st:tt | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>($method, mrfn!(|_mruby, $slf: $st| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def!($method:expr, | $slf:ident : $st:tt, $( $n:ident : $t:tt ),* | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>($method, mrfn!(|_mruby, $slf: $st, $( $n : $t ),*| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def!($method:expr, | $mrb:ident, $slf:ident : $st:tt | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>($method, mrfn!(|$mrb, $slf: $st| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def!($method:expr, | $mrb:ident, $slf:ident : $st:tt, $( $n:ident : $t:tt ),* | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>($method, mrfn!(|$mrb, $slf: $st, $( $n : $t ),*| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };

    // class methods
    ( $mruby:expr, $name:ty, def_self!($method:expr, | $slf:ident : $st:tt | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_class_method_for::<$name, _>($method, mrfn!(|_mruby, $slf: $st| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def_self!($method:expr, | $slf:ident : $st:tt, $( $n:ident : $t:tt ),* | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_class_method_for::<$name, _>($method, mrfn!(|_mruby, $slf: $st, $( $n : $t ),*| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def_self!($method:expr, | $mrb:ident, $slf:ident : $st:tt | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_class_method_for::<$name, _>($method, mrfn!(|$mrb, $slf: $st| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def_self!($method:expr, | $mrb:ident, $slf:ident : $st:tt, $( $n:ident : $t:tt ),* | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_class_method_for::<$name, _>($method, mrfn!(|$mrb, $slf: $st, $( $n : $t ),*| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };

    // initialize args
    ( $mruby:expr, $name:ty, def!("initialize", | ; $args:ident | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>("initialize", mrfn!(|_mruby, slf: Value; $args:ident| {
            slf.init($block)
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def!("initialize", | $( $n:ident : $t:tt ),* ; $args:ident | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>("initialize", mrfn!(|_mruby, slf: Value, $( $n : $t ),*; $args| {
            slf.init($block)
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def!("initialize", | $mrb:ident; $args:ident | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>("initialize", mrfn!(|$mrb, slf: Value; $args| {
            slf.init($block)
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def!("initialize", | $mrb:ident, $( $n:ident : $t:tt ),* ; $args:ident | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>("initialize", mrfn!(|$mrb, slf: Value, $( $n : $t ),* ; $args| {
            slf.init($block)
        }));

        defines!($mruby, $name, $( $rest )*);
    };

    // instance methods args
    ( $mruby:expr, $name:ty, def!($method:expr, | $slf:ident : $st:tt; $args:ident | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>($method, mrfn!(|_mruby, $slf: $st; $args| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def!($method:expr, | $slf:ident : $st:tt, $( $n:ident : $t:tt ),* ; $args:ident | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>($method, mrfn!(|_mruby, $slf: $st, $( $n : $t ),* ; $args| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def!($method:expr, | $mrb:ident, $slf:ident : $st:tt; $args:ident | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>($method, mrfn!(|$mrb, $slf: $st; $args| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def!($method:expr, | $mrb:ident, $slf:ident : $st:tt, $( $n:ident : $t:tt ),* ; $args:ident | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_method_for::<$name, _>($method, mrfn!(|$mrb, $slf: $st, $( $n : $t ),* ; $args| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };

    // class methods args
    ( $mruby:expr, $name:ty, def_self!($method:expr, | $slf:ident : $st:tt; $args:ident | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_class_method_for::<$name, _>($method, mrfn!(|_mruby, $slf: $st; $args| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def_self!($method:expr, | $slf:ident : $st:tt, $( $n:ident : $t:tt ),* ; $args:ident | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_class_method_for::<$name, _>($method, mrfn!(|_mruby, $slf: $st, $( $n : $t ),* ; $args| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def_self!($method:expr, | $mrb:ident, $slf:ident : $st:tt; $args:ident | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_class_method_for::<$name, _>($method, mrfn!(|$mrb, $slf: $st; $args| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };
    ( $mruby:expr, $name:ty, def_self!($method:expr, | $mrb:ident, $slf:ident : $st:tt, $( $n:ident : $t:tt ),* ; $args:ident | $block:expr ); $( $rest:tt )* ) => {
        $mruby.def_class_method_for::<$name, _>($method, mrfn!(|$mrb, $slf: $st, $( $n : $t ),* ; $args| {
            $block
        }));

        defines!($mruby, $name, $( $rest )*);
    };
}

/// A `macro` that comes in handy when defining class in order to remove a large part of the
/// clutter and ensure correction. It automates and simplifies the implementation of the
/// `MrubyFile` `trait`. Thus, any type provided to `mrclass!` will get an `MrubyFile`
/// implementation.
///
/// The macro takes a Rust type, an optional mruby `Class` name, and a block as arguments. Inside
/// of the block you can define mruby methods with the `def!` and `def_self!` helpers which are
/// not visible outside of this macro.
///
/// # Examples
///
/// Use `def!` to define mruby instance methods. `"initialize"` is a special type of instance
/// method which require you to return an initialized type. Apart from this, all methods require
/// you to return a `Value`. Apart from that, `"initialize"` is the only method that doesn't
/// take itself as an argument.
///
/// *Note:* `mruby` argument is optional.
///
/// ```
/// # #[macro_use] extern crate mrusty;
/// use mrusty::*;
///
/// # fn main() {
/// let mruby = Mruby::new();
///
/// struct Cont {
///     value: i32
/// };
///
/// mrclass!(Cont, "Container", {
///     def!("initialize", |v: i32| {
///         Cont { value: v }
///     });
///
///     def!("value", |mruby, slf: Cont| {
///         mruby.fixnum(slf.value)
///     });
/// });
///
/// Cont::require(mruby.clone()); // needs to be required manually
///
/// let result = mruby.run("Container.new(3).value").unwrap();
///
/// assert_eq!(result.to_i32().unwrap(), 3);
/// # }
/// ```
/// <br/>
///
/// Use `def_self!` to define mruby class methods.
///
/// *Note:* `mruby` argument is optional.
///
/// ```
/// # #[macro_use] extern crate mrusty;
/// use mrusty::*;
///
/// # fn main() {
/// let mruby = Mruby::new();
///
/// struct Cont {
///     value: i32
/// };
///
/// mrclass!(Cont, "Container", {
///     def_self!("hi", |mruby, slf: Value| {
///         mruby.string("hi")
///     });
/// });
///
/// Cont::require(mruby.clone()); // needs to be required manually
///
/// let result = mruby.run("Container.hi").unwrap();
///
/// assert_eq!(result.to_str().unwrap(), "hi");
/// # }
/// ```
#[macro_export]
macro_rules! mrclass {
    ( $name:tt ) => {
        impl MrubyFile for $name {
            fn require(mruby: MrubyType) {
                mruby.def_class_for::<$name>(stringify!($name));
            }
        }
    };
    ( $name:ty, { $( $rest:tt )* } ) => {
        impl MrubyFile for $name {
            fn require(mruby: MrubyType) {
                mruby.def_class_for::<$name>(stringify!($name));

                defines!(mruby, $name, $( $rest )*);
            }
        }
    };
    ( $name:tt, $mrname:expr ) => {
        impl MrubyFile for $name {
            fn require(mruby: MrubyType) {
                mruby.def_class_for::<$name>($mrname);
            }
        }
    };
    ( $name:ty, $mrname:expr, { $( $rest:tt )* } ) => {
        impl MrubyFile for $name {
            fn require(mruby: MrubyType) {
                mruby.def_class_for::<$name>($mrname);

                defines!(mruby, $name, $( $rest )*);
            }
        }
    };
}
