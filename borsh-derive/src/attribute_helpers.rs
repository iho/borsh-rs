use syn::{Attribute, Expr, Lit, Meta, Path};

use quote::ToTokens;

pub fn contains_skip(attrs: &[Attribute]) -> bool {
    // skip
    attrs.iter().any(|attr| attr.path().is_ident("borsh"))
}

pub fn contains_initialize_with(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs.iter() {
        if attr.path().is_ident("borsh") {
            // let value = attr.meta.to_token_stream().to_string();
            // dbg!(value);
            if let Meta::NameValue(value) = attr.meta.clone() {
                let vvalue = value.value.to_token_stream().to_string();
                dbg!(&vvalue);
                if let Expr::Lit(lit) = value.value {
                    if let Lit::Str(lit) = lit.lit {
                        return Some(lit.to_token_stream().to_string());
                    }
                }
            } else {
                match attr.meta.clone() {
                    Meta::Path(path) => {
                        if path.is_ident("borsh") {
                            return Some("".to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    None
}
