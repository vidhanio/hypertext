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
/// use hypertext::{GlobalAttributes, Renderable, Rendered, html_elements, maud};
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
#[cfg(feature = "maud")]
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
#[macro_export]
#[cfg(feature = "maud")]
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
/// use hypertext::{GlobalAttributes, Renderable, Rendered, html_elements, rsx};
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
#[cfg(feature = "rsx")]
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
#[cfg(feature = "rsx")]
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

/// A type that can be rendered to a string.
///
/// # Example
///
/// ```
/// use hypertext::{Renderable, Rendered, html_elements, maud};
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

    /// Converts this value into a [`Box<dyn Renderable>`].
    ///
    /// This is useful for dynamically generated HTML that needs to be passed
    /// around as a trait object without being pre-rendered. A common use case
    /// is when returning [`Lazy`] closures from branched code, where the
    /// different concrete [`Fn(&mut String)`] implementations stored in the
    /// [`Lazy`] closures need to be converted to the same type.
    ///
    /// Note that this is usually not necessary when using [`maud!`] or
    /// [`rsx!`], as they provide branching syntax within the macro that allows
    /// you to generate the same type without needing to convert to a trait
    /// object.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hypertext::{Renderable, html_elements, maud};
    ///
    /// fn cake_status_dyn(likes_cake: bool) -> impl Renderable {
    ///     if likes_cake {
    ///         maud! { div { "I like cake!" } }.dyn_renderable()
    ///     } else {
    ///         maud! { div { "I don't like cake!" } }.dyn_renderable()
    ///     }
    /// }
    ///
    /// // instead, could be written as:
    ///
    /// fn cake_status(likes_cake: bool) -> impl Renderable {
    ///     maud! {
    ///         div {
    ///             @if likes_cake {
    ///                 "I like cake!"
    ///             } @else {
    ///                 "I don't like cake!"
    ///             }
    ///         }
    ///     }
    /// }
    /// # assert_eq!(cake_status_dyn(true).render(), cake_status(true).render());
    /// # assert_eq!(cake_status_dyn(false).render(), cake_status(false).render());
    /// ```
    ///
    /// [`maud!`]: crate::maud
    /// [`Fn(&mut String)`]: core::ops::Fn
    #[inline]
    fn dyn_renderable<'a>(self) -> Box<dyn Renderable + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
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
///
/// [`maud!`]: crate::maud
#[derive(Clone, Copy)]
#[must_use = "`Lazy` does nothing unless `.render_to()` or `.render()` is called"]
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
