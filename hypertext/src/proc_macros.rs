pub use hypertext_macros::*;

#[cfg(feature = "alloc")]
pub use self::alloc::String;

#[cfg(feature = "alloc")]
mod alloc {
    extern crate alloc;

    pub use alloc::string::String;
}
