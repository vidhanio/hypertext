//! Types used for validation of HTML elements and attributes.

/// An HTML element.
pub trait Element {
    /// The kind of this element.
    type Kind: ElementKind;
}
/// An element kind.
///
/// This can be either [`Normal`] or [`Void`]. A [`Normal`] element will always
/// have a closing tag, and can have children. A [`Void`] element will never
/// have a closing tag, and cannot have children.
pub trait ElementKind: sealed::Sealed {}

/// A normal HTML element.
///
/// This element has a closing tag and can have children.
///
/// # Example
///
/// ```html
/// <div>
///   Hello, world!
/// </div>
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Normal;

impl ElementKind for Normal {}

/// A void HTML element.
///
/// This element does not have a closing tag and cannot have children.
///
/// # Example
///
/// ```html
/// <img src="image.png" alt="An image">
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Void;

impl ElementKind for Void {}

mod sealed {
    use super::{Normal, Void};

    pub trait Sealed {}
    impl Sealed for Normal {}
    impl Sealed for Void {}
}

/// An HTML attribute.
#[derive(Debug, Clone, Copy)]
pub struct Attribute;

/// An HTML attribute namespace.
#[derive(Debug, Clone, Copy)]
pub struct AttributeNamespace;

/// An HTML attribute symbol.
#[derive(Debug, Clone, Copy)]
pub struct AttributeSymbol;

/// Global HTML attributes.
///
/// This trait must be in scope to use standard HTML attributes such as
/// [`class`](Self::class) and [`id`](Self::id). This trait is implemented
/// by every HTML element specified in [`crate::html_elements`].
///
/// # Usage With Custom Elements
///
/// ```
/// use hypertext::prelude::*;
///
/// mod html_elements {
///     #![expect(non_camel_case_types)]
///
///     pub use hypertext::html_elements::*;
///     use hypertext::validation::{Element, GlobalAttributes, Normal};
///
///     pub struct custom_element;
///
///     impl Element for custom_element {
///         type Kind = Normal;
///     }
///
///     impl GlobalAttributes for custom_element {}
/// }
///
/// assert_eq!(
///     maud! { custom-element title="abc" { "Hello, world!" } }.render(),
///     Rendered(r#"<custom-element title="abc">Hello, world!</custom-element>"#),
/// );
/// ```
#[expect(non_upper_case_globals)]
pub trait GlobalAttributes: Element {
    /// Used as a guide for creating a keyboard shortcut that activates or
    /// focuses the element.
    const access_key: Attribute = Attribute;

    /// The autocapitalization behavior to use when the text is edited through
    /// non-keyboard methods.
    const autocapitalize: Attribute = Attribute;

    /// Indicates whether the element should be automatically focused when the
    /// page is loaded.
    const autofocus: Attribute = Attribute;

    /// The class of the element.
    #[doc(alias = ".")]
    const class: Attribute = Attribute;

    /// Whether the element is editable.
    const contenteditable: Attribute = Attribute;

    /// The text directionality of the element.
    const dir: Attribute = Attribute;

    /// Whether the element is draggable.
    const draggable: Attribute = Attribute;

    /// A hint as to what the `enter` key should do.
    const enterkeyhint: Attribute = Attribute;

    /// Whether the element is hidden from view.
    const hidden: Attribute = Attribute;

    /// A unique identifier for the element.
    #[doc(alias = "#")]
    const id: Attribute = Attribute;

    /// Mark an element and its children as inert, disabling interaction.
    const inert: Attribute = Attribute;

    /// Specifies what kind of input mechanism would be most helpful for users
    /// entering content.
    const inputmode: Attribute = Attribute;

    /// Specify which element this is a custom variant of.
    const is: Attribute = Attribute;

    /// A global identifier for the item.
    const itemid: Attribute = Attribute;

    /// A property that the item has.
    const itemprop: Attribute = Attribute;

    /// A list of additional elements to crawl to find the name-value pairs of
    /// the item.
    const itemref: Attribute = Attribute;

    /// Creates a new item, a group of name-value pairs.
    const itemscope: Attribute = Attribute;

    /// The item types of the item.
    const itemtype: Attribute = Attribute;

    /// The language of the element.
    const lang: Attribute = Attribute;

    /// A cryptographic nonce ("number used once") which can be used by Content
    /// Security Policy to determine whether or not a given fetch will be
    /// allowed to proceed.
    const nonce: Attribute = Attribute;

    /// When specified, the element won't be rendered until it becomes shown, at
    /// which point it will be rendered on top of other page content.
    const popover: Attribute = Attribute;

    /// The slot the element is inserted in.
    const slot: Attribute = Attribute;

    /// Whether the element is spellchecked or not.
    const spellcheck: Attribute = Attribute;

    /// The CSS styling to apply to the element.
    const style: Attribute = Attribute;

    /// Customize the index of the element for sequential focus navigation.
    const tabindex: Attribute = Attribute;

    /// A text description of the element.
    const title: Attribute = Attribute;

    /// Whether the element is to be translated when the page is localized.
    const translate: Attribute = Attribute;
}
