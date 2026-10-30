use core::{
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
    mem, ptr,
};

use super::{Cow, String, Vec};
use crate::{
    Renderable, Rendered,
    context::{AttributeValue, Context, MathMlNode, Node, NodeKind, SvgNode},
};

/// A buffer used for rendering HTML.
///
/// This is a wrapper around [`String`] that prevents accidental XSS
/// vulnerabilities by disallowing direct rendering of raw HTML into the buffer
/// without clearly opting into the risk of doing so.
///
/// A buffer created with [`Buffer::chunked`] additionally keeps static
/// strings pushed via [`Buffer::dangerously_push_static_str`] as borrowed
/// chunks instead of copying them, which can be retrieved with
/// [`Buffer::into_chunks`].
#[derive(Clone, PartialEq, Eq)]
// `with_context` relies on `Buffer<C>` having the same layout for every `C`.
#[repr(C)]
pub struct Buffer<C: Context = Node> {
    inner: String,
    chunks: Option<Vec<Cow<'static, str>>>,
    context: PhantomData<C>,
}

/// A buffer used for rendering attribute values.
///
/// This is a type alias for [`Buffer<AttributeValue>`].
pub type AttributeBuffer = Buffer<AttributeValue>;

/// A buffer used for rendering SVG nodes.
///
/// This is a type alias for [`Buffer<SvgNode>`].
pub type SvgBuffer = Buffer<SvgNode>;

