use confu::{self, Confu};
// use syn::DeriveInput;

#[derive(Confu)]
#[confu_prefix = "EXP_APP_"]
struct AppConfig {
    #[blur]
    password: String,
}

// enum ConfigItemKind
fn main() {
    let build_info = confu::build_info!("EXP_APP_");

    println!("{:?}", build_info);

    AppConfig::confu();
}
