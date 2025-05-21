//! Re-exported items for convenience.
//!
//! This module re-exports all the commonly used items from the crate,
//! so you can use them without having to import them individually. It also
//! re-exports the [`html_elements`] module, and any [framework-specific
//! attribute traits](crate::frameworks) that have been enabled, as well as
//! the [`GlobalAttributes`] trait.
#![allow(unused_imports)]

#[cfg(feature = "alloc")]
pub use crate::{AttributeRenderable, Renderable, Rendered, attribute, component, maud, rsx};
pub use crate::{frameworks::*, html_elements, validation::GlobalAttributes};
