//! SVG element definitions.
//!
//! This module provides type-checked SVG element definitions for use with
//! [`svg::maud!`](crate::svg::maud!) and [`svg::rsx!`](crate::svg::rsx!)
//! macros. Each element is defined as a unit struct that implements
//! [`Element<Kind = Xml>`](super::Element) and
//! [`SvgGlobalAttributes`](super::attributes::SvgGlobalAttributes).
//!
//! To add custom SVG elements, create a module named `hypertext_svg_elements`
//! that re-exports this module's contents and adds your own definitions:
//!
//! ```
//! mod hypertext_svg_elements {
//!     use hypertext::define_svg_elements;
//!     pub use hypertext::validation::hypertext_svg_elements::*;
//!
//!     define_svg_elements! {
//!         /// A custom SVG element.
//!         my_custom_element {
//!             /// A custom attribute.
//!             my_attr
//!         }
//!     }
//! }
//! ```
use crate::define_svg_elements;

define_svg_elements! {
    /// A hyperlink to other web pages, files, locations in the same page,
    /// or email addresses.
    a {
        /// The URL the hyperlink points to.
        href

        /// Where to display the linked URL.
        target

        /// The URL of the resource as a compatible replacement for `href`.
        #[deprecated = "use `href` instead"]
        xlink_href

        /// The MIME type of the linked resource.
        r#type

        /// A human-readable title for the linked resource.
        download

        /// The ping URL list.
        ping

        /// The relationship between the current document and the linked
        /// URL.
        rel

        /// The language of the linked resource.
        hreflang

        /// Referrer policy for fetches initiated by the element.
        referrerpolicy
    }

    /// A container used to group other SVG elements. Transformations
    /// applied to the `<g>` element are performed on its child elements.
    g

    /// A container for storing graphical objects that will be used at a
    /// later time. Objects created inside a `<defs>` element are not
    /// rendered directly.
    defs

    /// A container that defines a new coordinate system and viewport.
    svg {
        /// The displayed width of the rectangular viewport.
        width

        /// The displayed height of the rectangular viewport.
        height

        /// The `x` coordinate of the SVG container.
        x

        /// The `y` coordinate of the SVG container.
        y

        /// The SVG viewport coordinates.
        viewBox

        /// How the SVG fragment must be deformed if displayed with a
        /// different aspect ratio.
        preserveAspectRatio
    }

    /// A symbol element defines graphical template objects that can be
    /// instantiated by a `<use>` element. Used for reusable graphics.
    symbol {
        /// The displayed width of the symbol viewport.
        width

        /// The displayed height of the symbol viewport.
        height

        /// The `x` coordinate of the symbol.
        x

        /// The `y` coordinate of the symbol.
        y

        /// The SVG viewport coordinates for the symbol.
        viewBox

        /// How the symbol must be deformed if displayed with a
        /// different aspect ratio.
        preserveAspectRatio

        /// The `refX` coordinate of the symbol.
        refX

        /// The `refY` coordinate of the symbol.
        refY
    }

    /// References another element and indicates that its children shall be
    /// rendered at the given point. It has the same effect as cloning the
    /// referenced element and applying transformations to it.
    r#use {
        /// A URL reference to an element/fragment.
        href

        /// The `x` coordinate of the element.
        x

        /// The `y` coordinate of the element.
        y

        /// The width of the element.
        width

        /// The height of the element.
        height

        /// The URL of the resource as a compatible replacement for `href`.
        #[deprecated = "use `href` instead"]
        xlink_href
    }

    /// A container for conditionally processing or rendering SVG elements
    /// based on the evaluation of `requiredExtensions` and
    /// `systemLanguage` attributes.
    switch

    /// An accessible, long-text description of any SVG container or
    /// graphics element.
    desc

    /// A container for metadata information. The content should be
    /// structured data from other XML namespaces.
    metadata

    /// A human-readable title for the element. Only the first child
    /// `<title>` of each container or graphics element is used.
    title

    /// Draws circles based on a center point and a radius.
    circle {
        /// The x-axis coordinate of the center of the circle.
        cx

        /// The y-axis coordinate of the center of the circle.
        cy

        /// The radius of the circle.
        r

        /// The total length for the circle's circumference, in user
        /// units.
        pathLength
    }

    /// Draws ellipses based on a center coordinate and both their x and y
    /// radius.
    ellipse {
        /// The x-axis coordinate of the center of the ellipse.
        cx

        /// The y-axis coordinate of the center of the ellipse.
        cy

        /// The radius of the ellipse on the x axis.
        rx

        /// The radius of the ellipse on the y axis.
        ry

        /// The total length for the ellipse's circumference, in user
        /// units.
        pathLength
    }

    /// A basic shape used to create a line connecting two points.
    line {
        /// The x-axis coordinate of the line starting point.
        x1

        /// The y-axis coordinate of the line starting point.
        y1

        /// The x-axis coordinate of the line ending point.
        x2

        /// The y-axis coordinate of the line ending point.
        y2

        /// The total path length in user units.
        pathLength
    }

    /// Defines a closed shape consisting of a set of connected straight
    /// line segments. The last point is connected to the first point.
    polygon {
        /// The list of points defining the polygon.
        points

        /// The total path length in user units.
        pathLength
    }

    /// An element creating a shape consisting of a set of connected
    /// straight line segments. Unlike `<polygon>`, the last point is not
    /// connected to the first point.
    polyline {
        /// The list of points defining the polyline.
        points

        /// The total path length in user units.
        pathLength
    }

    /// Draws rectangles, defined by their position, width, and height.
    /// Corners may optionally be rounded.
    rect {
        /// The x coordinate of the rect.
        x

        /// The y coordinate of the rect.
        y

        /// The width of the rect.
        width

        /// The height of the rect.
        height

        /// The horizontal corner radius of the rect.
        rx

        /// The vertical corner radius of the rect.
        ry

        /// The total length of the rectangle's perimeter, in user units.
        pathLength
    }

    /// The generic element to define a shape. All basic shapes can be
    /// created with a `path` element.
    path {
        /// The definition of the shape of the path, specified as a string
        /// of [path commands](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d#path_commands).
        d

        /// The total path length in user units.
        pathLength
    }


    /// Draws text. It can be positioned as a whole, and glyphs can be
    /// individually positioned via attributes and CSS properties.
    text {
        /// The x position of the text.
        x

        /// The y position of the text.
        y

        /// Shifts the text position horizontally from a previous text
        /// element.
        dx

        /// Shifts the text position vertically from a previous text
        /// element.
        dy

        /// Rotates orientation of each individual glyph.
        rotate

        /// How the length of the text is determined.
        lengthAdjust

        /// A target length for the text.
        textLength
    }

    /// Renders text along the shape of a `<path>` element, allowing
    /// text to follow curves.
    textPath {
        /// A URL reference to the path element.
        href

        /// How the length of the text is determined.
        lengthAdjust

        /// Which method to use to render individual glyphs along the path.
        method

        /// Where on the path the text should start being rendered.
        startOffset

        /// The spacing between glyphs along the path.
        spacing

        /// Which side of the path the text should be placed on.
        side

        /// A target length for the text.
        textLength

        /// An offset from the start of the path.
        path

        /// The URL of the resource as a compatible replacement for `href`.
        #[deprecated = "use `href` instead"]
        xlink_href
    }

    /// A positioning and styling element for text within `<text>` and
    /// other `<tspan>` elements.
    tspan {
        /// The x position of the text span.
        x

        /// The y position of the text span.
        y

        /// Shifts the text position horizontally.
        dx

        /// Shifts the text position vertically.
        dy

        /// Rotates orientation of each individual glyph.
        rotate

        /// How the length of the text is determined.
        lengthAdjust

        /// A target length for the text.
        textLength
    }

    /// Defines a linear gradient used to fill or stroke graphical
    /// elements.
    linearGradient {
        /// The x-axis coordinate of the start of the gradient vector.
        x1

        /// The y-axis coordinate of the start of the gradient vector.
        y1

        /// The x-axis coordinate of the end of the gradient vector.
        x2

        /// The y-axis coordinate of the end of the gradient vector.
        y2

        /// The coordinate system for the gradient vector.
        gradientUnits

        /// An additional transformation applied to the gradient
        /// coordinate system.
        gradientTransform

        /// The method used to fill a shape beyond the gradient vector's
        /// bounds.
        spreadMethod

        /// A URL reference to a template gradient element.
        href

        /// The URL of the resource as a compatible replacement for `href`.
        #[deprecated = "use `href` instead"]
        xlink_href
    }

    /// Defines a radial gradient used to fill or stroke graphical
    /// elements.
    radialGradient {
        /// The x-axis coordinate of the end circle of the radial gradient.
        cx

        /// The y-axis coordinate of the end circle of the radial gradient.
        cy

        /// The radius of the end circle of the radial gradient.
        r

        /// The x-axis coordinate of the start circle of the radial
        /// gradient.
        fx

        /// The y-axis coordinate of the start circle of the radial
        /// gradient.
        fy

        /// The radius of the start circle of the radial gradient.
        fr

        /// The coordinate system for the gradient geometry.
        gradientUnits

        /// An additional transformation applied to the gradient
        /// coordinate system.
        gradientTransform

        /// The method used to fill a shape beyond the gradient's bounds.
        spreadMethod

        /// A URL reference to a template gradient element.
        href

        /// The URL of the resource as a compatible replacement for `href`.
        #[deprecated = "use `href` instead"]
        xlink_href
    }

    /// Defines a color and its position in a gradient. This element is
    /// always a child of a `<linearGradient>` or `<radialGradient>`.
    stop {
        /// Where the gradient stop is placed along the gradient vector.
        offset
    }

    /// Defines a clipping path. It restricts the region to which paint
    /// can be applied.
    clipPath {
        /// The coordinate system for the contents of the `<clipPath>`.
        clipPathUnits
    }

    /// Defines an alpha mask for compositing the current object into the
    /// background.
    mask {
        /// The x coordinate of the mask area.
        x

        /// The y coordinate of the mask area.
        y

        /// The width of the mask area.
        width

        /// The height of the mask area.
        height

        /// The coordinate system for the mask's geometry attributes.
        maskUnits

        /// The coordinate system for the contents of the `<mask>`.
        maskContentUnits
    }

    /// Defines a graphical object to be drawn at given positions along a
    /// `<path>`, `<line>`, `<polyline>`, or `<polygon>` element.
    marker {
        /// The width of the marker viewport.
        markerWidth

        /// The height of the marker viewport.
        markerHeight

        /// The coordinate system for the marker's `markerWidth` and
        /// `markerHeight` attributes.
        markerUnits

        /// The orientation of the marker relative to the shape it is
        /// attached to.
        orient

        /// The x coordinate of the marker's reference point.
        refX

        /// The y coordinate of the marker's reference point.
        refY

        /// The SVG viewport coordinates for the marker.
        viewBox

        /// How the marker must be deformed if displayed with a
        /// different aspect ratio.
        preserveAspectRatio
    }

    /// Defines a pattern used to fill or stroke graphical elements.
    pattern {
        /// The x coordinate of the pattern tile.
        x

        /// The y coordinate of the pattern tile.
        y

        /// The width of the pattern tile.
        width

        /// The height of the pattern tile.
        height

        /// The coordinate system for the pattern's geometry attributes.
        patternUnits

        /// The coordinate system for the contents of the pattern.
        patternContentUnits

        /// An additional transformation applied to the pattern
        /// coordinate system.
        patternTransform

        /// A URL reference to a template pattern element.
        href

        /// The SVG viewport coordinates for the pattern.
        viewBox

        /// How the pattern must be deformed if displayed with a
        /// different aspect ratio.
        preserveAspectRatio

        /// The URL of the resource as a compatible replacement for `href`.
        #[deprecated = "use `href` instead"]
        xlink_href
    }

    /// Defines a filter effect by grouping atomic filter primitives.
    filter {
        /// The x coordinate of the filter region.
        x

        /// The y coordinate of the filter region.
        y

        /// The width of the filter region.
        width

        /// The height of the filter region.
        height

        /// The coordinate system for the filter's geometry attributes.
        filterUnits

        /// The coordinate system for the filter primitives within.
        primitiveUnits
    }

    /// Composites two objects together using common blending modes.
    feBlend {
        /// The first input for the filter primitive.
        r#in

        /// The second input for the blending operation.
        in2

        /// The blending mode.
        mode

        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Changes the component values in each pixel based on a color
    /// matrix transformation.
    feColorMatrix {
        /// The first input for the filter primitive.
        r#in

        /// The type of matrix operation.
        r#type

        /// The values for the color matrix.
        values

        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Performs component-wise remapping of the input graphic's color
    /// channels using its child `feFuncR`, `feFuncG`, `feFuncB`, and
    /// `feFuncA` elements.
    feComponentTransfer {
        /// The first input for the filter primitive.
        r#in

        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Performs the combination of two input images using a compositing
    /// operation.
    feComposite {
        /// The first input for the filter primitive.
        r#in

        /// The second input for the compositing operation.
        in2

        /// The compositing operation to perform.
        operator

        /// Coefficient for the `arithmetic` operator.
        k1

        /// Coefficient for the `arithmetic` operator.
        k2

        /// Coefficient for the `arithmetic` operator.
        k3

        /// Coefficient for the `arithmetic` operator.
        k4

        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Applies a matrix convolution filter effect to an input image.
    feConvolveMatrix {
        /// The first input for the filter primitive.
        r#in

        /// The order of the kernel matrix.
        order

        /// The list of numbers making up the kernel matrix.
        kernelMatrix

        /// The divisor applied to the kernel sum.
        divisor

        /// A value added to each result pixel.
        bias

        /// The x position in the kernel to use as the target pixel.
        targetX

        /// The y position in the kernel to use as the target pixel.
        targetY

        /// How the input image is extended at the edges.
        edgeMode

        /// The intended distance in user space for dx and dy in the
        /// kernel.
        kernelUnitLength

        /// Whether to preserve the alpha channel.
        preserveAlpha

        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Lights an image using the alpha channel as a bump map. The
    /// resulting image is an RGBA opaque image based on the light color.
    feDiffuseLighting {
        /// The first input for the filter primitive.
        r#in

        /// The scale factor for the surface height.
        surfaceScale

        /// The diffuse reflection constant.
        diffuseConstant

        /// The intended distance in user space for dx and dy in the
        /// kernel.
        kernelUnitLength

        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Uses the pixel values from the image from `in2` to spatially
    /// displace the image from `in`.
    feDisplacementMap {
        /// The first input for the filter primitive.
        r#in

        /// The second input (displacement map).
        in2

        /// The scale factor for the displacement.
        scale

        /// Which color channel from `in2` to use for x displacement.
        xChannelSelector

        /// Which color channel from `in2` to use for y displacement.
        yChannelSelector

        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Defines a distant light source to be used within a lighting filter
    /// primitive.
    feDistantLight {
        /// The direction angle for the light source on the XY plane,
        /// clockwise from the x-axis.
        azimuth

        /// The direction angle for the light source from the XY plane
        /// toward the Z axis.
        elevation
    }

    /// Creates a drop shadow of the input image.
    feDropShadow {
        /// The first input for the filter primitive.
        r#in

        /// The x offset of the drop shadow.
        dx

        /// The y offset of the drop shadow.
        dy

        /// The standard deviation for the blur operation.
        stdDeviation

        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Fills the filter subregion with the color and opacity defined by
    /// `flood-color` and `flood-opacity`.
    feFlood {
        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Defines the transfer function for the alpha component of the input
    /// graphic of its parent `<feComponentTransfer>`.
    feFuncA {
        /// The type of component transfer function.
        r#type

        /// The lookup table for the transfer function.
        tableValues

        /// The slope of the linear function.
        slope

        /// The intercept of the linear function.
        intercept

        /// The amplitude of the gamma function.
        amplitude

        /// The exponent of the gamma function.
        exponent

        /// The offset of the gamma function.
        offset
    }

    /// Defines the transfer function for the blue component of the input
    /// graphic of its parent `<feComponentTransfer>`.
    feFuncB {
        /// The type of component transfer function.
        r#type

        /// The lookup table for the transfer function.
        tableValues

        /// The slope of the linear function.
        slope

        /// The intercept of the linear function.
        intercept

        /// The amplitude of the gamma function.
        amplitude

        /// The exponent of the gamma function.
        exponent

        /// The offset of the gamma function.
        offset
    }

    /// Defines the transfer function for the green component of the input
    /// graphic of its parent `<feComponentTransfer>`.
    feFuncG {
        /// The type of component transfer function.
        r#type

        /// The lookup table for the transfer function.
        tableValues

        /// The slope of the linear function.
        slope

        /// The intercept of the linear function.
        intercept

        /// The amplitude of the gamma function.
        amplitude

        /// The exponent of the gamma function.
        exponent

        /// The offset of the gamma function.
        offset
    }

    /// Defines the transfer function for the red component of the input
    /// graphic of its parent `<feComponentTransfer>`.
    feFuncR {
        /// The type of component transfer function.
        r#type

        /// The lookup table for the transfer function.
        tableValues

        /// The slope of the linear function.
        slope

        /// The intercept of the linear function.
        intercept

        /// The amplitude of the gamma function.
        amplitude

        /// The exponent of the gamma function.
        exponent

        /// The offset of the gamma function.
        offset
    }

    /// Blurs the input image by the amount specified in `stdDeviation`.
    feGaussianBlur {
        /// The first input for the filter primitive.
        r#in

        /// The standard deviation for the blur operation.
        stdDeviation

        /// How the input image is extended at the edges.
        edgeMode

        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Fetches image data from an external source and provides the pixel
    /// data as output.
    feImage {
        /// A URL reference to an image resource or element.
        href

        /// How the fetched image is fitted into the filter primitive
        /// subregion.
        preserveAspectRatio

        /// How the element handles crossorigin requests.
        crossorigin

        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result

        /// The URL of the resource as a compatible replacement for `href`.
        #[deprecated = "use `href` instead"]
        xlink_href
    }

    /// Composites input image layers on top of each other using the
    /// `<feMergeNode>` child elements.
    feMerge {
        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Represents a layer in a `<feMerge>` composite.
    feMergeNode {
        /// The input for this merge node.
        r#in
    }

    /// Used to erode or dilate the input image.
    feMorphology {
        /// The first input for the filter primitive.
        r#in

        /// Whether to erode or dilate the input image.
        operator

        /// The radius (or radii) for the morphology operation.
        radius

        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Offsets the input image. It is useful for creating drop shadow
    /// effects.
    feOffset {
        /// The first input for the filter primitive.
        r#in

        /// The amount to offset the input graphic along the x-axis.
        dx

        /// The amount to offset the input graphic along the y-axis.
        dy

        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Defines a point light source to be used within a lighting filter
    /// primitive.
    fePointLight {
        /// The x location of the light source in the coordinate system.
        x

        /// The y location of the light source in the coordinate system.
        y

        /// The z location of the light source in the coordinate system.
        z
    }

    /// Lights a source graphic using its alpha channel as a bump map,
    /// producing a specularly lit image.
    feSpecularLighting {
        /// The first input for the filter primitive.
        r#in

        /// The scale factor for the surface height.
        surfaceScale

        /// The specular reflection constant.
        specularConstant

        /// The exponent for the specular term.
        specularExponent

        /// The intended distance in user space for dx and dy in the
        /// kernel.
        kernelUnitLength

        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Defines a spot light source to be used within a lighting filter
    /// primitive.
    feSpotLight {
        /// The x location of the light source.
        x

        /// The y location of the light source.
        y

        /// The z location of the light source.
        z

        /// The x location of the point the light source is pointing at.
        pointsAtX

        /// The y location of the point the light source is pointing at.
        pointsAtY

        /// The z location of the point the light source is pointing at.
        pointsAtZ

        /// The exponent value controlling the focus of the light source.
        specularExponent

        /// The angle in degrees between the spot light axis and the
        /// cone.
        limitingConeAngle
    }

    /// Fills the filter primitive subregion with a repeated, tiled
    /// pattern of the input image.
    feTile {
        /// The first input for the filter primitive.
        r#in

        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Creates an image using the Perlin turbulence function, allowing
    /// synthesis of artificial textures like clouds or marble.
    feTurbulence {
        /// The base frequency for the noise function.
        baseFrequency

        /// The number of octaves for the noise function.
        numOctaves

        /// The starting number for the pseudo-random number generator.
        seed

        /// Whether the filter primitive should perform a noise or
        /// turbulence function.
        stitchTiles

        /// The type of turbulence function.
        r#type

        /// The x coordinate of the filter primitive subregion.
        x

        /// The y coordinate of the filter primitive subregion.
        y

        /// The width of the filter primitive subregion.
        width

        /// The height of the filter primitive subregion.
        height

        /// The assigned name for the filter primitive's result.
        result
    }

    /// Includes an image inside SVG documents. It can display raster
    /// image files or other SVG files.
    image {
        /// A URL reference to the image resource.
        href

        /// The x coordinate of the image.
        x

        /// The y coordinate of the image.
        y

        /// The width of the image.
        width

        /// The height of the image.
        height

        /// How the image should be fitted into the reference rectangle.
        preserveAspectRatio

        /// How the element handles crossorigin requests.
        crossorigin

        /// The image decoding hint.
        decoding

        /// The URL of the resource as a compatible replacement for `href`.
        #[deprecated = "use `href` instead"]
        xlink_href
    }

    /// Provides the ability to include foreign XML/HTML content within an
    /// SVG document. When used inside an SVG context (either embedded in
    /// HTML or in a standalone [`svg::maud!`](crate::svg::maud!) /
    /// [`svg::rsx!`](crate::svg::rsx!) macro), `foreignObject` switches its
    /// children back to HTML validation, allowing standard HTML elements
    /// inside SVG.
    ///
    /// # Example
    ///
    /// ```
    /// use hypertext::prelude::*;
    ///
    /// let result = maud! {
    ///     div {
    ///         svg width="200" height="200" {
    ///             circle cx="100" cy="100" r="50" fill="blue";
    ///             foreignObject x="25" y="75" width="150" height="50" {
    ///                 p { "Hello from HTML!" }
    ///             }
    ///         }
    ///     }
    /// }
    /// .render();
    ///
    /// assert_eq!(
    ///     result.as_inner(),
    ///     concat!(
    ///         r#"<div><svg width="200" height="200">"#,
    ///         r#"<circle cx="100" cy="100" r="50" fill="blue"/>"#,
    ///         r#"<foreignObject x="25" y="75" width="150" height="50">"#,
    ///         "<p>Hello from HTML!</p>",
    ///         "</foreignObject>",
    ///         "</svg></div>",
    ///     ),
    /// );
    /// ```
    foreignObject {
        /// The x coordinate of the foreignObject.
        x

        /// The y coordinate of the foreignObject.
        y

        /// The width of the foreignObject.
        width

        /// The height of the foreignObject.
        height
    }


    /// Provides a way to animate an attribute of an element over time.
    animate {
        /// The name of the CSS property or attribute to animate.
        attributeName

        /// A semicolon-separated list of values defining the animation
        /// sequence.
        values

        /// The starting value of the animation.
        from

        /// The ending value of the animation.
        to

        /// A relative offset value for the animation.
        by

        /// The begin time of the animation.
        begin

        /// The simple duration of the animation.
        dur

        /// The end time of the animation.
        end

        /// The minimum value of the animation.
        min

        /// The maximum value of the animation.
        max

        /// Whether and how to restart the animation.
        restart

        /// The number of times the animation should repeat.
        repeatCount

        /// The total duration for repeating the animation.
        repeatDur

        /// Whether the animation effect should remain active after the
        /// animation ends.
        fill

        /// The interpolation mode for the animation.
        calcMode

        /// The key times associated with each animation value.
        keyTimes

        /// The key splines controlling the pacing of the animation.
        keySplines

        /// Whether the animation is additive.
        additive

        /// Whether the animation is cumulative.
        accumulate

        /// A URL reference to the element to animate.
        href
    }

    /// Causes a referenced element to move along a motion path.
    animateMotion {
        /// A semicolon-separated list of values defining the animation
        /// sequence.
        values

        /// The starting value of the animation.
        from

        /// The ending value of the animation.
        to

        /// A relative offset value for the animation.
        by

        /// The begin time of the animation.
        begin

        /// The simple duration of the animation.
        dur

        /// The end time of the animation.
        end

        /// The minimum value of the animation.
        min

        /// The maximum value of the animation.
        max

        /// Whether and how to restart the animation.
        restart

        /// The number of times the animation should repeat.
        repeatCount

        /// The total duration for repeating the animation.
        repeatDur

        /// Whether the animation effect should remain active after the
        /// animation ends.
        fill

        /// The interpolation mode for the animation.
        calcMode

        /// The key times associated with each animation value.
        keyTimes

        /// The key splines controlling the pacing of the animation.
        keySplines

        /// Whether the animation is additive.
        additive

        /// Whether the animation is cumulative.
        accumulate

        /// How far along the path the object should be for each
        /// `keyTimes` value.
        keyPoints

        /// The motion path definition.
        path

        /// Post-multiply transform applied to the animated element.
        rotate

        /// The origin for the motion path.
        origin

        /// A URL reference to the element to animate.
        href
    }

    /// Animates a transformation attribute on a target element, allowing
    /// animations to control translation, scaling, rotation, and/or
    /// skewing.
    animateTransform {
        /// The name of the CSS property or attribute to animate.
        attributeName

        /// A semicolon-separated list of values defining the animation
        /// sequence.
        values

        /// The starting value of the animation.
        from

        /// The ending value of the animation.
        to

        /// A relative offset value for the animation.
        by

        /// The begin time of the animation.
        begin

        /// The simple duration of the animation.
        dur

        /// The end time of the animation.
        end

        /// The minimum value of the animation.
        min

        /// The maximum value of the animation.
        max

        /// Whether and how to restart the animation.
        restart

        /// The number of times the animation should repeat.
        repeatCount

        /// The total duration for repeating the animation.
        repeatDur

        /// Whether the animation effect should remain active after the
        /// animation ends.
        fill

        /// The interpolation mode for the animation.
        calcMode

        /// The key times associated with each animation value.
        keyTimes

        /// The key splines controlling the pacing of the animation.
        keySplines

        /// Whether the animation is additive.
        additive

        /// Whether the animation is cumulative.
        accumulate

        /// The type of transformation to animate.
        r#type

        /// A URL reference to the element to animate.
        href
    }

    /// A sub-element for `<animateMotion>` that provides the ability to
    /// reference an external path as the motion path definition.
    mpath {
        /// A URL reference to the `<path>` element defining the motion
        /// path.
        href

        /// The URL of the resource as a compatible replacement for `href`.
        #[deprecated = "use `href` instead"]
        xlink_href
    }

    /// Sets the value of an attribute for a specified duration. It
    /// supports all attribute types.
    set {
        /// The name of the CSS property or attribute to set.
        attributeName

        /// The value to apply to the target attribute.
        to

        /// The begin time of the animation.
        begin

        /// The simple duration of the animation.
        dur

        /// The end time of the animation.
        end

        /// The minimum value of the animation.
        min

        /// The maximum value of the animation.
        max

        /// Whether and how to restart the animation.
        restart

        /// The number of times the animation should repeat.
        repeatCount

        /// The total duration for repeating the animation.
        repeatDur

        /// Whether the animation effect should remain active after the
        /// animation ends.
        fill

        /// A URL reference to the element to animate.
        href
    }

    /// Allows authors to include dynamic script and data blocks in SVG
    /// documents.
    script {
        /// The type of the script.
        r#type

        /// A URL reference to an external script.
        href

        /// How the element handles crossorigin requests.
        crossorigin

        /// The URL of the resource as a compatible replacement for `href`.
        #[deprecated = "use `href` instead"]
        xlink_href
    }

    /// Allows specifying stylesheet rules directly within SVG content.
    style {
        /// The type of the style sheet language.
        r#type

        /// The intended destination medium for style information.
        media

        /// The title of the style sheet.
        title
    }

    /// Defines a particular view of an SVG document. The view can be
    /// activated by referencing the `<view>` element's `id` as target
    /// fragment of a URL.
    view {
        /// The SVG viewport coordinates for the view.
        viewBox

        /// How the SVG fragment should scale to fit the viewport.
        preserveAspectRatio
    }
}
