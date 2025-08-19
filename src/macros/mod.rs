#[cfg(feature = "alloc")]
mod alloc;

/// Generates static HTML attributes.
///
/// This will return a [`RawAttribute<&'static str>`](crate::RawAttribute),
/// which can be used in `const` contexts.
///
/// Note that the macro cannot process any dynamic content, so you cannot use
/// any expressions inside the macro.
///
/// # Example
///
/// ```
/// use hypertext::{RawAttribute, attribute_static, prelude::*};
///
/// assert_eq!(
///     attribute_static! { "my attribute " 1 }.into_inner(),
///     "my attribute 1"
/// );
/// ```
pub use hypertext_macros::attribute_static;
/// Generates static HTML using [`maud!`] syntax.
///
/// This will return a [`Raw<&'static str>`](crate::Raw), which can be used in
/// `const` contexts.
///
/// Note that the macro cannot process any dynamic content, so you cannot use
/// any expressions inside the macro.
///
/// # Example
///
/// ```
/// use hypertext::{Raw, maud_static, prelude::*};
///
/// assert_eq!(
///     maud_static! {
///         div #profile title="Profile" {
///             h1 { "Alice" }
///        }
///     }
///     .into_inner(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#,
/// );
/// ```
pub use hypertext_macros::maud_static;
/// Generates static HTML using [`rsx!`] syntax.
///
/// This will return a [`Raw<&'static str>`](crate::Raw), which can be used in
/// `const` contexts.
///
/// Note that the macro cannot process any dynamic content, so you cannot use
/// any expressions inside the macro.
///
/// # Examples
///
/// ```
/// use hypertext::{Raw, prelude::*, rsx_static};
///
/// assert_eq!(
///     rsx_static! {
///         <div id="profile" title="Profile">
///             <h1>Alice</h1>
///         </div>
///     }
///     .into_inner(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#,
/// );
/// ```
///
/// ## Using `file`
///
/// If the named argument `file` is provided, the contents of the file will be
/// interpreted as input to this macro. The path is interpreted relative to the
/// `CARGO_MANIFEST_DIR` environment variable, which is usually the root of
/// your crate.
///
/// `static.html`:
///
/// ```html
/// <div id="profile" title="Profile">
///     <h1>Alice</h1>
/// </div>
/// ```
///
/// ```
/// use hypertext::{Raw, prelude::*, rsx_static};
/// # macro_rules! rsx_static { (file = "static.html") => { hypertext::rsx_static! { <div id="profile" title="Profile"><h1>Alice</h1></div> } }; }
///
/// assert_eq!(
///     rsx_static!(file = "static.html").into_inner(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#,
/// );
/// ```
pub use hypertext_macros::rsx_static;

#[cfg(feature = "alloc")]
pub use self::alloc::*;
