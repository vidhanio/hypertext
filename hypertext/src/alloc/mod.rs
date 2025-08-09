#![allow(clippy::doc_markdown)]

mod impls;
mod macros;

extern crate alloc;

use alloc::string::String;
use core::{
    fmt::{self, Debug, Display, Formatter, Write},
    marker::PhantomData,
    ptr,
};

pub use self::macros::*;
use crate::{AttributeValue, Context, Node, Raw, Rendered};

impl Raw<String> {
    /// Converts the [`Raw<String>`] into a [`Rendered<String>`].
    #[inline]
    #[must_use]
    pub fn rendered(self) -> Rendered<String> {
        Rendered(self.inner)
    }
}

/// The buffer used for rendering HTML.
///
/// This is a wrapper around [`String`] that prevents accidental XSS
/// vulnerabilities by disallowing direct rendering of raw HTML into the buffer
/// without clearly indicating the risk of doing so.
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Buffer<C = Node> {
    inner: String,
    phantom: PhantomData<C>,
}

impl Buffer {
    /// Creates a new, empty [`Buffer`].
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: String::new(),
            phantom: PhantomData,
        }
    }

    /// Creates a new [`Buffer`] from the given [`String`].
    ///
    /// It is recommended to add a `// XSS SAFETY` comment above the usage of
    /// this function to indicate why it is safe to directly use the
    /// contained raw HTML.
    #[inline]
    #[must_use]
    pub const fn dangerously_from_string(value: String) -> Self {
        Self {
            inner: value,
            phantom: PhantomData,
        }
    }

    /// Turn this into an [`&mut AttributeBuffer`](AttributeBuffer).
    #[inline]
    pub const fn as_attribute_buffer(&mut self) -> &mut AttributeBuffer {
        // SAFETY:
        // - Both `Buffer<C>` and `AttributeBuffer` are `#[repr(transparent)]` wrappers
        //   around `String`, differing only in the zero-sized `PhantomData` marker
        //   type.
        // - `PhantomData` does not affect memory layout, so the layout of `Buffer<C>`
        //   and `AttributeBuffer` is guaranteed to be identical by Rust's type system.
        // - This cast only changes the marker type and does not affect the actual data
        //   or its validity.
        // - The lifetime of the reference is preserved, and there are no aliasing or
        //   validity issues, as both types are functionally identical at runtime.
        unsafe { &mut *ptr::from_mut(self).cast::<AttributeBuffer>() }
    }

    /// Render the buffer to a [`Rendered<String>`].
    #[inline]
    #[must_use]
    pub fn rendered(self) -> Rendered<String> {
        Rendered(self.inner)
    }
}

impl<C: Context> Buffer<C> {
    /// Get a mutable reference to the inner [`String`].
    ///
    /// This should only be needed in very specific cases, such as manually
    /// constructing raw HTML, usually within a [`Renderable::render_to`]
    /// implementation.
    ///
    /// It is recommended to add a `// XSS SAFETY` comment above the usage of
    /// this method to indicate why it is safe to directly write to the
    /// underlying buffer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hypertext::{Buffer, prelude::*};
    ///
    /// fn get_some_html() -> String {
    ///     // get html from some source, such as a CMS
    ///     "<h2>Some HTML from the CMS</h2>".into()
    /// }
    ///
    /// let mut buffer = Buffer::new();
    ///
    /// maud! {
    ///     h1 { "My Document!" }
    /// }
    /// .render_to(&mut buffer);
    ///
    /// // XSS SAFETY: The CMS sanitizes the HTML before returning it.
    /// buffer.dangerously_get_string().push_str(&get_some_html());
    ///
    /// assert_eq!(
    ///     buffer.rendered().as_inner(),
    ///     "<h1>My Document!</h1><h2>Some HTML from the CMS</h2>"
    /// )
    /// ```
    #[inline]
    pub const fn dangerously_get_string(&mut self) -> &mut String {
        &mut self.inner
    }
}

