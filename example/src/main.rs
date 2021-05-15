use confu::Confu;
mod config;

fn main() {
    let config: config::AppConfig = config::AppConfig::confu();
    // let config: config::AppConfig = AppConfig::confu();
    config.show();
}
