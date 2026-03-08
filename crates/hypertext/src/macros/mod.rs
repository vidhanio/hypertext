pub mod attribute;
pub mod mathml;
pub mod maud;
#[cfg(feature = "alloc")]
mod renderable;
pub mod rsx;
pub mod svg;

/// Generates simple implementations of the builder methods
/// for a type implementing `Default`.
///
/// # Example
///
/// ```
/// use hypertext::{Buffer, DefaultBuilder, Lazy, prelude::*};
///
/// #[renderable(builder = DefaultBuilder)]
/// #[derive(Default)]
/// fn component<'a>(
///     id: &'a str,
///     tabindex: u32,
///     children: Lazy<fn(&mut Buffer)>,
/// ) -> impl Renderable {
///     rsx! {
///         <div id=(id) tabindex=(tabindex)>
///             (children)
///         </div>
///     }
/// }
/// ```
///
/// Expands to:
///
/// ```ignore
/// impl<'a> Component<'a> {
///     fn builder() -> Self {
///         Self::default()
///     }
///
///     fn build(self) -> Self {
///         self
///     }
///
///     #[must_use]
///     fn id(mut self, id: &'a str) -> Self {
///         self.id = id;
///         self
///     }
///
///     #[must_use]
///     fn tabindex(mut self, tabindex: u32) -> Self {
///         self.tabindex = tabindex;
///         self
///     }
///
///     #[must_use]
///     fn children(mut self, children: Lazy<fn(&mut Buffer)>) -> Self {
///         self.children = children;
///         self
///     }
/// }
/// ```
pub use hypertext_macros::DefaultBuilder;
/// Generates an attribute value, returning a
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
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::attribute;
/// Generates HTML using Maud syntax, returning a [`Lazy`](crate::Lazy).
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
/// # Optional Attributes
///
/// Attributes can be made conditional using a `[condition]` toggle suffix.
/// When the condition is `false`, the attribute is omitted entirely.
///
/// ## Boolean (empty) attributes
///
/// Use `attr[condition]` to include a boolean attribute only when `condition`
/// is `true`:
///
/// ```
/// use hypertext::prelude::*;
///
/// let is_checked = true;
/// let is_disabled = false;
///
/// assert_eq!(
///     maud! {
///         input checked[is_checked] disabled[is_disabled];
///     }
///     .render()
///     .as_inner(),
///     r#"<input checked>"#
/// );
/// ```
///
/// ## Attributes with values
///
/// Use `attr="value"[condition]` to include an attribute with a value only
/// when `condition` is `true`:
///
/// ```
/// use hypertext::prelude::*;
///
/// let highlighted = true;
///
/// assert_eq!(
///     maud! {
///         p style="background: yellow"[highlighted] { "text" }
///     }
///     .render()
///     .as_inner(),
///     r#"<p style="background: yellow">text</p>"#
/// );
/// ```
///
/// ## `Option`al attributes
///
/// Use `attr=[expr]` (where `expr` evaluates to an `Option`) to include an
/// attribute only when the value is `Some`:
///
/// ```
/// use hypertext::prelude::*;
///
/// let title: Option<&str> = Some("Hello");
/// let label: Option<&str> = None;
///
/// assert_eq!(
///     maud! {
///         div title=[title] aria-label=[label] { "content" }
///     }
///     .render()
///     .as_inner(),
///     r#"<div title="Hello">content</div>"#
/// );
/// ```
///
/// ## Optional classes
///
/// The `.class` shorthand also supports `[condition]` and `=[option_expr]`
/// suffixes:
///
/// ```
/// use hypertext::prelude::*;
///
/// let active = true;
/// let extra: Option<&str> = Some("highlighted");
///
/// assert_eq!(
///     maud! {
///         div .base .active[active] .[extra] { "content" }
///     }
///     .render()
///     .as_inner(),
///     r#"<div class="base active highlighted">content</div>"#
/// );
/// ```
///
/// For more details on the rest of Maud's syntax, see the [Maud Book](https://maud.lambda.xyz).
///
/// # Example
///
/// ```
/// use hypertext::prelude::*;
///
/// let name = "Alice";
///
/// assert_eq!(
///     maud! {
///         div #profile title="Profile" {
///             h1 { (name) }
///        }
///     }
///     .render()
///     .as_inner(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#
/// );
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::maud;
/// Generates HTML using RSX syntax, returning a [`Lazy`](crate::Lazy).
///
/// # Optional Attributes
///
/// Attributes can be made conditional using a `[condition]` toggle suffix.
/// When the condition is `false`, the attribute is omitted entirely.
///
/// ## Boolean (empty) attributes
///
/// Use `attr[condition]` to include a boolean attribute only when `condition`
/// is `true`:
///
/// ```
/// use hypertext::prelude::*;
///
/// let is_checked = true;
/// let is_disabled = false;
///
/// assert_eq!(
///     rsx! {
///         <input checked[is_checked] disabled[is_disabled] />
///     }
///     .render()
///     .as_inner(),
///     r#"<input checked>"#
/// );
/// ```
///
/// ## Attributes with values
///
/// Use `attr="value"[condition]` to include an attribute with a value only
/// when `condition` is `true`:
///
/// ```
/// use hypertext::prelude::*;
///
/// let highlighted = true;
///
/// assert_eq!(
///     rsx! {
///         <p style="background: yellow"[highlighted]>"text"</p>
///     }
///     .render()
///     .as_inner(),
///     r#"<p style="background: yellow">text</p>"#
/// );
/// ```
///
/// ## `Option`al attributes
///
/// Use `attr=[expr]` (where `expr` evaluates to an `Option`) to include an
/// attribute only when the value is `Some`:
///
/// ```
/// use hypertext::prelude::*;
///
/// let title: Option<&str> = Some("Hello");
/// let label: Option<&str> = None;
///
/// assert_eq!(
///     rsx! {
///         <div title=[title] aria-label=[label]>"content"</div>
///     }
///     .render()
///     .as_inner(),
///     r#"<div title="Hello">content</div>"#
/// );
/// ```
///
/// # Examples
///
/// ```
/// use hypertext::prelude::*;
///
/// let name = "Alice";
///
/// assert_eq!(
///     rsx! {
///         <div id="profile" title="Profile">
///             <h1>(name)</h1>
///         </div>
///     }
///     .render()
///     .as_inner(),
///     r#"<div id="profile" title="Profile"><h1>Alice</h1></div>"#
/// );
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "alloc")))]
pub use hypertext_macros::rsx;

#[cfg(feature = "alloc")]
pub use self::renderable::*;
