extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(WesternCalendar)]
pub fn western_calendar_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    
    impl_western_calendar(&ast)
}

fn impl_western_calendar(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl WesternCalendar for #name { }
    };

    gen.into()
}