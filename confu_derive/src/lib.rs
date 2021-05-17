#![deny(missing_docs)]
//! Should not be used directly. It's a *proc macro only* crate, a dependency
//! of [confu](https://docs.rs/confu) crate.

use proc_macro::{self, TokenStream};
use proc_macro_error::{abort, proc_macro_error, OptionExt};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Lit, Meta, MetaNameValue};

/// derives [`confu::Confu`](https://docs.rs/confu/*/confu/trait.Confu.html) trait methods
///
/// See [confu](https://docs.rs/confu) crate documentation or the
/// [examples in repo](https://github.com/izirku/confu/tree/main/examples) for usage examples.
///
/// An optional configuration prefix can be specified `#[confu_prefix = "PREFIX_]`:
///
/// ```rust
/// #[derive(Confu)]
/// #[confu_prefix = "PREFIX_"]
/// struct Config {
///   // ...
/// }
/// ```
///
/// following are the field attributes that can be used:
///
/// - `#[default = "default value"]` - specify a default value
/// - `#[protect]` - display `xxxxxxx` instead of the actual value to protect it
/// - `#[hide]` - do not display configuration item at all
/// - `#[require]` - mark configuration item as required
#[proc_macro_derive(Confu, attributes(confu_prefix, default, protect, hide, require))]
#[proc_macro_error]
pub fn confu_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let mut prefix = String::from("");
    if ast.attrs.len() == 1 {
        let meta = ast.attrs[0].parse_meta().unwrap();
        if meta.path().is_ident("confu_prefix") {
            if let Meta::NameValue(MetaNameValue { ref lit, .. }) = meta {
                if let Lit::Str(s) = lit {
                    prefix = s.value();
                }
            }
        }
    }

    let mut resolvers: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut printers: Vec<proc_macro2::TokenStream> = Vec::new();

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        abort!(
            name,
            "Only the non-empty structs with named fields are supported"
        )
    };

    for field in fields.iter() {
        let ident = &field.ident.clone().expect_or_abort("bad struct field name");

        let env_var_name = format!(
            "{}{}",
            prefix.to_uppercase(),
            ident.to_string().to_uppercase()
        );

        // process attributes if such are present
        let mut do_require = false;
        let mut do_protect = false;
        let mut do_hide = false;
        let mut has_default = false;
        let mut default = String::from("");

        for attr in field.attrs.iter() {
            let meta = if let Ok(meta) = attr.parse_meta() {
                meta
            } else {
                abort!(attr, "unable to parse attribute Meta for field `{}`", ident);
            };

            let ident = meta
                .path()
                .get_ident()
                .expect_or_abort("bad struct field name");

            if ident == "require" {
                do_require = true;
            } else if ident == "default" {
                if let Meta::NameValue(MetaNameValue { ref lit, .. }) = meta {
                    if let Lit::Str(s) = lit {
                        default = s.value();
                        has_default = true;
                    }
                }
            } else if ident == "hide" {
                do_hide = true;
            } else if ident == "protect" {
                do_protect = true;
            } else {
                abort!(ident, "unsupported attribute");
            }
        }

        // disallow weird attribute combinations
        if do_require && has_default {
            abort!(ident, "#[require] and #[default = ...] together on `{}` makes no sense. Use one, not both.", name);
            // abort!(ident, "#[require] and #[default = ...] together on `{}.{}` makes no sense. Use one, not both.", name, ident);
        }
        if do_hide && do_protect {
            abort!(
                ident,
                "#[hide] and #[protect] together on `{}.{}` makes no sense. Use one, not both.",
                name,
                ident
            );
        }

        resolvers.push(quote_resolver(
            &ident,
            &env_var_name,
            do_require,
            has_default,
            &default,
        ));

        if !do_hide {
            printers.push(quote_printer(
                &ident,
                &env_var_name,
                do_require,
                do_protect,
                has_default,
                &default,
            ));
        }
    } // for each field

    let build_type = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };

    let build_ver =
        ::std::env::var(format!("{}VERSION", &prefix)).unwrap_or(String::from("<unspecified>"));

    let expanded: proc_macro2::TokenStream = quote! {
        impl Confu for #name {
            fn confu() -> Self {
                Self {
                    #(#resolvers,)*
                }
            }

            fn show(&self) {
                println!("  build: {}\nversion: {}\n", #build_type, #build_ver);
                #(#printers)*
            }
        }
    };
    TokenStream::from(expanded)
}

fn quote_resolver(
    ident: &Ident,
    key: &str,
    is_required: bool,
    has_default: bool,
    default: &str,
) -> proc_macro2::TokenStream {
    quote! {#ident : {
        // see if we have a runtime argument provided
        let maybe_from_args = ::std::env::args().skip(1).find_map(|arg| {
            if let Some((k, v)) = arg.trim_matches('-').split_once('=') {
                if k == #key.to_lowercase() {
                    Some(String::from(v))
                } else {
                    None
                }
            } else {
                None
            }
        });

        // if argument was provided as a runtime argument, use it,
        // otherwise, see if a corresponding environment variable is set,
        // finally use default if provided.
        let maybe = match maybe_from_args {
            Some(val) => Some(val),
            None => {
                let maybe_from_env = env::var(#key);
                match maybe_from_env {
                    Ok(val) => Some(val),
                    _ => {
                        if #has_default {
                            Some(String::from(#default))
                        } else {
                            None
                        }
                    } ,
                }
            }
        };

        // 1. return a resulting argument if we were able to find it one way
        //    or the other, or an empty string.
        // 2. panic if argument was required but was not provided
        match maybe {
            Some(val) => val,
            None => {
                if !#is_required {
                    String::from("")
                } else {
                    // we could wrap the whole thing in Result, and return an Error.
                    // Instead, to keep things simple, we will just panic! at runtime
                    // if a required env/arg was not provided, since the program
                    // probably require is anyway to function.
                    //
                    // Maybe at some later date consider to "over-engineer" things.
                   panic!("required argument {} was not provided.", format!("{}/--{}", #key, #key.to_lowercase()));
                }
            }
        }
    }}
}

fn quote_printer(
    ident: &Ident,
    key: &str,
    is_required: bool,
    is_protected: bool,
    has_default: bool,
    default: &str,
) -> proc_macro2::TokenStream {
    let mut arg_name = String::with_capacity(42);
    let mut arg_note = String::with_capacity(42);
    arg_name.push_str(&key);
    arg_name.push_str("/--");
    arg_name.push_str(&key.to_lowercase());

    if has_default || is_required {
        arg_note.push('(');
    }

    if has_default {
        arg_note.push_str("default: \"");
        if !is_protected {
            arg_note.push_str(&default);
        } else {
            arg_note.push_str("xxxxxxx");
        }
        arg_note.push('"');
    }

    if is_required {
        arg_note.push_str("required");
    }

    if has_default || is_required {
        arg_note.push(')');
    }

    if is_protected {
        quote! {
            println!("{}=xxxxxxx  {}", #arg_name, #arg_note);
        }
    } else {
        quote! {
            println!("{}={}  {}", #arg_name, &self.#ident, #arg_note);
        }
    }
}
