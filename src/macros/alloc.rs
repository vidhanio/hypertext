#![expect(clippy::doc_markdown)]

extern crate alloc;

/// Derives [`Renderable`](crate::Renderable) for a type.
///
/// This is used in conjunction with `#[maud]`/`#[rsx]`, as well as
/// `#[attribute]`.
///
/// # Examples
///
/// ## [`#[maud(...)]`](maud)
///
/// Derives [`Renderable`](crate::Renderable) via the contents of
/// `#[maud(...)]`, which will be interpreted as input to [`maud!`].
///
/// This is mutually exclusive with `#[rsx(...)]`.
///
/// ```
/// use hypertext::prelude::*;
///
/// #[derive(Renderable)]
/// #[maud(span { "My name is " (self.name) "!" })]
/// pub struct Person {
///     name: String,
/// }
///
/// assert_eq!(
///     maud! { div { (Person { name: "Alice".into() }) } }
///         .render()
///         .as_inner(),
///     "<div><span>My name is Alice!</span></div>"
/// );
/// ```
///
/// ## [`#[rsx(...)]`](rsx)
///
/// Derives [`Renderable`](crate::Renderable) via the contents of `#[rsx(...)]`,
/// which will be interpreted as input to [`rsx!`].
///
/// This is mutually exclusive with `#[maud(...)]`.
///
/// ```
/// use hypertext::prelude::*;
///
/// #[derive(Renderable)]
/// #[rsx(
///     <span>"My name is " (self.name) "!"</span>
/// )]
/// pub struct Person {
///     name: String,
/// }
///
/// assert_eq!(
///     rsx! { <div> (Person { name: "Alice".into() }) </div> }
///         .render()
///         .as_inner(),
///     "<div><span>My name is Alice!</span></div>"
/// );
/// ```
///
/// ## [`#[attribute(...)]`](attribute)
///
/// Derives [`Renderable<AttributeValue>`](crate::Renderable)
/// via the contents of `#[attribute(...)]`, which will be interpreted as input
/// to [`attribute!`].
///
/// This can be used in conjunction with `#[rsx]`/`#[maud]`, as this will
/// derive the [`Renderable<AttributeValue>`](crate::Renderable) implementation,
/// whereas `#[maud(...)]`/`#[rsx(...)]` will derive the
/// [`Renderable<Node>`](crate::Renderable) implementation.
///
/// ```
/// use hypertext::prelude::*;
///
/// #[derive(Renderable)]
/// #[attribute((self.x) "," (self.y))]
/// pub struct Coordinates {
///     x: i32,
///     y: i32,
/// }
///
/// assert_eq!(
///     maud! { div title=(Coordinates { x: 10, y: 20 }) { "Location" } }
///         .render()
///         .as_inner(),
///     r#"<div title="10,20">Location</div>"#
/// );
/// ```
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::Renderable;
/// Generate an attribute value, returning a
/// [`LazyAttribute`](crate::LazyAttribute).
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
///     r#"<div title="x01234">Hi!</div>"#
/// );
/// ```
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::attribute;
/// Generate an attribute value, borrowing the environment.
///
/// This is identical to [`attribute!`], except that it does not take
/// ownership of the environment. This is useful when you want to build
/// a [`LazyAttribute`](crate::LazyAttribute) using some captured variables, but
/// you still want to be able to use the captured variables after the
/// invocation.
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::attribute_borrow;
/// Generate HTML using Maud syntax, returning a [`Lazy`](crate::Lazy).
///
/// Note that this is not a complete 1:1 port of [Maud](https://maud.lambda.xyz)'s
/// syntax as it is stricter in some cases to prevent anti-patterns.
///
/// Some key differences are:
/// - `#` ([`id`](crate::validation::attributes::GlobalAttributes::id)
///   shorthand), if present, must be the first attribute.
/// - `.` ([`class`](crate::validation::attributes::GlobalAttributes::class)
///   shorthand), if present, come after `#` (if present) and before other
///   attributes.
///
/// Additionally, the `DOCTYPE` constant present in maud is replaced
/// with a new `!DOCTYPE` syntax, which will render `<!DOCTYPE html>` in its
/// place.
///
/// For more details on the rest of Maud's syntax, see the [Maud Book](https://maud.lambda.xyz).
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
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::maud;
/// Generate HTML using [`maud!`] syntax, borrowing the environment.
///
/// This is identical to [`maud!`], except that it does not take ownership
/// of the environment. This is useful when you want to build a
/// [`Lazy`](crate::Lazy) using some captured variables, but you still want to
/// be able to use the captured variables after the invocation.
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::maud_borrow;
/// Convert a function returning a [`Renderable`](crate::Renderable) into a
/// struct that implements [`Renderable`](crate::Renderable).
///
/// This macro generates a struct that has fields corresponding to the
/// function's parameters, and implements [`Renderable`](crate::Renderable)
/// by calling the function with the struct's fields as arguments.
///
/// There are three types of parameters that are supported, described in
/// the table below:
///
/// | Parameter Type | Stored As | Example Types |
/// |----------------|-----------|---------------|
/// | `T`            | `T`       | [`bool`], integers, floats, other [`Copy`] types |
/// | `&T`           | `T`       | [`&String`](alloc::string::String) |
/// | `&'a T`        | `&'a T`   | [`&'a str`][str], [`&'a [T]`](slice), other cheap borrowed types |
///
/// The name of the generated struct is derived from the function name by
/// converting it to PascalCase. If you would like to set a different name,
/// you can specify it as `#[renderable(MyStructName)]` on the function.
///
/// The visibility of the generated struct is determined by the visibility
/// of the function. If you would like to set a different visibility,
/// you can specify it as `#[renderable(pub)]`,
/// `#[renderable(pub(crate))]`, etc. on the function.
///
/// You can combine both of these by setting an attribute like
/// `#[renderable(pub MyStructName)]`.
///
/// # Example
///
/// ```
/// use hypertext::prelude::*;
///
/// #[renderable]
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
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::renderable;
/// Generate HTML using rsx syntax, returning a [`Lazy`](crate::Lazy).
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
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::rsx;
/// Generate HTML using [`rsx!`] syntax, borrowing the environment.
///
/// This is identical to [`rsx!`], except that it does not take ownership of
/// the environment. This is useful when you want to build a
/// [`Lazy`](crate::Lazy) using some captured variables, but you still want to
/// be able to use the captured variables after the invocation.
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::rsx_borrow;