impl Debug for Buffer {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Buffer").field(&self.inner).finish()
    }
}

/// A buffer used for rendering HTML attributes values.
///
/// This is a type alias for [`Buffer<AttributeValue>`].
pub type AttributeBuffer = Buffer<AttributeValue>;

/// A type that can be rendered as an HTML node.
///
/// For [`Renderable<Node>`] (a.k.a. [`Renderable`]) implementations, this
/// must render complete HTML nodes. The implementation must escape `&` to
/// `&amp;`, `<` to `&lt;`, and `>` to `&gt;` if rendering string-like types.
///
/// For [`Renderable<AttributeValue>`] implementations, this must render an
/// attribute value which will eventually be surrounded by double quotes. The
/// implementation must escape `&` to `&amp;`, `<` to `&lt;`, `>` to `&gt;`, and
/// `"` to `&quot;`.
///
/// # Example
///
/// ```
/// use hypertext::{Buffer, prelude::*};
///
/// pub struct Person {
///     name: String,
///     age: u8,
/// }
///
/// impl Renderable for Person {
///     fn render_to(&self, buffer: &mut Buffer) {
///         maud! {
///             div {
///                 h1 { (self.name) }
///                 p { "Age: " (self.age) }
///             }
///         }
///         .render_to(buffer);
///     }
/// }
///
/// let person = Person {
///     name: "Alice".into(),
///     age: 20,
/// };
///
/// assert_eq!(
///     maud! { main { (person) } }.render().as_inner(),
///     r#"<main><div><h1>Alice</h1><p>Age: 20</p></div></main>"#,
/// );
/// ```
pub trait Renderable<C: Context = Node> {
    /// Renders this value to the buffer.
    fn render_to(&self, buffer: &mut Buffer<C>);

    /// Renders this value to a string. This is a convenience method that
    /// calls [`render_to`] on a new [`String`] and returns the result.
    ///
    /// If overriden for performance reasons, this must match the implementation
    /// of [`render_to`].
    ///
    /// [`render_to`]: Renderable::render_to
    #[inline]
    fn render(&self) -> Rendered<String>
    where
        Self: Renderable,
    {
        let mut buffer = Buffer::new();
        self.render_to(&mut buffer);
        buffer.rendered()
    }
}

/// An extension trait for [`Renderable`] types.
///
/// This trait provides an additional method for pre-rendering values.
pub trait RenderableExt: Renderable {
    /// Pre-renders the value and stores it in a [`Raw`] so that it can be
    /// re-used among multiple renderings without re-computing the value.
    ///
    /// This should generally be avoided to avoid unnecessary allocations, but
    /// may be useful if it is more expensive to compute and render the value.
    #[inline]
    fn memoize(&self) -> Raw<String> {
        // XSS SAFETY: The value has already been rendered and is assumed as safe.
        Raw::dangerously_create(self.render().into_inner())
    }
}

impl<T: Renderable> RenderableExt for T {}

/// A value lazily rendered via a closure.
///
/// This is the type returned by [`maud!`] and [`rsx!`], as well as their `move`
/// variants.
///
/// For [`Lazy<F, Node>`] (a.k.a. [`Lazy<F>`]), this must render complete
/// HTML nodes. The closure must escape `&` to `&amp;`, `<` to `&lt;`,
/// and `>` to `&gt;` if rendering string-like types.
///
/// For [`Lazy<F, AttributeValue>`] (a.k.a. [`LazyAttribute<F>`]), this must
/// render an attribute value which will eventually be surrounded by double
/// quotes. The closure must escape `&` to `&amp;`, `<` to `&lt;`, `>` to
/// `&gt;`, and `"` to `&quot;`.
#[derive(Clone, Copy)]
#[must_use = "`Lazy` does nothing unless `.render()` or `.render_to()` is called"]
pub struct Lazy<F, C = Node> {
    f: F,
    phantom: PhantomData<C>,
}

