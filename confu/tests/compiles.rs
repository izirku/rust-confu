#![allow(dead_code)]
use confu::Confu;
use std::env;
#[derive(Confu)]
#[confu_prefix = "APP_"]
struct Config {
    #[default = "postgres"]
    db_user: String,

    #[protect]
    #[default = "postgres"]
    db_password: String,

    #[default = "127.0.0.1"]
    api_host: String,

    #[require]
    telemetry: String,

    #[hide]
    super_secret_stuff: String,
}

fn main() {}
