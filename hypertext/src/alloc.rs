#![allow(clippy::doc_markdown)]

extern crate alloc;

use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    rc::Rc,
    string::String,
    sync::Arc,
    vec::Vec,
};
use core::fmt::{self, Debug, Display, Formatter, Write};

/// Derive [`AttributeRenderable`] for a type via its [`Display`]
/// implementation.
///
/// The implementation will automatically escape special characters for you.
///
/// You must also implement [`Renderable`] for the type, either manually or
/// using [`#[derive(Renderable)]`](macro@Renderable).
///
/// # Example
///
/// ```
/// use hypertext::prelude::*;
///
/// #[derive(AttributeRenderable)]
/// #[attribute((self.x) ", " (self.y))]
/// pub struct Position {
///     x: i32,
///     y: i32,
/// }
///
/// assert_eq!(
///     maud! { div title=(Position { x: 10, y: 20 }) {} }
///         .render()
///         .as_inner(),
///     r#"<div title="10, 20"></div>"#
/// );
/// ```
pub use hypertext_macros::AttributeRenderable;
/// Derive [`Renderable`] for a type.
///
/// # Example
///
/// ```
/// use hypertext::prelude::*;
///
/// #[derive(Renderable)]
/// #[maud("My name is " (self.name) "!")]
/// pub struct Person {
///     name: String,
/// }
///
/// assert_eq!(
///     maud! { div { (Person { name: "Alice".into() }) } }
///         .render()
///         .as_inner(),
///     r#"<div>My name is Alice!</div>"#
/// );
/// ```
pub use hypertext_macros::Renderable;
/// Generate an HTML attribute, returning a [`LazyAttribute`].
///
/// # Example
///
/// ```
/// use hypertext::prelude::*;
///
/// let attr = attribute! { "x" @for i in 0..5 { (i) } };
///
/// assert_eq!(
///     maud! { div title=attr { "Hi!" } }.render().as_inner(),
///     "<div title=\"x01234\">Hi!</div>"
/// );
/// ```
pub use hypertext_macros::attribute;
/// Generate an HTML attribute, borrowing the environment.
///
/// This is identical to [`attribute!`], except that it does not take ownership
/// of the environment. This is useful when you want to build a
/// [`LazyAttribute`] using some captured variables, but you still want to be
/// able to use the variables after the [`LazyAttribute`] is created.
pub use hypertext_macros::attribute_borrow;
/// Convert a function returning a [`Renderable`] into a component.
///
/// This is a procedural macro that takes a function and generates a
/// struct that holds the function's parameters. The struct implements
/// [`Renderable`] and can be used as a component.
///
/// There are three types of parameters that are supported:
/// - `T`: Stored as `T` in the struct, and will use [`Copy`] to provide the
///   value to the function.
/// - `&T`: Stored as `T` in the struct, and will borrow the value from the
///   struct when calling the function.
/// - `&'a T`: Stored as `&'a T` in the struct, useful for borrowing unsized
///   types such as [`str`] or [`[T]`](slice) without needing to convert them to
///   their owned counterparts.
///
/// The name of the generated struct is derived from the function name by
/// converting it to PascalCase. If you would like to set a different name,
/// you can specify it as `#[component(MyComponentName)]` on the function.
///
/// The visibility of the generated struct is determined by the visibility of
/// the function. If you would like to set a different visibility, you can
/// specify it as `#[component(pub)]`, `#[component(pub(crate))]`, etc. on the
/// function.
///
/// You can combine both of these by setting an attribute like `#[component(pub
/// MyComponentName)]`.
///
/// # Example
///
/// ```
/// use hypertext::prelude::*;
///
/// #[component]
/// fn nav_bar<'a>(title: &'a str, subtitle: &String) -> impl Renderable {
///     maud! {
///         nav {
///             h1 { (title) }
///             h2 { (subtitle) }
///         }
///     }
/// }
///
/// assert_eq!(
///     maud! {
///          div {
///              NavBar title="My Nav Bar" subtitle=("My Subtitle".to_owned());
///          }
///     }
///     .render()
///     .as_inner(),
///     "<div><nav><h1>My Nav Bar</h1><h2>My Subtitle</h2></nav></div>",
/// );
/// ```
pub use hypertext_macros::component;
/// Generate HTML using [`maud`] syntax, returning a [`Lazy`].
///
/// Note that this is not a complete 1:1 port of [`maud`]'s syntax as it is
/// stricter in some places to prevent anti-patterns.
///
/// Some key differences are:
/// - Attribute keys must be simple punctuation-separated identifiers.
/// - [`id`]'s shorthand (`#`), if specified, must be the first attribute.
/// - [`class`]'s shorthand (`.`), if specified must be the second group of
///   attributes.
///
/// Additionally, adding `!DOCTYPE` at the beginning of the invocation will
/// render `"<!DOCTYPE html>"`.
///
/// For more details, see the [maud book](https://maud.lambda.xyz).
///
/// # Example
///
/// ```
/// use hypertext::prelude::*;
///
/// assert_eq!(
///     maud! {
///         div #profile title="Profile" {
///             h1 { "Alice" }
///        }
///     }
///     .render()
///     .as_inner(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#
/// );
/// ```
///
/// [`maud`]: https://docs.rs/maud
/// [`id`]: crate::validation::GlobalAttributes::id
/// [`class`]: crate::validation::GlobalAttributes::class
pub use hypertext_macros::maud;
/// Generate HTML using [`maud!`] syntax, borrowing the environment.
///
/// This is identical to [`maud!`], except that it does not take ownership of
/// the environment. This is useful when you want to build a [`Lazy`] using
/// some captured variables, but you still want to be able to use the variables
/// after the [`Lazy`] is created.
///
/// [`maud!`]: crate::maud
pub use hypertext_macros::maud_borrow;
/// Generate HTML using rsx syntax, returning a [`Lazy`].
///
/// # Example
///
/// ```
/// use hypertext::prelude::*;
///
/// assert_eq!(
///     rsx! {
///         <div id="profile" title="Profile">
///             <h1>Alice</h1>
///         </div>
///     }
///     .render()
///     .as_inner(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#
/// );
/// ```
pub use hypertext_macros::rsx;
/// Generate HTML using [`rsx!`] syntax, borrowing the environment.
///
/// This is identical to [`rsx!`], except that it does not take ownership of
/// the environment. This is useful when you want to build a [`Lazy`] using
/// some captured variables, but you still want to be able to use the variables
/// after the [`Lazy`] is created.
pub use hypertext_macros::rsx_borrow;

