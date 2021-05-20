use confu::{self, Confu};

#[derive(Debug, Confu)]
#[confu_prefix = "APP_"]
pub struct AppConfig {
    #[default = "postgres"]
    db_user: String,

    #[protect]
    #[default = "postgres"]
    db_password: String,

    #[default = "127.0.0.1"]
    api_host: String,

    // if not provided, will cause runtime panic
    #[require]
    telemetry: String,

    #[hide]
    super_secret_stuff: String,
}
