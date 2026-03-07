//! All built-in elements.
//!
//! This module can be overridden in your own crate to add custom HTML elements.
//! See the documentation for [`define_elements!`] or
//! [`GlobalAttributes`](crate::validation::attributes::GlobalAttributes) for
//! more information.

use crate::{define_elements, define_void_elements};

define_elements! {
    /// Represents the root (top-level element) of an HTML document, so it
    /// is also referred to as the *root element*. All other elements must
    /// be descendants of this element.
    html

    /// Contains machine-readable information (metadata) about the
    /// document, like its title, scripts, and style sheets.
    head

    /// Defines the document's title that is shown in a browser's title
    /// bar or a page's tab.
    title

    /// Contains style information (typically CSS) for a document or part
    /// of a document.
    style {
        /// The media the style should be applied to.
        media

        /// Whether the element is potentially render-blocking.
        blocking
    }

    /// Represents the content of an HTML document. There can be only one
    /// `<body>` element in a document.
    body

    /// Represents a self-contained composition in a document, page,
    /// application, or site, which is intended to be independently
    /// distributable or reusable (e.g., in syndication).
    article

    /// Represents a generic standalone section of a document, which
    /// doesn't have a more specific semantic element to represent it.
    section

    /// Represents a section of a page whose purpose is to provide
    /// navigation links, either within the current document or to other
    /// documents.
    nav

    /// Represents a portion of a document whose content is only
    /// indirectly related to the document's main content. Asides are
    /// frequently presented as sidebars or call-out boxes.
    aside

    /// A level 1 section heading. `<h1>` is the highest section level.
    h1

    /// A level 2 section heading.
    h2

    /// A level 3 section heading.
    h3

    /// A level 4 section heading.
    h4

    /// A level 5 section heading.
    h5

    /// A level 6 section heading. `<h6>` is the lowest section level.
    h6

    /// Represents a heading grouped with any secondary content, such as
    /// subheadings, an alternative title, or a tagline.
    hgroup

    /// Represents introductory content, typically a group of
    /// introductory or navigational aids.
    header

    /// Represents a footer for its nearest ancestor sectioning content
    /// or sectioning root element. Typically contains information about
    /// the author, copyright data, or links to related documents.
    footer

    /// Indicates that its contents provide contact information for the
    /// nearest ancestor `<article>` or `<body>` element.
    address

    /// Represents a paragraph.
    p

    /// Represents preformatted text which is to be presented exactly as
    /// written in the HTML file.
    pre

    /// Indicates that the enclosed text is an extended quotation.
    /// Usually rendered with indentation.
    blockquote {
        /// A URL that designates a source document or message for the
        /// quoted information.
        cite
    }

    /// Represents an ordered list of items, typically rendered as a
    /// numbered list.
    ol {
        /// Whether the list should be displayed in descending order.
        reversed

        /// The starting value of the list.
        start

        /// The numbering type of the list marker.
        r#type
    }

    /// Represents an unordered list of items, typically rendered as a
    /// bulleted list.
    ul

    /// A semantic alternative to `<ul>`, representing an unordered list
    /// of items displayed as an interactive menu of commands.
    menu

    /// Represents an item in a list.
    li {
        /// The ordinal value of the list item.
        value
    }

    /// Represents a description list, containing groups of terms (using
    /// `<dt>`) and descriptions (using `<dd>`).
    dl

    /// Specifies a term in a description or definition list.
    dt

    /// Provides the description, definition, or value for the preceding
    /// term (`<dt>`) in a description list (`<dl>`).
    dd

    /// Represents self-contained content, potentially with an optional
    /// caption specified using the `<figcaption>` element.
    figure

    /// Represents a caption or legend describing the rest of the
    /// contents of its parent `<figure>` element.
    figcaption

    /// Represents the dominant content of the `<body>` of a document.
    /// The main content area consists of content that is directly
    /// related to the central topic of a document.
    main

    /// Represents a part that contains a set of form controls or other
    /// content related to performing a search or filtering operation.
    search

    /// A generic container for flow content. It has no effect on the
    /// content or layout until styled in some way using CSS.
    div

    /// Creates a hyperlink to web pages, files, email addresses,
    /// locations in the same page, or anything else a URL can address.
    /// The destination is specified by the [`href`](Self::href)
    /// attribute.
    a {
        /// The URL that the hyperlink points to.
        href

        /// Where to display the linked URL.
        target

        /// Causes the browser to treat the linked URL as a download.
        download

        /// A space-separated list of URLs to ping when the hyperlink
        /// is followed.
        ping

        /// The relationship of the linked URL as space-separated link
        /// types.
        rel

        /// The language of the linked resource.
        hreflang

        /// A hint of the MIME type of the linked URL.
        r#type

        /// How much of the referrer to send when following the link.
        referrerpolicy
    }

    /// Marks text that has stress emphasis.
    em

    /// Indicates that its contents have strong importance, seriousness,
    /// or urgency.
    strong

    /// Represents side-comments and small print, like copyright and
    /// legal text.
    small

    /// Renders text with a strikethrough, representing content that is
    /// no longer accurate or no longer relevant.
    s

    /// Used to mark the title of a cited creative work, such as a book,
    /// paper, essay, poem, song, film, or other work.
    cite

    /// Indicates that the enclosed text is a short inline quotation.
    q {
        /// A URL that designates a source document or message for the
        /// quoted information.
        cite
    }

    /// Indicates the defining instance of a term.
    dfn

    /// Represents an abbreviation or acronym, optionally providing a
    /// full description via the `title` global attribute.
    abbr

    /// Represents small annotations rendered above, below, or next to
    /// base text, used for showing pronunciation of East Asian
    /// characters.
    ruby

    /// Specifies the ruby text component of a ruby annotation, used to
    /// provide pronunciation, translation, or transliteration
    /// information for East Asian typography.
    rt

    /// Provides fall-back parentheses for browsers that do not support
    /// display of ruby annotations using the `<ruby>` element.
    rp

    /// Links its content with a machine-readable equivalent specified
    /// in the [`value`](Self::value) attribute.
    data {
        /// The machine-readable translation of the element's content.
        value
    }

    /// Represents a specific period in time. It may include the
    /// [`datetime`](Self::datetime) attribute to translate dates into a
    /// machine-readable format.
    time {
        /// The machine-readable date/time value of the element.
        datetime
    }

    /// Displays its contents styled in a fashion intended to indicate
    /// that the text is a short fragment of computer code.
    code

    /// Represents the name of a variable in a mathematical expression
    /// or a programming context.
    var

    /// Used to enclose inline text which represents sample (or quoted)
    /// output from a computer program.
    samp

    /// Represents a span of inline text denoting textual user input
    /// from a keyboard, voice input, or any other text entry device.
    kbd

    /// Specifies inline text which is to be displayed as superscript.
    sup

    /// Specifies inline text which is to be displayed as subscript.
    sub

    /// Represents a range of text that is set off from the normal text
    /// for some reason, such as idiomatic text, technical terms, or
    /// taxonomic designations, rendered in italic type.
    i

    /// Used to draw attention to the element's contents, without
    /// indicating extra importance. Typically rendered in boldface.
    b

    /// Represents a span of inline text which should be rendered in a
    /// way that indicates that it has a non-textual annotation, such as
    /// a misspelling. Rendered as an underline by default.
    u

    /// Represents text which is marked or highlighted for reference or
    /// notation purposes.
    mark

    /// Tells the browser's bidirectional algorithm to treat the text it
    /// contains in isolation from its surrounding text.
    bdi

    /// Overrides the current directionality of text, so that the text
    /// within is rendered in a different direction.
    bdo

    /// A generic inline container for phrasing content, which does not
    /// inherently represent anything.
    span

    /// Represents a range of text that has been added to a document.
    ins {
        /// A URL to a resource that explains the change.
        cite

        /// The date and (optionally) time of the change.
        datetime
    }

    /// Represents a range of text that has been deleted from a
    /// document.
    del {
        /// A URL to a resource that explains the change.
        cite

        /// The date and (optionally) time of the change.
        datetime
    }

    /// Contains zero or more `<source>` elements and one `<img>`
    /// element to offer alternative versions of an image for different
    /// display/device scenarios.
    picture

    /// Embeds a nested browsing context, embedding another HTML page
    /// into the current one.
    iframe {
        /// The URL of the page to embed.
        src

        /// Inline HTML to embed, overriding the `src` attribute.
        srcdoc

        /// A targetable name for the embedded browsing context.
        name

        /// Controls the restrictions applied to the content embedded in
        /// the `<iframe>`.
        sandbox

        /// Specifies a permissions policy for the `<iframe>`.
        allow

        /// Whether to allow the `<iframe>` to activate fullscreen mode.
        allowfullscreen

        /// The width of the frame in CSS pixels.
        width

        /// The height of the frame in CSS pixels.
        height

        /// How much of the referrer to send when fetching the frame's
        /// resource.
        referrerpolicy

        /// Indicates when the browser should load the iframe.
        loading
    }

    /// Represents an external resource, which can be treated as an
    /// image, a nested browsing context, or a resource to be handled by
    /// a plugin.
    object {
        /// The address of the resource.
        data

        /// The content type of the resource specified by `data`.
        r#type

        /// The name of a valid browsing context.
        name

        /// Associates the element with a `<form>` element.
        form

        /// The width of the display resource in CSS pixels.
        width

        /// The height of the display resource in CSS pixels.
        height
    }

    /// Embeds a media player which supports video playback into the
    /// document. Can also be used for audio content, but the `<audio>`
    /// element may provide a more appropriate user experience.
    video {
        /// The URL of the video to embed.
        src

        /// How the element handles crossorigin requests.
        crossorigin

        /// A URL for an image to show while the video is downloading.
        poster

        /// Hints how much buffering the media resource will likely need.
        preload

        /// Whether the media will automatically start playing.
        autoplay

        /// Whether the video should be played inline rather than
        /// fullscreen.
        playsinline

        /// Whether the media should restart after reaching the end.
        r#loop

        /// Whether the audio will be initially silenced.
        muted

        /// Whether to display playback controls to the user.
        controls

        /// The width of the video's display area in CSS pixels.
        width

        /// The height of the video's display area in CSS pixels.
        height
    }

    /// Used to embed sound content in documents. It may contain one or
    /// more audio sources, represented using the `src` attribute or the
    /// `<source>` element.
    audio {
        /// The URL of the audio to embed.
        src

        /// How the element handles crossorigin requests.
        crossorigin

        /// Hints how much buffering the media resource will likely need.
        preload

        /// Whether the media will automatically start playing.
        autoplay

        /// Whether the media should restart after reaching the end.
        r#loop

        /// Whether the audio will be initially silenced.
        muted

        /// Whether to display playback controls to the user.
        controls
    }

    /// Used with `<area>` elements to define an image map (a clickable
    /// link area).
    map {
        /// The name of the map to reference from the `usemap` attribute.
        name
    }

    /// Represents tabular data — information presented in a
    /// two-dimensional table.
    table

    /// Specifies the caption (or title) of a table.
    caption

    /// Defines a group of columns within a table.
    colgroup {
        /// The number of consecutive columns the element spans.
        span
    }

    /// Encapsulates a set of table rows, indicating that they comprise
    /// the body of the table.
    tbody

    /// Defines a set of rows that constitute the head of the columns of
    /// the table.
    thead

    /// Defines a set of rows summarizing the columns of the table
    /// (the table footer).
    tfoot

    /// Defines a row of cells in a table.
    tr

    /// Defines a cell of a table that contains data.
    td {
        /// The number of columns the cell extends.
        colspan

        /// The number of rows the cell extends.
        rowspan

        /// A list of `id` values of the `<th>` elements that apply to
        /// this cell.
        headers
    }

    /// Defines a cell as the header of a group of table cells.
    th {
        /// The number of columns the header cell extends.
        colspan

        /// The number of rows the header cell extends.
        rowspan

        /// A list of `id` values of the `<th>` elements that apply to
        /// this cell.
        headers

        /// The group of cells the header element relates to.
        scope

        /// A short abbreviated description of the header cell's
        /// content.
        abbr
    }

    /// Represents a document section containing interactive controls
    /// for submitting information.
    form {
        /// The character encodings to use for form submission.
        accept_charset

        /// The URL that processes the form submission.
        action

        /// The default setting for autofill for controls in the form.
        autocomplete

        /// The encoding type to use for form submission.
        enctype

        /// The HTTP method to use for form submission.
        method

        /// The name of the form, used in the `document.forms` API.
        name

        /// Whether to skip form control validation on submission.
        novalidate

        /// Where to display the response after submitting the form.
        target

        /// The relationship between the current document and the linked
        /// resource.
        rel
    }

    /// Represents a caption for an item in a user interface. Associating
    /// a `<label>` with a form control provides usability and
    /// accessibility benefits.
    label {
        /// The `id` of the form control to associate with this label.
        r#for
    }

    /// An interactive element activated by a user, used to submit forms
    /// or trigger actions anywhere in a document.
    button {
        /// Whether the button is disabled.
        disabled

        /// Associates the button with a `<form>` element.
        form

        /// The URL that processes the form submission (overrides the
        /// form's `action`).
        formaction

        /// The encoding type for form submission (overrides the form's
        /// `enctype`).
        formenctype

        /// The HTTP method for form submission (overrides the form's
        /// `method`).
        formmethod

        /// Whether to skip validation on form submission (overrides the
        /// form's `novalidate`).
        formnovalidate

        /// Where to display the response from form submission (overrides
        /// the form's `target`).
        formtarget

        /// The name of the button, submitted as part of form data.
        name

        /// Targets a popover element to toggle, show, or hide.
        popovertarget

        /// Indicates whether a targeted popover element is to be
        /// toggled, shown, or hidden.
        popovertargetaction

        /// The default behavior of the button.
        r#type

        /// The value associated with the button's `name` when submitted
        /// with form data.
        value
    }

    /// Represents a control that provides a menu of options for the user
    /// to select from.
    select {
        /// Hint for form autofill feature.
        autocomplete

        /// Whether the select control is disabled.
        disabled

        /// Associates the element with a `<form>` element.
        form

        /// Whether multiple options can be selected at once.
        multiple

        /// The name of the control, submitted with form data.
        name

        /// Whether selecting an option is required for form submission.
        required

        /// The number of visible option rows in the list.
        size
    }

    /// Contains a set of `<option>` elements that represent the
    /// permissible or recommended options available to choose from
    /// within other controls.
    datalist

    /// Creates a grouping of options within a `<select>` element.
    optgroup {
        /// Whether the options in this group are disabled.
        disabled

        /// The label for the option group, displayed by the browser.
        label
    }

    /// Represents an item contained in a `<select>`, an `<optgroup>`,
    /// or a `<datalist>` element.
    option {
        /// Whether the option is disabled.
        disabled

        /// The text label for the option.
        label

        /// Whether the option is selected by default.
        selected

        /// The value to be submitted with form data.
        value
    }

    /// Represents a multi-line plain-text editing control, useful for
    /// cases like allowing users to enter comments or reviews.
    textarea {
        /// Hint for form autofill feature.
        autocomplete

        /// The visible width of the text control, in average character
        /// widths.
        cols

        /// The name used to send the element's directionality in form
        /// submission.
        dirname

        /// Whether the text control is disabled.
        disabled

        /// Associates the element with a `<form>` element.
        form

        /// The maximum number of characters the user can enter.
        maxlength

        /// The minimum number of characters required.
        minlength

        /// The name of the control, submitted with form data.
        name

        /// A hint to the user of what can be entered in the control.
        placeholder

        /// Whether the text is read-only.
        readonly

        /// Whether the control is required for form submission.
        required

        /// The number of visible text lines for the control.
        rows

        /// How the value of the control is to be wrapped for form
        /// submission.
        wrap
    }

    /// Represents the result of a calculation or user action.
    output {
        /// A space-separated list of `id` values of elements that
        /// contributed to the output.
        r#for

        /// Associates the element with a `<form>` element.
        form

        /// The name of the element, used for form submission.
        name
    }

    /// Displays an indicator showing the completion progress of a task,
    /// typically displayed as a progress bar.
    progress {
        /// The current value of the progress indicator.
        value

        /// The maximum value of the progress indicator.
        max
    }

    /// Represents either a scalar value within a known range or a
    /// fractional value.
    meter {
        /// The current numeric value of the element.
        value

        /// The lower bound of the measured range.
        min

        /// The upper bound of the measured range.
        max

        /// The upper numeric bound of the low end of the range.
        low

        /// The lower numeric bound of the high end of the range.
        high

        /// The optimal numeric value of the element.
        optimum
    }

    /// Groups several controls as well as labels within a web form,
    /// typically rendered with a border.
    fieldset {
        /// Whether the descendant form controls are disabled.
        disabled

        /// Associates the element with a `<form>` element.
        form

        /// The name of the group, used in the `form.elements` API.
        name
    }

    /// Represents a caption for the content of its parent `<fieldset>`.
    legend

    /// Creates a disclosure widget in which information is visible only
    /// when the widget is toggled into an open state.
    details {
        /// The name of a group of mutually-exclusive `<details>`
        /// elements.
        name

        /// Whether the details content is currently visible.
        open
    }

    /// Specifies a summary, caption, or legend for a `<details>`
    /// element's disclosure box.
    summary

    /// Represents a dialog box or other interactive component, such as
    /// a dismissible alert, inspector, or subwindow.
    dialog {
        /// Whether the dialog is active and can be interacted with.
        open
    }

    /// Used to embed executable code or data, typically JavaScript. It
    /// can also be used with other languages such as WebGL's shader
    /// programming language and JSON.
    script {
        /// The URL of an external script to load.
        src

        /// The type of script represented.
        r#type

        /// Prevents execution in user agents that support module
        /// scripts.
        nomodule

        /// Execute the script asynchronously, without blocking parsing.
        r#async

        /// Defer script execution until the document has been parsed.
        defer

        /// How the element handles crossorigin requests.
        crossorigin

        /// Integrity metadata for Subresource Integrity checks.
        integrity

        /// How much of the referrer to send when fetching the script.
        referrerpolicy

        /// Whether the element is potentially render-blocking.
        blocking

        /// Sets the priority for fetches initiated by the element.
        fetchpriority
    }

    /// Defines a section of HTML to be inserted if a script type on the
    /// page is unsupported or scripting is turned off in the browser.
    noscript

    /// A mechanism for holding HTML that is not to be rendered
    /// immediately when a page is loaded but may be instantiated
    /// subsequently during runtime using JavaScript.
    template {
        /// Enables streaming declarative shadow roots.
        shadowrootmode

        /// Sets delegates focus on a declarative shadow root.
        shadowrootdelegatesfocus
    }

    /// A placeholder inside a web component that you can fill with your
    /// own markup, letting you create separate DOM trees and present
    /// them together.
    slot {
        /// The name of the slot.
        name
    }

    /// A container element used to draw graphics and animations via
    /// scripting (usually JavaScript), such as 2D graphics and WebGL.
    canvas {
        /// The width of the canvas in CSS pixels.
        width

        /// The height of the canvas in CSS pixels.
        height
    }
}