use crate::{Raw, RawAttribute, Rendered};

impl Raw<String> {
    /// Converts the [`Raw<String>`] into a [`Rendered<String>`].
    #[inline]
    #[must_use]
    pub fn rendered(self) -> Rendered<String> {
        Rendered(self.0)
    }
}

/// The buffer used for rendering HTML.
///
/// This is a wrapper around [`String`] that prevents accidental XSS
/// vulnerabilities by disallowing direct rendering of raw HTML into the buffer
/// without clearly indicating the risk of doing so.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Buffer(String);

impl Buffer {
    /// Creates a new, empty [`Buffer`].
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(String::new())
    }

    /// Creates a new [`Buffer`] from the given [`String`].
    ///
    /// It is recommended to add a `// XSS Safety` comment above the usage of
    /// this function to indicate why it is safe to directly use the
    /// contained raw HTML.
    #[inline]
    #[must_use]
    pub const fn dangerously_from_string(value: String) -> Self {
        Self(value)
    }

    /// Get a mutable reference to the inner [`String`].
    ///
    /// This should only be needed in very specific cases, such as manually
    /// constructing raw HTML, usually within a [`Renderable::render_to`]
    /// implementation.
    ///
    /// It is recommended to add a `// XSS Safety` comment above the usage of
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
    /// // XSS Safety: The CMS sanitizes the HTML before returning it.
    /// buffer.dangerously_get_string().push_str(&get_some_html());
    ///
    /// assert_eq!(
    ///     buffer.rendered().as_inner(),
    ///     "<h1>My Document!</h1><h2>Some HTML from the CMS</h2>"
    /// )
    /// ```
    #[inline]
    pub const fn dangerously_get_string(&mut self) -> &mut String {
        &mut self.0
    }

    /// Turn this into an [`AttributeBuffer`].
    #[inline]
    pub const fn as_attribute_buffer(&mut self) -> AttributeBuffer<'_> {
        AttributeBuffer(self)
    }

    /// Render the buffer to a [`Rendered<String>`].
    #[inline]
    #[must_use]
    pub fn rendered(self) -> Rendered<String> {
        Rendered(self.0)
    }
}

/// A type that can be rendered as an HTML node.
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
pub trait Renderable {
    /// Renders this value to the given string.
    ///
    /// This does not necessarily have to escape special characters, as it is
    /// intended to be used for rendering raw HTML. If being implemented on a
    /// string-like type, this should escape `&` to `&amp;`, `<` to `&lt;`, and
    /// `>` to `&gt;`.
    fn render_to(&self, buffer: &mut Buffer);

