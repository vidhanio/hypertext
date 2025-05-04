//! Re-exported items for convenience.
//!
//! This module re-exports all the commonly used items from the crate,
//! so you can use them without having to import them individually. It also
//! re-exports the [`html_elements`] module, and any [framework-specific
//! attribute traits](crate::frameworks) that have been enabled, as well as
//! the [`GlobalAttributes`] trait.
#[cfg(feature = "alloc")]
pub use crate::Renderable;
#[cfg(all(feature = "alloc", feature = "maud"))]
pub use crate::maud;
#[cfg(all(feature = "alloc", feature = "rsx"))]
pub use crate::rsx;
pub use crate::{GlobalAttributes, frameworks::*, html_elements};