define_void_elements! {
    /// Defines a hot-spot region on an image map, and optionally
    /// associates it with a hypertext link. This element is used only
    /// within a `<map>` element.
    area {
        /// Alternative text for the area, displayed when the image is
        /// not available.
        alt

        /// The coordinates that define the hot-spot region.
        coords

        /// The shape of the associated hot spot.
        shape

        /// The hyperlink target for the area.
        href

        /// Where to display the linked URL.
        target

        /// Causes the browser to treat the linked URL as a download.
        download

        /// A space-separated list of URLs to ping when the hyperlink
        /// is followed.
        ping

        /// The relationship of the linked URL as space-separated link
        /// types.
        rel

        /// How much of the referrer to send when following the link.
        referrerpolicy
    }

    /// Specifies the base URL to use for all relative URLs in a
    /// document. There can be only one `<base>` element in a document.
    base {
        /// The base URL to be used throughout the document for relative
        /// URLs.
        href

        /// The default browsing context for hyperlink navigation and
        /// form submission.
        target
    }

    /// Produces a line break in text (carriage-return).
    br

    /// Defines one or more columns in a column group represented by
    /// its parent `<colgroup>`.
    col {
        /// The number of consecutive columns the element spans.
        span
    }

    /// Embeds external content at the specified point in the document.
    /// This content is provided by an external application or other
    /// source of interactive content such as a browser plug-in.
    embed {
        /// The URL of the resource being embedded.
        src

        /// The MIME type of the embedded resource.
        r#type

        /// The width of the embedded content in CSS pixels.
        width

        /// The height of the embedded content in CSS pixels.
        height
    }

    /// Represents a thematic break between paragraph-level elements,
    /// typically rendered as a horizontal rule.
    hr

    /// Embeds an image into the document.
    img {
        /// Alternative text describing the image for accessibility.
        alt

        /// The image URL.
        src

        /// A list of one or more source sizes and URLs for responsive
        /// images.
        srcset

        /// A set of source size descriptors for responsive images.
        sizes

        /// How the element handles crossorigin requests.
        crossorigin

        /// The partial URL of an image map associated with the element.
        usemap

        /// Whether the image is a server-side image map.
        ismap

        /// The intrinsic width of the image in pixels.
        width

        /// The intrinsic height of the image in pixels.
        height

        /// How much of the referrer to send when fetching the image.
        referrerpolicy

        /// A hint to the browser as to whether to decode the image
        /// synchronously or asynchronously.
        decoding

        /// Indicates when the browser should load the image.
        loading

        /// Sets the priority for fetches initiated by the element.
        fetchpriority
    }

    /// Used to create interactive controls for web-based forms to
    /// accept data from the user. How an `<input>` works varies
    /// depending on the value of its [`type`](Self::type) attribute.
    input {
        /// A hint for the expected file type in file upload controls.
        accept

        /// Alternative text for image inputs.
        alt

        /// Hint for form autofill feature.
        autocomplete

        /// The media capture method in file upload controls.
        capture

        /// Whether the control is checked (for checkboxes and radio
        /// buttons).
        checked

        /// The name used to send the element's directionality in form
        /// submission.
        dirname

        /// Whether the form control is disabled.
        disabled

        /// Associates the element with a `<form>` element.
        form

        /// The URL that processes the form submission (for submit
        /// buttons).
        formaction

        /// The encoding type for form submission (for submit buttons).
        formenctype

        /// The HTTP method for form submission (for submit buttons).
        formmethod

        /// Whether to skip validation on form submission (for submit
        /// buttons).
        formnovalidate

        /// Where to display the response from form submission (for
        /// submit buttons).
        formtarget

        /// The height of an image input in pixels.
        height

        /// Identifies a `<datalist>` of pre-defined options to suggest.
        list

        /// The maximum value for numeric or date inputs.
        max

        /// The maximum number of characters the user can enter.
        maxlength

        /// The minimum value for numeric or date inputs.
        min

        /// The minimum number of characters required.
        minlength

        /// Whether to allow multiple values.
        multiple

        /// The name of the control, submitted with form data.
        name

        /// A regex pattern the value must match to be valid.
        pattern

        /// A hint to the user of what can be entered in the control.
        placeholder

        /// Targets a popover element to toggle, show, or hide.
        popovertarget

        /// Indicates whether a targeted popover element is to be
        /// toggled, shown, or hidden.
        popovertargetaction

        /// Whether the value is read-only.
        readonly

        /// Whether the control is required for form submission.
        required

        /// The width of the control, in characters.
        size

        /// The URL for image or submit inputs.
        src

        /// The stepping interval for numeric or date inputs.
        step

        /// The type of control to display.
        r#type

        /// The initial value of the control.
        value

        /// The width of an image input in pixels.
        width
    }

    /// Specifies relationships between the current document and an
    /// external resource, most commonly used to link to stylesheets but
    /// also used for site icons and other things.
    link {
        /// The URL of the linked resource.
        href

        /// How the element handles crossorigin requests.
        crossorigin

        /// The relationship of the linked resource to the current
        /// document.
        rel

        /// The media the linked resource applies to.
        media

        /// Integrity metadata for Subresource Integrity checks.
        integrity

        /// The language of the linked resource.
        hreflang

        /// A hint of the MIME type of the linked resource.
        r#type

        /// How much of the referrer to send when fetching the resource.
        referrerpolicy

        /// The sizes of the icons (for `rel="icon"`).
        sizes

        /// A list of source sizes and URLs for responsive images (for
        /// `rel="preload"`).
        imagesrcset

        /// Image sizes for different page layouts (for
        /// `rel="preload"`).
        imagesizes

        /// The potential destination for a preload request (for
        /// `rel="preload"` and `rel="modulepreload"`).
        r#as

        /// Whether the element is potentially render-blocking.
        blocking

        /// The color for a mask icon (for `rel="mask-icon"`).
        color

        /// Whether the linked stylesheet is disabled.
        disabled

        /// Sets the priority for fetches initiated by the element.
        fetchpriority
    }

    /// Represents metadata that cannot be represented by other HTML
    /// meta-related elements, like `<base>`, `<link>`, `<script>`,
    /// `<style>`, or `<title>`.
    meta {
        /// The name of the metadata.
        name

        /// A pragma directive.
        http_equiv

        /// The value of the metadata element.
        content

        /// The character encoding declaration for the document.
        charset

        /// The media the metadata applies to.
        media
    }

    /// Specifies multiple media resources for the `<picture>`,
    /// `<audio>`, or `<video>` elements.
    source {
        /// The MIME type of the media resource.
        r#type

        /// The media query for the resource's intended media.
        media

        /// The URL of the media resource.
        src

        /// A list of source sizes and URLs for responsive images.
        srcset

        /// Image sizes for different page layouts.
        sizes

        /// The intrinsic width of the image in pixels.
        width

        /// The intrinsic height of the image in pixels.
        height
    }

    /// Used as a child of the `<audio>` and `<video>` elements to
    /// specify timed text tracks (such as subtitles or captions).
    track {
        /// How the text track is meant to be used.
        kind

        /// The URL of the track file.
        src

        /// The language of the text track data.
        srclang

        /// A user-readable title of the text track.
        label

        /// Whether the track should be enabled by default.
        default
    }

    /// Represents a word break opportunity — a position within text
    /// where the browser may optionally break a line.
    wbr
}

