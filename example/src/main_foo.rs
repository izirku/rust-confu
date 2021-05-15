#![feature(trace_macros)]
mod confu;
// use std::stringify;

// const CONFU_PREFIX: &str = "APP_";
// const CONFU_APP_BUILD: &str = "develop";
// const CONFU_APP_DESCR: &str = "MIT";

// macro_rules! with_prefix {
//     ("",) => {

//     };
// }
macro_rules! env_build_info {
    () => {{
        let app_build: Option<&'static str> = option_env!("APP_BUILD");
        // let app_build: &'static str = app_build.unwrap_or("develop");
        match app_build {
            Some(val) => val,
            None => {
                if cfg!(debug_assertions) {
                    "develop"
                } else {
                    "release"
                }
            }
        }
    }};
    ($prefix:literal) => {{
        let app_build: Option<&'static str> = option_env!(concat!($prefix, "_BUILD"));
        // let app_build: &'static str = app_build.unwrap_or("develop");
        match app_build {
            Some(val) => val,
            None => {
                if cfg!(debug_assertions) {
                    "develop"
                } else {
                    "release"
                }
            }
        }
    }};
}

// const APP_BUILD: Option<&'static str> = env_build_info!();
trace_macros!(true);
const APP_BUILD: &str = env_build_info!("FOO");
trace_macros!(false);

fn main() {
    // let confu_app_version: Option<&'static str> = option_env!(stringify!("FOO"));
    // if let Some(app_build) = APP_BUILD {
    //     println!("Hello, {}!", app_build);
    // }
    println!("Hello, {}!", APP_BUILD);
}
