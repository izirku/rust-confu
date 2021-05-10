use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Confu, attributes(confu_prefix, blur))]
pub fn confu_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    println!("{:#?}", input);
    let name = &input.ident;
    let expanded = quote! {
        impl Confu for #name {
            fn confu() {
                println!("Confu is not yet implemented for {}", stringify!(#name));
            }
        }
    };
    TokenStream::from(expanded)
}
