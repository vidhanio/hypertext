extern crate alloc;

use alloc::string::String;
use core::fmt::{self, Display, Write};

/// Render HTML using rsx syntax.
///
/// # Example
///
/// ```
/// use hypertext::{html, html_elements, GlobalAttributes};
///
/// assert_eq!(
///     html! {
///         <div id="profile" title="Profile">
///             <h1>Alice</h1>
///         </div>
///     }
///     .as_str(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#,
/// );
/// ```
pub use hypertext_macros::html;
/// Render HTML lazily using rsx syntax.
///
/// This is the recommended way to compose multiple nested elements
/// together.
///
/// ```
/// use hypertext::{html, html_elements, html_lazy, GlobalAttributes, Render};
///
/// // must include lifetime of `name` in return type, as it is borrowed
/// // in the lazy closure.
/// fn user_name(name: &str) -> impl Render + '_ {
///     // does not allocate here at all
///     html_lazy! { <h1>{ (name) }</h1> }
/// }
///
/// assert_eq!(
///     // only allocates once here
///     html! {
///         <div id="profile" title="Profile">
///             { (user_name("Alice")) }
///         </div>
///     }
///     .as_str(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#,
/// );
/// ```
pub use hypertext_macros::html_lazy;
/// Render HTML using [`maud`] syntax.
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
/// For more details, see the [maud book](https://maud.lambda.xyz).
///
/// # Example
///
/// ```
/// use hypertext::{html_elements, maud, GlobalAttributes};
///
/// assert_eq!(
///     maud! {
///         div #profile title="Profile" {
///             h1 { "Alice" }
///        }
///     }
///     .as_str(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#,
/// );
/// ```
///
/// [`maud`]: https://docs.rs/maud
/// [`id`]: crate::GlobalAttributes::id
/// [`class`]: crate::GlobalAttributes::class
pub use hypertext_macros::maud;
/// Render HTML lazily using [`maud`] syntax.
///
/// This is the recommended way to compose multiple nested elements
/// together.
///
/// For details about the syntax, see [`maud!`].
///
/// ```
/// use hypertext::{html_elements, maud, maud_lazy, GlobalAttributes, Render};
///
/// // must include lifetime of `name` in return type, as it is borrowed
/// // in the lazy closure.
/// fn user_name(name: &str) -> impl Render + '_ {
///     // does not allocate here at all
///     maud_lazy! { h1 { (name) } }
/// }
///
/// assert_eq!(
///     // only allocates once here
///     maud! {
///         div #profile title="Profile" {
///             (user_name("Alice"))
///         }
///     }
///     .as_str(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#,
/// );
/// ```
///
/// [`maud`]: https://docs.rs/maud
pub use hypertext_macros::maud_lazy;

use crate::Rendered;

impl<T: Into<Self>> From<Rendered<T>> for alloc::string::String {
    #[inline]
    fn from(Rendered(value): Rendered<T>) -> Self {
        value.into()
    }
}

/// A value rendered via its [`Display`] implementation.
///
/// This will handle escaping special characters for you.
#[derive(Debug, Clone, Copy)]
pub struct Displayed<T: Display>(pub T);

impl<T: Display> Render for Displayed<T> {
    #[inline]
    fn render_to(self, output: &mut String) {
        struct Escaper<'a>(&'a mut String);

        impl<'a> fmt::Write for Escaper<'a> {
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

/// A lazily rendered value.
///
/// This is used internally by [`maud_lazy!`](super::maud_lazy) and
/// [`html_lazy!`].
///
/// This is the recommended way to compose multiple nested elements
/// together.
///
/// The renderer must handle escaping any special characters.
#[derive(Debug, Clone, Copy)]
pub struct Lazy<F: FnOnce(&mut String)>(pub F);

impl<F: FnOnce(&mut String)> Render for Lazy<F> {
    #[inline]
    fn render_to(self, output: &mut String) {
        self.0(output);
    }
}

/// A value that is rendered without escaping.
///
/// This is useful for rendering raw HTML, but should be used with caution
/// as it can lead to XSS vulnerabilities if used incorrectly. If you are
/// unsure, render the `&str` itself instead, which will escape any special
/// characters.
#[derive(Debug, Clone, Copy)]
pub struct PreEscaped<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> Render for PreEscaped<T> {
    #[inline]
    fn render_to(self, output: &mut String) {
        output.push_str(self.0.as_ref());
    }
}

/// An extension trait for [`Iterator`]s that can be rendered.
pub trait RenderIterator: IntoIterator
where
    Self::Item: Render,
{
    /// Renders each item in this iterator.
    ///
    /// # Example
    ///
    /// ```
    /// use hypertext::{html_elements, maud, maud_lazy, GlobalAttributes, Render, RenderIterator};
    ///
    /// let items = ["milks", "eggs", "bread"];
    ///
    /// assert_eq!(
    ///     maud! {
    ///         ul #shopping-list {
    ///             (items
    ///                 .iter()
    ///                 .map(|&item| maud_lazy! { li { (item) } })
    ///                 .render_all())
    ///         }
    ///     }.as_str(),
    ///     r#"<ul id="shopping-list"><li>milks</li><li>eggs</li><li>bread</li></ul>"#
    /// );
    fn render_all(self) -> impl Render;
}

impl<I: IntoIterator> RenderIterator for I
where
    Self::Item: Render,
{
    #[inline]
    fn render_all(self) -> impl Render {
        Lazy(|output| {
            self.into_iter().for_each(|item| {
                item.render_to(output);
            });
        })
    }
}

/// A type that can be rendered to a string.
///
/// # Example
///
/// ```
/// use hypertext::{html_elements, maud, maud_lazy, Render};
///
/// pub struct Person {
///     name: String,
///     age: u8,
/// }
///
/// impl Render for Person {
///     fn render_to(self, output: &mut String) {
///         maud_lazy! {
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
///     maud! { main { (person) } }.as_str(),
///     r#"<main><div><h1>Alice</h1><p>Age: 20</p></div></main>"#,
/// );
/// ```
pub trait Render {
    /// Renders this type to the given string.
    ///
    /// The implementation must handle escaping any special characters.
    fn render_to(self, output: &mut String);
}

impl Render for char {
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

impl Render for &str {
    #[inline]
    fn render_to(self, output: &mut String) {
        html_escape::encode_single_quoted_attribute_to_string(self, output);
    }
}

impl Render for String {
    #[inline]
    fn render_to(self, output: &mut String) {
        html_escape::encode_single_quoted_attribute_to_string(self, output);
    }
}

impl Render for bool {
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
            impl Render for $Ty {
                #[inline]
                fn render_to(self, output: &mut String) {
                    output.push_str(itoa::Buffer::new().format(self));
                }
            }
        )*
    };
}

macro_rules! render_via_ryu {
    ($($Ty:ty)*) => {
        $(
            impl Render for $Ty {
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

render_via_itoa! {
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
}
