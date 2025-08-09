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
