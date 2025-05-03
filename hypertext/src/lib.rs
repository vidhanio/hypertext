//! A blazing fast type-checked HTML macro crate.
//!
//! # Features
//!
//! ## Speed
//!
//! The macros generate code that is as fast as writing HTML to a string by
//! hand, and intelligently combines what would be multiple `push_str` calls
//! into one if there is no dynamic content between them.
//!
//! The entire crate is `#![no_std]` compatible, and allocation is completely
//! optional if you don't use any dynamic content. Disabling the `alloc` feature
//! and using [`maud_static!`]/[`rsx_static!`] will result in an
//! [`Rendered<&str>`], which can even be used in `const` contexts!
//!
//! The crate gives extreme importance to lazy rendering and minimizing
//! allocation, so it will only render the HTML to a string when you finally
//! call [`Renderable::render`] at the end. This makes composing nested HTML
//! elements extremely cheap.
//!
//! ## Type-Checking
//!
//! All macros are validated at compile time, so you can't ever misspell an
//! element/attribute or use invalid attributes.
//!
//! It does this by looking for a module in your current namespace named
//! `html_elements` (all the valid HTML elements are defined in this crate
//! already in [`html_elements`], but it doesn't hard-code this module so you
//! can define your own elements).
//!
//! It then imports each element you use in your macro invocation as a
//! struct, and then proceeds to attempt to access the corresponding associated
//! type for each attribute you use.
//!
//! # Examples
//!
//! ```rust
//! use hypertext::{html_elements, maud, GlobalAttributes, Renderable};
//!
//! # assert_eq!(
//! maud! {
//!     div #main title="Main Div" {
//!         h1.important {
//!             "Hello, world!"
//!         }
//!     }
//! }
//! .render()
//! # ,
//!
//! // expands to (roughly):
//!
//! {
//!     const _: () = {
//!         html_elements::div;
//!         html_elements::h1;
//!         let _: hypertext::Attribute = html_elements::div::id;
//!         let _: hypertext::Attribute = html_elements::div::title;
//!         let _: hypertext::Attribute = html_elements::h1::class;
//!     };
//!
//!     hypertext::Lazy(|hypertext_output: &mut String| {
//!         hypertext_output.push_str(
//!             r#"<div id="main" title="Main Div"><h1 class="important">Hello, world!</h1></div>"#
//!         );
//!     })
//! }
//! .render()
//! # );
//! ```
//!
//! This approach is also extremely extensible, as you can define your own
//! traits to add attributes for your favourite libraries! In fact, this is
//! exactly what [`GlobalAttributes`] does, and why it is required in the above
//! example. [`GlobalAttributes`] defines all the global attributes that can be
//! used on any element, for example [`id`](GlobalAttributes::id),
//! [`class`](GlobalAttributes::class) and [`title`](GlobalAttributes::title).
//!
//! Here's an example of how you could define your own attributes for use with
//! the wonderful frontend library [htmx](https://htmx.org):
//! ```rust
//! use hypertext::{Attribute, GlobalAttributes, Renderable, html_elements, maud};
//!
//! trait HtmxAttributes: GlobalAttributes {
//!     const hx_get: Attribute = Attribute;
//!     const hx_post: Attribute = Attribute;
//!     // ...
//! }
//!
//! impl<T: GlobalAttributes> HtmxAttributes for T {}
//!
//! assert_eq!(
//!     //          vvvvvv note that it converts `-` to `_` for you during checking!
//!     maud! { div hx-get="/api/endpoint" { "Hello, world!" } }.render(),
//!     r#"<div hx-get="/api/endpoint">Hello, world!</div>"#,
//! );
//! ```
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::missing_inline_in_public_items)]

#[cfg(feature = "alloc")]
mod alloc;
mod attributes;
pub mod html_elements;
/// HTMX attributes for use with [`maud!`] and [`rsx!`].
#[cfg(feature = "htmx")]
mod htmx;
mod web;

