use proc_macro::{self, TokenStream};

use quote::quote;
use stringcase::kebab_case;
use syn::{DeriveInput, Fields, Ident, parse_macro_input};

#[proc_macro_derive(AutoStrEnum)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let enum_name = &ast.ident;

    let variants = if let syn::Data::Enum(enum_) = ast.data {
        enum_.variants
    } else {
        panic!("{enum_name:?} is not an enum...")
    };

    let field_idents = variants
        .iter()
        .map(|v| match v.fields {
            Fields::Unit => v.ident.clone(),
            _ => panic!("non-unit field..."),
        })
        .collect::<Vec<Ident>>();

    let transformed_field_idents = field_idents
        .iter()
        .map(|v| kebab_case(&v.to_string()))
        .collect::<Vec<String>>();

    let result = quote! {
        impl StrEnum for #enum_name {
            const NAME: &'static str = "foo";
            fn to_static_str(&self) -> &'static str {
                match self {
                    #(Self::#field_idents => #transformed_field_idents,)*
                }
            }
            fn into_iter() -> impl Iterator<Item = (&'static str, Self)> {
                [
                    #((Self::#field_idents.to_static_str(), Self::#field_idents),)*
                ]
                .into_iter()
            }
        }
        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let msg = self.to_static_str();
                write!(f, "{msg}")
            }
        }
    };
    result.into()
}
