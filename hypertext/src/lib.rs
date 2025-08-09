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
//! optional if you don't use any dynamic content.
//!
//! The crate gives extreme importance to lazy rendering and minimizing
//! allocation, so it will only render the HTML to a string when you finally
//! call [`Renderable::render`] at the end. This makes composing nested HTML
//! elements extremely cheap.
//!
//! ## Type-Checking
//!
//! All macro invocations are validated at compile time, so you can't ever
//! misspell an element/attribute or use invalid attributes.
//!
//! It does this by looking in your current namespace, or a module named
//! `html_elements` (all the valid HTML elements are defined in this crate
//! already in [`html_elements`], but it doesn't hard-code this module so you
//! can define your own elements).
//!
//! It then imports each element you use in your macro invocation, and then
//! attempts to access the corresponding associated type for each attribute you
//! use.
//!
//! # Example
//!
//! ```rust
//! use hypertext::prelude::*;
//!
//! # assert_eq!(
//! maud! {
//!     div #main title="Main Div" {
//!         h1.important {
//!             "Hello, world!"
//!         }
//!
//!         @for i in 1..=3 {
//!             p.{ "p-" (i) } {
//!                 "This is paragraph number " (i)
//!             }
//!         }
//!     }
//! }
//! # .render(),
//!
//! // expands to (roughly):
//!
//! hypertext::Lazy::dangerously_create(move |buffer: &mut hypertext::Buffer| {
//!     const _: () = {
//!         use html_elements::*;
//!
//!         #[doc(hidden)]
//!         const fn check_element<
//!             T: hypertext::validation::Element<Kind = K>,
//!             K: hypertext::validation::ElementKind,
//!         >() {
//!         }
//!
//!         check_element::<h1, hypertext::validation::Normal>();
//!         let _: hypertext::validation::Attribute = h1::class;
//!
//!         check_element::<p, hypertext::validation::Normal>();
//!         let _: hypertext::validation::Attribute = p::class;
//!
//!         check_element::<div, hypertext::validation::Normal>();
//!         let _: hypertext::validation::Attribute = div::id;
//!         let _: hypertext::validation::Attribute = div::title;
//!     };
//!     buffer
//!         .dangerously_get_string()
//!         .push_str("<div id=\"main\" title=\"Main Div\">");
//!     {
//!         buffer
//!             .dangerously_get_string()
//!             .push_str("<h1 class=\"important\">Hello, world!</h1>");
//!         for i in 1..=3 {
//!             buffer.dangerously_get_string().push_str("<p class=\"p-");
//!             hypertext::Renderable::render_to(&i, &mut buffer.as_attribute_buffer());
//!             buffer
//!                 .dangerously_get_string()
//!                 .push_str("\">This is paragraph number ");
//!             hypertext::Renderable::render_to(&i, buffer);
//!             buffer.dangerously_get_string().push_str("</p>");
//!         }
//!     }
//!     buffer.dangerously_get_string().push_str("</div>");
//! })
//! # .render());
//! ```
//!
//! This approach is also extremely extensible, as you can define your own
//! traits to add attributes for your favourite libraries! In fact, this is
//! exactly what [`GlobalAttributes`] does, and why it is required in the above
//! example, as it defines the attributes that can be used on any element, for
//! example [`id`], [`class`], and [`title`]. This library comes with built-in
//! support for many popular frontend attribute-based frameworks in
//! [`attributes`], such as [`HtmxAttributes`] and [`AlpineJsAttributes`]
//!
//! Here's an example of how you could define your own attributes for use with
//! the wonderful frontend library [htmx](https://htmx.org):
//!
//! ```rust
//! use hypertext::{
//!     prelude::*,
//!     validation::{Attribute, AttributeNamespace},
//! };
//!
//! trait HtmxAttributes: GlobalAttributes {
//!     const hx_get: Attribute = Attribute;
//!     const hx_on: AttributeNamespace = AttributeNamespace;
//!     // ...
//! }
//!
//! impl<T: GlobalAttributes> HtmxAttributes for T {}
//!
//! assert_eq!(
//!     maud! {
//!         div hx-get="/api/endpoint" hx-on:click="alert('Hello, world!')" {
//!         //  ^^^^^^ note that it converts `-` to `_` for you during checking!
//!             "Hello, world!"
//!         }
//!     }
//!     .render()
//!     .as_inner(),
//!     r#"<div hx-get="/api/endpoint" hx-on:click="alert('Hello, world!')">Hello, world!</div>"#,
//! );
//! ```
//!
//! Wrapping an attribue name in quotes will bypass the type-checking, so you
//! can use any attribute you want, even if it doesn't exist in the current
//! context.
//!
//! ```rust
//! use hypertext::prelude::*;
//!
//! assert_eq!(
//!     maud! {
//!         div "custom-attribute"="value" { "Hello, world!" }
//!     }
//!     .render()
//!     .as_inner(),
//!     r#"<div custom-attribute="value">Hello, world!</div>"#,
//! );
//! ```
//!
//! This library also supports component structs, which are simply structs that
//! implement [`Renderable`] If an element name is capitalized, it will be
//! treated as a component, with attributes representing the struct fields. The
//! [`#[component]`](component) macro can be used to easily turn functions into
//! components.
//!
//! ```rust
//! use hypertext::{Buffer, prelude::*};
//!
//! struct Repeater<R: Renderable> {
//!     count: usize,
//!     children: R,
//! }
//!
//! impl<R: Renderable> Renderable for Repeater<R> {
//!     fn render_to(&self, buffer: &mut Buffer) {
//!         maud! {
//!             @for i in 0..self.count {
//!                 (self.children)
//!             }
//!         }
//!         .render_to(buffer);
//!     }
//! }
//!
//! assert_eq!(
//!     maud! {
//!        div {
//!            Repeater count=3 {
//!                // children are passed as a `Lazy` to the `children` field
//!                p { "Hi!" }
//!            }
//!         }
//!     }
//!     .render()
//!     .as_inner(),
//!     "<div><p>Hi!</p><p>Hi!</p><p>Hi!</p></div>"
//! );
//! ```
//!
//! [`GlobalAttributes`]: attributes::GlobalAttributes
//! [`id`]: attributes::GlobalAttributes::id
//! [`class`]: attributes::GlobalAttributes::class
//! [`title`]: attributes::GlobalAttributes::title
//! [`HtmxAttributes`]: attributes::HtmxAttributes
//! [`AlpineJsAttributes`]: attributes::HtmxAttributes
#![no_std]
#![warn(clippy::missing_inline_in_public_items)]
#![cfg_attr(docsrs, expect(internal_features))]
#![cfg_attr(docsrs, feature(rustdoc_internals, doc_auto_cfg))]