/// A buffer used for rendering MathML nodes.
///
/// This is a type alias for [`Buffer<MathMlNode>`].
pub type MathMlBuffer = Buffer<MathMlNode>;

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
            chunks: None,
            context: PhantomData,
        }
    }

    /// Creates a new, empty chunked [`Buffer<C>`].
    ///
    /// Static strings pushed to a chunked buffer are stored as borrowed
    /// [`Cow<'static, str>`](Cow) chunks instead of being copied, while
    /// dynamic content is collected into owned chunks between them. Use
    /// [`Buffer::into_chunks`] to retrieve the chunks, e.g. to stream them
    /// as an HTTP response body without concatenating them first.
    ///
    /// # Example
    ///
    /// ```
    /// use std::borrow::Cow;
    ///
    /// use hypertext::{Buffer, prelude::*};
    ///
    /// let name = "Alice";
    /// let mut buffer = Buffer::chunked();
    /// buffer.push(maud! { h1 { "Hello, " (name) "!" } });
    ///
    /// assert_eq!(
    ///     buffer.into_chunks(),
    ///     [
    ///         Cow::Borrowed("<h1>Hello, "),
    ///         Cow::Owned("Alice".into()),
    ///         Cow::Borrowed("!</h1>"),
    ///     ],
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn chunked() -> Self {
        Self {
            inner: String::new(),
            chunks: Some(Vec::new()),
            context: PhantomData,
        }
    }

    /// Pushes a [`Renderable<C>`] value to the buffer.
    ///
    /// This is a convenience method that calls
    /// [`value.render_to(self)`](Renderable::render_to).
    #[inline]
    pub fn push(&mut self, value: impl Renderable<C>) {
        value.render_to(self);
    }

    /// Pushes a static pre-escaped string to the buffer.
    ///
    /// In a [chunked](Buffer::chunked) buffer, the string is stored as a
    /// borrowed chunk without being copied. Otherwise, it is appended to the
    /// inner [`String`].
    ///
    /// The string must follow the same escaping rules as
    /// [`dangerously_get_string`](Buffer::dangerously_get_string).
    ///
    /// It is recommended to add a `// XSS SAFETY` comment above the usage of
    /// this method to indicate why it is safe to directly write to the
    /// underlying buffer.
    #[inline]
    pub fn dangerously_push_static_str(&mut self, s: &'static str) {
        match &mut self.chunks {
            Some(chunks) if !s.is_empty() => {
                if !self.inner.is_empty() {
                    chunks.push(Cow::Owned(mem::take(&mut self.inner)));
                }
                chunks.push(Cow::Borrowed(s));
            }
            Some(_) => {}
            None => self.inner.push_str(s),
        }
    }

    /// Reserves capacity for at least `additional` more bytes of output.
    ///
    /// This is only a hint, and is ignored by [chunked](Buffer::chunked)
    /// buffers, where most of the output is not stored in the inner
    /// [`String`].
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        if self.chunks.is_none() {
            self.inner.reserve(additional);
        }
    }

    /// Gets a mutable reference to the inner [`String`].
    ///
    /// In a [chunked](Buffer::chunked) buffer, this is the owned chunk
    /// currently being written to.
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
    ///
    /// For a [chunked](Buffer::chunked) buffer, this concatenates all chunks.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> String {
        match self.chunks {
            Some(chunks) => {
                let mut string = String::with_capacity(
                    chunks.iter().map(|chunk| chunk.len()).sum::<usize>() + self.inner.len(),
                );
                for chunk in &chunks {
                    string.push_str(chunk);
                }
                string.push_str(&self.inner);
                string
            }
            None => self.inner,
        }
    }

    /// Extracts the rendered chunks from the buffer.
    ///
    /// For a [chunked](Buffer::chunked) buffer, static strings are borrowed
    /// and dynamic content is owned. Otherwise, the whole buffer is returned
    /// as a single owned chunk. Empty chunks are never returned.
    #[inline]
    #[must_use]
    pub fn into_chunks(self) -> Vec<Cow<'static, str>> {
        let mut chunks = self.chunks.unwrap_or_default();
        if !self.inner.is_empty() {
            chunks.push(Cow::Owned(self.inner));
        }
        chunks
    }

    /// Converts this into an [`&mut Buffer<C2>`](Buffer), where `Self:
    /// AsMut<Buffer<C2>>`.
    ///
    /// This is mostly used for converting from [`Buffer`] to
    /// [`AttributeBuffer`].
    #[inline]
    pub fn with_context<C2: Context>(&mut self) -> &mut Buffer<C2>
    where
        C: compatible::CompatibleWith<C2>,
    {
        // SAFETY: `Buffer<C>` is `#[repr(C)]` and `C` only appears in the
        // zero-sized `PhantomData` marker, so `Buffer<C>` and `Buffer<C2>`
        // have identical layouts. `CompatibleWith` ensures the caller only
        // changes the context marker in ways that preserve the escaping
        // contract.
        unsafe { &mut *ptr::from_mut(self).cast::<Buffer<C2>>() }
    }
}

impl<K: NodeKind> Buffer<Node<K>> {
    /// Renders the buffer to a [`Rendered<String, K>`].
    #[inline]
    #[must_use]
    pub fn rendered(self) -> Rendered<String, K> {
        Rendered::new(self.into_inner())
    }

    /// Renders the buffer to a [`Rendered<Vec<Cow<'static, str>>, K>`].
    ///
    /// See [`Buffer::into_chunks`].
    #[inline]
    #[must_use]
    pub fn rendered_chunks(self) -> Rendered<Vec<Cow<'static, str>>, K> {
        Rendered::new(self.into_chunks())
    }
}

impl<C: Context> Default for Buffer<C> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Context> Debug for Buffer<C> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut tuple = f.debug_tuple("Buffer");
        if let Some(chunks) = &self.chunks {
            tuple.field(chunks);
        }
        tuple.field(&self.inner).finish()
    }
}

mod compatible {
    use crate::context::{AttributeValue, Context, Node, NodeKind};

    pub trait CompatibleWith<C: Context> {}

    impl CompatibleWith<Self> for AttributeValue {}
    impl<K1: NodeKind, K2: NodeKind> CompatibleWith<Node<K2>> for Node<K1> {}
    impl<K: NodeKind> CompatibleWith<AttributeValue> for Node<K> {}
}
