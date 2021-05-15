pub trait Confu {
    fn confu() -> Self;
    fn show(&self);
}

// #[macro_use]
// extern crate confu_derive;
// #[cfg(feature = "confu_derive")]
#[allow(unused_imports)]
pub use confu_derive::*;