#[cfg(feature = "alloc")]
mod alloc;
pub mod attributes;
pub mod html_elements;
#[cfg(feature = "mathml")]
mod mathml;
pub mod validation;
mod web;

pub mod prelude;

use core::{fmt::Debug, marker::PhantomData};

/// Render static HTML attributes.
///
/// This will return a [`RawAttribute<&str>`], which can be used in `const`
/// contexts.
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
/// Render static HTML using [`maud`] syntax.
///
/// For details about the syntax, see [`maud!`].
///
/// This will return a [`Raw<&str>`], which can be used in `const`
/// contexts.
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
///
/// [`maud`]: https://docs.rs/maud
pub use hypertext_macros::maud_static;
/// Render static HTML using rsx syntax.
///
/// This will return a [`Raw<&str>`], which can be used in `const`
/// contexts.
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

/// The context that the value is being rendered to.
///
/// This can be either [`Node`] or an [`AttributeValue`]. A [`Node`]
/// represents an HTML node, while an [`AttributeValue`] represents an attribute
/// value which will eventually be surrounded by double quotes.
///
/// This is used to ensure that the correct rendering methods are called
/// for each context, and to prevent errors such as accidentally rendering
/// an HTML element into an attribute value.
pub trait Context: sealed::Sealed {}

/// An HTML element node.
///
/// All types and traits that are generic over [`Context`] use [`Node`]
/// as the default for the generic type parameter.
///
/// Traits and types with this marker type expect complete HTML nodes. The
/// value/implementation must escape `&` to `&amp;`, `<` to `&lt;`, and `>` to
/// `&gt;` if rendering string-like types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Node {}

impl Context for Node {}

/// An HTML attribute value.
///
/// Traits and types with this marker type expect an attribute value which will
/// eventually be surrounded by double quotes. The value/implementation must
/// escape `&` to `&amp;`, `<` to `&lt;`, `>` to `&gt;`, and `"` to `&quot;`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeValue {}

