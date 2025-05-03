extern crate alloc;

use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    rc::Rc,
    string::String,
    sync::Arc,
    vec::Vec,
};
use core::fmt::{self, Display, Write};

/// Generate HTML using [`maud`] syntax.
///
/// Note that this is not a complete 1:1 port of [`maud`]'s syntax as it is
/// stricter in some places to prevent anti-patterns.
///
/// Some key differences are:
/// - Attribute keys must be simple punctuation-separated identifiers.
/// - [`id`]'s shorthand (`#`), if specified, must be the first attribute.
/// - [`class`]'s shorthand (`.`), if specified must be the second group of
///   attributes.
/// - Optional [`class`]es (`.some-class[condition]`) must come after all
///   required [`class`]es.
///
/// Additionally, adding `!DOCTYPE` at the beginning of the invocation will
/// render `"<!DOCTYPE html>"`.
///
/// For more details, see the [maud book](https://maud.lambda.xyz).
///
/// # Example
///
/// ```
/// use hypertext::{GlobalAttributes, Renderable, html_elements, maud};
///
/// assert_eq!(
///     maud! {
///         div #profile title="Profile" {
///             h1 { "Alice" }
///        }
///     }
///     .render(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#,
/// );
/// ```
///
/// [`maud`]: https://docs.rs/maud
/// [`id`]: crate::GlobalAttributes::id
/// [`class`]: crate::GlobalAttributes::class
pub use hypertext_macros::maud;
/// Generate HTML using [`maud`] syntax.
///
/// This macro is identical to [`maud!`], except that it adds `move` to the
/// generated closure, allowing it to take ownership of its environment. You
/// will most likely need this when using [`maud!`] inside an iterator method.
pub use hypertext_macros::maud_move;
/// Generate HTML using rsx syntax.
///
/// # Example
///
/// ```
/// use hypertext::{GlobalAttributes, Renderable, html_elements, rsx};
///
/// assert_eq!(
///     rsx! {
///         <div id="profile" title="Profile">
///             <h1>Alice</h1>
///         </div>
///     }
///     .render(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#,
/// );
/// ```
pub use hypertext_macros::rsx;
/// Generate HTML using [`rsx!`] syntax.
///
/// This macro is identical to [`rsx!`], except that it adds `move` to the
/// generated closure, allowing it to take ownership of its environment. You
/// will most likely need this when using [`rsx!`] inside an iterator method.
pub use hypertext_macros::rsx_move;

use crate::{Raw, Rendered};

/// Generate a [`Box<dyn Renderable>`] using [`maud!`] syntax.
///
/// This macro is identical to [`maud!`], except that it returns a
/// [`Box<dyn Renderable>`] instead of a [`Lazy`] closure. This is useful for
/// dynamically generated HTML that needs to be passed around as a trait object
/// without being pre-rendered.
#[macro_export]
macro_rules! maud_dyn {
    ($($tokens:tt)*) => {
        {
            extern crate alloc;

            alloc::boxed::Box::new($crate::maud!($($tokens)*)) as alloc::boxed::Box<dyn $crate::Renderable>
        }
    };
}

/// Generate a [`Box<dyn Renderable>`] using [`rsx!`] syntax.
///
/// This macro is identical to [`rsx!`], except that it returns a
/// [`Box<dyn Renderable>`] instead of a [`Lazy`] closure. This is useful for
/// dynamically generated HTML that needs to be passed around as a trait object
/// without being pre-rendered.
#[macro_export]
macro_rules! rsx_dyn {
    ($($tokens:tt)*) => {
        {
            extern crate alloc;

            alloc::boxed::Box::new($crate::rsx!($($tokens)*)) as alloc::boxed::Box<dyn $crate::Renderable>
        }
    };
}

impl<T: Into<Self>> From<Rendered<T>> for String {
    #[inline]
    fn from(Rendered(value): Rendered<T>) -> Self {
        value.into()
    }
}

/// A type that can be rendered to a string.
///
/// # Example
///
/// ```
/// use hypertext::{Renderable, html_elements, maud};
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
///     r#"<main><div><h1>Alice</h1><p>Age: 20</p></div></main>"#,
/// );
/// ```
pub trait Renderable {
    /// Renders this type to the given string.
    ///
    /// This must handle escaping any special characters and match the
    /// implementation of [`render`] and [`memoize`].
    ///
    /// [`render`]: Renderable::render
    /// [`memoize`]: Renderable::memoize
    fn render_to(&self, output: &mut String);

    /// Renders this value to a string. This is a convenience method that
    /// calls [`render_to`] on a new [`String`] and returns the result.
    ///
    /// This must handle escaping any special characters and match the
    /// implementation of [`render_to`] and [`memoize`].
    ///
    /// [`render_to`]: Renderable::render_to
    /// [`memoize`]: Renderable::memoize
    #[inline]
    fn render(&self) -> Rendered<String> {
        let mut output = String::new();
        self.render_to(&mut output);
        Rendered(output)
    }

