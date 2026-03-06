pub mod attribute;
pub mod maud;
#[cfg(feature = "alloc")]
mod renderable;
pub mod rsx;

/// Generates an attribute value, returning a
/// [`LazyAttribute`](crate::LazyAttribute).
///
/// # Example
///
/// ```
/// use hypertext::prelude::*;
///
/// let attr = attribute! { "x" @for i in 0..5 { (i) } };
///
/// assert_eq!(
///     maud! { div title=attr { "Hi!" } }.render().as_inner(),
///     r#"<div title="x01234">Hi!</div>"#
/// );
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::attribute;
/// Generates HTML using Maud syntax, returning a [`Lazy`](crate::Lazy).
///
/// Note that this is not a complete 1:1 port of [Maud](https://maud.lambda.xyz)'s
/// syntax as it is stricter in some cases to prevent anti-patterns.
///
/// Some key differences are:
/// - `#` ([`id`](crate::validation::attributes::GlobalAttributes::id)
///   shorthand), if present, must be the first attribute.
/// - `.` ([`class`](crate::validation::attributes::GlobalAttributes::class)
///   shorthand), if present, come after `#` (if present) and before other
///   attributes.
///
/// Additionally, the `DOCTYPE` constant present in maud is replaced
/// with a new `!DOCTYPE` syntax, which will render `<!DOCTYPE html>` in its
/// place.
///
/// For more details on the rest of Maud's syntax, see the [Maud Book](https://maud.lambda.xyz).
///
/// # Example
///
/// ```
/// use hypertext::prelude::*;
///
/// let name = "Alice";
///
/// assert_eq!(
///     maud! {
///         div #profile title="Profile" {
///             h1 { (name) }
///        }
///     }
///     .render()
///     .as_inner(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#
/// );
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::maud;
/// Generates HTML using RSX syntax, returning a [`Lazy`](crate::Lazy).
///
/// # Examples
///
/// ```
/// use hypertext::prelude::*;
///
/// let name = "Alice";
///
/// assert_eq!(
///     rsx! {
///         <div id="profile" title="Profile">
///             <h1>(name)</h1>
///         </div>
///     }
///     .render()
///     .as_inner(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#
/// );
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::rsx;

#[cfg(feature = "alloc")]
pub use self::renderable::*;
