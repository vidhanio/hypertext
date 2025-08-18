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
//! optional (via [`maud_static!`] and [`rsx_static!`]) if you don't use any
//! dynamic content.
//!
//! The crate gives extreme importance to lazy rendering and minimizing
//! allocation, so it will only render the HTML to a string when you finally
//! call [`.render()`][RenderableExt::render] at the end. This makes composing
//! nested HTML elements extremely cheap.
//!
//! ## Type-Checking
//!
//! All macro invocations are validated at compile time, so you can't ever
//! misspell an element/attribute, use an element or attribute that doesn't
//! exist, use an attribute on an element that doesn't support it, or
//! accidentally use [void elements](validation::Void) as [normal
//! elements](validation::Normal) or vice versa.
//!
//! More details on how this works can be found in the [`validation`
//! module-level documentation](validation).
//!
//! # Example
//!
//! ```
//! use hypertext::prelude::*;
//! # use hypertext::{Lazy, context::{AttributeValue, Node}, Buffer, validation::{Attribute, Element, ElementKind, Normal}};
//!
//! # let maud_result =
//! maud! {
//!     div #main title="Main Div" {
//!         h1 .important.blue {
//!             "Hello, world!"
//!         }
//!
//!         @for i in 1..=3 {
//!             p #(i) style="background: gray"[i % 2 == 0] {
//!                 "This is paragraph number " (i)
//!             }
//!         }
//!     }
//! }
//! # .render();
//!
//! // or alternatively:
//!
//! # let rsx_result =
//! rsx! {
//!     <div id="main" title="Main Div">
//!         <h1 class="important blue">"Hello, world!"</h1>
//!         @for i in 1..=3 {
//!             <p id=(i) style="background: gray"[i % 2 == 0]>
//!                 "This is paragraph number " (i)
//!             </p>
//!        }
//!     </div>
//! }
//! # .render();
//!
//! // expands to:
//!
//! # assert_eq!(maud_result, rsx_result);
//! # assert_eq!(maud_result,
//! Lazy::<_, Node>::dangerously_create(move |buffer: &mut Buffer| {
//!     const _: fn() = || {
//!         use hypertext_elements::*;
//!         fn check_element<K: ElementKind>(_: impl Element<Kind = K>) {}
//!     
//!         check_element::<Normal>(h1);
//!         let _: Attribute = <h1>::class;
//!         check_element::<Normal>(p);
//!         let _: Attribute = <p>::class;
//!         check_element::<Normal>(div);
//!         let _: Attribute = <div>::id;
//!         let _: Attribute = <div>::title;
//!     };
//!     buffer
//!         .dangerously_get_string()
//!         .push_str(r#"<div id="main" title="Main Div">"#);
//!     {
//!         buffer
//!             .dangerously_get_string()
//!             .push_str(r#"<h1 class="important blue">Hello, world!</h1>"#);
//!         for i in 1..=3 {
//!             buffer.dangerously_get_string().push_str(r#"<p id=""#);
//!             i.render_to(buffer.with_context::<AttributeValue>());
//!             buffer.dangerously_get_string().push_str(r#"""#);
//!             if i % 2 == 0 {
//!                 buffer
//!                     .dangerously_get_string()
//!                     .push_str(r#" style="background: gray""#);
//!             }
//!             buffer
//!                 .dangerously_get_string()
//!                 .push_str(">This is paragraph number ");
//!             i.render_to(buffer.with_context::<Node>());
//!             buffer.dangerously_get_string().push_str("</p>");
//!         }
//!     }
//!     buffer.dangerously_get_string().push_str("</div>");
//! })
//! # .render());
//! ```

#![no_std]
#![warn(clippy::missing_inline_in_public_items)]
#![cfg_attr(all(docsrs, not(doctest)), expect(internal_features))]
#![cfg_attr(
    all(docsrs, not(doctest)),
    feature(rustdoc_internals, doc_cfg, doc_auto_cfg)
)]

pub mod context;
mod macros;
pub mod prelude;
#[cfg(feature = "alloc")]
mod renderable;
pub mod validation;
mod web_frameworks;

use core::{fmt::Debug, marker::PhantomData};

use self::context::{AttributeValue, Context, Node};
pub use self::macros::*;
#[cfg(feature = "alloc")]
pub use self::renderable::*;

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
/// ```
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
    context: PhantomData<C>,
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
            context: PhantomData,
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
/// This type is returned by [`RenderableExt::render`] ([`Rendered<String>`]),
/// as well as [`Raw<T>::rendered`] ([`Rendered<T>`]).
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
