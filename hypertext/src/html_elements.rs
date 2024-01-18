//! HTML elements.

macro_rules! elements {
    {
        $(
            $(#[$element_meta:meta])*
            $element:ident $(
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
            $(#[$element_meta])*
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, Copy)]
            pub struct $element;

            impl $element {
                $(
                    $(
                        $(#[$attr_meta])*
                        #[allow(non_upper_case_globals)]
                        pub const $attr: crate::attributes::Attribute = crate::attributes::Attribute;
                    )*
                )?
            }

            impl crate::attributes::GlobalAttributes for $element {}
        )*
    }
}

elements! {
    /// The root of an HTML document.
    html

    /// A collection of metadata for the document.
    head

    /// The document's title or name.
    title

    /// Allows authors to specify the document base URL for the purposes of
    /// parsing URLs, and the name of the default navigable for the purposes of
    /// following hyperlinks.
    base {
        /// Document base URL
        href

        /// Default navigable for hyperlink navigation and form submission
        target
    }

    /// Allows authors to link their document to other resources.
    link {
        /// Address of the hyperlink
        href

        /// How the element handles crossorigin requests
        crossorigin

        /// Relationship between the document containing the hyperlink and the
        /// destination resource
        rel

        /// Applicable media
        media

        /// Integrity metadata used in _Subresource Integrity_ checks
        integrity

        /// Language of the linked resource
        hreflang

        /// Hint for the type of the referenced resource
        r#type

        /// Referrer policy for fetches initiated by the element
        referrerpolicy

        /// Sizes of the icons (for `rel="icon"`)
        sizes

        /// Images to use in different situations, e.g., high-resolution
        /// displays, small monitors, etc. (for `rel="preload"`)
        imagesrcset

        /// Image sizes for different page layouts (for `rel="preload"`)
        imagesizes

        /// Potential destination for a preload request (for `rel="preload"` and
        /// `rel="modulepreload"`)
        r#as

        /// Whether the element is potentially render-blocking
        blocking

        /// Color to use when customizing a site's icon (for `rel="mask-icon"`)
        color

        /// Whether the link is disabled
        disabled

        /// Sets the priority for fetches initiated by the element
        fetchpriority
    }

    /// Various kinds of metadata that cannot be expressed using the `title`,
    /// `base`, `link`, `style`, and `script` elements.
    meta {
        /// Metadata name
        name

        /// Pragma directive
        http_equiv

        /// Value of the element
        content

        /// Character encoding declaration
        charset

        /// Applicable media
        media
    }

    /// Allows authors to embed CSS style sheets in their documents.
    style {
        /// Applicable media
        media

        /// Whether the element is potentially render-blocking
        blocking
    }

    /// The contents of the document.
    body

    /// A complete, or self-contained, composition in a document, page,
    /// application, or site and that is, in principle, independently
    /// distributable or reusable, e.g. in syndication.
    article

    /// A generic section of a document or application.
    section

    /// A section of a page that links to other pages or to parts within the
    /// page: a section with navigation links.
    nav

    /// A section of a page that consists of content that is tangentially
    /// related to the content around the `aside` element, and which could be
    /// considered separate from that content.
    aside

    /// Heading for its section.
    h1

    /// Heading for its section.
    h2

    /// Heading for its section.
    h3

    /// Heading for its section.
    h4

    /// Heading for its section.
    h5

    /// Heading for its section.
    h6

    /// A heading and related content.
    hgroup

    /// A group of introductory or navigational aids.
    header

    /// A footer for its nearest ancestor sectioning content element, or for the
    /// body element if there is no such ancestor.
    footer

    /// The contact information for its nearest article or body element
    /// ancestor.
    address

    /// A paragraph.
    p

    /// A paragraph-level thematic break, e.g., a scene change in a story, or a
    /// transition to another topic within a section of a reference book;
    /// alternatively, it represents a separator between a set of options of a
    /// select element.
    hr

    /// A block of preformatted text, in which structure is represented by
    /// typographic conventions rather than by elements.
    pre

    /// A section that is quoted from another source.
    blockquote {
        /// Link to the source of the quotation or more information about the
        /// edit
        cite
    }


    /// A list of items, where the items have been intentionally ordered, such
    /// that changing the order would change the meaning of the document.
    ol {
        /// Number the list backwards
        reversed

        /// Starting value of the list
        start

        /// Kind of list marker
        r#type
    }

    /// A list of items, where the order of the items is not important — that
    /// is, where changing the order would not materially change the meaning of
    /// the document.
    ul

    /// A toolbar consisting of its contents, in the form of an unordered list
    /// of items (represented by li elements), each of which represents a
    /// command that the user can perform or activate.
    menu

    /// A list item.
    li {
        /// Ordinal value of the list item
        value
    }

    /// An association list consisting of zero or more name-value groups (a
    /// description list).
    dl

    /// The term, or name, part of a term-description group in a description
    /// list (`dl` element).
    dt


    /// The description, definition, or value, part of a term-description group
    /// in a description list (`dl` element).
    dd

    /// Some flow content, optionally with a caption, that is self-contained
    /// (like a complete sentence) and is typically referenced as a single unit
    /// from the main flow of the document.
    figure

    /// A caption or legend for the rest of the contents of the `figcaption`
    /// element's parent `figure` element, if any.
    figcaption

    /// The dominant contents of the document.
    main

    /// A part of a document or application that contains a set of form controls
    /// or other content related to performing a search or filtering operation.
    search

    /// No special meaning at all.
    div

    /// A hyperlink (a hypertext anchor) labeled by its contents.
    a {
        /// Address of the hyperlink
        href

        /// Navigable for hyperlink navigation
        target

        /// Whether to download the resource instead of navigating to it, and
        /// its filename if so
        download

        /// URLs to ping
        ping

        /// Relationship between the location in the document containing the
        /// hyperlink and the destination resource
        rel

        /// Language of the linked resource
        hreflang

        /// Hint for the type of the referenced resource
        r#type

        /// Referrer policy for fetches initiated by the element
        referrerpolicy
    }

    /// Stress emphasis of its contents.
    em

    /// Strong importance, seriousness, or urgency for its contents.
    strong

    /// Side comments such as small print.
    small

    /// Contents that are no longer accurate or no longer relevant.
    s

    /// The title of a work (e.g. a book, a paper, an essay, a poem, a score, a
    /// song, a script, a film, a TV show, a game, a sculpture, a painting, a
    /// theatre production, a play, an opera, a musical, an exhibition, a legal
    /// case report, a computer program, etc.).
    cite

    /// Some phrasing content quoted from another source.
    q {
        /// Link to the source of the quotation or more information about the
        /// edit
        cite
    }

    /// The defining instance of a term.
    dfn

    /// An abbreviation or acronym, optionally with its expansion.
    abbr

    /// Allows one or more spans of phrasing content to be marked with ruby
    /// annotations.
    ruby

    /// The ruby text component of a ruby annotation.
    rt

    /// Parentheses or other content around a ruby text component of a ruby
    /// annotation, to be shown by user agents that don't support ruby
    /// annotations.
    rp

    /// Provide a machine-readable form of those contents in the `value`
    /// attribute.
    data {
        /// Machine-readable value
        value
    }

    /// Provide a machine-readable form of those contents in the `datetime`
    /// attribute.
    time {
        /// Machine-readable value
        datetime
    }

    /// A fragment of computer code.
    code

    /// A variable.
    var

    /// Sample or quoted output from another program or computing system.
    samp

    /// User input (typically keyboard input, although it may also be used to
    /// represent other input, such as voice commands).
    kbd

    /// A superscript.
    sup

    /// A subscript.
    sub

    /// A span of text in an alternate voice or mood, or otherwise offset from
    /// the normal prose in a manner indicating a different quality of text,
    /// such as a taxonomic designation, a technical term, an idiomatic phrase
    /// from another language, transliteration, a thought, or a ship name in
    /// Western texts.
    i

    /// A span of text to which attention is being drawn for utilitarian
    /// purposes without conveying any extra importance and with no implication
    /// of an alternate voice or mood, such as key words in a document abstract,
    /// product names in a review, actionable words in interactive text-driven
    /// software, or an article lede.
    b

    /// A span of text with an unarticulated, though explicitly rendered,
    /// non-textual annotation, such as labeling the text as being a proper name
    /// in Chinese text (a Chinese proper name mark), or labeling the text as
    /// being misspelt.
    u

    /// A run of text in one document marked or highlighted for reference
    /// purposes, due to its relevance in another context.
    mark

    /// A span of text that is to be isolated from its surroundings for the
    /// purposes of bidirectional text formatting.
    bdi

    /// Explicit text directionality formatting control for its children.
    bdo

    /// No special meaning.
    span

    /// A line break.
    br

    /// A line break opportunity.
    wbr

    /// An addition to the document.
    ins {
        /// Link to the source of the quotation or more information about the
        /// edit
        cite

        /// Date and (optionally) time of the change
        datetime
    }

    /// A removal from the document.
    del {
        /// Link to the source of the quotation or more information about the
        /// edit
        cite

        /// Date and (optionally) time of the change
        datetime
    }

    /// A container which provides multiple sources to its contained `img`
    /// element to allow authors to declaratively control or give hints to the
    /// user agent about which image resource to use, based on the screen pixel
    /// density, viewport size, image format, and other factors.
    picture

    /// Allows authors to specify multiple alternative source sets for `img`
    /// elements or multiple alternative media resources for media elements.
    source {
        /// Type of embedded resource
        r#type

        /// Applicable media
        media

        /// Address of the resource
        src

        /// Images to use in different situations, e.g., high-resolution
        /// displays, small monitors, etc.
        srcset

        /// Image sizes for different page layouts
        sizes

        /// Horizontal dimension
        width

        /// Vertical dimension
        height
    }

    /// An image.
    img {
        /// Replacement text for use when images are not available
        alt

        /// Address of the resource
        src

        /// Images to use in different situations, e.g., high-resolution
        /// displays, small monitors, etc.
        srcset

        /// Image sizes for different page layouts
        sizes

        /// How the element handles crossorigin requests
        crossorigin

        /// Name of image map to use
        usemap

        /// Whether the image is a server-side image map
        ismap

        /// Horizontal dimension
        width

        /// Vertical dimension
        height

        /// Referrer policy for fetches initiated by the element
        referrerpolicy

        /// Decoding hint to use when processing this image for presentation
        decoding

        /// Used when determining loading deferral
        loading

        /// Sets the priority for fetches initiated by the element
        fetchpriority
    }

    /// Contains a content navigable.
    iframe {
        /// Address of the resource
        src

        /// A document to render in the `iframe`
        srcdoc

        /// Name of content navigable
        name

        /// Security rules for nested content
        sandbox

        ///  Permissions policy to be applied to the `iframe`'s contents
        allow

        /// Whether to allow the `iframe`'s contents to use
        /// `requestFullscreen()`
        allowfullscreen

        /// Horizontal dimension
        width

        /// Vertical dimension
        height

        /// Referrer policy for fetches initiated by the element
        referrerpolicy

        /// Used when determining loading deferral
        loading
    }

    /// An integration point for an external application or interactive content.
    embed {
        /// Address of the resource
        src

        /// Type of embedded resource
        r#type

        /// Horizontal dimension
        width

        /// Vertical dimension
        height
    }

    /// An external resource, which, depending on the type of the resource, will
    /// either be treated as an image or as a child navigable.
    object {
        /// Address of the resource
        data

        /// Type of embedded resource
        r#type

        /// Name of content navigable
        name

        /// Associates the element with a `form` element
        form

        /// Horizontal dimension
        width

        /// Vertical dimension
        height
    }

    /// Used for playing videos or movies, and audio files with captions.
    video {
        /// Address of the resource
        src

        /// How the element handles crossorigin requests
        crossorigin

        /// Poster frame to show prior to video playback
        poster

        /// Hints how much buffering the media resource will likely need
        preload

        /// Hint that the media resource can be started automatically when the
        /// page is loaded
        autoplay

        /// Encourage the user agent to display video content within the
        /// element's playback area
        playsinline

        /// Whether to loop the media resource
        r#loop

        /// Whether to mute the media resource by default
        muted

        /// Show user agent controls
        controls

        /// Horizontal dimension
        width

        /// Vertical dimension
        height
    }

    /// A sound or audio stream.
    audio {
        /// Address of the resource
        src

        /// How the element handles crossorigin requests
        crossorigin

        /// Hints how much buffering the media resource will likely need
        preload

        /// Hint that the media resource can be started automatically when the
        /// page is loaded
        autoplay

        /// Whether to loop the media resource
        r#loop

        /// Whether to mute the media resource by default
        muted

        /// Show user agent controls
        controls
    }

    /// Allows authors to specify explicit external timed text tracks for media
    /// elements.
    track {
        /// The type of text track
        kind

        /// Address of the resource
        src

        /// Language of the text track
        srclang

        /// User-visible label
        label

        /// Enable the track if no other text track is more suitable
        default
    }

    /// Defines an image map.
    map {
        /// Name of image map to reference from the `usemap` attribute
        name
    }

    /// Either a hyperlink with some text and a corresponding area on an image
    /// map, or a dead area on an image map.
    area {
        /// Replacement text for use when images are not available
        alt

        /// Coordinates for the shape to be created in an image map
        coords

        /// The kind of shape to be created in an image map
        shape

        /// Address of the hyperlink
        href

        /// Navigable for hyperlink navigation
        target

        /// Whether to download the resource instead of navigating to it, and
        /// its filename if so
        download

        /// URLs to ping
        ping

        /// Relationship between the location in the document containing the
        /// hyperlink and the destination resource
        rel

        /// Referrer policy for fetches initiated by the element
        referrerpolicy
    }

    /// Data with more than one dimension, in the form of a table.
    table

    /// The title of the `table` that is its parent, if it has a parent and that
    /// is a `table` element.
    caption

    /// A group of one or more columns in the `table` that is its parent, if it
    /// has a parent and that is a `table` element.
    colgroup {
        /// Number of columns spanned by the element
        span
    }

    /// One or more columns in the column group represented by a parent
    /// `colgroup`.
    col {
        /// Number of columns spanned by the element
        span
    }

    /// A block of rows that consist of a body of data for the parent `table`
    /// element, if the `tbody` element has a parent and it is a `table`.
    tbody

    /// The block of rows that consist of the column labels (headers) and any
    /// ancillary non-header cells for the parent `table` element, if the
    /// `thead` element has a parent and it is a `table`.
    thead

    /// The block of rows that consist of the column summaries (footers) for the
    /// parent `table` element, if the `tfoot` element has a parent and it is a
    /// `table`.
    tfoot

    /// A row of cells in a table.
    tr

    /// A data cell in a table.
    td {
        /// Number of columns that the cell is to span
        colspan

        /// Number of rows that the cell is to span
        rowspan

        /// The header cells for this cell
        headers
    }

    /// A header cell in a table.
    th {
        /// Number of columns that the cell is to span
        colspan

        /// Number of rows that the cell is to span
        rowspan

        /// The header cells for this cell
        headers

        /// Specifies which cells the header cell applies to
        scope

        /// Alternative label to use for the header cell when referencing the
        /// cell in other contexts
        abbr
    }

    /// A hyperlink that can be manipulated through a collection of
    /// form-associated elements, some of which can represent editable values
    /// that can be submitted to a server for processing.
    form {
        /// Character encodings to use for form submission
        accept_charset

        /// URL to use for form submission
        action

        /// Default setting for autofill feature for controls in the form
        autocomplete

        /// Entry list encoding type to use for form submission
        enctype

        /// Variant to use for form submission
        method

        /// Name of form to use in the `document.forms` API
        name

        /// Bypass form control validation for form submission
        novalidate

        /// Navigable for form submission
        target

        /// Relationship between the location in the document containing the
        /// hyperlink and the destination resource
        rel
    }

    /// A caption in a user interface.
    label {
        /// Associate the label with form control
        r#for
    }

    /// A typed data field, usually with a form control to allow the user to
    /// edit the data.
    input {
        /// Hint for expected file type in file upload controls
        accept

        /// Replacement text for use when images are not available
        alt

        /// Hint for form autofill feature
        autocomplete

        /// Whether the control is checked
        checked

        /// Name of form control to use for sending the element's directionality
        /// in form submission
        dirname

        /// Whether the form control is disabled
        disabled

        /// Associates the element with a `form` element
        form

        /// URL to use for form submission
        formaction

        /// Entry list encoding type to use for form submission
        formenctype

        /// Variant to use for form submission
        formmethod

        /// Bypass form control validation for form submission
        formnovalidate

        /// Navigable for form submission
        formtarget

        /// Vertical dimension
        height

        /// List of autocomplete options
        list

        /// Maximum value
        max

        /// Maximum length of value
        maxlength

        /// Minimum value
        min

        /// Minimum length of value
        minlength

        /// Whether to allow multiple values
        multiple

        /// Name of the element to use for form submission and in the
        /// `form.elements` API
        name

        /// Pattern to be matched by the form control's value
        pattern

        /// User-visible label to be placed within the form control
        placeholder

        /// Targets a popover element to toggle, show, or hide
        popovertarget

        /// Indicates whether a targeted popover element is to be toggled,
        /// shown, or hidden
        popovertargetaction

        /// Whether to allow the value to be edited by the user
        readonly

        /// Whether the control is required for form submission
        required

        /// Size of the control
        size

        /// Address of the resource
        src

        /// Granularity to be matched by the form control's value
        step

        /// Type of form control
        r#type

        /// Value of the form control
        value

        /// Horizontal dimension
        width
    }

    /// A button labeled by its contents.
    button {
        /// Whether the form control is disabled
        disabled

        /// Associates the element with a `form` element
        form

        /// URL to use for form submission
        formaction

        /// Entry list encoding type to use for form submission
        formenctype

        /// Variant to use for form submission
        formmethod

        /// Bypass form control validation for form submission
        formnovalidate

        /// Navigable for form submission
        formtarget

        /// Name of the element to use for form submission and in the
        /// `form.elements` API
        name

        /// Targets a popover element to toggle, show, or hide
        popovertarget

        /// Indicates whether a targeted popover element is to be toggled,
        /// shown, or hidden
        popovertargetaction

        /// Type of button
        r#type

        /// Value to be used for form submission
        value
    }

    /// A control for selecting amongst a set of options.
    select {
        /// Hint for form autofill feature
        autocomplete

        /// Whether the form control is disabled
        disabled

        /// Associates the element with a `form` element
        form

        /// Whether to allow multiple values
        multiple

        /// Name of the element to use for form submission and in the
        /// `form.elements` API
        name

        /// Whether the control is required for form submission
        required

        /// Size of the control
        size
    }

    /// A set of option elements that represent predefined options for other
    /// controls.
    datalist


    /// A group of `option` elements with a common label.
    optgroup {
        /// Whether the form control is disabled
        disabled

        /// User-visible label
        label
    }

    /// An option in a `select` element or as part of a list of suggestions in a
    /// `datalist` element.
    option {
        /// Whether the form control is disabled
        disabled

        /// User-visible label
        label

        /// Whether the option is selected by default
        selected

        /// Value to be used for form submission
        value
    }

    /// A multiline plain text edit control for the element's **raw value**.
    textarea {
        /// Hint for form autofill feature
        autocomplete

        /// Maximum number of characters per line
        cols

        /// Name of form control to use for sending the element's directionality
        /// in form submission
        dirname

        /// Whether the form control is disabled
        disabled

        /// Associates the element with a `form` element
        form

        /// Maximum length of value
        maxlength

        /// Minimum length of value
        minlength

        /// Name of the element to use for form submission and in the
        /// `form.elements` API
        name

        /// User-visible label to be placed within the form control
        placeholder

        /// Whether to allow the value to be edited by the user
        readonly

        /// Whether the control is required for form submission
        required

        /// Number of lines to show
        rows

        /// How the value of the form control is to be wrapped for form
        /// submission
        wrap
    }

    /// The result of a calculation performed by the application, or the result
    /// of a user action.
    output {
        /// Specifies controls from which the output was calculated
        r#for

        /// Associates the element with a `form` element
        form

        /// Name of the element to use for form submission and in the
        /// `form.elements` API
        name
    }

    /// The completion progress of a task.
    progress {
        /// Current value of the element
        value

        /// Upper bound of range
        max
    }

    /// A scalar measurement within a known range, or a fractional value; for
    /// example disk usage, the relevance of a query result, or the fraction of
    /// a voting population to have selected a particular candidate.
    meter {
        /// Current value of the element
        value

        /// Lower bound of range
        min

        /// Upper bound of range
        max

        /// High limit of low range
        low

        /// Low limit of high range
        high

        /// Optimum value in gauge
        optimum
    }

    /// A set of form controls (or other content) grouped together, optionally
    /// with a caption.
    fieldset {
        /// Whether the descendant form controls, except any inside legend, are
        /// disabled
        disabled

        /// Associates the element with a `form` element
        form

        /// Name of the element to use in the `form.elements` API
        name
    }

    /// A caption for the rest of the contents of the `legend` element's parent
    /// `fieldset` element, if any.
    legend

    /// A disclosure widget from which the user can obtain additional
    /// information or controls.
    details {
        /// Name of group of mutually-exclusive details elements
        name

        /// Whether the details are visible
        open
    }

    /// A summary, caption, or legend for the rest of the contents of the
    /// summary element's parent details element, if any.
    summary

    /// A transitory part of an application, in the form of a small window
    /// ("dialog box"), which the user interacts with to perform a task or
    /// gather information.
    dialog {
        /// Whether the dialog box is showing
        open
    }

    /// Allows authors to include dynamic script and data blocks in their
    /// documents.
    script {
        /// Address of the resource
        src

        /// Type of script
        r#type

        /// Prevents execution in user agents that support module scripts
        nomodule

        /// Execute script when available, without blocking while fetching
        r#async

        /// Defer script execution
        defer

        /// How the element handles crossorigin requests
        crossorigin

        /// Integrity metadata used in _Subresource Integrity_ checks
        integrity

        /// Referrer policy for fetches initiated by the element
        referrerpolicy

        /// Whether the element is potentially render-blocking
        blocking

        /// Sets the priority for fetches initiated by the element
        fetchpriority
    }

    /// Does nothing if scripting is enabled, shows the fallback content if
    /// scripting is disabled.
    noscript

    /// Used to declare fragments of HTML that can be cloned and inserted in the
    /// document by script.
    template {
        /// Enables streaming declarative shadow roots
        shadowrootmode

        /// Sets delegates focus on a declarative shadow root
        shadowrootdelegatesfocus
    }

    /// A slot in a shadow tree that can be filled with an arbitrary node.
    slot {
        /// Name of shadow tree slot
        name
    }

    /// A resolution-dependent bitmap canvas, which can be used for rendering
    /// graphs, game graphics, art, or other visual images on the fly.
    /// A canvas element.
    canvas {
        /// Horizontal dimension
        width

        /// Vertical dimension
        height
    }
}

macro_rules! void {
    ($($el:ident)*) => {
        $(impl crate::VoidElement for $el {})*
    };
}

void! {
    area base br col embed hr img input link meta source track wbr
}

#[cfg(feature = "svg")]
macro_rules! svg_elements {
    {
        $(
            $(#[$element_meta:meta])*
            $element:ident $(
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
            $(#[$element_meta])*
            #[allow(non_camel_case_types)]
            #[allow(missing_docs)] // TODO
            #[derive(Debug, Clone, Copy)]
            pub struct $element;

            impl $element {
                $(
                    $(
                        $(#[$attr_meta])*
                        #[allow(non_upper_case_globals)]
                        #[allow(missing_docs)] // TODO
                        pub const $attr: crate::attributes::Attribute = crate::attributes::Attribute;
                    )*
                )?
            }

            impl crate::attributes::GlobalSVGAttributes for $element {}
        )*
    }
}

#[cfg(feature = "svg")]
svg_elements! {
    circle {
        cx
        cy
        r
        pathLength 
    } 

    defs

    desc

    ellipse {
        cx
        cy
        rx
        ry
        pathLength 
    } 

    g

    image {
        x
        y
        width
        height
        href
        preserveAspectRatio
        crossorigin
        decoding 
    }

    line {
        x1
        x2
        y1
        y2
        pathLength 
    }

    linearGradient {
        gradientUnits
        gradientTransform
        href
        spreadMethod
        x1
        y1
        y2 
    }

    mask {
        width
        height
        maskContentUnits
        maskUnits
        x
        y 
    }

    metadata

    path {
        d
        pathLength 
    }

    pattern {
        width
        height
        href
        patternContentUnits
        patternTransform
        patternUnits
        preserveAspectRatio
        viewBox
        x
        y 
    }

    polygon {
        points
        pathLength 
    }

    polyline {
        points
        pathLength 
    }

    radialGradient {
        cx
        cy
        fr
        fx
        fy
        gradientUnits
        gradientTransform
        href
        r
        spreadMethod 
    }

    rect {
        x
        y
        width
        height
        rx
        ry
        pathLength 
    }

    stop {
        offset
        stop_color
        stop_opacity 
    }

    svg {
        baseProfile
        preserveAspectRatio
        version
        viewBox
        width
        height
        x
        y 
    }

    symbol {
        width
        height
        preserveAspectRatio
        refX
        refY
        viewBox
        x
        y 
    }

    text {
        x
        y
        dx
        dy
        rotate
        lengthAdjust
        textLength 
    }

    textPath {
        href
        lengthAdjust
        method
        spacing
        startOffset
        textLength 
    }

    tspan {
        x
        y
        dx
        dy
        rotate
        lengthAdjust
        textLength 
    }

    r#use {
        href
        x
        y
        width
        height 
    }

    view {
        viewBox
        preserveAspectRatio 
    }
}

#[cfg(feature = "svg")]
void! {
    circle ellipse image line path polygon polyline rect stop r#use view
}

#[cfg(feature = "svg")]
macro_rules! presentation {
    ($($el:ident)*) => {
        $(impl crate::attributes::PresentationSVGAttributes for $el {})*
    };
}

#[cfg(feature = "svg")]
presentation! {
    a circle ellipse g image line path pattern polygon polyline rect stop symbol text textPath tspan r#use svg
}
