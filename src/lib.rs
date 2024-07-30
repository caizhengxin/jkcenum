#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "jkcenum_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate jkcenum_derive;

#[cfg(feature = "jkcenum_derive")]
pub use jkcenum_derive::JkcEnum;

#[cfg(not(feature = "std"))]
use core::{
    marker::Sized,
    option::Option,
    option::Option::{Some, None},
    result::Result,
    result::Result::Ok,
};
#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};


pub mod errors;


pub trait FromInt {
    type Err;

    fn from_int(v: isize) -> Result<Self, Self::Err>
        where
            Self: Sized,
    ;

    fn from_int_to_string(v: isize) -> Option<String>
        where
            Self: Sized + ToString,
    {
        if let Ok(value) = Self::from_int(v) {
            return Some(value.to_string())
        }

        None
    }
}