    /// Renders this value to a string. This is a convenience method that
    /// calls [`render_to`] on a new [`String`] and returns the result.
    ///
    /// If overriden for performance reasons, this must match the implementation
    /// of [`render_to`].
    ///
    /// [`render_to`]: Renderable::render_to
    #[inline]
    fn render(&self) -> Rendered<String> {
        let mut buffer = Buffer::new();
        self.render_to(&mut buffer);
        buffer.rendered()
    }
}

/// An extension trait for [`Renderable`] types.
///
/// This trait provides additional methods for rendering and pre-rendering
/// values.
pub trait RenderableExt: Renderable {
    /// Pre-renders the value and stores it in a [`Raw`] so that it can be
    /// re-used among multiple renderings without re-computing the value.
    ///
    /// This should generally be avoided to avoid unnecessary allocations, but
    /// may be useful if it is more expensive to compute and render the value.
    #[inline]
    fn memoize(&self) -> Raw<String> {
        // XSS Safety: The value has already been rendered and escaped.
        Raw::dangerously_create(self.render().into_inner())
    }
}

impl<T: Renderable> RenderableExt for T {}

/// The buffer used for rendering HTML attribute values.
///
/// This is a wrapper around [`Buffer`] that prevents accidentally
/// rendering node-level HTML into an attribute value, which would lead to XSS
/// vulnerabilities.
#[derive(Debug, PartialEq, Eq)]
pub struct AttributeBuffer<'a>(&'a mut Buffer);

impl AttributeBuffer<'_> {
    /// Get a mutable reference to the inner [`String`].
    ///
    /// This should only be needed in very specific cases, such as manually
    /// constructing pre-escaped HTML attributes, usually within a
    /// [`AttributeRenderable::render_attribute_to`] implementation.
    ///
    /// It is recommended to add a `// XSS Safety` comment above the usage of
    /// this method to indicate why it is safe to directly write to the
    /// underlying buffer.
    #[inline]
    pub const fn dangerously_get_string(&mut self) -> &mut String {
        self.0.dangerously_get_string()
    }
}

/// A value that can be rendered as an HTML attribute.
///
/// This is present to disallow accidentally rendering [`Renderable`] types
/// to attributes, as [`Renderable`]s do not necessarily have to be escaped and
/// can contain raw HTML.
pub trait AttributeRenderable {
    /// Renders this value to the given string for use as an attribute value.
    ///
    /// This must escape `&` to `&amp;`, `<` to `&lt;`, `>` to `&gt;`, and `"`
    /// to `&quot;`.
    fn render_attribute_to(&self, buffer: &mut AttributeBuffer);
}

/// A value lazily rendered via a closure.
///
/// This is the type returned by [`maud!`] and [`rsx!`], as well as their `move`
/// variants.
///
/// [`maud!`]: crate::maud
#[derive(Clone, Copy)]
#[must_use = "`Lazy` does nothing unless `.render()` or `.render_to()` is called"]
pub struct Lazy<F>(pub F);

impl<F: Fn(&mut Buffer)> Renderable for Lazy<F> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        (self.0)(buffer);
    }
}

impl<F> Debug for Lazy<F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Lazy").finish_non_exhaustive()
    }
}

/// An attribute value lazily rendered via a closure.
///
/// This is the type returned by [`attribute!`] and [`attribute_borrow!`].
#[derive(Clone, Copy)]
#[must_use = "`LazyAttribute` does nothing unless `.render()` or `.render_to()` is called"]
pub struct LazyAttribute<F>(pub F);

impl<F: Fn(&mut AttributeBuffer)> AttributeRenderable for LazyAttribute<F> {
    #[inline]
    fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
        (self.0)(buffer);
    }
}

impl<F> Debug for LazyAttribute<F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("LazyAttribute").finish_non_exhaustive()
    }
}

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

        // XSS Safety: `ElementEscaper` will escape special characters.
        _ = ElementEscaper(buffer.dangerously_get_string()).write_fmt(*self);
    }
}

impl AttributeRenderable for fmt::Arguments<'_> {
    #[inline]
    fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
        struct AttributeEscaper<'a>(&'a mut String);

        impl Write for AttributeEscaper<'_> {
            #[inline]
            fn write_str(&mut self, s: &str) -> fmt::Result {
                html_escape::encode_double_quoted_attribute_to_string(s, self.0);
                Ok(())
            }
        }

        // XSS Safety: `AttributeEscaper` will escape special characters.
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

impl<T: Display> Renderable for Displayed<T> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        format_args!("{}", self.0).render_to(buffer);
    }
}

