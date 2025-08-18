#[cfg(feature = "alloc")]
mod alloc;

/// Generate static HTML attributes.
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
/// Generate static HTML using Maud syntax.
///
/// For details about the syntax, see [`maud!`].
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
/// Generate static HTML using rsx syntax.
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
pub use hypertext_macros::rsx_static;

#[cfg(feature = "alloc")]
pub use self::alloc::*;
