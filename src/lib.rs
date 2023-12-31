#[cfg(feature = "jkcenum_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate jkcenum_derive;

#[cfg(feature = "jkcenum_derive")]
pub use jkcenum_derive::JkcEnum;

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
