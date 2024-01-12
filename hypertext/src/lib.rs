//! A blazing fast library for writing type-checked HTML in Rust.
//!
//! # Features
//!
//! ## Fast
//!
//! The macros generate code that is as fast as writing HTML to a string by
//! hand. The macro automatically combines what would be multiple `push_str`
//! calls into one if there is no dynamic content between them.
//!
//! The entire library is `#![no_std]` compatible, and allocation is completely
//! optional if you don't use any dynamic content. Disabling the `alloc` feature
//! and using [`maud_static!`]/[`html_static!`] will result in an `&'static str`
//! which can even be used in `const` contexts!
//!
//! The library also natively provides support for lazy rendering, which is
//! useful for composing multiple nested components together. This results in
//! only one final allocation, rather than allocating multiple times unnessarily
//! then concatenating them together. See [`Lazy`], [`maud_lazy!`], or
//! [`html_lazy!`] information.
//!
//! ## Type-Checked
//!
//! All macros are validated at compile time, so you can't ever misspell an
//! element/attribute or use invalid attributes.
//!
//! It does this by looking for a module in your current namespace named
//! `html_elements` (all the valid HTML elements are defined in this crate
//! already in [`html_elements`], but it doesn't hard-code this module so you
//! can define your own elements).
//!
//! It then imports each element you use in your [`maud!`] as a struct, and then
//! proceeds to attempt to access the corresponding associated type for each
//! attribute you use.
//!
//! For example, if you use [`maud!`] like this:
//!
//! ```
//! use hypertext::{html_elements, maud, GlobalAttributes};
//!
//! let result = maud! {
//!     div #main title="Main Div" {
//!         h1.important {
//!             "Hello, world!"
//!         }
//!     }
//! };
//! ```
//!
//! It will generate code like this:
//!
//! ```rust
//! # use hypertext::{html_elements, maud, GlobalAttributes};
//! # assert_eq!(maud! { div #main title="Main Div" { h1.important { "Hello, world!" } } },
//! {
//!     const _: () = {
//!         html_elements::div;
//!         html_elements::h1;
//!         let _: hypertext::Attribute = html_elements::div::id;
//!         let _: hypertext::Attribute = html_elements::div::title;
//!         let _: hypertext::Attribute = html_elements::h1::class;
//!     };
//!
//!     let mut hypertext_output = String::new();
//!     hypertext_output.push_str(
//!         r#"<div id="main" title="Main Div"><h1 class="important">Hello, world!</h1></div>"#,
//!     );
//!     hypertext::Rendered(hypertext_output)
//! }
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
//!
//! ```rust
//! use hypertext::{html_elements, maud, Attribute, GlobalAttributes};
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
//!     maud! { div hx-get="/api/endpoint" { "Hello, world!" } }.as_str(),
//!     r#"<div hx-get="/api/endpoint">Hello, world!</div>"#,
//! );
//! ```
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "alloc")]
mod alloc;
mod attributes;
pub mod html_elements;

pub use attributes::{Attribute, GlobalAttributes};
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
/// use hypertext::{html_elements, html_static, GlobalAttributes};
///
/// assert_eq!(
///     html_static! {
///         <div id="profile" title="Profile">
///             <h1>Alice</h1>
///         </div>
///     }
///     .as_str(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#,
/// );
/// ```
pub use hypertext_macros::html_static;
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
/// use hypertext::{html_elements, maud_static, GlobalAttributes};
///
/// assert_eq!(
///     maud_static! {
///         div #profile title="Profile" {
///             h1 { "Alice" }
///        }
///     }
///     .as_str(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#,
/// );
/// ```
///
/// [`maud`]: https://docs.rs/maud
pub use hypertext_macros::maud_static;

#[cfg(feature = "alloc")]
pub use self::alloc::*;

/// Elements that can be self-closing.
pub trait VoidElement {}

macro_rules! void {
    ($($el:ident)*) => {
        $(impl VoidElement for html_elements::$el {})*
    };
}

void! {
    area base br col embed hr img input link meta source track wbr
}

/// A rendered HTML string.
///
/// The type returned by [`maud!`] and [`html!`] ([`Rendered<String>`]), as well
/// as [`maud_static!`] and [`html_static!`] ([`Rendered<&str>`]).
///
/// This type intentionally does **not** implement [`Render`] to prevent
/// anti-patterns such as allocating an entire page to a string then embedding
/// that string in another page. To compose multiple nested components together,
/// consider using [`Lazy`] (most likely through [`maud_lazy!`] or
/// [`html_lazy!`]) instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rendered<T>(pub T);

impl<T: AsRef<str>> Rendered<T> {
    /// Returns the rendered HTML as an `&str`.
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl<T: AsRef<str>> AsRef<str> for Rendered<T> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
