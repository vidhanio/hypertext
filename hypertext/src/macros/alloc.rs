#![allow(clippy::doc_markdown)]
/// Derive [`Renderable`](crate::Renderable) for a type.
///
/// # Examples
///
/// ## [`maud!`]
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
/// ## [`rsx!`]
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
/// ## [`attribute!`]
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
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
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
///     "<div title=\"x01234\">Hi!</div>"
/// );
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub use hypertext_macros::attribute;
/// Generate an attribute value, borrowing the environment.
///
/// This is identical to [`attribute!`], except that it does not take
/// ownership of the environment. This is useful when you want to build
/// a [`LazyAttribute`] using some captured variables, but you still
/// want to be able to use the variables after the [`LazyAttribute`] is
/// created.
///
/// [`LazyAttribute`]: crate::LazyAttribute
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub use hypertext_macros::attribute_borrow;
/// Convert a function returning a [`Renderable`](crate::Renderable) into a
/// component.
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
/// The visibility of the generated struct is determined by the visibility
/// of the function. If you would like to set a different visibility,
/// you can specify it as `#[component(pub)]`,
/// `#[component(pub(crate))]`, etc. on the function.
///
/// You can combine both of these by setting an attribute like
/// `#[component(pub MyComponentName)]`.
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
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub use hypertext_macros::component;
/// Generate HTML using [`maud`] syntax, returning a [`Lazy`](crate::Lazy).
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
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub use hypertext_macros::maud;
/// Generate HTML using [`maud!`] syntax, borrowing the environment.
///
/// This is identical to [`maud!`], except that it does not take ownership
/// of the environment. This is useful when you want to build a [`Lazy`]
/// using some captured variables, but you still want to be able to use
/// the variables after the [`Lazy`] is created.
///
/// [`Lazy`]: crate::Lazy
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub use hypertext_macros::maud_borrow;
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
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub use hypertext_macros::rsx;
/// Generate HTML using [`rsx!`] syntax, borrowing the environment.
///
/// This is identical to [`rsx!`], except that it does not take ownership of
/// the environment. This is useful when you want to build a [`Lazy`] using
/// some captured variables, but you still want to be able to use the
/// variables after the [`Lazy`] is created.
///
/// [`Lazy`]: crate::Lazy
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub use hypertext_macros::rsx_borrow;
