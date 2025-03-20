use proc_macro::{self, TokenStream};

use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

#[proc_macro_derive(AutoConfigBuilder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    else {
        panic!("{struct_name} is not a struct...")
    };

    #[expect(
        clippy::unwrap_used,
        reason = "this unwrap occurs during syntax parsing, not in generated code"
    )]
    let idents: Vec<Ident> = fields
        .named
        .iter()
        .map(|field| field.ident.clone().unwrap())
        .collect();

    let output = quote! {
        impl #struct_name {
            #(
                pub fn #idents(mut self, value: Option<String>) -> Self {
                    self.#idents = value;
                    self
                }
            )*
            pub fn new() -> Self {
                Self::default()
            }
        }
    };
    output.into()
}
