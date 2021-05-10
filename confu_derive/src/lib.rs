use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, Meta, MetaNameValue};

#[proc_macro_derive(Confu, attributes(confu_prefix, blur))]
pub fn confu_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    println!("{:#?}", input);
    let name = &input.ident;
    let mut prefix = String::new();
    if input.attrs.len() == 1 {
        let meta = input.attrs[0].parse_meta().unwrap();
        if meta.path().is_ident("confu_prefix") {
            if let Meta::NameValue(MetaNameValue { ref lit, .. }) = meta {
                if let Lit::Str(s) = lit {
                    prefix = s.value();
                }
            }
        }
    }

    let expanded: proc_macro2::TokenStream = quote! {
        impl Confu for #name {
            fn confu() {
                println!("Confu is not yet implemented for struct {} with prefix '{}'", stringify!(#name), #prefix);
            }
        }
    };
    TokenStream::from(expanded)
}

// KEEP SAKE
// let mut prefix: Option<String> = None;
// for attr in &input.attrs {
//     let meta = attr.parse_meta().unwrap();
//     // println!("attr name: {:?}", attr.name());
//     match &meta {
//         Meta::NameValue(nv) => {
//             // println!("attr: {:#?}", attr);

//             println!("attr: {:?}", meta.path().is_ident("confu_prefix"));
//             if meta.path().is_ident("confu_prefix") {
//                 if let Lit::Str(lstr) = &nv.lit {
//                     prefix = Option::Some(lstr.value());
//                 }
//             }
//             println!("attr: {:?}", nv.lit);
//         }
//         // Meta::NameValue(attr) => {
//         //     // println!("attr: {:#?}", attr);
//         //     println!("attr: {:?}", attr.path);
//         // }
//         _ => {
//             println!("unexpected attr: {:#?}", meta);
//         }
//     }
// }
