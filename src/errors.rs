#[cfg(not(feature = "std"))]
use core::{
    fmt::Debug,
    write,
};
#[cfg(not(feature = "std"))]
use alloc::string::String;
use thiserror_no_std::Error;


#[derive(Debug, Error)]
pub enum FromStrParseError {
    #[error("invalid str: `{0}`")]
    InvalidStr(String),
}


#[derive(Debug, Error)]
pub enum FromIntParseError {
    #[error("invalid value: `{0}`")]
    InvalidInt(isize),
}
