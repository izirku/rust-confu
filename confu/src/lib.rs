#[derive(Debug)]
pub struct BuildInfo {
    pub build: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub enum ConfuError {
    MissingRequired(String),
}
/// Returns either `develop` or `release` based on an optimization level.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_build_type {
    () => {{
        if cfg!(debug_assertions) {
            "develop"
        } else {
            "release"
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_build_version {
    ($prefix:literal) => {{
        let app_build: Option<&'static str> = option_env!(concat!($prefix, "VERSION"));
        match app_build {
            Some(val) => val,
            None => "",
        }
    }};
}

/// Retrieves application `BuildInfo` at compile time.
///
/// # Arguments
/// `prefix` - can be `""`, `"APP"` or `"APP_"`
#[macro_export]
macro_rules! build_info {
    ($prefix:literal) => {
        $crate::BuildInfo {
            build: String::from($crate::__impl_build_type!()),
            version: String::from($crate::__impl_build_version!($prefix)),
        }
    };
}

pub trait Confu {
    fn confu();
}

// #[allow(unused_imports)]
// #[macro_use]
// extern crate confu_derive;
// #[cfg(feature = "confu_derive")]
pub use confu_derive::*;
