use proc_macro::{self, TokenStream};

use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

#[proc_macro_derive(StringConfBuilder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        panic!("{} is not a struct...", struct_name)
    };

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
        }
    };
    output.into()
}
