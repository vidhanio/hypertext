//! The [`Context`] trait and its implementors.

/// A marker trait to represent the context that the value is being rendered to.
///
/// This can be either [`Node`] or an [`AttributeValue`]. A [`Node`]
/// represents an HTML node, while an [`AttributeValue`] represents an attribute
/// value which will eventually be surrounded by double quotes.
///
/// This is used to ensure that the correct rendering methods are called
/// for each context, and to prevent errors such as accidentally rendering
/// an HTML element into an attribute value.
pub trait Context: sealed::Sealed {}

/// A marker type to represent a complete element node.
///
/// All types and traits that are generic over [`Context`] use [`Node`]
/// as the default for the generic type parameter.
///
/// Traits and types with this marker type expect complete HTML nodes. If
/// rendering string-like types, the value/implementation must escape `&` to
/// `&amp;`, `<` to `&lt;`, and `>` to `&gt;`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Node;

impl Context for Node {}

/// A marker type to represent an attribute value.
///
/// Traits and types with this marker type expect an attribute value which will
/// eventually be surrounded by double quotes. The value/implementation must
/// escape `&` to `&amp;`, `<` to `&lt;`, `>` to `&gt;`, and `"` to `&quot;`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct AttributeValue;

impl Context for AttributeValue {}

mod sealed {
    use super::{AttributeValue, Node};

    pub trait Sealed {}
    impl Sealed for Node {}
    impl Sealed for AttributeValue {}
}