impl Context for AttributeValue {}

mod sealed {
    use super::{AttributeValue, Node};

    pub trait Sealed {}
    impl Sealed for Node {}
    impl Sealed for AttributeValue {}
}

/// A raw pre-escaped value.
///
/// For [`Raw<T, Node>`] (a.k.a. [`Raw<T>`]), this must contain complete HTML
/// nodes. The value must escape `&` to `&amp;`, `<` to `&lt;`, and `>` to
/// `&gt;` if rendering string-like types.
///
/// For [`Raw<T, AttributeValue>`] (a.k.a. [`RawAttribute<T>`]), this must
/// contain an attribute value which will eventually be surrounded by double
/// quotes. The value must escape `&` to `&amp;`, `<` to `&lt;`, `>` to `&gt;`,
/// and `"` to `&quot;`.
///
/// This is the type returned by [`maud_static!`] and [`rsx_static!`]
/// ([`Raw<&'static str>`]), as well as [`attribute_static!`]
/// ([`RawAttribute<&'static str>`]).
///
/// This is useful for rendering raw HTML, but should be used with caution
/// as it can lead to XSS vulnerabilities if used incorrectly. If you are
/// unsure, render the value itself, as its [`Renderable`] implementation will
/// escape any dangerous characters.
///
/// # Example
///
/// ```rust
/// use hypertext::{Raw, prelude::*};
///
/// fn get_some_html() -> String {
///     // get html from some source, such as a CMS
///     "<h2>Some HTML from the CMS</h2>".into()
/// }
///
/// assert_eq!(
///     maud! {
///         h1 { "My Document!" }
///         // XSS SAFETY: The CMS sanitizes the HTML before returning it.
///         (Raw::dangerously_create(get_some_html()))
///     }
///     .render()
///     .as_inner(),
///     "<h1>My Document!</h1><h2>Some HTML from the CMS</h2>"
/// )
/// ```
#[derive(Clone, Copy, Default, Eq, Hash)]
pub struct Raw<T, C = Node> {
    inner: T,
    phantom: PhantomData<C>,
}

impl<T, C> Raw<T, C> {
    /// Creates a new [`Raw`] from the given value.
    ///
    /// It is recommended to add a `// XSS SAFETY` comment above the usage of
    /// this function to indicate why it is safe to directly use the
    /// contained raw HTML.
    #[inline]
    pub const fn dangerously_create(value: T) -> Self {
        Self {
            inner: value,
            phantom: PhantomData,
        }
    }

    /// Extracts the inner value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Gets a reference to the inner value.
    #[inline]
    pub const fn as_inner(&self) -> &T {
        &self.inner
    }
}

impl<'a> Raw<&'a str> {
    /// Converts the [`Raw<&str>`] into a [`Rendered<&str>`].
    #[inline]
    #[must_use]
    pub const fn rendered(self) -> Rendered<&'a str> {
        Rendered(self.inner)
    }
}

impl<T: PartialEq<U>, C, U> PartialEq<Raw<U, C>> for Raw<T, C> {
    #[inline]
    fn eq(&self, other: &Raw<U, C>) -> bool {
        self.inner == other.inner
    }
}

impl<T: Debug, C> Debug for Raw<T, C> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Raw").field(&self.inner).finish()
    }
}

/// A raw pre-escaped attribute value.
///
/// This is a type alias for [`Raw<T, Attribute>`].
pub type RawAttribute<T> = Raw<T, AttributeValue>;

/// A rendered HTML string.
///
/// This type is returned by [`Renderable::render`] ([`Rendered<String>`]), as
/// well as [`Raw::rendered`] ([`Rendered<&str>`]).
///
/// This type intentionally does **not** implement [`Renderable`] to discourage
/// anti-patterns such as rendering to a string then embedding that HTML string
/// into another page. To do this, you should use [`Raw`], or use
/// [`RenderableExt::memoize`].
#[derive(Debug, Clone, Copy, Eq, Hash)]
pub struct Rendered<T>(T);

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

impl<T: PartialEq<U>, U> PartialEq<Rendered<U>> for Rendered<T> {
    #[inline]
    fn eq(&self, other: &Rendered<U>) -> bool {
        self.0 == other.0
    }
}
