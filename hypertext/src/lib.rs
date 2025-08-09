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
//! `hypertext_elements` (all the valid HTML elements are defined in this crate
//! already in [`hypertext_elements`](validation::hypertext_elements), but it
//! doesn't hard-code this module so you can define your own elements).
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
//!         use hypertext_elements::*;
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
//! [`validation::attributes`], such as [`HtmxAttributes`] and
//! [`AlpineJsAttributes`]
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
//! implement [`Renderable`]. If an element name is capitalized, it will be
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
//! [`GlobalAttributes`]: validation::attributes::GlobalAttributes
//! [`id`]: validation::attributes::GlobalAttributes::id
//! [`class`]: validation::attributes::GlobalAttributes::class
//! [`title`]: validation::attributes::GlobalAttributes::title
//! [`HtmxAttributes`]: validation::attributes::HtmxAttributes
//! [`AlpineJsAttributes`]: validation::attributes::AlpineJsAttributes

#![no_std]
#![warn(clippy::missing_inline_in_public_items)]
#![cfg_attr(docsrs, expect(internal_features))]
#![cfg_attr(docsrs, feature(rustdoc_internals, doc_cfg, doc_auto_cfg))]

#[cfg(feature = "alloc")]
mod alloc;
pub mod context;
mod macros;
pub mod prelude;
pub mod validation;
mod web_frameworks;

use core::{fmt::Debug, marker::PhantomData};

#[cfg(feature = "alloc")]
pub use self::alloc::*;
use self::context::{AttributeValue, Context, Node};
pub use self::macros::*;

/// A raw pre-escaped string.
///
/// For [`Raw<T, Node>`] (a.k.a. [`Raw<T>`]), this must contain complete HTML
/// nodes. If rendering string-like types, the value must escape `&` to `&amp;`,
/// `<` to `&lt;`, and `>` to `&gt;`.
///
/// For [`Raw<T, AttributeValue>`] (a.k.a. [`RawAttribute<T>`]), this must
/// contain an attribute value which will eventually be surrounded by double
/// quotes. The value must escape `&` to `&amp;`, `<` to `&lt;`, `>` to `&gt;`,
/// and `"` to `&quot;`.
///
/// This is useful for rendering raw HTML, but should be used with caution
/// as it can lead to XSS vulnerabilities if used incorrectly. If you are
/// unsure, render the string itself, as its [`Renderable`] implementation will
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
pub struct Raw<T: AsRef<str>, C: Context = Node> {
    inner: T,
    phantom: PhantomData<C>,
}

impl<T: AsRef<str>, C: Context> Raw<T, C> {
    /// Creates a new [`Raw`] from the given string.
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
    pub const fn into_inner(self) -> T {
        // SAFETY: `Raw<T, C>` has exactly one non-zero-sized field, which is `inner`.
        unsafe { const_precise_live_drops_hack!(self.inner) }
    }

    /// Gets a reference to the inner value.
    #[inline]
    pub const fn as_inner(&self) -> &T {
        &self.inner
    }

    /// Gets a reference to the inner value as an [`&str`][str].
    #[inline]
    pub fn as_str(&self) -> &str {
        self.inner.as_ref()
    }
}

impl<T: AsRef<str>> Raw<T> {
    /// Converts the [`Raw<T>`] into a [`Rendered<T>`].
    #[inline]
    #[must_use]
    pub const fn rendered(self) -> Rendered<T> {
        // SAFETY: `Raw<T>` has exactly one non-zero-sized field, which is `inner`.
        let value = unsafe { const_precise_live_drops_hack!(self.inner) };
        Rendered(value)
    }
}

impl<T: AsRef<str> + PartialEq<U>, C: Context, U: AsRef<str>> PartialEq<Raw<U, C>> for Raw<T, C> {
    #[inline]
    fn eq(&self, other: &Raw<U, C>) -> bool {
        self.inner == other.inner
    }
}

impl<T: AsRef<str>, C: Context> Debug for Raw<T, C> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Raw").field(&self.inner.as_ref()).finish()
    }
}

/// A raw pre-escaped attribute value.
///
/// This is a type alias for [`Raw<T, Attribute>`].
pub type RawAttribute<T> = Raw<T, AttributeValue>;

/// A rendered HTML string.
///
/// This type is returned by [`Renderable::render`] ([`Rendered<String>`]), as
/// well as [`Raw<T>::rendered`] ([`Rendered<T>`]).
///
/// This type intentionally does **not** implement [`Renderable`] to discourage
/// anti-patterns such as rendering to a string then embedding that HTML string
/// into another page. To do this, you should use [`RenderableExt::memoize`], or
/// use [`Raw`] directly.
#[derive(Debug, Clone, Copy, Default, Eq, Hash)]
pub struct Rendered<T>(T);

impl<T> Rendered<T> {
    /// Extracts the inner value.
    #[inline]
    pub const fn into_inner(self) -> T {
        // SAFETY: `Rendered<T>` has only one field, which is `0`.
        unsafe { const_precise_live_drops_hack!(self.0) }
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

/// Workaround for [`const_precise_live_drops`](https://github.com/rust-lang/rust/issues/73255) being unstable.
///
/// # Safety
///
/// - `$self` must be a struct with exactly 1 non-zero-sized field
/// - `$field` must be the name/index of that field
macro_rules! const_precise_live_drops_hack {
    ($self:ident. $field:tt) => {{
        let this = core::mem::ManuallyDrop::new($self);
        (&raw const (*(&raw const this).cast::<Self>()).$field).read()
    }};
}
pub(crate) use const_precise_live_drops_hack;
