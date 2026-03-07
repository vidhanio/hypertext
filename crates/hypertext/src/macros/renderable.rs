#![expect(clippy::doc_markdown)]

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
