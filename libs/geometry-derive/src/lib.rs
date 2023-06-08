//! A crate to custom derive `Position` for any type that has a field named `vector`

extern crate proc_macro;

use quote::quote;
use syn;

use crate::proc_macro::TokenStream;

#[proc_macro_derive(Position)]
pub fn position_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl Position for #name {
            fn x(&self) -> f32 { self.vector.position.x }
            fn y(&self) -> f32 { self.vector.position.y }
            fn x_mut(&mut self) -> &mut f32 { &mut self.vector.position.x }
            fn y_mut(&mut self) -> &mut f32 { &mut self.vector.position.y }
            fn position(&self) -> Point { self.vector.position }
        }
    };
    gen.into()
}
