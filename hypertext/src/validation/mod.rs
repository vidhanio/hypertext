//! Types and traits used for validation of HTML elements and attributes.
pub mod attributes;
pub mod hypertext_elements;
#[cfg(feature = "mathml")]
mod mathml;

/// A marker trait for type-checked elements.
pub trait Element {
    /// The kind of this element.
    type Kind: ElementKind;
}
/// A marker trait to represent the kind of an element.
///
/// This can be either [`Normal`] or [`Void`]. A [`Normal`] element will always
/// have a closing tag, and can have children. A [`Void`] element will never
/// have a closing tag, and cannot have children.
pub trait ElementKind: sealed::Sealed {}

/// A marker type to represent a normal element.
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

/// A marker type to represent a void element.
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

/// A standard attribute.
#[derive(Debug, Clone, Copy)]
pub struct Attribute;

/// An attribute namespace.
#[derive(Debug, Clone, Copy)]
pub struct AttributeNamespace;

/// An attribute prefixed by a symbol.
#[derive(Debug, Clone, Copy)]
pub struct AttributeSymbol;

/// Define custom elements.
///
/// This macro should be called from within a module named `hypertext_elements`.
///
/// # Example
///
/// ```rust
/// mod hypertext_elements {
///     use hypertext::validation::define_elements;
///     // Re-export all standard HTML elements
///     pub use hypertext::validation::hypertext_elements::*;
///
///     define_elements! {
///         /// A custom web component that greets the user.
///         simple_greeting {
///             /// The name of the person to greet.
///             name
///         }
///
///         /// An element representing a coordinate.
///         coordinate {
///             /// The x coordinate.
///             x
///
///             /// The y coordinate.
///             y
///         }
///     }
/// }
///
/// // Now, you can use the custom elements like this:
///
/// use hypertext::prelude::*;
///
/// assert_eq!(
///     maud! {
///         simple-greeting name="Alice" {
///             coordinate x=1 y=2 {}
///         }
///     }
///     .render()
///     .as_inner(),
///     r#"<simple-greeting name="Alice"><coordinate x="1" y="2"></coordinate></simple-greeting>"#,
/// )
/// ```
#[macro_export]
macro_rules! define_elements {
    {
        $(
            $(#[$meta:meta])*
            $name:ident $(
                {
                    $(
                        $(#[$attr_meta:meta])*
                        $attr:ident
                    )*
                }
            )?
        )*
    } => {
        $(
            $(#[$meta])*
            #[expect(
                non_camel_case_types,
                reason = "camel case types will be interpreted as components"
            )]
            #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
            pub struct $name;

            $(
                #[allow(non_upper_case_globals)]
                impl $name {
                    $(
                        $(#[$attr_meta])*
                        pub const $attr: $crate::validation::Attribute = $crate::validation::Attribute;
                    )*
                }
            )?

            impl $crate::validation::Element for $name {
                type Kind = $crate::validation::Normal;
            }

            impl $crate::validation::attributes::GlobalAttributes for $name {}
        )*
    }
}
pub use define_elements;

/// Define custom void elements.
///
/// This macro should be called from within a module named `hypertext_elements`.
///
/// # Example
/// ```rust
/// mod hypertext_elements {
///     // Re-export all standard HTML elements
///     use hypertext::validation::define_void_elements;
///     pub use hypertext::validation::hypertext_elements::*;
///
///     define_void_elements! {
///         /// A custom void element that greets the user.
///         simple_greeting {
///             /// The name of the person to greet.
///             name
///         }
///     }
/// }
///
/// // Now, you can use the custom elements like this:
///
/// use hypertext::prelude::*;
///
/// assert_eq!(
///     maud! {
///         simple-greeting name="Alice";
///     }
///     .render()
///     .as_inner(),
///     r#"<simple-greeting name="Alice">"#,
/// )
/// ```
#[macro_export]
macro_rules! define_void_elements {
    {
        $(
            $(#[$meta:meta])*
            $name:ident $(
                {
                    $(
                        $(#[$attr_meta:meta])*
                        $attr:ident
                    )*
                }
            )?
        )*
    } => {
        $(
            $(#[$meta])*
            #[expect(
                non_camel_case_types,
                reason = "camel case types will be interpreted as components"
            )]
            #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
            pub struct $name;

            $(
                #[allow(non_upper_case_globals)]
                impl $name {
                    $(
                        $(#[$attr_meta])*
                        pub const $attr: $crate::validation::Attribute = $crate::validation::Attribute;
                    )*
                }
            )?

            impl $crate::validation::Element for $name {
                type Kind = $crate::validation::Void;
            }

            impl $crate::validation::attributes::GlobalAttributes for $name {}
        )*
    }
}
pub use define_void_elements;
