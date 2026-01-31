//! Re-exported items for convenience.
//!
//! This module re-exports all the commonly used items from the crate,
//! so you can use them without having to import them individually. It also
//! re-exports the [`hypertext_elements`] module, and any [framework-specific
//! attribute traits](crate::validation::attributes) that have been enabled, as
//! well as the [`GlobalAttributes`] trait.
#[cfg(feature = "alloc")]
pub use crate::{Renderable, RenderableExt as _, Rendered};
pub use crate::{
    macros::*,
    validation::{attributes::*, hypertext_elements},
};
