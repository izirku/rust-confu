#![warn(missing_docs)]
//! Confu is a no frills Rust application configuration library. It supports
//! getting configuration from environment, command line arguments and defaults.
//!
//! Specificity: *defaults* -> *environment* -> *arguments*. Arguments being the
//! most specific, will take precedence over the corresponding environment
//! values, or default values, if such were also defined.
//!
//! # Examples
//!
//! ```rust
//! use confu::Confu;
//! use std::env
//!
//! #[derive(Confu)]
//! #[confu_prefix = "APP_"]
//! struct Config {
//!     #[default = "postgres"]
//!     db_user: String,
//!
//!     #[protect]
//!     #[default = "postgres"]
//!     db_password: String,
//!
//!     #[default = "127.0.0.1"]
//!     api_host: String,
//!
//!     #[require]
//!     telemetry: String,
//!
//!     #[hide]
//!     super_secret_stuff: String,
//! }
//!
//! fn main() {
//!     let config = Config::confu();
//!     config.show();
//! }

/// derived automatically for any *non-enum like* struct that
/// contain only the named `String` fields via `#[derive(Confu)]`.
///
/// (note: any other struct types or enums, and use of non-String fields is ***UB***, currently)
pub trait Confu {
    /// Fetches the configuration as specified in a *struct* decorated with
    /// `#[derive(Confu)]`.
    ///
    /// # Panics
    ///
    /// When a *required* argument was not provided.
    ///
    /// # Returns
    ///
    /// An instance of a *struct* it was derived for, with configuration values
    /// populated. Note that, *optional* fields are set to an empty `String`.
    fn confu() -> Self;
    /// Displays the configuration and build information.
    ///
    /// Such information is displayed in a rather minimalistic fascion, and is
    /// suitable for "untrusted" environments, such as *cloud deploys*, where
    /// *logging* private information is not a good idea.
    ///
    /// - fields marked with `#[protect]`
    ///   are displayed as `ENV_VAR_NAME/--cmd_arg_name="xxxxxxx"`
    /// - fields marked with `#[hide]` are not shown at all
    fn show(&self);
}

// #[macro_use]
// extern crate confu_derive;
// #[cfg(feature = "confu_derive")]
#[allow(unused_imports)]
pub use confu_derive::*;
