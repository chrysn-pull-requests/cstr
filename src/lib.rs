//! A macro for getting `&'static CStr` from literal.
//!
//! This macro checks whether the given literal is valid for `CStr`
//! at compile time, and returns a static reference of `CStr`.
//!
//! Note that it currently cannot be used to initialize constants due
//! to restriction of Rust.
//!
//! Also, it currently only supports a UTF-8 string as input because
//! Rust's tokenizer only accepts that without the `b` prefix. This
//! may be expanded in the future if necessary.
//!
//! # Example
//!
//! ```
//! #[macro_use] extern crate cstr;
//! use std::ffi::CStr;
//!
//! # fn main() {
//! let test = cstr!("hello");
//! assert_eq!(test, CStr::from_bytes_with_nul(b"hello\0").unwrap());
//! # }
//! ```
//!
//! # Note for 2018 edition
//!
//! On Rust 2018 edition, you can not simply use `cstr::cstr!(...)` or
//! `use cstr::cstr;` due to limitation of procedural-masquerade,
//! the crate we use to build this macro.
//!
//! You need to stick with 2015 edition's `#[macro_use]` or import
//! everything from this crate via `use cstr::*;`.

#[allow(unused_imports)]
#[macro_use]
extern crate cstr_macros;
#[macro_use]
extern crate procedural_masquerade;

#[doc(hidden)]
pub use cstr_macros::*;

define_invoke_proc_macro!(cstr__invoke_build_bytes);

#[macro_export]
macro_rules! cstr {
    ($t: tt) => {
        {
            cstr__invoke_build_bytes! {
                cstr_internal__build_bytes!($t)
            }
            unsafe {
                ::std::ffi::CStr::from_bytes_with_nul_unchecked(BYTES)
            }
        }
    }
}
