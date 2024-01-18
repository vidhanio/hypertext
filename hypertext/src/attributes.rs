/// An HTML attribute.
#[derive(Debug, Clone, Copy)]
pub struct Attribute;

/// Global HTML attributes.
///
/// This trait must be in scope to use well-known attributes such as
/// [`class`](Self::class) and [`id`](Self::id). This trait is implemented
/// by every HTML element specified in [`crate::html_elements`].
///
/// # Usage With Custom Elements
///
/// ```
/// use hypertext::{maud, GlobalAttributes, Renderable}; // `GlobalAttributes` must be in scope!
///
/// mod html_elements {
///     #![allow(non_camel_case_types)]
///
///     pub use hypertext::html_elements::*;
///     use hypertext::GlobalAttributes;
///
///     pub struct custom_element;
///
///     impl GlobalAttributes for custom_element {}
/// }
///
/// assert_eq!(
///     maud! { custom-element title="abc" { "Hello, world!" } }.render(),
///     r#"<custom-element title="abc">Hello, world!</custom-element>"#,
/// );
/// ```
#[allow(non_upper_case_globals, clippy::module_name_repetitions)]
pub trait GlobalAttributes {
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

/// Global SVG attributes.
///
/// This trait must be in scope to use well-known attributes such as
/// [`class`](Self::class) and [`id`](Self::id). This trait is implemented
/// by every SVG element specified in [`crate::svg_elements`].
///
#[cfg(feature = "svg")]
#[allow(non_upper_case_globals, clippy::module_name_repetitions)]
pub trait GlobalSVGAttributes {
    /// The XML namespace, only required on the outermost SVG element.
    const xmlns: Attribute = Attribute;

    /// The class of the element.
    const class: Attribute = Attribute;

    /// A unique identifier for the element.
    const id: Attribute = Attribute;

    /// The language of the element.
    const lang: Attribute = Attribute;

    /// The CSS styling to apply to the element.
    const style: Attribute = Attribute;

    /// Customize the index of the element for sequential focus navigation.
    const tabindex: Attribute = Attribute;
}

/// Presentation SVG attributes.
///
/// This trait must be in scope to use presentation attributes such as
/// [`fill`](Self::fill) and [`stroke`](Self::stroke).
#[cfg(feature = "svg")]
#[allow(non_upper_case_globals, clippy::module_name_repetitions)]
pub trait PresentationSVGAttributes {

    /// Defines or associates a clipping path with the element.
    const clip_path: Attribute = Attribute;

    /// Works as the fill-rule attribute, but applies to clipPath defenitions.
    /// Only applies to graphics elements contained within a clipPath element.
    const clip_rule: Attribute = Attribute;

    /// Provides a potential indirect value for the fill, stroke, stop-color, flood-color and
    /// lighting-color attributes.
    const color: Attribute = Attribute;

    /// Specifies color space for gradient interpolations, color animations and alpha compositing.
    const color_interpolation: Attribute = Attribute;

    /// Lets you control the rendering of graphical or container elements.
    const display: Attribute = Attribute;

    /// Defines the color for shapes or text, or the final state of an animation.
    const fill: Attribute = Attribute;

    /// Defines the opacity of the paint server (color, gradient, pattern, ..) of a shape.
    const fill_opacity: Attribute = Attribute;

    /// Defines the algorithm to use to determine the *inside* part of a shape.
    const fill_rule: Attribute = Attribute;

    /// Specifies the filter effects defined by the filter element that shall be applied..
    const filter: Attribute = Attribute;

    /// Mainly used to bind a given mask element with the element the attribute belongs to.
    const mask: Attribute = Attribute;

    /// Specifies the transparency of an object or a group of objects.
    const opacity: Attribute = Attribute;

    /// Provides hints to the renderer about what tradeoffs to make when rendering shapes.
    const shape_rendering: Attribute = Attribute;

    /// Defines the color, gradient or pattern used to paint the outline of the shape.
    const stroke: Attribute = Attribute;

    /// Defines the pattern of dashes and gaps used to paint the outline of the shape.
    const stroke_dasharray: Attribute = Attribute;

    /// Defines the offset on the rendering of a dash array.
    const stroke_dashoffset: Attribute = Attribute;

    /// Defines the shape to be used at the end of open subpaths.
    const stroke_linecap: Attribute = Attribute;

    /// Defines the shape to be used at the corners of paths when they are stroked.
    const stroke_linejoin: Attribute = Attribute;

    /// Defines the limit on the ratio of the miter length to the stroke-width used to draw a miter
    /// join. When the limit is exceeded, the join is converted from a miter to a bevel.
    const stroke_miterlimit: Attribute = Attribute;

    /// Defines the opacity of the paint server (color, gradient, pattern, ..) of a shape.
    const stroke_opacity: Attribute = Attribute;

    /// Defines the width of the stroke to be applied to the shape.
    const stroke_width: Attribute = Attribute;

    /// Defines a list of transform definitions that are applied to an element and its children.
    const transform: Attribute = Attribute;
    
    /// Specifies the vector effect to use when drawing an object.
    const vector_effect: Attribute = Attribute;

    /// Lets you control the visibility of graphical elements.
    const visibility: Attribute = Attribute;
}
