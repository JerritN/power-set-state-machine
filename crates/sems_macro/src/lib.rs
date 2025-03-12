use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;

#[proc_macro_derive(Truth)]
pub fn truth_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let gen = quote! {
        use sems_core::Id;
        use std::any::{TypeId};

        impl Truth for #name {
            #[inline]
            fn id() -> Id {
                TypeId::of::<#name>()
            }
        }
    };
    gen.into()
}