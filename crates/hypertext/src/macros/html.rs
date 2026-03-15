//! `html!` aliases for [`rsx!`](crate::rsx!) macros.
//!
//! These exist because the Dioxus CLI (`dx serve --hotpatch`) intercepts
//! `rsx!` invocations by name, fails to parse hypertext's syntax, and
//! silently swallows the change. Using `html!` avoids that collision.
//!
//! See <https://github.com/vidhanio/hypertext/issues/123>.

/// Alias for [`rsx::borrow!`](crate::rsx::borrow!).
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::html_borrow as borrow;
/// Alias for [`rsx::file!`](crate::rsx::file!).
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::html_file as file;
/// Alias for [`rsx::file_borrow!`](crate::rsx::file_borrow!).
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::html_file_borrow as file_borrow;
/// Alias for [`rsx::simple!`](crate::rsx::simple!).
pub use hypertext_macros::html_simple as simple;
