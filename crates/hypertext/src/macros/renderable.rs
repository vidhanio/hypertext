#![expect(clippy::doc_markdown)]

/// Derives [`Renderable`](crate::Renderable) for a type.
///
/// This is used in conjunction with `#[maud]`/`#[rsx]`, as well as
/// `#[attribute]`.
///
/// # Examples
///
/// ## `#[maud(...)]`
///
/// Derives [`Renderable`](crate::Renderable) via the contents of
/// `#[maud(...)]`, which will be interpreted as input to
/// [`maud!`](crate::maud!).
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
/// ## `#[rsx(...)]`
///
/// Derives [`Renderable`](crate::Renderable) via the contents of `#[rsx(...)]`,
/// which will be interpreted as input to [`rsx!`](crate::rsx!).
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
/// ## `#[attribute(...)]`
///
/// Derives [`Renderable<AttributeValue>`](crate::Renderable)
/// via the contents of `#[attribute(...)]`, which will be interpreted as input
/// to [`attribute!`](crate::attribute!).
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
///
/// ## `#[renderable(node = ...)]`
///
/// When deriving a node renderable with `#[maud(...)]` or `#[rsx(...)]`, you
/// can override the node context used by the generated implementation.
///
/// This is useful for SVG and MathML renderables so they only type-check in the
/// corresponding macro context.
///
/// ```
/// use hypertext::prelude::*;
///
/// #[derive(Renderable)]
/// #[renderable(node = svg)]
/// #[maud(circle cx=(self.cx) cy=(self.cy) r=(self.r);)]
/// struct Circle {
///     cx: u32,
///     cy: u32,
///     r: u32,
/// }
///
/// assert_eq!(
///     svg::maud! {
///         svg {
///             (Circle { cx: 50, cy: 50, r: 40 })
///         }
///     }
///     .render()
///     .as_inner(),
///     r#"<svg><circle cx="50" cy="50" r="40"/></svg>"#,
/// );
/// ```
///
/// ## Using with [`#[derive(Builder)]`](crate::Builder)
///
/// Combining [`#[derive(Renderable)]`](derive@crate::Renderable) with
/// [`#[derive(Builder)]`](crate::Builder) makes a struct usable as a component
/// in the [`maud!`](crate::maud!) and [`rsx!`](crate::rsx!) macros via the
/// component syntax.
///
/// ### [`maud!`](crate::maud!)
///
/// ```
/// use hypertext::{Builder, prelude::*};
///
/// #[derive(Builder, Renderable)]
/// #[maud(
///     div {
///         h1 { (self.title) }
///         p { (self.body) }
///     }
/// )]
/// pub struct Card {
///     title: String,
///     body: String,
/// }
///
/// assert_eq!(
///     maud! {
///         main {
///             Card title=("My Title".into()) body=("My Body".into());
///         }
///     }
///     .render()
///     .as_inner(),
///     "<main><div><h1>My Title</h1><p>My Body</p></div></main>",
/// );
/// ```
///
/// ### [`rsx!`](crate::rsx!)
///
/// ```
/// use hypertext::{Builder, prelude::*};
///
/// #[derive(Builder, Renderable)]
/// #[rsx(
///     <div>
///         <h1>(self.title)</h1>
///         <p>(self.body)</p>
///     </div>
/// )]
/// pub struct Card {
///     title: String,
///     body: String,
/// }
///
/// assert_eq!(
///     rsx! {
///         <main>
///             <Card title=("My Title".into()) body=("My Body".into())>
///         </main>
///     }
///     .render()
///     .as_inner(),
///     "<main><div><h1>My Title</h1><p>My Body</p></div></main>",
/// );
/// ```
///
/// ### With default field values
///
/// [`#[derive(Builder)]`](crate::Builder) automatically treats `Option<T>`
/// fields as optional — their setter accepts a `T` and wraps it in `Some`,
/// and they default to `None` when omitted.
///
/// ```
/// use hypertext::{Builder, prelude::*};
///
/// #[derive(Builder, Renderable)]
/// #[maud(
///     div {
///         h1 { (self.title) }
///         @if let Some(subtitle) = &self.subtitle {
///             h2 { (subtitle) }
///         }
///     }
/// )]
/// pub struct Header {
///     title: String,
///     subtitle: Option<String>,
/// }
///
/// assert_eq!(
///     maud! {
///         Header title=("Hello".into());
///         Header title=("Hello".into()) subtitle=("World".into());
///     }
///     .render()
///     .as_inner(),
///     "<div><h1>Hello</h1></div><div><h1>Hello</h1><h2>World</h2></div>",
/// );
/// ```
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::Renderable;
/// Turns a function returning a [`Renderable`](crate::Renderable) into a
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
/// | `&T`           | `T`       | [`&String`](crate::alloc::string::String) |
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
/// fn nav_bar<'a>(title: &'a str, subtitle: &String, add_smiley: bool) -> impl Renderable {
///     maud! {
///         nav {
///             h1 { (title) }
///             h2 { (subtitle) }
///             @if add_smiley {
///                 span { ":)" }
///             }
///         }
///     }
/// }
///
/// assert_eq!(
///     maud! {
///          div {
///              NavBar title="My Nav Bar" subtitle=("My Subtitle".to_owned()) add_smiley=true;
///          }
///     }
///     .render()
///     .as_inner(),
///     "<div><nav><h1>My Nav Bar</h1><h2>My Subtitle</h2><span>:)</span></nav></div>"
/// );
/// ```
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::renderable;
