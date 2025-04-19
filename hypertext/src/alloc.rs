extern crate alloc;

use alloc::{borrow::Cow, rc::Rc, string::String, sync::Arc};
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
/// use hypertext::{html_elements, maud, GlobalAttributes, Renderable};
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
/// use hypertext::{html_elements, rsx, GlobalAttributes, Renderable};
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

use crate::Rendered;

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
/// use hypertext::{html_elements, maud, Renderable};
///
/// pub struct Person {
///     name: String,
///     age: u8,
/// }
///
/// impl Renderable for Person {
///     fn render_to(self, output: &mut String) {
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
pub trait Renderable
where
    Self: Sized,
{
    /// Renders this type to the given string.
    ///
    /// The implementation must handle escaping any special characters.
    fn render_to(self, output: &mut String);

    /// Renders this value to a string.
    #[inline]
    fn render(self) -> Rendered<String> {
        let mut output = String::new();
        self.render_to(&mut output);
        Rendered(output)
    }
}

/// A value rendered via its [`Display`] implementation.
///
/// This will handle escaping special characters for you.
#[derive(Debug, Clone, Copy)]
pub struct Displayed<T: Display>(pub T);

impl<T: Display> Renderable for Displayed<T> {
    #[inline]
    fn render_to(self, output: &mut String) {
        struct Escaper<'a>(&'a mut String);

        impl fmt::Write for Escaper<'_> {
            #[inline]
            fn write_str(&mut self, s: &str) -> fmt::Result {
                html_escape::encode_double_quoted_attribute_to_string(s, self.0);
                Ok(())
            }
        }

        // ignore errors, as we are writing to a string
        let _ = write!(Escaper(output), "{}", self.0);
    }
}

impl<F: FnOnce(&mut String)> Renderable for F {
    #[inline]
    fn render_to(self, output: &mut String) {
        self(output);
    }
}

/// A raw value that is rendered without escaping.
///
/// This is useful for rendering raw HTML, but should be used with caution
/// as it can lead to XSS vulnerabilities if used incorrectly. If you are
/// unsure, render the actual string instead, as its implementation will
/// escape any special characters.
#[derive(Debug, Clone, Copy)]
pub struct Raw<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> Renderable for Raw<T> {
    #[inline]
    fn render_to(self, output: &mut String) {
        output.push_str(self.0.as_ref());
    }
}

/// An extension trait for [`IntoIterator`]s that can be rendered.
pub trait RenderIterator: IntoIterator
where
    Self: Sized,
    Self::Item: Renderable,
{
    /// Renders each item in this iterator.
    ///
    /// # Example
    ///
    /// ```
    /// use hypertext::{html_elements, maud, maud_move, GlobalAttributes, Renderable, RenderIterator};
    ///
    /// let items = ["milks", "eggs", "bread"];
    ///
    /// assert_eq!(
    ///     maud! {
    ///         ul #shopping-list {
    ///             (items
    ///                 .iter()
    ///                 .map(|&item| maud_move! { li { (item) } })
    ///                 .render_all())
    ///         }
    ///     }.render(),
    ///     r#"<ul id="shopping-list"><li>milks</li><li>eggs</li><li>bread</li></ul>"#
    /// );
    #[inline]
    fn render_all(self) -> impl FnOnce(&mut String) {
        |output| {
            self.into_iter().for_each(|item| {
                item.render_to(output);
            });
        }
    }
}

impl<I: IntoIterator> RenderIterator for I where Self::Item: Renderable {}

impl Renderable for char {
    #[inline]
    fn render_to(self, output: &mut String) {
        match self {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#x27;"),
            '/' => output.push_str("&#x2f;"),
            c => output.push(c),
        }
    }
}

impl Renderable for &str {
    #[inline]
    fn render_to(self, output: &mut String) {
        html_escape::encode_double_quoted_attribute_to_string(self, output);
    }
}

impl Renderable for &String {
    #[inline]
    fn render_to(self, output: &mut String) {
        self.as_str().render_to(output);
    }
}

impl Renderable for String {
    #[inline]
    fn render_to(self, output: &mut String) {
        self.as_str().render_to(output);
    }
}

impl Renderable for Cow<'_, str> {
    #[inline]
    fn render_to(self, output: &mut String) {
        self.as_ref().render_to(output);
    }
}

impl Renderable for bool {
    #[inline]
    fn render_to(self, output: &mut String) {
        if self {
            output.push_str("true");
        } else {
            output.push_str("false");
        }
    }
}

macro_rules! render_via_itoa {
    ($($Ty:ty)*) => {
        $(
            impl Renderable for $Ty {
                #[inline]
                fn render_to(self, output: &mut String) {
                    output.push_str(itoa::Buffer::new().format(self));
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
                fn render_to(self, output: &mut String) {
                    output.push_str(ryu::Buffer::new().format(self));
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
    fn render_to(self, output: &mut String) {
        if let Some(value) = self {
            value.render_to(output);
        }
    }
}

impl<T> Renderable for Arc<T>
where
    for<'a> &'a T: Renderable,
{
    #[inline]
    fn render_to(self, output: &mut String) {
        (&*self).render_to(output);
    }
}

impl<T> Renderable for Rc<T>
where
    for<'a> &'a T: Renderable,
{
    #[inline]
    fn render_to(self, output: &mut String) {
        (&*self).render_to(output);
    }
}