    /// Pre-renders the value and stores it in a [`Raw`] so that it can be
    /// re-used among multiple renderings without re-computing the value.
    ///
    /// This should generally be avoided to avoid unnecessary allocations, but
    /// may be useful if it is more expensive to compute the value multiple
    /// times.
    ///
    /// This must handle escaping any special characters and match the
    /// implementation of [`render`] and [`render_to`].
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

/// A value rendered via its [`Display`] implementation.
///
/// This will handle escaping special characters for you.
#[derive(Debug, Clone, Copy)]
pub struct Displayed<T: Display>(pub T);

impl<T: Display> Renderable for Displayed<T> {
    #[inline]
    fn render_to(&self, output: &mut String) {
        struct Escaper<'a>(&'a mut String);

        impl fmt::Write for Escaper<'_> {
            #[inline]
            fn write_str(&mut self, s: &str) -> fmt::Result {
                html_escape::encode_double_quoted_attribute_to_string(s, self.0);
                Ok(())
            }
        }

        // ignore errors, as writing to a string is infallible
        _ = write!(Escaper(output), "{}", self.0);
    }
}

/// A value lazily rendered via a closure.
///
/// This is the type returned by [`maud!`] and [`rsx!`], as well as their `move`
/// variants.
#[derive(Debug, Clone, Copy)]
pub struct Lazy<F: Fn(&mut String)>(pub F);

impl<F: Fn(&mut String)> Renderable for Lazy<F> {
    #[inline]
    fn render_to(&self, output: &mut String) {
        (self.0)(output);
    }
}

impl<T: AsRef<str>> Renderable for Raw<T> {
    #[inline]
    fn render_to(&self, output: &mut String) {
        output.push_str(self.0.as_ref());
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        Rendered(self.0.as_ref().to_owned())
    }

    #[inline]
    fn memoize(&self) -> Raw<String> {
        Raw(self.0.as_ref().to_owned())
    }
}

impl Renderable for () {
    #[inline]
    fn render_to(&self, _: &mut String) {}

    #[inline]
    fn render(&self) -> Rendered<String> {
        Rendered(String::new())
    }

    #[inline]
    fn memoize(&self) -> Raw<String> {
        Raw(String::new())
    }
}

impl Renderable for char {
    #[inline]
    fn render_to(&self, output: &mut String) {
        match *self {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            c => output.push(c),
        }
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        Rendered(match *self {
            '&' => "&amp;".to_owned(),
            '<' => "&lt;".to_owned(),
            '>' => "&gt;".to_owned(),
            '"' => "&quot;".to_owned(),
            c => c.into(),
        })
    }

    #[inline]
    fn memoize(&self) -> Raw<String> {
        Raw(match *self {
            '&' => "&amp;".to_owned(),
            '<' => "&lt;".to_owned(),
            '>' => "&gt;".to_owned(),
            '"' => "&quot;".to_owned(),
            c => c.into(),
        })
    }
}

impl Renderable for str {
    #[inline]
    fn render_to(&self, output: &mut String) {
        html_escape::encode_double_quoted_attribute_to_string(self, output);
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        Rendered(html_escape::encode_double_quoted_attribute(self).into_owned())
    }

    #[inline]
    fn memoize(&self) -> Raw<String> {
        Raw(html_escape::encode_double_quoted_attribute(self).into_owned())
    }
}

impl Renderable for String {
    #[inline]
    fn render_to(&self, output: &mut String) {
        self.as_str().render_to(output);
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        self.as_str().render()
    }

    #[inline]
    fn memoize(&self) -> Raw<String> {
        self.as_str().memoize()
    }
}

impl Renderable for bool {
    #[inline]
    fn render_to(&self, output: &mut String) {
        output.push_str(if *self { "true" } else { "false" });
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        Rendered(if *self { "true" } else { "false" }.to_owned())
    }

    #[inline]
    fn memoize(&self) -> Raw<String> {
        Raw(if *self { "true" } else { "false" }.to_owned())
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
                fn render(&self) -> Rendered<String> {
                    Rendered(itoa::Buffer::new().format(*self).to_owned())
                }

                #[inline]
                fn memoize(&self) -> Raw<String> {
                    Raw(itoa::Buffer::new().format(*self).to_owned())
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
                fn render(&self) -> Rendered<String> {
                    Rendered(ryu::Buffer::new().format(*self).to_owned())
                }

                #[inline]
                fn memoize(&self) -> Raw<String> {
                    Raw(ryu::Buffer::new().format(*self).to_owned())
                }
            }
        )*
    };
}

render_via_ryu! {
    f32 f64
}

impl<T: Renderable> Renderable for Option<T> {
    #[inline]
    fn render_to(&self, output: &mut String) {
        if let Some(value) = self {
            value.render_to(output);
        }
    }

    #[inline]
    fn render(&self) -> Rendered<String> {
        self.as_ref()
            .map_or_else(|| Rendered(String::new()), Renderable::render)
    }

    #[inline]
    fn memoize(&self) -> Raw<String> {
        self.as_ref()
            .map_or_else(|| Raw(String::new()), Renderable::memoize)
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

impl<T: Renderable> Renderable for Vec<T> {
    #[inline]
    fn render_to(&self, output: &mut String) {
        self.as_slice().render_to(output);
    }
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