impl<T: Display> AttributeRenderable for Displayed<T> {
    #[inline]
    fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
        format_args!("{}", self.0).render_attribute_to(buffer);
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

impl<T: Debug> Renderable for Debugged<T> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        format_args!("{:?}", self.0).render_to(buffer);
    }
}

impl<T: Debug> AttributeRenderable for Debugged<T> {
    #[inline]
    fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
        format_args!("{:?}", self.0).render_attribute_to(buffer);
    }
}

impl<T: AsRef<str>> Renderable for Raw<T> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        // XSS Safety: `Raw` values are expected to be pre-escaped.
        buffer.dangerously_get_string().push_str(self.0.as_ref());
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        Rendered(self.0.as_ref().into())
    }
}

impl<T: AsRef<str>> Renderable for RawAttribute<T> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        // XSS Safety: Anything safe to use as an attribute value should be
        // safe to render as raw HTML.
        buffer.dangerously_get_string().push_str(self.0.as_ref());
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        Rendered(self.0.as_ref().into())
    }
}

impl<T: AsRef<str>> AttributeRenderable for RawAttribute<T> {
    #[inline]
    fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
        // XSS Safety: `RawAttribute` values are expected to be pre-escaped.
        buffer.dangerously_get_string().push_str(self.0.as_ref());
    }
}

impl Renderable for char {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        let s = buffer.dangerously_get_string();
        match *self {
            '&' => s.push_str("&amp;"),
            '<' => s.push_str("&lt;"),
            '>' => s.push_str("&gt;"),
            c => s.push(c),
        }
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        Rendered(match *self {
            '&' => "&amp;".into(),
            '<' => "&lt;".into(),
            '>' => "&gt;".into(),
            '"' => "&quot;".into(),
            c => c.into(),
        })
    }
}

impl AttributeRenderable for char {
    #[inline]
    fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
        let s = buffer.dangerously_get_string();
        match *self {
            '&' => s.push_str("&amp;"),
            '<' => s.push_str("&lt;"),
            '>' => s.push_str("&gt;"),
            '"' => s.push_str("&quot;"),
            c => s.push(c),
        }
    }
}

impl Renderable for str {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        html_escape::encode_text_to_string(self, buffer.dangerously_get_string());
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        Rendered(html_escape::encode_text(self).into_owned())
    }
}

impl AttributeRenderable for str {
    #[inline]
    fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
        html_escape::encode_double_quoted_attribute_to_string(
            self,
            buffer.dangerously_get_string(),
        );
    }
}

impl Renderable for String {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        self.as_str().render_to(buffer);
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        self.as_str().render()
    }
}

impl AttributeRenderable for String {
    #[inline]
    fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
        self.as_str().render_attribute_to(buffer);
    }
}

impl Renderable for bool {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        self.render_attribute_to(&mut buffer.as_attribute_buffer());
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        Rendered(if *self { "true" } else { "false" }.into())
    }
}

impl AttributeRenderable for bool {
    #[inline]
    fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
        buffer
            .dangerously_get_string()
            .push_str(if *self { "true" } else { "false" });
    }
}

macro_rules! render_via_itoa {
    ($($Ty:ty)*) => {
        $(
            impl Renderable for $Ty {
                #[inline]
                fn render_to(&self, buffer: &mut Buffer) {
                    self.render_attribute_to(&mut buffer.as_attribute_buffer());
                }

                #[inline]
                fn render(&self) -> Rendered<String> {
                    Rendered(itoa::Buffer::new().format(*self).into())
                }
            }

            impl AttributeRenderable for $Ty {
                #[inline]
                fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
                    buffer.dangerously_get_string().push_str(itoa::Buffer::new().format(*self));
                }
            }
        )*
    };
}

render_via_itoa! {
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
}

macro_rules! render_via_ryu {
    ($($Ty:ty)*) => {
        $(
            impl Renderable for $Ty {
                #[inline]
                fn render_to(&self, buffer: &mut Buffer) {
                    self.render_attribute_to(&mut buffer.as_attribute_buffer());
                }

                #[inline]
                fn render(&self) -> Rendered<String> {
                    Rendered(ryu::Buffer::new().format(*self).into())
                }
            }

            impl AttributeRenderable for $Ty {
                #[inline]
                fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
                    buffer.dangerously_get_string().push_str(ryu::Buffer::new().format(*self));
                }
            }
        )*
    };
}

render_via_ryu! {
    f32 f64
}