impl<F, C> Lazy<F, C> {
    /// Creates a new [`Lazy`] from the given closure.
    ///
    /// It is recommended to add a `// XSS SAFETY` comment above the usage of
    /// this function to indicate why it is safe to assume that the closure will
    /// not write possibly unsafe HTML to the buffer.
    #[inline]
    pub const fn dangerously_create(f: F) -> Self {
        Self {
            f,
            phantom: PhantomData,
        }
    }

    /// Extracts the inner closure.
    #[inline]
    pub fn into_inner(self) -> F {
        self.f
    }

    /// Gets a reference to the inner closure.
    #[inline]
    pub const fn as_inner(&self) -> &F {
        &self.f
    }
}

impl<F: Fn(&mut Buffer<C>), C: Context> Renderable<C> for Lazy<F, C> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer<C>) {
        (self.f)(buffer);
    }
}

impl<F, C> Debug for Lazy<F, C> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Lazy").finish_non_exhaustive()
    }
}

/// An attribute value lazily rendered via a closure.
///
/// This is a type alias for [`Lazy<F, AttributeValue>`].
pub type LazyAttribute<F> = Lazy<F, AttributeValue>;

impl Renderable for fmt::Arguments<'_> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        struct ElementEscaper<'a>(&'a mut String);

        impl Write for ElementEscaper<'_> {
            #[inline]
            fn write_str(&mut self, s: &str) -> fmt::Result {
                html_escape::encode_text_to_string(s, self.0);
                Ok(())
            }
        }

        // XSS SAFETY: `ElementEscaper` will escape special characters.
        _ = ElementEscaper(buffer.dangerously_get_string()).write_fmt(*self);
    }
}

impl Renderable<AttributeValue> for fmt::Arguments<'_> {
    #[inline]
    fn render_to(&self, buffer: &mut AttributeBuffer) {
        struct AttributeEscaper<'a>(&'a mut String);

        impl Write for AttributeEscaper<'_> {
            #[inline]
            fn write_str(&mut self, s: &str) -> fmt::Result {
                html_escape::encode_double_quoted_attribute_to_string(s, self.0);
                Ok(())
            }
        }

        // XSS SAFETY: `AttributeEscaper` will escape special characters.
        _ = AttributeEscaper(buffer.dangerously_get_string()).write_fmt(*self);
    }
}

/// A value rendered via its [`Display`] implementation.
///
/// This will handle escaping special characters for you.
///
/// This can be created more easily via the `%(expr)` syntax in [`maud!`] and
/// [`rsx!`], which will automatically wrap the expression in this type.
#[derive(Debug, Clone, Copy)]
pub struct Displayed<T>(pub T);

impl<C: Context, T: Display> Renderable<C> for Displayed<T>
where
    for<'a> fmt::Arguments<'a>: Renderable<C>,
{
    #[inline]
    fn render_to(&self, buffer: &mut Buffer<C>) {
        format_args!("{}", self.0).render_to(buffer);
    }
}

/// A value rendered via its [`Debug`] implementation.
///
/// This will handle escaping special characters for you.
///
/// This can be created more easily via the `?(expr)` syntax in [`maud!`] and
/// [`rsx!`], which will automatically wrap the expression in this type.
#[derive(Debug, Clone, Copy)]
pub struct Debugged<T>(pub T);

impl<C: Context, T: Debug> Renderable<C> for Debugged<T>
where
    for<'a> fmt::Arguments<'a>: Renderable<C>,
{
    #[inline]
    fn render_to(&self, buffer: &mut Buffer<C>) {
        format_args!("{:?}", self.0).render_to(buffer);
    }
}

impl<T: AsRef<str>, C: Context> Renderable<C> for Raw<T, C> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer<C>) {
        // XSS SAFETY: `Raw` values are expected to be pre-escaped for
        // the target node type.
        buffer
            .dangerously_get_string()
            .push_str(self.inner.as_ref());
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        Rendered(self.inner.as_ref().into())
    }
}
