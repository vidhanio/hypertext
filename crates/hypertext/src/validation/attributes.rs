//! Attribute traits.
//!
//! The main attribute trait is [`GlobalAttributes`], which provides access to
//! standard HTML attributes such as [`class`](GlobalAttributes::class) and
//! [`id`](GlobalAttributes::id). This trait is implemented by every HTML
//! element specified in
//! [`hypertext_elements`](crate::validation::hypertext_elements).
//!
//! In addition to the standard HTML attributes, this crate provides support for
//! attributes for popular frontend libraries through framework-specific traits.
//! To use these, you need to enable the corresponding feature in your
//! `Cargo.toml` file. For example, to use [`HtmxAttributes`], you would enable
//! the `htmx` feature:
//!
//! ```toml
//! [dependencies]
//! hypertext = { version = "*", features = ["htmx"] }
//! ```
//!
//! Then you can use the attributes in your code after bringing the trait into
//! scope via [`prelude::*`](crate::prelude):
//!
//! ```
//! # #[cfg(feature = "htmx")] {
//! use hypertext::prelude::*;
//!
//! # assert_eq!(
//! maud! {
//!     a hx-get="/about" { "About" }
//! }
//! # .render().as_inner(), r#"<a hx-get="/about">About</a>"#);
//! # }
//! ```
//!
//! It is also easy to define your own attributes for use with your favourite
//! frontend library if it isn't supported by this crate.
//!
//! ```
//! use hypertext::{prelude::*, validation::Attribute};
//!
//! trait MyLibraryAttributes: GlobalAttributes {
//!     const my_attr: Attribute = Attribute;
//!     const my_other_attr: Attribute = Attribute;
//!     // ...
//! }
//!
//! impl<T: GlobalAttributes> MyLibraryAttributes for T {}
//!
//! assert_eq!(
//!     maud! {
//!         // note that it converts `-` to `_` for you when validating attributes
//!         div my-attr="value" my-other-attr="other value" {
//!             "Hello, world!"
//!         }
//!     }
//!     .render()
//!     .as_inner(),
//!     r#"<div my-attr="value" my-other-attr="other value">Hello, world!</div>"#,
//! );
//! ```
//!
//! If the framework is widely used, consider contributing the trait to this
//! crate so that others can use it too!
#![expect(non_upper_case_globals)]
use crate::validation::{Attribute, AttributeNamespace, AttributeSymbol, Element};

/// Global HTML attributes.
///
/// This trait must be in scope to use standard HTML attributes such as
/// [`class`](Self::class) and [`id`](Self::id). This trait is implemented
/// by every HTML element specified in
/// [`hypertext_elements`](crate::validation::hypertext_elements).
///
/// # Usage With Custom Elements
///
/// ```
/// use hypertext::prelude::*;
///
/// mod hypertext_elements {
///     #![expect(non_camel_case_types)]
///
///     pub use hypertext::validation::hypertext_elements::*;
///     use hypertext::validation::{Element, Normal, attributes::GlobalAttributes};
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
///     maud! { custom-element #my-element title="abc" { "Hello, world!" } }
///         .render()
///         .as_inner(),
///     r#"<custom-element id="my-element" title="abc">Hello, world!</custom-element>"#
/// );
/// ```
///
/// The [`define_elements!`](crate::define_elements) and
/// [`define_void_elements!`](crate::define_void_elements) macros simplify this
/// process.
pub trait GlobalAttributes: Element {
    /// Used as a guide for creating a keyboard shortcut that activates or
    /// focuses the element.
    const access_key: Attribute = Attribute;

    /// The autocapitalization behavior to use when the text is edited
    /// through non-keyboard methods.
    const autocapitalize: Attribute = Attribute;

    /// Indicates whether the element should be automatically focused when
    /// the page is loaded.
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

    /// Specifies what kind of input mechanism would be most helpful for
    /// users entering content.
    const inputmode: Attribute = Attribute;

    /// Specify which element this is a custom variant of.
    const is: Attribute = Attribute;

    /// A global identifier for the item.
    const itemid: Attribute = Attribute;

    /// A property that the item has.
    const itemprop: Attribute = Attribute;

    /// A list of additional elements to crawl to find the name-value pairs
    /// of the item.
    const itemref: Attribute = Attribute;