macro_rules! render_via_deref {
    ($($Ty:ty)*) => {
        $(
            impl<T: Renderable + ?Sized> Renderable for $Ty {
                #[inline]
                fn render_to(&self, buffer: &mut Buffer) {
                    T::render_to(&**self, buffer);
                }

                #[inline]
                fn render(&self) -> Rendered<String> {
                    T::render(&**self)
                }
            }

            impl<T: AttributeRenderable + ?Sized> AttributeRenderable for $Ty {
                #[inline]
                fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
                    T::render_attribute_to(&**self, buffer);
                }
            }
        )*
    };
}

render_via_deref! {
    &T
    &mut T
    Box<T>
    Rc<T>
    Arc<T>
}

impl<'a, B: 'a + Renderable + ToOwned + ?Sized> Renderable for Cow<'a, B> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        B::render_to(&**self, buffer);
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        B::render(&**self)
    }
}

impl<'a, B: 'a + AttributeRenderable + ToOwned + ?Sized> AttributeRenderable for Cow<'a, B> {
    #[inline]
    fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
        B::render_attribute_to(&**self, buffer);
    }
}

impl<T: Renderable> Renderable for [T] {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        for item in self {
            item.render_to(buffer);
        }
    }
}

impl<T: Renderable, const N: usize> Renderable for [T; N] {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        self.as_slice().render_to(buffer);
    }
}

impl<T: Renderable> Renderable for Vec<T> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        self.as_slice().render_to(buffer);
    }
}

impl<T: Renderable> Renderable for Option<T> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        if let Some(value) = self {
            value.render_to(buffer);
        }
    }
}

impl<T: AttributeRenderable> AttributeRenderable for Option<T> {
    #[inline]
    fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
        if let Some(value) = self {
            value.render_attribute_to(buffer);
        }
    }
}

impl<T: Renderable, E: Renderable> Renderable for Result<T, E> {
    #[inline]
    fn render_to(&self, buffer: &mut Buffer) {
        match self {
            Ok(value) => value.render_to(buffer),
            Err(err) => err.render_to(buffer),
        }
    }
}

impl<T: AttributeRenderable, E: AttributeRenderable> AttributeRenderable for Result<T, E> {
    #[inline]
    fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
        match self {
            Ok(value) => value.render_attribute_to(buffer),
            Err(err) => err.render_attribute_to(buffer),
        }
    }
}

macro_rules! impl_tuple {
    () => {
        impl Renderable for () {
            #[inline]
            fn render_to(&self, _: &mut Buffer) {}
        }

        impl AttributeRenderable for () {
            #[inline]
            fn render_attribute_to(&self, _: &mut AttributeBuffer) {}
        }
    };
    (($i:tt $T:ident)) => {
        #[cfg_attr(docsrs, doc(fake_variadic))]
        #[cfg_attr(docsrs, doc = "This trait is implemented for tuples up to twelve items long.")]
        impl<$T: Renderable> Renderable for ($T,) {
            #[inline]
            fn render_to(&self, buffer: &mut Buffer) {
                self.$i.render_to(buffer);
            }
        }

        #[cfg_attr(docsrs, doc(fake_variadic))]
        #[cfg_attr(docsrs, doc = "This trait is implemented for tuples up to twelve items long.")]
        impl<$T: AttributeRenderable> AttributeRenderable for ($T,) {
            #[inline]
            fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
                self.$i.render_attribute_to(buffer);
            }
        }
    };
    (($i0:tt $T0:ident) $(($i:tt $T:ident))+) => {
        #[cfg_attr(docsrs, doc(hidden))]
        impl<$T0: Renderable, $($T: Renderable),*> Renderable for ($T0, $($T,)*) {
            #[inline]
            fn render_to(&self, buffer: &mut Buffer) {
                self.$i0.render_to(buffer);
                $(self.$i.render_to(buffer);)*
            }
        }

        #[cfg_attr(docsrs, doc(hidden))]
        impl<$T0: AttributeRenderable, $($T: AttributeRenderable),*> AttributeRenderable for ($T0, $($T,)*) {
            #[inline]
            fn render_attribute_to(&self, buffer: &mut AttributeBuffer) {
                self.$i0.render_attribute_to(buffer);
                $(self.$i.render_attribute_to(buffer);)*
            }
        }
    }
}

impl_tuple!();
impl_tuple!((0 T));
impl_tuple!((0 T0) (1 T1));
impl_tuple!((0 T0) (1 T1) (2 T2));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6) (7 T7));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6) (7 T7) (8 T8));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6) (7 T7) (8 T8) (9 T9));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6) (7 T7) (8 T8) (9 T9) (10 T10));
impl_tuple!((0 T0) (1 T1) (2 T2) (3 T3) (4 T4) (5 T5) (6 T6) (7 T7) (8 T8) (9 T9) (10 T10) (11 T11));
