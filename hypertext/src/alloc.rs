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

use variadics_please::all_tuples_enumerated;

/// Derive [`AttributeRenderable`] for a type via its [`Display`]
/// implementation.
///
/// The implementation will automatically escape special characters for you.
///
/// # Example
///
/// ```
/// use std::fmt::{self, Display, Formatter};
///
/// use hypertext::prelude::*;
///
/// #[derive(AttributeRenderable)]
/// pub struct Position {
///     x: i32,
///     y: i32,
/// }
///
/// impl Display for Position {
///     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
///         write!(f, "{},{}", self.x, self.y)
///     }
/// }
///
/// assert_eq!(
///     maud! { div title=(Position { x: 1, y: 2 }) {} }.render(),
///     Rendered(r#"<div title="1,2"></div>"#),
/// );
/// ```
#[cfg(feature = "alloc")]
pub use crate::proc_macros::AttributeRenderable;
/// Derive [`Renderable`] for a type via its [`Display`] implementation.
///
/// The implementation will automatically escape special characters for you.
///
/// # Example
///
/// ```
/// use std::fmt::{self, Display, Formatter};
///
/// use hypertext::prelude::*;
///
/// #[derive(Renderable)]
/// pub struct Person {
///     name: String,
/// }
///
/// impl Display for Person {
///     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
///         write!(f, "My name is {}!", self.name)
///     }
/// }
///
/// assert_eq!(
///     maud! { div { (Person { name: "Alice".into() }) } }.render(),
///     Rendered(r#"<div>My name is Alice!</div>"#),
/// );
/// ```
#[cfg(feature = "alloc")]
pub use crate::proc_macros::Renderable;
use crate::{Raw, Rendered};

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
/// use hypertext::{Rendered, prelude::*};
///
/// assert_eq!(
///     maud! {
///         div #profile title="Profile" {
///             h1 { "Alice" }
///        }
///     }
///     .render(),
///     Rendered(r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#),
/// );
/// ```
///
/// [`maud`]: https://docs.rs/maud
/// [`id`]: crate::GlobalAttributes::id
/// [`class`]: crate::GlobalAttributes::class
#[macro_export]
macro_rules! maud {
    ($($tokens:tt)*) => {
        {
            extern crate alloc;

            $crate::Lazy(move |output: &mut alloc::string::String| {
                $crate::proc_macros::maud_closure!($($tokens)*)(output)
            })
        }
    };
}

/// Generate HTML using [`maud!`] syntax, borrowing the environment.
///
/// This is identical to [`maud!`], except that it does not take ownership of
/// the environment. This is useful when you want to build a [`Lazy`] using
/// some captured variables, but you still want to be able to use the variables
/// after the [`Lazy`] is created.
///
/// [`maud!`]: crate::maud
#[macro_export]
macro_rules! maud_borrow {
    ($($tokens:tt)*) => {
        {
            extern crate alloc;

            $crate::Lazy($crate::proc_macros::maud_closure!($($tokens)*))
        }
    };
}

/// Generate HTML using rsx syntax, returning a [`Lazy`].
///
/// # Example
///
/// ```
/// use hypertext::{Rendered, prelude::*};
///
/// assert_eq!(
///     rsx! {
///         <div id="profile" title="Profile">
///             <h1>Alice</h1>
///         </div>
///     }
///     .render(),
///     Rendered(r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#),
/// );
/// ```
#[macro_export]
macro_rules! rsx {
    ($($tokens:tt)*) => {
        {
            extern crate alloc;

            $crate::Lazy(move |output: &mut alloc::string::String| {
                $crate::proc_macros::rsx_closure!($($tokens)*)(output)
            })
        }
    };
}

/// Generate HTML using [`rsx!`] syntax, borrowing the environment.
///
/// This is identical to [`rsx!`], except that it does not take ownership of
/// the environment. This is useful when you want to build a [`Lazy`] using
/// some captured variables, but you still want to be able to use the variables
/// after the [`Lazy`] is created.
#[macro_export]
macro_rules! rsx_borrow {
    ($($tokens:tt)*) => {
        {
            extern crate alloc;

            $crate::Lazy($crate::proc_macros::rsx_closure!($($tokens)*))
        }
    };
}

impl<T: Into<Self>> From<Rendered<T>> for String {
    #[inline]
    fn from(Rendered(value): Rendered<T>) -> Self {
        value.into()
    }
}

