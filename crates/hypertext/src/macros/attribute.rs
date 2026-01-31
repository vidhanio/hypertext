//! Variants of the [`attribute!`](crate::attribute!) macro.

/// Generates an attribute value, borrowing the environment.
///
/// This is identical to [`attribute!`](crate::attribute!), except that it does
/// not take ownership of the environment. This is useful when you want to build
/// a [`LazyAttribute`](crate::LazyAttribute) using some captured variables, but
/// you still want to be able to use the captured variables after the
/// invocation.
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_proc_macros::attribute_borrow as borrow;
/// Generates static HTML attributes.
///
/// This will return a [`RawAttribute<&'static str>`](crate::RawAttribute),
/// which can be used in `const` contexts.
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
///     attribute::simple! { "my attribute " 1 }.into_inner(),
///     "my attribute 1"
/// );
/// ```
pub use hypertext_proc_macros::attribute_simple as simple;
