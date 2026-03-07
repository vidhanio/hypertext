//! Types and traits for compile-time validation of elements and attributes.
//!
//! Elements are validated by searching for a unit struct with the same name
//! that implements [`Element`]. It searches your current scope, then falls back
//! to searching the contents of an imported module named `hypertext_elements`.
//! This library has a default [`hypertext_elements`] module, usually brought
//! into scope via [`use hypertext::prelude::*`](crate::prelude).
//!
//! Attributes are validated by accessing a constant with the same name
//! as the attribute on the element type. More details on the different types of
//! attributes and attribute traits can be found in the [`attributes`
//! module-level documentation](attributes).
//!
//! If you have an attribute that cannot be parsed by this library or you do not
//! care about being type-checked, you can skip validation by surrounding the
//! attribute name with double quotes. Additionally, any `data-*` attributes
//! will not be validated, so you can use them freely.
pub mod attributes;
pub mod hypertext_elements;

/// A marker trait for type checked elements.
pub trait Element {
    /// The kind of this element.
    type Kind: ElementKind;
}
/// A marker trait to represent the kind of an element.
///
/// This can be either [`Normal`] or [`Void`].
pub trait ElementKind: sealed::Sealed {}

/// A marker type to represent a normal element.
///
/// Types implementing [`Element<Kind = Normal>`] must have a closing tag, and
/// may have children.
///
/// # Examples
///
/// ## [`maud!`](crate::maud!)
///
/// ```
/// use hypertext::prelude::*;
///
/// assert_eq!(
///     maud! {
///         // `div` is `Element<Kind = Normal>`:
///         div { "content" }
///     }
///     .render()
///     .as_inner(),
///     "<div>content</div>",
/// );
/// ```
///
/// ## [`rsx!`](crate::rsx!)
///
/// ```
/// use hypertext::prelude::*;
///
/// assert_eq!(
///     rsx! {
///         // `div` is `Element<Kind = Normal>`:
///         <div>content</div>
///     }
///     .render()
///     .as_inner(),
///     "<div>content</div>",
/// );
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Normal;

impl ElementKind for Normal {}

/// A marker type to represent a void element.
///
/// Types that implement [`Element<Kind = Void>`] cannot have a closing tag and
/// cannot have children.
///
/// # Examples
///
/// ## [`maud!`](crate::maud!)
///
/// ```
/// use hypertext::prelude::*;
///
/// assert_eq!(
///     maud! {
///        // `img` is `Element<Kind = Void>`:
///        img src="image.png" alt="An image";
///     }
///     .render()
///     .as_inner(),
///     r#"<img src="image.png" alt="An image">"#,
/// );
/// ```
///
/// ## [`rsx!`](crate::rsx!)
///
/// ```
/// use hypertext::prelude::*;
///
/// assert_eq!(
///     rsx! {
///         // `img` is `Element<Kind = Void>`:
///         // just like in html, in `rsx!` syntax the `/` at the end is optional
///         <img src="image.png" alt="An image">
///     }
///     .render()
///     .as_inner(),
///     r#"<img src="image.png" alt="An image">"#,
/// );
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
///
/// This is the most common type of attribute. During type-checking, any
/// attribute that only contains alphanumeric characters, hyphens (`-`), or
/// underscores (`_`) will be validated as an [`Attribute`].
///
/// # Example
/// ```
/// use hypertext::{
///     prelude::*,
///     validation::{Attribute, Element, Normal},
/// };
///
/// struct my_element;
///
/// impl Element for my_element {
///     type Kind = Normal;
/// }
///
/// impl my_element {
///     pub const my_attr: Attribute = Attribute;
/// }
///
/// assert_eq!(
///     maud! {
///         my-element my-attr="value" { "content" }
///     }
///     .render()
///     .as_inner(),
///     r#"<my-element my-attr="value">content</my-element>"#
/// );
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Attribute;

/// An attribute namespace.
///
/// During type-checking, if an attribute contains a colon (`:`), the part
/// before the colon will be treated as the namespace, and the part after the
/// colon will not be validated.
///
/// # Example
///
/// ```
/// use hypertext::{
///     prelude::*,
///     validation::{AttributeNamespace, Element, Normal},
/// };
///
/// struct my_element;
///
/// impl Element for my_element {
///     type Kind = Normal;
/// }
///
/// impl my_element {
///     pub const my_ns: AttributeNamespace = AttributeNamespace;
/// }
///
/// assert_eq!(
///     maud! {
///         my-element my-ns:stuff="ns value" { "content" }
///     }
///     .render()
///     .as_inner(),
///     r#"<my-element my-ns:stuff="ns value">content</my-element>"#
/// );
/// ```
#[derive(Debug, Clone, Copy)]
pub struct AttributeNamespace;

/// An attribute prefixed by a symbol.
///
/// During type-checking, if an attribute starts with any of the symbols listed
/// below, it will be validated using the corresponding identifier, and the rest
/// of the attribute will not be validated.
///
/// - `@` (`_at`)
/// - `:` (`_colon`)
///
/// # Example
///
/// ```
/// use hypertext::{
///     prelude::*,
///     validation::{AttributeSymbol, Element, Normal},
/// };
///
/// struct my_element;
///
/// impl Element for my_element {
///     type Kind = Normal;
/// }
///
/// impl my_element {
///     pub const _at: AttributeSymbol = AttributeSymbol;
///     pub const _colon: AttributeSymbol = AttributeSymbol;
/// }
///
/// assert_eq!(
///     maud! {
///         my-element @other-stuff="value" :data="more value" { "content" }
///     }
///     .render()
///     .as_inner(),
///     r#"<my-element @other-stuff="value" :data="more value">content</my-element>"#
/// );
/// ```
#[derive(Debug, Clone, Copy)]
pub struct AttributeSymbol;

/// Define custom elements.
///
/// This macro should be called from within a module named `hypertext_elements`.
///
/// # Example
///
/// ```
/// mod hypertext_elements {
///     use hypertext::define_elements;
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
                reason = "camel case types will be interpreted as renderable structs"
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

/// Define custom void elements.
///
/// This macro should be called from within a module named `hypertext_elements`.
///
/// # Example
/// ```
/// mod hypertext_elements {
///     // Re-export all standard HTML elements
///     use hypertext::define_void_elements;
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
                reason = "camel case types will be interpreted as renderable structs"
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
