//! The [`Context`] trait and its implementors.

use core::marker::PhantomData;

/// A marker trait to represent the context that the value is being rendered to.
///
/// This can be either [`Node`] or an [`AttributeValue`]. A [`Node`]
/// represents a complete node in a specific markup context, while an
/// [`AttributeValue`] represents an attribute value which will eventually be
/// surrounded by double quotes.
///
/// This is used to ensure that the correct rendering methods are called
/// for each context, and to prevent errors such as accidentally rendering
/// an HTML element into an attribute value.
pub trait Context: sealed::Sealed {}

/// A marker trait for node kinds.
pub trait NodeKind: sealed::Sealed {}

/// A marker trait for XML node kinds.
pub trait XmlKind: sealed::Sealed {}

/// A marker type to represent HTML nodes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Html;

impl NodeKind for Html {}

/// A marker type to represent XML nodes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Xml<K: XmlKind>(PhantomData<K>);

impl<K: XmlKind> NodeKind for Xml<K> {}

/// A marker type to represent SVG nodes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Svg;

impl XmlKind for Svg {}

/// A marker type to represent MathML nodes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MathMl;

impl XmlKind for MathMl {}

/// A marker type to represent a complete element node.
///
/// All types and traits that are generic over [`Context`] use [`Node`]
/// as the default for the generic type parameter.
///
/// Traits and types with this marker type expect complete HTML nodes. If
/// rendering string-like types, the value/implementation must escape `&` to
/// `&amp;`, `<` to `&lt;`, and `>` to `&gt;`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Node<K: NodeKind = Html>(PhantomData<K>);

impl<K: NodeKind> Context for Node<K> {}

/// A marker type to represent an attribute value.
///
/// Traits and types with this marker type expect an attribute value which will
/// eventually be surrounded by double quotes. The value/implementation must
/// escape `&` to `&amp;`, `<` to `&lt;`, `>` to `&gt;`, and `"` to `&quot;`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct AttributeValue;

impl Context for AttributeValue {}

mod sealed {
    use super::{AttributeValue, Html, MathMl, Node, NodeKind, Svg, Xml, XmlKind};

    pub trait Sealed {}
    impl Sealed for Html {}
    impl Sealed for Svg {}
    impl Sealed for MathMl {}
    impl<K: XmlKind> Sealed for Xml<K> {}
    impl<K: NodeKind> Sealed for Node<K> {}
    impl Sealed for AttributeValue {}
}