/// A type that can be rendered as an HTML node.
///
/// # Example
///
/// ```
/// use hypertext::{Rendered, prelude::*};
///
/// pub struct Person {
///     name: String,
///     age: u8,
/// }
///
/// impl Renderable for Person {
///     fn render_to(&self, output: &mut String) {
///         maud! {
///             div {
///                 h1 { (self.name) }
///                 p { "Age: " (self.age) }
///             }
///         }
///         .render_to(output);
///     }
/// }
///
/// let person = Person {
///     name: "Alice".into(),
///     age: 20,
/// };
///
/// assert_eq!(
///     maud! { main { (person) } }.render(),
///     Rendered(r#"<main><div><h1>Alice</h1><p>Age: 20</p></div></main>"#),
/// );
/// ```
pub trait Renderable {
    /// Renders this value to the given string.
    ///
    /// This does not necessarily have to escape special characters, as it is
    /// intended to be used for rendering raw HTML. If being implemented on a
    /// string-like type, this should escape `&` to `&amp;`, `<` to `&lt;`, and
    /// `>` to `&gt;`.
    ///
    /// This must match the implementation of [`render`] and [`memoize`].
    ///
    /// [`render`]: Renderable::render
    /// [`memoize`]: Renderable::memoize
    fn render_to(&self, output: &mut String);

    /// Renders this value to a string. This is a convenience method that
    /// calls [`render_to`] on a new [`String`] and returns the result.
    ///
    /// If overriding this method for performance reasons, you should prefer to
    /// override [`memoize`] instead as the default implementation of this
    /// method calls [`memoize`] then wraps it in a [`Rendered`].
    ///
    /// This must match the implementation of [`render_to`] and [`memoize`].
    ///
    /// [`render_to`]: Renderable::render_to
    /// [`memoize`]: Renderable::memoize
    #[inline]
    fn render(&self) -> Rendered<String> {
        Rendered(self.memoize().into_inner())
    }

    /// Pre-renders the value and stores it in a [`Raw`] so that it can be
    /// re-used among multiple renderings without re-computing the value.
    ///
    /// This should generally be avoided to avoid unnecessary allocations, but
    /// may be useful if it is more expensive to compute the value multiple
    /// times.
    ///
    /// This must match the implementation of [`render`] and [`render_to`].
    ///
    /// [`render`]: Renderable::render
    /// [`render_to`]: Renderable::render_to
    #[inline]
    fn memoize(&self) -> Raw<String> {
        let mut output = String::new();
        self.render_to(&mut output);
        Raw(output)
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
    fn render_attribute_to(&self, output: &mut String);
}

/// A value lazily rendered via a closure.
///
/// This is the type returned by [`maud!`] and [`rsx!`], as well as their `move`
/// variants.
///
/// [`maud!`]: crate::maud
#[derive(Clone, Copy)]
#[must_use = "`Lazy` does nothing unless `.render()` or `.render_to()` is called"]
pub struct Lazy<F: Fn(&mut String)>(pub F);

impl<F: Fn(&mut String)> Renderable for Lazy<F> {
    #[inline]
    fn render_to(&self, output: &mut String) {
        (self.0)(output);
    }
}

impl<F: Fn(&mut String)> Debug for Lazy<F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Lazy").finish_non_exhaustive()
    }
}

/// An extension trait for [`Display`] types to allow them to be escaped and
/// rendered as HTML.
pub trait DisplayExt: Display {
    /// Makes this value renderable and escapes it for use in HTML.
    #[inline]
    fn renderable(&self) -> Displayed<&Self> {
        Displayed(self)
    }
}

impl<T: Display> DisplayExt for T {}

/// A value rendered via its [`Display`] implementation.
///
/// This will handle escaping special characters for you.
///
/// This can be created more easily via the [`DisplayExt::renderable`] method.
#[derive(Debug, Clone, Copy)]
pub struct Displayed<T: Display>(pub T);

impl<T: Display> Renderable for Displayed<T> {
    #[inline]
    fn render_to(&self, output: &mut String) {
        struct Escaper<'a>(&'a mut String);

        impl fmt::Write for Escaper<'_> {
            #[inline]
            fn write_str(&mut self, s: &str) -> fmt::Result {
                html_escape::encode_text_to_string(s, self.0);
                Ok(())
            }
        }

        _ = write!(Escaper(output), "{}", self.0);
    }
}

impl<T: Display> AttributeRenderable for Displayed<T> {
    #[inline]
    fn render_attribute_to(&self, output: &mut String) {
        struct Escaper<'a>(&'a mut String);

        impl fmt::Write for Escaper<'_> {
            #[inline]
            fn write_str(&mut self, s: &str) -> fmt::Result {
                html_escape::encode_double_quoted_attribute_to_string(s, self.0);
                Ok(())
            }
        }

        _ = write!(Escaper(output), "{}", self.0);
    }
}

impl<T: AsRef<str>> Renderable for Raw<T> {
    #[inline]
    fn render_to(&self, output: &mut String) {
        output.push_str(self.0.as_ref());
    }

    #[inline]
    fn memoize(&self) -> Raw<String> {
        Raw(self.0.as_ref().into())
    }
}

impl AttributeRenderable for () {
    #[inline]
    fn render_attribute_to(&self, _: &mut String) {}
}

impl Renderable for char {
    #[inline]
    fn render_to(&self, output: &mut String) {
        match *self {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            c => output.push(c),
        }
    }