pub use attributes::{Attribute, AttributeNamespace, GlobalAttributes};
/// Use HTMX attributes in your HTML elements.
#[cfg(feature = "htmx")]
pub use htmx::HtmxAttributes;
/// Render static HTML using [`maud`] syntax.
///
/// For details about the syntax, see [`maud!`].
///
/// This will return a [`Rendered<&str>`], which can be used in `const`
/// contexts.
///
/// Note that the macro cannot process any dynamic content, so you cannot use
/// any expressions inside the macro.
///
/// # Example
///
/// ```
/// use hypertext::{GlobalAttributes, html_elements, maud_static};
///
/// assert_eq!(
///     maud_static! {
///         div #profile title="Profile" {
///             h1 { "Alice" }
///        }
///     },
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#,
/// );
/// ```
///
/// [`maud`]: https://docs.rs/maud
pub use hypertext_macros::maud_static;
/// Render static HTML using rsx syntax.
///
/// This will return a [`Rendered<&str>`], which can be used in `const`
/// contexts.
///
/// Note that the macro cannot process any dynamic content, so you cannot use
/// any expressions inside the macro.
///
/// # Example
///
/// ```
/// use hypertext::{GlobalAttributes, html_elements, rsx_static};
///
/// assert_eq!(
///     rsx_static! {
///         <div id="profile" title="Profile">
///             <h1>Alice</h1>
///         </div>
///     },
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#,
/// );
/// ```
pub use hypertext_macros::rsx_static;

#[cfg(feature = "alloc")]
pub use self::alloc::*;

/// Elements that can be self-closing.
pub trait VoidElement {}

/// A raw value that is rendered without escaping.
///
/// This is the type returned by [`maud_static!`] and [`rsx_static!`]
/// ([`Raw<&str>`]).
///
/// This is useful for rendering raw HTML, but should be used with caution
/// as it can lead to XSS vulnerabilities if used incorrectly. If you are
/// unsure, render the string itself, as its [`Renderable`] implementation will
/// escape any dangerous characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Raw<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> Raw<T> {
    /// Extracts the inner value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Gets a reference to the inner value.
    #[inline]
    pub const fn as_inner(&self) -> &T {
        &self.0
    }

    /// Directly render the raw value.
    #[inline]
    pub fn rendered(self) -> Rendered<T> {
        Rendered(self.0)
    }
}

impl<T: AsRef<str>> PartialEq<&str> for Raw<T> {
    #[inline]
    fn eq(&self, &other: &&str) -> bool {
        self.0.as_ref() == other
    }
}

impl<T: AsRef<str>> PartialEq<Raw<T>> for &str {
    #[inline]
    fn eq(&self, other: &Raw<T>) -> bool {
        *self == other.0.as_ref()
    }
}

/// A rendered HTML string.
///
/// This type is returned by [`Renderable::render`] ([`Rendered<String>`]), as
/// well as [`Raw::rendered`].
///
/// This type intentionally does **not** implement [`Renderable`] to prevent
/// anti-patterns such as rendering to a string then embedding that HTML string
/// into another page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rendered<T>(pub T);

impl<T> Rendered<T> {
    /// Extracts the inner value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Gets a reference to the inner value.
    #[inline]
    pub const fn as_inner(&self) -> &T {
        &self.0
    }
}

impl<T: AsRef<str>> Rendered<T> {
    /// Returns the rendered HTML as an `&str`.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl<T: AsRef<str>> AsRef<str> for Rendered<T> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T: AsRef<str>> PartialEq<&str> for Rendered<T> {
    #[inline]
    fn eq(&self, &other: &&str) -> bool {
        self.0.as_ref() == other
    }
}

impl<T: AsRef<str>> PartialEq<Rendered<T>> for &str {
    #[inline]
    fn eq(&self, other: &Rendered<T>) -> bool {
        *self == other.0.as_ref()
    }
}
