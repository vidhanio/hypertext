//! Re-exported items for convenience.
//!
//! This module re-exports all the commonly used items from the crate,
//! so you can use them without having to import them individually. It also
//! re-exports the [`hypertext_elements`] module, and any [framework-specific
//! attribute traits](crate::validation::attributes) that have been enabled, as
//! well as the [`GlobalAttributes`] trait.
#[cfg(feature = "alpine")]
pub use crate::validation::attributes::AlpineJsAttributes;
#[cfg(feature = "htmx")]
pub use crate::validation::attributes::HtmxAttributes;
#[cfg(feature = "hyperscript")]
pub use crate::validation::attributes::HyperscriptAttributes;
#[cfg(feature = "opengraph")]
pub use crate::validation::attributes::OpenGraphMeta;
#[cfg(feature = "alloc")]
pub use crate::{Renderable, RenderableExt as _, Rendered};
pub use crate::{
    macros::*,
    validation::{
        attributes::{
            AriaAttributes, EventHandlerAttributes, GlobalAttributes, MathMlGlobalAttributes,
            SvgGlobalAttributes,
        },
        hypertext_elements, hypertext_mathml_elements, hypertext_svg_elements,
    },
};
