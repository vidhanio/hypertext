//! SVG macros.
//!
//! These macros generate SVG markup using the same syntax as the HTML macros
//! ([`maud!`](crate::maud!) and [`rsx!`](crate::rsx!)), but validate elements
//! against [`hypertext_svg_elements`](crate::validation::hypertext_svg_elements)
//! and emit self-closing tags (`/>`) for elements without children.
//!
//! These macros are intended for generating standalone SVG documents. To
//! embed inline SVG inside HTML, use the regular [`maud!`](crate::maud!) or
//! [`rsx!`](crate::rsx!) macros with an
//! [`<svg>`](crate::validation::hypertext_elements::svg) element — the
//! context switches to SVG validation automatically.

pub mod maud;
pub mod rsx;

/// Generates SVG using Maud syntax, returning a [`Lazy`](crate::Lazy).
///
/// This macro works like [`maud!`](crate::maud!) but validates elements
/// against [`hypertext_svg_elements`](crate::validation::hypertext_svg_elements)
/// and emits self-closing tags (`/>`) for elements without children.
///
/// In Maud syntax, `;` produces a self-closing element (`<foo/>`), while
/// `{}` produces an element with an explicit closing tag (`<foo></foo>`).
///
/// # Example
///
/// ```
/// use hypertext::prelude::*;
///
/// let radius = 40;
///
/// assert_eq!(
///     svg::maud! {
///         svg width="100" height="100" {
///             circle cx="50" cy="50" r=(radius) fill="red";
///         }
///     }
///     .render()
///     .as_inner(),
///     r#"<svg width="100" height="100"><circle cx="50" cy="50" r="40" fill="red"/></svg>"#,
/// );
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::svg_maud as maud;
/// Generates SVG using RSX syntax, returning a [`Lazy`](crate::Lazy).
///
/// This macro works like [`rsx!`](crate::rsx!) but validates elements
/// against [`hypertext_svg_elements`](crate::validation::hypertext_svg_elements)
/// and emits self-closing tags (`/>`) for elements without children.
///
/// In RSX syntax, `<foo/>` produces a self-closing element, while
/// `<foo></foo>` produces an element with an explicit closing tag.
///
/// # Example
///
/// ```
/// use hypertext::prelude::*;
///
/// let radius = 40;
///
/// assert_eq!(
///     svg::rsx! {
///         <svg width="100" height="100">
///             <circle cx="50" cy="50" r=(radius) fill="red" />
///         </svg>
///     }
///     .render()
///     .as_inner(),
///     r#"<svg width="100" height="100"><circle cx="50" cy="50" r="40" fill="red"/></svg>"#,
/// );
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::svg_rsx as rsx;
