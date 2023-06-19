// TODO: re-enable this lint when we bump msrv to 1.58
#![allow(clippy::uninlined_format_args)]
extern crate proc_macro;
use borsh_derive_internal::*;
use borsh_schema_derive_internal::*;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::crate_name;
use proc_macro_crate::FoundCrate;
use quote::ToTokens;
use syn::{
    parse_macro_input, DeriveInput, Ident, ItemEnum, ItemStruct, ItemUnion, Meta, MetaNameValue,
};

#[proc_macro_derive(BorshSerialize, attributes(borsh_skip, use_discriminant))]
pub fn borsh_serialize(input: TokenStream) -> TokenStream {
    let name = &crate_name("borsh").unwrap();
    let name = match name {
        FoundCrate::Itself => "borsh",
        FoundCrate::Name(name) => name.as_str(),
    };
    let cratename = Ident::new(name, Span::call_site());

    let for_derive_input = input.clone();
    let derive_input = parse_macro_input!(for_derive_input as DeriveInput);

    // Read the additional data
    let mut use_discriminant = None;
    for attr in &derive_input.attrs {
        if attr.path().is_ident("use_discriminant") {
            match attr.meta.clone() {
                Meta::NameValue(value) => match value {
                    MetaNameValue {
                        path,
                        eq_token: _,
                        value,
                    } => {
                        if path.is_ident("use_discriminant") {
                            let value = value.to_token_stream().to_string();
                            use_discriminant = match value.as_str() {
                                "true" => Some(true),
                                "false" => Some(false),
                                _ => {
                                    return TokenStream::from(
                                        syn::Error::new(Span::call_site(), "`use_discriminant` ")
                                            .to_compile_error(),
                                    );
                                }
                            };
                        }
                    }
                },
                _ => {}
            }
        }
    }

    let res = if let Ok(input) = syn::parse::<ItemStruct>(input.clone()) {
        struct_ser(&input, cratename)
    } else if let Ok(input) = syn::parse::<ItemEnum>(input.clone()) {
        enum_ser(&input, cratename, use_discriminant)
    } else if let Ok(input) = syn::parse::<ItemUnion>(input) {
        union_ser(&input, cratename)
    } else {
        // Derive macros can only be defined on structs, enums, and unions.
        unreachable!()
    };
    TokenStream::from(match res {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    })
}

#[proc_macro_derive(BorshDeserialize, attributes(borsh_skip, borsh_init, use_discriminant))]
pub fn borsh_deserialize(input: TokenStream) -> TokenStream {
    let name = &crate_name("borsh").unwrap();
    let name = match name {
        FoundCrate::Itself => "borsh",
        FoundCrate::Name(name) => name.as_str(),
    };
    let cratename = Ident::new(name, Span::call_site());

    let for_derive_input = input.clone();
    let derive_input = parse_macro_input!(for_derive_input as DeriveInput);

    // Read the additional data
    let mut use_discriminant = None;
    for attr in &derive_input.attrs {
        if attr.path().is_ident("use_discriminant") {
            match attr.meta.clone() {
                Meta::NameValue(value) => match value {
                    MetaNameValue {
                        path,
                        eq_token: _,
                        value,
                    } => {
                        if path.is_ident("use_discriminant") {
                            let value = value.to_token_stream().to_string();
                            use_discriminant = match value.as_str() {
                                "true" => Some(true),
                                "false" => Some(false),
                                _ => {
                                    return TokenStream::from(
                                        syn::Error::new(Span::call_site(), "`use_discriminant` ")
                                            .to_compile_error(),
                                    );
                                }
                            };
                        }
                    }
                },
                _ => {}
            }
        }
    }

    let res = if let Ok(input) = syn::parse::<ItemStruct>(input.clone()) {
        struct_de(&input, cratename)
    } else if let Ok(input) = syn::parse::<ItemEnum>(input.clone()) {
        enum_de(&input, cratename, use_discriminant)
    } else if let Ok(input) = syn::parse::<ItemUnion>(input) {
        union_de(&input, cratename)
    } else {
        // Derive macros can only be defined on structs, enums, and unions.
        unreachable!()
    };
    TokenStream::from(match res {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    })
}

#[proc_macro_derive(BorshSchema, attributes(borsh_skip, use_discriminant))]
pub fn borsh_schema(input: TokenStream) -> TokenStream {
    let name = &crate_name("borsh").unwrap();
    let name = match name {
        FoundCrate::Itself => "borsh",
        FoundCrate::Name(name) => name.as_str(),
    };
    let cratename = Ident::new(name, Span::call_site());

    let res = if let Ok(input) = syn::parse::<ItemStruct>(input.clone()) {
        process_struct(&input, cratename)
    } else if let Ok(input) = syn::parse::<ItemEnum>(input.clone()) {
        process_enum(&input, cratename)
    } else if syn::parse::<ItemUnion>(input).is_ok() {
        Err(syn::Error::new(
            Span::call_site(),
            "Borsh schema does not support unions yet.",
        ))
    } else {
        // Derive macros can only be defined on structs, enums, and unions.
        unreachable!()
    };
    TokenStream::from(match res {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    })
}
