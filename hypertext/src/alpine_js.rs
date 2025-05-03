#[cfg(feature = "alpine-js")]
use crate::{Attribute, AttributeNamespace, GlobalAttributes};

/// Attributes for AlpineJs elements.
///
/// [AlpineJs Reference](https://alpinejs.dev/)
#[allow(non_upper_case_globals, clippy::doc_markdown)]
pub trait AlpineJsAttributes: GlobalAttributes {
    /// Declare a new Alpine component and its data for a block of HTML
    const x_data: Attribute = Attribute;
    /// Dynamically set HTML attributes on an element
    const x_bind: AttributeNamespace = AttributeNamespace;
    /// Listen for browser events on an element
    const x_on: AttributeNamespace = AttributeNamespace;
    /// Set the text content of an element
    const x_text: Attribute = Attribute;
    /// Set the inner HTML of an element
    const x_html: Attribute = Attribute;
    /// Synchronize a piece of data with an input element
    const x_model: Attribute = Attribute;
    /// Toggle the visibility of an element
    const x_show: Attribute = Attribute;
    /// Transition an element in and out using CSS transitions
    const x_transition: Attribute = Attribute;
    /// Repeat a block of HTML based on a data set
    const x_for: Attribute = Attribute;
    /// Conditionally add/remove a block of HTML from the page entirely
    const x_if: Attribute = Attribute;
    /// Run code when an element is initialized by Alpine
    const x_init: Attribute = Attribute;
    /// Execute a script each time one of its dependencies change
    const x_effect: Attribute = Attribute;
    /// Reference elements directly by their specified keys using the $refs magic property
    const x_ref: Attribute = Attribute;
    /// Hide a block of HTML until after Alpine is finished initializing its contents
    const x_cloak: Attribute = Attribute;
    /// Prevent a block of HTML from being initialized by Alpine
    const x_ignore: Attribute = Attribute;
}

impl<T: GlobalAttributes> AlpineJsAttributes for T {}
