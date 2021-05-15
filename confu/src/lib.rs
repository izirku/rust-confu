#[derive(Debug, Clone)]
pub enum ConfuError {
    MissingRequired(String),
}

pub trait Confu {
    fn confu() -> Self;
    fn show(&self);
}

// #[allow(unused_imports)]
// #[macro_use]
// extern crate confu_derive;
// #[cfg(feature = "confu_derive")]
pub use confu_derive::*;
