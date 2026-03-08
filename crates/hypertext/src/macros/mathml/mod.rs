//! MathML macros.
//!
//! These macros generate MathML markup using the same syntax as the HTML
//! macros ([`maud!`](crate::maud!) and [`rsx!`](crate::rsx!)), but validate
//! elements against
//! [`hypertext_mathml_elements`](crate::validation::hypertext_mathml_elements)
//! and emit self-closing tags (`/>`) for elements without children.

pub mod maud;
pub mod rsx;

/// Generates MathML using Maud syntax, returning a [`Lazy`](crate::Lazy).
///
/// This macro works like [`maud!`](crate::maud!) but validates elements
/// against
/// [`hypertext_mathml_elements`](crate::validation::hypertext_mathml_elements)
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
/// assert_eq!(
///     mathml::maud! {
///         math display="block" {
///             mfrac {
///                 mn { "1" }
///                 mn { "2" }
///             }
///         }
///     }
///     .render()
///     .as_inner(),
///     r#"<math display="block"><mfrac><mn>1</mn><mn>2</mn></mfrac></math>"#,
/// );
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::mathml_maud as maud;
/// Generates MathML using RSX syntax, returning a [`Lazy`](crate::Lazy).
///
/// This macro works like [`rsx!`](crate::rsx!) but validates elements
/// against
/// [`hypertext_mathml_elements`](crate::validation::hypertext_mathml_elements)
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
/// assert_eq!(
///     mathml::rsx! {
///         <math display="block">
///             <mfrac>
///                 <mn>1</mn>
///                 <mn>2</mn>
///             </mfrac>
///         </math>
///     }
///     .render()
///     .as_inner(),
///     r#"<math display="block"><mfrac><mn>1</mn><mn>2</mn></mfrac></math>"#,
/// );
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::mathml_rsx as rsx;