    /// Creates a new item, a group of name-value pairs.
    const itemscope: Attribute = Attribute;

    /// The item types of the item.
    const itemtype: Attribute = Attribute;

    /// The language of the element.
    const lang: Attribute = Attribute;

    /// A cryptographic nonce ("number used once") which can be used by
    /// Content Security Policy to determine whether or not a given
    /// fetch will be allowed to proceed.
    const nonce: Attribute = Attribute;

    /// When specified, the element won't be rendered until it becomes
    /// shown, at which point it will be rendered on top of other
    /// page content.
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

/// [ARIA](https://www.w3.org/TR/wai-aria/) attributes.
#[expect(missing_docs)]
pub trait AriaAttributes: GlobalAttributes {
    const role: Attribute = Attribute;
    const aria_activedescendant: Attribute = Attribute;
    const aria_atomic: Attribute = Attribute;
    const aria_autocomplete: Attribute = Attribute;
    const aria_braillelabel: Attribute = Attribute;
    const aria_brailleroledescription: Attribute = Attribute;
    const aria_busy: Attribute = Attribute;
    const aria_checked: Attribute = Attribute;
    const aria_colcount: Attribute = Attribute;
    const aria_colindex: Attribute = Attribute;
    const aria_colindextext: Attribute = Attribute;
    const aria_colspan: Attribute = Attribute;
    const aria_controls: Attribute = Attribute;
    const aria_current: Attribute = Attribute;
    const aria_describedby: Attribute = Attribute;
    const aria_description: Attribute = Attribute;
    const aria_details: Attribute = Attribute;
    const aria_disabled: Attribute = Attribute;
    const aria_dropeffect: Attribute = Attribute;
    const aria_errormessage: Attribute = Attribute;
    const aria_expanded: Attribute = Attribute;
    const aria_flowto: Attribute = Attribute;
    const aria_grabbed: Attribute = Attribute;
    const aria_haspopup: Attribute = Attribute;
    const aria_hidden: Attribute = Attribute;
    const aria_invalid: Attribute = Attribute;
    const aria_keyshortcuts: Attribute = Attribute;
    const aria_label: Attribute = Attribute;
    const aria_labelledby: Attribute = Attribute;
    const aria_level: Attribute = Attribute;
    const aria_live: Attribute = Attribute;
    const aria_modal: Attribute = Attribute;
    const aria_multiline: Attribute = Attribute;
    const aria_multiselectable: Attribute = Attribute;
    const aria_orientation: Attribute = Attribute;
    const aria_owns: Attribute = Attribute;
    const aria_placeholder: Attribute = Attribute;
    const aria_posinset: Attribute = Attribute;
    const aria_pressed: Attribute = Attribute;
    const aria_readonly: Attribute = Attribute;
    const aria_relevant: Attribute = Attribute;
    const aria_required: Attribute = Attribute;
    const aria_roledescription: Attribute = Attribute;
    const aria_rowcount: Attribute = Attribute;
    const aria_rowindex: Attribute = Attribute;
    const aria_rowindextext: Attribute = Attribute;
    const aria_rowspan: Attribute = Attribute;
    const aria_selected: Attribute = Attribute;
    const aria_setsize: Attribute = Attribute;
    const aria_sort: Attribute = Attribute;
    const aria_valuemax: Attribute = Attribute;
    const aria_valuemin: Attribute = Attribute;
    const aria_valuenow: Attribute = Attribute;
    const aria_valuetext: Attribute = Attribute;
}

impl<T: GlobalAttributes> AriaAttributes for T {}

/// Event handler attributes.
#[expect(missing_docs)]
pub trait EventHandlerAttributes: GlobalAttributes {
    const onabort: Attribute = Attribute;
    const onautocomplete: Attribute = Attribute;
    const onautocompleteerror: Attribute = Attribute;
    const onblur: Attribute = Attribute;
    const oncancel: Attribute = Attribute;
    const oncanplay: Attribute = Attribute;
    const oncanplaythrough: Attribute = Attribute;
    const onchange: Attribute = Attribute;
    const onclick: Attribute = Attribute;
    const onclose: Attribute = Attribute;
    const oncontextmenu: Attribute = Attribute;
    const oncuechange: Attribute = Attribute;
    const ondblclick: Attribute = Attribute;
    const ondrag: Attribute = Attribute;
    const ondragend: Attribute = Attribute;
    const ondragenter: Attribute = Attribute;
    const ondragleave: Attribute = Attribute;
    const ondragover: Attribute = Attribute;
    const ondragstart: Attribute = Attribute;
    const ondrop: Attribute = Attribute;
    const ondurationchange: Attribute = Attribute;
    const onemptied: Attribute = Attribute;
    const onended: Attribute = Attribute;
    const onerror: Attribute = Attribute;
    const onfocus: Attribute = Attribute;
    const oninput: Attribute = Attribute;
    const oninvalid: Attribute = Attribute;
    const onkeydown: Attribute = Attribute;
    const onkeypress: Attribute = Attribute;
    const onkeyup: Attribute = Attribute;
    const onload: Attribute = Attribute;
    const onloadeddata: Attribute = Attribute;
    const onloadedmetadata: Attribute = Attribute;
    const onloadstart: Attribute = Attribute;
    const onmousedown: Attribute = Attribute;
    const onmouseenter: Attribute = Attribute;
    const onmouseleave: Attribute = Attribute;
    const onmousemove: Attribute = Attribute;
    const onmouseout: Attribute = Attribute;
    const onmouseover: Attribute = Attribute;
    const onmouseup: Attribute = Attribute;
    const onmousewheel: Attribute = Attribute;
    const onpause: Attribute = Attribute;
    const onplay: Attribute = Attribute;
    const onplaying: Attribute = Attribute;
    const onprogress: Attribute = Attribute;
    const onratechange: Attribute = Attribute;
    const onreset: Attribute = Attribute;
    const onresize: Attribute = Attribute;
    const onscroll: Attribute = Attribute;
    const onseeked: Attribute = Attribute;
    const onseeking: Attribute = Attribute;
    const onselect: Attribute = Attribute;
    const onshow: Attribute = Attribute;
    const onsort: Attribute = Attribute;
    const onstalled: Attribute = Attribute;
    const onsubmit: Attribute = Attribute;
    const onsuspend: Attribute = Attribute;
    const ontimeupdate: Attribute = Attribute;
    const ontoggle: Attribute = Attribute;
    const onvolumechange: Attribute = Attribute;
    const onwaiting: Attribute = Attribute;
}

impl<T: GlobalAttributes> EventHandlerAttributes for T {}

/// Attributes for use with [htmx](https://htmx.org/).
pub trait HtmxAttributes: GlobalAttributes {
    /// Issues a `GET` to the specified URL
    const hx_get: Attribute = Attribute;