    #[inline]
    fn memoize(&self) -> Raw<String> {
        Raw(match *self {
            '&' => "&amp;".into(),
            '<' => "&lt;".into(),
            '>' => "&gt;".into(),
            c => c.into(),
        })
    }
}

impl AttributeRenderable for char {
    #[inline]
    fn render_attribute_to(&self, output: &mut String) {
        match *self {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            c => output.push(c),
        }
    }
}

impl Renderable for str {
    #[inline]
    fn render_to(&self, output: &mut String) {
        html_escape::encode_text_to_string(self, output);
    }

    #[inline]
    fn memoize(&self) -> Raw<String> {
        Raw(html_escape::encode_text(self).into_owned())
    }
}

impl AttributeRenderable for str {
    #[inline]
    fn render_attribute_to(&self, output: &mut String) {
        html_escape::encode_double_quoted_attribute_to_string(self, output);
    }
}

impl Renderable for String {
    #[inline]
    fn render_to(&self, output: &mut String) {
        self.as_str().render_to(output);
    }

    #[inline]
    fn memoize(&self) -> Raw<String> {
        self.as_str().memoize()
    }
}

impl AttributeRenderable for String {
    #[inline]
    fn render_attribute_to(&self, output: &mut String) {
        self.as_str().render_attribute_to(output);
    }
}

impl Renderable for bool {
    #[inline]
    fn render_to(&self, output: &mut String) {
        output.push_str(if *self { "true" } else { "false" });
    }

    #[inline]
    fn memoize(&self) -> Raw<String> {
        Raw(if *self { "true" } else { "false" }.into())
    }
}

impl AttributeRenderable for bool {
    #[inline]
    fn render_attribute_to(&self, output: &mut String) {
        self.render_to(output);
    }
}

macro_rules! render_via_itoa {
    ($($Ty:ty)*) => {
        $(
            impl Renderable for $Ty {
                #[inline]
                fn render_to(&self, output: &mut String) {
                    output.push_str(itoa::Buffer::new().format(*self));
                }

                #[inline]
                fn memoize(&self) -> Raw<String> {
                    Raw(itoa::Buffer::new().format(*self).into())
                }
            }

            impl AttributeRenderable for $Ty {
                #[inline]
                fn render_attribute_to(&self, output: &mut String) {
                    self.render_to(output);
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
                fn render_to(&self, output: &mut String) {
                    output.push_str(ryu::Buffer::new().format(*self));
                }

                #[inline]
                fn memoize(&self) -> Raw<String> {
                    Raw(ryu::Buffer::new().format(*self).into())
                }
            }

            impl AttributeRenderable for $Ty {
                #[inline]
                fn render_attribute_to(&self, output: &mut String) {
                    self.render_to(output);
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
                fn render_to(&self, output: &mut String) {
                    T::render_to(&**self, output);
                }

                #[inline]
                fn render(&self) -> Rendered<String> {
                    T::render(&**self)
                }

                #[inline]
                fn memoize(&self) -> Raw<String> {
                    T::memoize(&**self)
                }
            }

            impl<T: AttributeRenderable + ?Sized> AttributeRenderable for $Ty {
                #[inline]
                fn render_attribute_to(&self, output: &mut String) {
                    T::render_attribute_to(&**self, output);
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
    fn render_to(&self, output: &mut String) {
        B::render_to(&**self, output);
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        B::render(&**self)
    }

    #[inline]
    fn memoize(&self) -> Raw<String> {
        B::memoize(&**self)
    }
}

impl<'a, B: 'a + AttributeRenderable + ToOwned + ?Sized> AttributeRenderable for Cow<'a, B> {
    #[inline]
    fn render_attribute_to(&self, output: &mut String) {
        B::render_attribute_to(&**self, output);
    }
}

impl<T: Renderable> Renderable for [T] {
    #[inline]
    fn render_to(&self, output: &mut String) {
        for item in self {
            item.render_to(output);
        }
    }
}

impl<T: Renderable, const N: usize> Renderable for [T; N] {
    #[inline]
    fn render_to(&self, output: &mut String) {
        self.as_slice().render_to(output);
    }
}

impl<T: Renderable> Renderable for Vec<T> {
    #[inline]
    fn render_to(&self, output: &mut String) {
        self.as_slice().render_to(output);
    }
}

impl Renderable for () {
    #[inline]
    fn render_to(&self, _: &mut String) {}
}

macro_rules! impl_variadic {
    ($(#[$meta:meta])* $(($n:tt, $T:ident)),*) => {
        $(#[$meta])*
        impl<$($T: Renderable),*> Renderable for ($($T,)*) {
            #[inline]
            fn render_to(&self, output: &mut String) {
                $(
                    self.$n.render_to(output);
                )*
            }
        }
    }
}

all_tuples_enumerated!(
    #[doc(fake_variadic)]
    impl_variadic,
    1,
    15,
    T
);
