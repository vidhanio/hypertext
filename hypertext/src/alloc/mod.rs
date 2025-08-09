#![allow(clippy::doc_markdown)]

mod impls;
mod macros;

extern crate alloc;

use alloc::string::String;
use core::{
    fmt::{self, Debug, Display, Formatter},
    marker::PhantomData,
    ptr,
};

pub use self::macros::*;
use crate::{
    Raw, Rendered, const_precise_live_drops_hack,
    context::{AttributeValue, Context, Node},
};

/// The buffer used for rendering HTML.
///
/// This is a wrapper around [`String`] that prevents accidental XSS
/// vulnerabilities by disallowing direct rendering of raw HTML into the buffer
/// without clearly indicating the risk of doing so.
#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Buffer<C: Context = Node> {
    inner: String,
    phantom: PhantomData<C>,
}

/// A buffer used for rendering attribute values.
///
/// This is a type alias for [`Buffer<AttributeValue>`].
pub type AttributeBuffer = Buffer<AttributeValue>;

#[expect(
    clippy::missing_const_for_fn,
    reason = "`Buffer` does not make sense in `const` contexts"
)]
impl Buffer {
    /// Creates a new, empty [`Buffer`].
    #[inline]
    #[must_use]
    pub fn new() -> Self {
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
    pub fn dangerously_from_string(string: String) -> Self {
        Self {
            inner: string,
            phantom: PhantomData,
        }
    }

    /// Turn this into an [`&mut AttributeBuffer`](AttributeBuffer).
    #[inline]
    pub fn as_attribute_buffer(&mut self) -> &mut AttributeBuffer {
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

#[expect(
    clippy::missing_const_for_fn,
    reason = "`Buffer` does not make sense in `const` contexts"
)]
impl<C: Context> Buffer<C> {
    /// Get a mutable reference to the inner [`String`].
    ///
    /// For [`Buffer<Node>`] (a.k.a. [`Buffer`]) writes, the caller must push
    /// complete HTML nodes. If rendering string-like types, the pushed contents
    /// must escape `&` to `&amp;`, `<` to `&lt;`, and `>` to `&gt;`.
    ///
    /// For [`Buffer<AttributeValue>`] (a.k.a. [`AttributeBuffer`]) writes, the
    /// caller must push attribute values which will eventually be surrounded by
    /// double quotes. The pushed contents must escape `&` to `&amp;`, `<` to
    /// `&lt;`, `>` to `&gt;`, and `"` to `&quot;`.
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
    pub fn dangerously_get_string(&mut self) -> &mut String {
        &mut self.inner
    }
}

impl Debug for Buffer {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Buffer").field(&self.inner).finish()
    }
}

/// A type that can be rendered as an HTML node.
///
/// For [`Renderable<Node>`] (a.k.a. [`Renderable`]) implementations, this
/// must render complete HTML nodes. If rendering string-like types, the
/// implementation must escape `&` to `&amp;`, `<` to `&lt;`, and `>` to `&gt;`.
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
/// HTML nodes. If rendering string-like types, the closure must escape `&` to
/// `&amp;`, `<` to `&lt;`, and `>` to `&gt;`.
///
/// For [`Lazy<F, AttributeValue>`] (a.k.a. [`LazyAttribute<F>`]), this must
/// render an attribute value which will eventually be surrounded by double
/// quotes. The closure must escape `&` to `&amp;`, `<` to `&lt;`, `>` to
/// `&gt;`, and `"` to `&quot;`.
#[derive(Clone, Copy)]
#[must_use = "`Lazy` does nothing unless `.render()` or `.render_to()` is called"]
pub struct Lazy<F: Fn(&mut Buffer<C>), C: Context = Node> {
    f: F,
    phantom: PhantomData<C>,
}

/// An attribute value lazily rendered via a closure.
///
/// This is a type alias for [`Lazy<F, AttributeValue>`].
pub type LazyAttribute<F> = Lazy<F, AttributeValue>;

impl<F: Fn(&mut Buffer<C>), C: Context> Lazy<F, C> {
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
    pub const fn into_inner(self) -> F {
        // SAFETY: `Lazy<F, C>` has exactly one non-zero-sized field, which is `f`.
        unsafe { const_precise_live_drops_hack!(self.f) }
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

impl<F: Fn(&mut Buffer<C>), C: Context> Debug for Lazy<F, C> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Lazy").finish_non_exhaustive()
    }
}

/// A value rendered via its [`Display`] implementation.
///
/// This will handle escaping special characters for you.
///
/// This can be created more easily via the `%(...)` syntax in [`maud!`] and
/// [`rsx!`], which will automatically wrap the expression in this type.
#[derive(Debug, Clone, Copy)]
#[doc(alias = "%(...)")]
pub struct Displayed<T: Display>(pub T);

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
/// This can be created more easily via the `?(...)` syntax in [`maud!`] and
/// [`rsx!`], which will automatically wrap the expression in this type.
#[derive(Debug, Clone, Copy)]
#[doc(alias = "?(...)")]
pub struct Debugged<T: Debug>(pub T);

impl<C: Context, T: Debug> Renderable<C> for Debugged<T>
where
    for<'a> fmt::Arguments<'a>: Renderable<C>,
{
    #[inline]
    fn render_to(&self, buffer: &mut Buffer<C>) {
        format_args!("{:?}", self.0).render_to(buffer);
    }
}
