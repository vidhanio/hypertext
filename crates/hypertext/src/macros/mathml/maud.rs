//! Variants of the [`mathml::maud!`](crate::mathml::maud!) macro.

/// Generates MathML using [`mathml::maud!`](crate::mathml::maud!) syntax,
/// borrowing the environment.
///
/// This is identical to [`mathml::maud!`](crate::mathml::maud!), except that
/// it does not take ownership of the environment. This is useful when you want
/// to build a [`Lazy`](crate::Lazy) using some captured variables, but you
/// still want to be able to use the captured variables after the invocation.
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::mathml_maud_borrow as borrow;
/// Generates static MathML using [`mathml::maud!`](crate::mathml::maud!)
/// syntax.
///
/// This will return a [`Raw<&'static str>`](crate::Raw), which can be used
/// in `const` contexts.
///
/// Note that the macro cannot process any dynamic content, so you cannot
/// use any expressions inside the macro.
///
/// # Example
///
/// ```
/// use hypertext::prelude::*;
///
/// assert_eq!(
///     mathml::maud::simple! {
///         math display="block" {
///             mfrac {
///                 mn { "1" }
///                 mn { "2" }
///             }
///         }
///     }
///     .into_inner(),
///     r#"<math display="block"><mfrac><mn>1</mn><mn>2</mn></mfrac></math>"#,
/// );
/// ```
pub use hypertext_macros::mathml_maud_simple as simple;