macro_rules! define_mathml_elements {
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
            #[expect(missing_docs)]
            #[expect(
                non_camel_case_types,
                reason = "camel case types will be interpreted as components"
            )]
            #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
            pub struct $name;

            $(
                #[allow(non_upper_case_globals)]
                #[expect(missing_docs)]
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

            impl $crate::validation::attributes::MathMlGlobalAttributes for $name {}
        )*
    }
}

define_mathml_elements! {
    math {
        display
    }

    annotation {
        encoding
    }

    annotation_xml {
        encoding
    }

    menclose {
        notation
    }

    merror

    mfrac {
        linethickness
    }

    mi {
        mathvariant
    }

    mmultiscripts {
        mathvariant
    }

    mn

    mo {
        accent
        fence
        form
        largeop
        lspace
        maxsize
        minsize
        movablelimits
        rspace
        separator
        stretchy
        symmetric
    }

    mover {
        accent
    }

    mpadded {
        depth
        height
        lspace
        voffset
        width
    }

    mphantom

    mprescripts

    mroot

    mrow

    ms

    mspace {
        depth
        height
        width
    }

    msqrt

    mstyle

    msub

    msubsup

    msup

    mtable {
        align
        columnalign
        columnlines
        columnspacing
        frame
        framespacing
        rowalign
        rowlines
        rowspacing
        width
    }

    mtd {
        columnspan
        rowspan
        columnalign
        rowalign
    }

    mtext

    mtr {
        columnalign
        rowalign
    }

    munder {
        accentunder
    }

    munderover {
        accent
        accentunder
    }

    semantics
}
