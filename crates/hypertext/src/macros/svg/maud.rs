//! Variants of the [`svg::maud!`](crate::svg::maud!) macro.

/// Generates SVG using [`svg::maud!`](crate::svg::maud!) syntax, borrowing
/// the environment.
///
/// This is identical to [`svg::maud!`](crate::svg::maud!), except that it
/// does not take ownership of the environment. This is useful when you want
/// to build a [`Lazy`](crate::Lazy) using some captured variables, but you
/// still want to be able to use the captured variables after the invocation.
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::svg_maud_borrow as borrow;
/// Generates static SVG using [`svg::maud!`](crate::svg::maud!) syntax.
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
///     svg::maud::simple! {
///         circle cx="50" cy="50" r="40" fill="red";
///     }
///     .into_inner(),
///     r#"<circle cx="50" cy="50" r="40" fill="red"/>"#,
/// );
/// ```
pub use hypertext_macros::svg_maud_simple as simple;
