#![cfg_attr(not(feature = "std"), no_std)]

extern crate proc_macro;

#[cfg(not(feature = "std"))]
extern crate alloc;

mod attribute;
mod derive_enum;
mod iter;

use proc_macro::TokenStream;
use attribute::ContainerAttributes;
use virtue::prelude::*;


#[proc_macro_derive(JkcEnum, attributes(jenum))]
pub fn derive_jkcenum(input: TokenStream) -> TokenStream {
    derive_jkcenum_inner(input).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_jkcenum_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, attributes, body) = parse.into_generator();
    let attributes = attributes
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    match body {
        Body::Struct(_body) => {
            return Err(Error::custom("not support struct."));
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
            }
            .generate_jkcenum(&mut generator)?;
        }
    }

    generator.export_to_file("jenum", "JkcEnum");
    generator.finish()
}


#[proc_macro_derive(JenumToString, attributes(jenum))]
pub fn derive_jkcenum_to_string(input: TokenStream) -> TokenStream {
    derive_jkcenum_to_string_inner(input).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_jkcenum_to_string_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, attributes, body) = parse.into_generator();
    let attributes = attributes
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    match body {
        Body::Struct(_body) => {
            return Err(Error::custom("not support struct."));
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
            }
            .generate_enum_display(&mut generator)?;
        }
    }

    generator.export_to_file("jenum", "JkcEnum");
    generator.finish()
}


#[proc_macro_derive(JenumToVec, attributes(jenum))]
pub fn derive_jkcenum_to_vec(input: TokenStream) -> TokenStream {
    derive_jkcenum_to_vec_inner(input).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_jkcenum_to_vec_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, attributes, body) = parse.into_generator();
    let attributes = attributes
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    match body {
        Body::Struct(_body) => {
            return Err(Error::custom("not support struct."));
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
            }
            .generate_enum_to_vec(&mut generator)?;
        }
    }

    generator.export_to_file("jenum", "JkcEnum");
    generator.finish()
}


#[proc_macro_derive(JenumFromStr, attributes(jenum))]
pub fn derive_jkcenum_from_str(input: TokenStream) -> TokenStream {
    derive_jkcenum_from_str_inner(input).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_jkcenum_from_str_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, attributes, body) = parse.into_generator();
    let attributes = attributes
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    match body {
        Body::Struct(_body) => {
            return Err(Error::custom("not support struct."));
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
            }
            .generate_enum_from_str(&mut generator)?;
        }
    }

    generator.export_to_file("jenum", "JkcEnum");
    generator.finish()
}


#[proc_macro_derive(JenumFromInt, attributes(jenum))]
pub fn derive_jkcenum_from_int(input: TokenStream) -> TokenStream {
    derive_jkcenum_from_int_inner(input).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_jkcenum_from_int_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, attributes, body) = parse.into_generator();
    let attributes = attributes
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    match body {
        Body::Struct(_body) => {
            return Err(Error::custom("not support struct."));
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
            }
            .generate_enum_from_int(&mut generator)?;
        }
    }

    generator.export_to_file("jenum", "JkcEnum");
    generator.finish()
}
