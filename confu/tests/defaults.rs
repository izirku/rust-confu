#![allow(dead_code)]
use confu::Confu;
#[derive(Confu)]
#[confu_prefix = "APP_"]
struct Config {
    #[default = "postgres"]
    db_user: String,

    #[protect]
    #[default = "postgres"]
    db_password: String,
}

fn main() {
    let config = Config::confu();
    assert_eq!(config.db_user, "postgres");
    assert_eq!(config.db_user, "postgres");
}
