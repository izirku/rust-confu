use confu::Confu;
mod config;

fn main() {
    let config: config::AppConfig = config::AppConfig::confu();
    config.show();
}
