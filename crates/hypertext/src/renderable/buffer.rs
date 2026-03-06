use core::{
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
    ptr,
};

use super::String;
use crate::{
    Renderable, Rendered,
    context::{AttributeValue, Context, Node},
};

/// A buffer used for rendering HTML.
///
/// This is a wrapper around [`String`] that prevents accidental XSS
/// vulnerabilities by disallowing direct rendering of raw HTML into the buffer
/// without clearly opting into the risk of doing so.
#[derive(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Buffer<C: Context = Node> {
    inner: String,
    context: PhantomData<C>,
}

/// A buffer used for rendering attribute values.
///
/// This is a type alias for [`Buffer<AttributeValue>`].
pub type AttributeBuffer = Buffer<AttributeValue>;

#[expect(
    clippy::missing_const_for_fn,
    reason = "`Buffer` does not make sense in `const` contexts"
)]
impl<C: Context> Buffer<C> {
    /// Creates a new, empty [`Buffer<C>`].
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        // XSS SAFETY: The buffer is empty and does not contain any HTML.
        Self::dangerously_from_string(String::new())
    }

    /// Creates a new [`Buffer<C>`] from the given [`String`].
    ///
    /// It is recommended to add a `// XSS SAFETY` comment above the usage of
    /// this function to indicate why the original string is safe to be used in
    /// this context.
    #[inline]
    #[must_use]
    pub fn dangerously_from_string(string: String) -> Self {
        Self {
            inner: string,
            context: PhantomData,
        }
    }

    /// Creates a new [`&mut Buffer<C>`](Buffer) from the given [`&mut
    /// String`](String).
    ///
    /// It is recommended to add a `// XSS SAFETY` comment above the usage of
    /// this function to indicate why the original string is safe to be used in
    /// this context.
    #[inline]
    #[must_use]
    pub fn dangerously_from_string_mut(string: &mut String) -> &mut Self {
        // SAFETY:
        // - `Buffer<C>` is a `#[repr(transparent)]` wrapper around `String`, differing
        //   only in the zero-sized `PhantomData` marker type.
        // - `PhantomData` does not affect memory layout, so the layout of `Buffer<C>`
        //   and `String` is guaranteed to be identical by Rust's type system.
        // - The lifetime of the reference is preserved, and there are no aliasing or
        //   validity issues, as both types are functionally identical at runtime.
        unsafe { &mut *ptr::from_mut(string).cast::<Self>() }
    }

    /// Pushes a [`Renderable<C>`] value to the buffer.
    ///
    /// This is a convenience method that calls
    /// [`value.render_to(self)`](Renderable::render_to).
    #[inline]
    pub fn push(&mut self, value: impl Renderable<C>) {
        value.render_to(self);
    }

    /// Gets a mutable reference to the inner [`String`].
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
    /// ```
    /// use hypertext::{Buffer, prelude::*};
    ///
    /// fn get_some_html() -> String {
    ///     // get html from some source, such as a CMS
    ///     "<h2>Some HTML from the CMS</h2>".into()
    /// }
    ///
    /// let mut buffer = Buffer::new();
    ///
    /// buffer.push(maud! {
    ///     h1 { "My Document!" }
    /// });
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

    /// Extracts the inner [`String`] from the buffer.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> String {
        self.inner
    }

    /// Converts this into an [`&mut Buffer<C2>`](Buffer), where `Self:
    /// AsMut<Buffer<C2>>`.
    ///
    /// This is mostly used for converting from [`Buffer`] to
    /// [`AttributeBuffer`].
    #[inline]
    pub fn with_context<C2: Context>(&mut self) -> &mut Buffer<C2>
    where
        Self: AsMut<Buffer<C2>>,
    {
        self.as_mut()
    }
}

impl Buffer {
    /// Renders the buffer to a [`Rendered<String>`].
    #[inline]
    #[must_use]
    pub fn rendered(self) -> Rendered<String> {
        Rendered(self.inner)
    }
}

impl<C: Context> AsMut<Self> for Buffer<C> {
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl AsMut<AttributeBuffer> for Buffer {
    #[inline]
    fn as_mut(&mut self) -> &mut AttributeBuffer {
        // SAFETY: Both `Buffer<C>` and `AttributeBuffer` are `#[repr(transparent)]`
        // wrappers around `String`, differing only in the zero-sized `PhantomData`
        // marker type.
        unsafe { &mut *ptr::from_mut(self).cast::<AttributeBuffer>() }
    }
}

impl<C: Context> Default for Buffer<C> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for Buffer {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Buffer").field(&self.inner).finish()
    }
}
