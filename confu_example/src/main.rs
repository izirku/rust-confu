use confu::{self, Confu};
use std::env;

#[derive(Debug, Confu)]
#[confu_prefix = "APP_"]
struct AppConfig {
    #[require]
    #[default = "postgres"]
    db_user: String,

    #[require]
    #[protect]
    #[default = "postgres"]
    db_password: String,

    #[default = "127.0.0.1"]
    api_host: String,

    #[hide]
    super_secret_stuff: String,
}

// enum ConfigItemKind
fn main() {
    let _config: AppConfig = AppConfig::confu();
    AppConfig::show();
}
