//! Give enums superpowers.
//!
//! ```
//! #[derive(StrEnum, Copy, Clone, Debug)]
//! pub enum RngType {
//!     #[default]
//!     OsRng,
//!     Csprng,
//! }
//! ```
//! expands to:
//! ```
//! impl RngType {
//!     pub const fn default_const() -> Self {
//!         Self::OsRng
//!     }
//! }
//! impl StrEnum for RngType {
//!     const NAME: &'static str = "RngType";
//!     const NAME_MEMBER_ARR: &[(&'static str, Self)] = &["os-rng", "csprng"];
//!     fn to_static_str(&self) -> &'static str {
//!         match self {
//!             Self::OsRng => "os-rng",
//!             Self::Csprng => "csprng",
//!         }
//!     }
//!     fn into_iter() -> impl Iterator<Item = &'static (&'static str, Self)> {
//!         Self::NAME_MEMBER_ARR.into_iter()
//!     }
//! }
//! impl std::fmt::Display for RngType {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         let msg = self.to_static_str();
//!         write!(f, "{msg}")
//!     }
//! }
//! impl Default for RngType {
//!     fn default() -> Self { Self::default_const() }
//! }
use proc_macro::{self, TokenStream};

use quote::quote;
use stringcase::kebab_case;
use syn::{DeriveInput, Fields, Ident, Meta, parse_macro_input};

/// Provides `StrEnum` derive macro
///
/// # Panics
///
/// Will panic if used on a non-enum.
/// Will panic if more than one "default" helper attribute is used.
/// Will panic if used on an enum with non-unit field members.
#[proc_macro_derive(StrEnum, attributes(default))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let enum_name = &ast.ident;

    let variants = if let syn::Data::Enum(enum_) = ast.data {
        enum_.variants
    } else {
        panic!("{enum_name:?} is not an enum...")
    };

    let mut default_ident_maybe: Option<Ident> = None;

    // TODO pretty this up with destructuring or something
    // TODO instead of panics and asserts, return from this function with compile_error! filled in
    let field_idents = variants
        .iter()
        .map(|variant| match variant.fields {
            Fields::Unit => {
                for attr in &variant.attrs {
                    if let Meta::Path(path) = &attr.meta {
                        if let Some(ident) = path.get_ident() {
                            if ident == "default" {
                                assert!(
                                    default_ident_maybe.is_none(),
                                    "duplicate default helper attributes are not allowed"
                                );
                                default_ident_maybe = Some(variant.ident.clone());
                            }
                        }
                    }
                }
                variant.ident.clone()
            }
            _ => {
                panic!("{} is not unit field...", variant.ident);
            }
        })
        .collect::<Vec<Ident>>();

    let transformed_field_idents = field_idents
        .iter()
        .map(|v| kebab_case(&v.to_string()))
        .collect::<Vec<String>>();

    let default = default_ident_maybe.expect("A default attribute must be set for StrEnum");

    let result = quote! {
        impl #enum_name {
            pub const fn default_const() -> Self {
                Self::#default
            }
        }
        impl StrEnum for #enum_name {
            const NAME: &'static str = stringify!(#enum_name);
            const NAME_MEMBER_ARR: &[(&'static str, Self)] = &[ #( (#transformed_field_idents, Self::#field_idents) ,)* ];
            fn to_static_str(&self) -> &'static str {
                match self {
                    #(Self::#field_idents => #transformed_field_idents,)*
                }
            }
            fn into_iter() -> impl Iterator<Item = &'static (&'static str, Self)> {
                Self::NAME_MEMBER_ARR.into_iter()
            }
        }
        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let msg = self.to_static_str();
                write!(f, "{msg}")
            }
        }
        impl Default for #enum_name {
            fn default() -> Self { Self::default_const() }
        }
    };
    result.into()
}
