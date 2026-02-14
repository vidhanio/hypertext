//! Variants of the [`maud!`](crate::maud!) macro.

/// Generates HTML using [`maud!`](crate::maud!) syntax, borrowing the
/// environment.
///
/// This is identical to [`maud!`](crate::maud!), except that it does not take
/// ownership of the environment. This is useful when you want to build a
/// [`Lazy`](crate::Lazy) using some captured variables, but you still want to
/// be able to use the captured variables after the invocation.
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_proc_macros::maud_borrow as borrow;
/// Generates static HTML using [`maud!`](crate::maud!) syntax.
///
/// This will return a [`Raw<&'static str>`](crate::Raw), which can be used
/// in `const` contexts.
///a
/// Note that the macro cannot process any dynamic content, so you cannot
/// use any expressions inside the macro.
///
/// # Example
///
/// ```
/// use hypertext::prelude::*;
///
/// assert_eq!(
///     maud::simple! {
///         div #profile title="Profile" {
///             h1 { "Alice" }
///        }
///     }
///     .into_inner(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#,
/// );
/// ```
pub use hypertext_proc_macros::maud_simple as simple;
