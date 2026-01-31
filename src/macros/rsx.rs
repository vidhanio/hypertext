//! Variants of the [`rsx!`](crate::rsx!) macro.

/// Generates HTML using [`rsx!`](crate::rsx!) syntax, borrowing the
/// environment.
///
/// This is identical to [`rsx!`](crate::rsx!), except that it does not take
/// ownership of the environment. This is useful when you want to build a
/// [`Lazy`](crate::Lazy) using some captured variables, but you still want to
/// be able to use the captured variables after the invocation.
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_proc_macros::rsx_borrow as borrow;
/// Generates static HTML using [`rsx!`](crate::rsx!) syntax.
///
/// This will return a [`Raw<&'static str>`](crate::Raw), which can be used
/// in `const` contexts.
///
/// Note that the macro cannot process any dynamic content, so you cannot
/// use any expressions inside the macro.
///
/// # Examples
///
/// ```
/// use hypertext::prelude::*;
///
/// assert_eq!(
///     rsx::simple! {
///         <div id="profile" title="Profile">
///             <h1>Alice</h1>
///         </div>
///     }
///     .into_inner(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#,
/// );
/// ```
pub use hypertext_proc_macros::rsx_simple as simple;
