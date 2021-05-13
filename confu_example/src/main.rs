use confu::{self, Confu};
// use syn::DeriveInput;
use std::{env, fmt::format};
#[derive(Confu)]
#[confu_prefix = "APP_"]
struct AppConfig {
    #[default = "postgres"]
    db_user: String,

    #[protect]
    #[default = "postgres"]
    db_password: String,

    #[hide]
    super_secret_stuff: String,
}

// enum ConfigItemKind
fn main() {
    let build_info = confu::build_info!("EXP_APP_");

    println!("{:?}", build_info);

    AppConfig::confu();

    let maybe_from_args = std::env::args().skip(1).find_map(|arg| {
        if let Some((k, v)) = arg.trim_matches('-').split_once('=') {
            if k == "APP_VERSION".to_lowercase() {
                Some(String::from(v))
            } else {
                None
            }
        } else {
            None
        }
    });

    let result = match maybe_from_args {
        Some(val) => Some(val),
        None => {
            let maybe_from_env = env::var("APP_VERSION");
            match maybe_from_env {
                Ok(val) => Some(val),
                _ => Some(String::from("FOO")),
            }
        }
    };

    println!("FROM_ARGS: {:?}", result);
    // env::var("APP_VERSION").unwrap_or(std::env::args().find_map(f));
}