    /// Issues a `POST` to the specified URL
    const hx_post: Attribute = Attribute;

    /// Handle events with inline scripts on elements
    const hx_on: AttributeNamespace = AttributeNamespace;

    /// Push a URL into the browser location bar to create history
    const hx_push_url: Attribute = Attribute;

    /// Select content to swap in from a response
    const hx_select: Attribute = Attribute;

    /// Select content to swap in from a response, somewhere other than the
    /// target (out of band)
    const hx_select_oob: Attribute = Attribute;

    /// Controls how content will swap in (`outerHTML`, `beforeend`,
    /// `afterend`, …)
    const hx_swap: Attribute = Attribute;

    /// Mark element to swap in from a response (out of band)
    const hx_swap_oob: Attribute = Attribute;

    /// Specifies the target element to be swapped
    const hx_target: Attribute = Attribute;

    /// Specifies the event that triggers the request
    const hx_trigger: Attribute = Attribute;

    /// Add values to submit with the request (JSON format)
    const hx_vals: Attribute = Attribute;

    /// Add [progressive enhancement](https://en.wikipedia.org/wiki/Progressive_enhancement) for links and forms
    const hx_boost: Attribute = Attribute;

    /// Shows a `confirm()` dialog before issuing a request
    const hx_confirm: Attribute = Attribute;

    /// Issues a `DELETE` to the specified URL
    const hx_delete: Attribute = Attribute;
    /// Disables htmx processing for the given node and any children nodes
    const hx_disable: Attribute = Attribute;

    /// Adds the `disabled` attribute to the specified elements while a
    /// request is in flight
    const hx_disabled_elt: Attribute = Attribute;

    /// Control and disable automatic attribute inheritance for child nodes
    const hx_disinherit: Attribute = Attribute;

    /// Changes the request encoding type
    const hx_encoding: Attribute = Attribute;

    /// Extensions to use for this element
    const hx_ext: Attribute = Attribute;
    /// Adds to the headers that will be submitted with the request
    const hx_headers: Attribute = Attribute;

    /// Prevent sensitive data being saved to the history cache
    const hx_history: Attribute = Attribute;

    /// The element to snapshot and restore during history navigation
    const hx_history_elt: Attribute = Attribute;

    /// Include additional data in requests
    const hx_include: Attribute = Attribute;

    /// The element to put the `htmx-request` class on during the request
    const hx_indicator: Attribute = Attribute;
    /// Control and enable automatic attribute inheritance for child nodes
    /// if it has been disabled by default
    const hx_inherit: Attribute = Attribute;

    /// Filters the parameters that will be submitted with a request
    const hx_params: Attribute = Attribute;

    /// Issues a `PATCH` to the specified URL
    const hx_patch: Attribute = Attribute;

    /// Specifies elements to keep unchanged between requests
    const hx_preserve: Attribute = Attribute;

    /// Shows a `prompt()` before submitting a request
    const hx_prompt: Attribute = Attribute;

    /// Issues a `PUT` to the specified URL
    const hx_put: Attribute = Attribute;

    /// Replace the URL in the browser location bar
    const hx_replace_url: Attribute = Attribute;

    /// Configures various aspects of the request
    const hx_request: Attribute = Attribute;

    /// Control how requests made by different elements are synchronized
    const hx_sync: Attribute = Attribute;

    /// Force elements to validate themselves before a request
    const hx_validate: Attribute = Attribute;

    /// Adds values dynamically to the parameters to submit with the request
    /// (deprecated, please use [`hx-vals`](Self::hx_vals))
    #[deprecated = "use `hx-vals` instead"]
    const hx_vars: Attribute = Attribute;
}

impl<T: GlobalAttributes> HtmxAttributes for T {}

/// Attributes for use with [hyperscript](https://hyperscript.org/).
pub trait HyperscriptAttributes: GlobalAttributes {
    /// The `_` hyperscript tag
    const __: Attribute = Attribute;
}

impl<T: GlobalAttributes> HyperscriptAttributes for T {}

/// Attributes for use with [Alpine.js](https://alpinejs.dev/).
pub trait AlpineJsAttributes: GlobalAttributes {
    /// Declare a new Alpine component and its data for a block of HTML
    const x_data: Attribute = Attribute;

    /// Dynamically set HTML attributes on an element
    const x_bind: AttributeNamespace = AttributeNamespace;

    /// Dynamically set HTML attributes on an element
    ///
    /// Shorthand for [`x-bind`](Self::x_bind).
    #[doc(alias = ":")]
    const _colon: AttributeSymbol = AttributeSymbol;

    /// Listen for browser events on an element
    const x_on: AttributeNamespace = AttributeNamespace;

    /// Listen for browser events on an element
    ///
    /// Shorthand for [`x-on`](Self::x_on).
    #[doc(alias = "@")]
    const _at: AttributeSymbol = AttributeSymbol;

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

    /// Execute a script each time const one: Attribute = Attribute; of its
    /// dependencies change
    const x_effect: Attribute = Attribute;

    /// Reference elements directly by their specified keys using the $refs
    /// magic property
    const x_ref: Attribute = Attribute;

    /// Hide a block of HTML until after Alpine is finished initializing its
    /// contents
    const x_cloak: Attribute = Attribute;

    /// Prevent a block of HTML from being initialized by Alpine
    const x_ignore: Attribute = Attribute;
}

impl<T: GlobalAttributes> AlpineJsAttributes for T {}

#[expect(missing_docs)]
pub trait MathMlGlobalAttributes: Element {
    const autofocus: Attribute = Attribute;

    #[doc(alias = ".")]
    const class: Attribute = Attribute;

    const dir: Attribute = Attribute;

    const displaystyle: Attribute = Attribute;

    #[doc(alias = "#")]
    const id: Attribute = Attribute;

    const mathbackground: Attribute = Attribute;

    const mathcolor: Attribute = Attribute;

    const mathsize: Attribute = Attribute;

    const nonce: Attribute = Attribute;

    const scriptlevel: Attribute = Attribute;

    const style: Attribute = Attribute;

    const tabindex: Attribute = Attribute;
}
