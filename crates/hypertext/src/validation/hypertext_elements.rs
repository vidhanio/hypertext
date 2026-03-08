//! HTML element definitions.
//!
//! This module provides type-checked HTML element definitions for use with
//! [`maud!`](crate::maud!) and [`rsx!`](crate::rsx!) macros. Each element is
//! defined as a unit struct that implements
//! [`Element<Kind = Normal>`](super::Element) or
//! [`Element<Kind = Void>`](super::Element) and
//! [`GlobalAttributes`](super::attributes::GlobalAttributes).
//!
//! To add custom HTML elements, create a module named `hypertext_elements`
//! that re-exports this module's contents and adds your own definitions:
//!
//! ```
//! mod hypertext_elements {
//!     use hypertext::define_elements;
//!     pub use hypertext::validation::hypertext_elements::*;
//!
//!     define_elements! {
//!         /// A custom HTML element.
//!         my_custom_element {
//!             /// A custom attribute.
//!             my_attr
//!         }
//!     }
//! }
//! ```

use crate::{define_elements, define_void_elements};

define_elements! {
    /// Represents the root (top-level element) of an HTML document, so it is
    /// also referred to as the *root element*. All other elements must be
    /// descendants of this element.
    html

    /// Contains machine-readable information (metadata) about the document,
    /// like its [`title`], scripts, and style sheets.
    head

    /// Defines the document's title that is shown in a browser's title bar or
    /// a page's tab. It only contains text; tags within the element, if any,
    /// are also treated as plain text.
    title

    /// Contains style information for a document or part of a document. It
    /// contains CSS, which is applied to the contents of the document
    /// containing this element.
    style {
        /// The media the style should be applied to.
        media

        /// Whether the element is potentially render-blocking.
        blocking
    }

    /// Represents the content of an HTML document. There can be only one
    /// [`body`] element in a document.
    body

    /// Indicates that the enclosed HTML provides contact information for a
    /// person or people, or for an organization.
    address

    /// Represents a self-contained composition in a document, page,
    /// application, or site, which is intended to be independently
    /// distributable or reusable (e.g., in syndication). Examples include a
    /// forum post, a magazine or newspaper article, a blog entry, a product
    /// card, a user-submitted comment, an interactive widget or gadget, or any
    /// other independent item of content.
    article

    /// Represents a portion of a document whose content is only indirectly
    /// related to the document's main content. Asides are frequently presented
    /// as sidebars or call-out boxes.
    aside

    /// Represents a footer for its nearest ancestor sectioning content or
    /// sectioning root element. A [`footer`] typically contains information
    /// about the author of the section, copyright data, or links to related
    /// documents.
    footer

    /// Represents introductory content, typically a group of introductory or
    /// navigational aids. It may contain some heading elements but also a
    /// logo, a search form, an author name, and other elements.
    header

    /// Represents the highest section level heading. [`h1`] is the highest
    /// section level and [`h6`] is the lowest.
    h1

    /// Represents a level 2 section heading.
    h2

    /// Represents a level 3 section heading.
    h3

    /// Represents a level 4 section heading.
    h4

    /// Represents a level 5 section heading.
    h5

    /// Represents the lowest section level heading. [`h1`] is the highest
    /// section level and [`h6`] is the lowest.
    h6

    /// Represents a heading grouped with any secondary content, such as
    /// subheadings, an alternative title, or a tagline.
    hgroup

    /// Represents the dominant content of the body of a document. The main
    /// content area consists of content that is directly related to or expands
    /// upon the central topic of a document, or the central functionality of
    /// an application.
    main

    /// Represents a section of a page whose purpose is to provide navigation
    /// links, either within the current document or to other documents. Common
    /// examples of navigation sections are menus, tables of contents, and
    /// indexes.
    nav

    /// Represents a generic standalone section of a document, which doesn't
    /// have a more specific semantic element to represent it. Sections should
    /// always have a heading, with very few exceptions.
    section

    /// Represents a part that contains a set of form controls or other content
    /// related to performing a search or filtering operation.
    search

    /// Indicates that the enclosed text is an extended quotation. Usually,
    /// this is rendered visually by indentation. A URL for the source of the
    /// quotation may be given using the [`cite`](Self::cite) attribute, while a
    /// text representation of the source can be given using the [`cite`]
    /// element.
    blockquote {
        /// A URL that designates a source document or message for the
        /// information quoted.
        cite
    }

    /// Provides the description, definition, or value for the preceding term
    /// ([`dt`]) in a description list ([`dl`]).
    dd

    /// The generic container for flow content. It has no effect on the content
    /// or layout until styled in some way using CSS (e.g., styling is directly
    /// applied to it, or some kind of layout model like flexbox is applied to
    /// its parent element).
    div

    /// Represents a description list. The element encloses a list of groups of
    /// terms (specified using the [`dt`] element) and descriptions (provided by
    /// [`dd`] elements). Common uses for this element are to implement a
    /// glossary or to display metadata (a list of key-value pairs).
    dl

    /// Specifies a term in a description or definition list, and as such must
    /// be used inside a [`dl`] element. It is usually followed by a [`dd`]
    /// element; however, multiple [`dt`] elements in a row indicate several
    /// terms that are all defined by the immediate next [`dd`] element.
    dt

    /// Represents a caption or legend describing the rest of the contents of
    /// its parent [`figure`] element.
    figcaption

    /// Represents self-contained content, potentially with an optional
    /// caption, which is specified using the [`figcaption`] element. The
    /// figure, its caption, and its contents are referenced as a single unit.
    figure

    /// Represents an item in a list. It must be contained in a parent element:
    /// an ordered list ([`ol`]), an unordered list ([`ul`]), or a menu
    /// ([`menu`]). In menus and unordered lists, list items are usually
    /// displayed using bullet points. In ordered lists, they are usually
    /// displayed with an ascending counter on the left, such as a number or
    /// letter.
    li {
        /// The ordinal value of the list item.
        value
    }

    /// A semantic alternative to [`ul`], but treated by browsers (and exposed
    /// through the accessibility tree) as no different than [`ul`]. It
    /// represents an unordered list of items (which are represented by [`li`]
    /// elements).
    menu

    /// Represents an ordered list of items — typically rendered as a numbered
    /// list.
    ol {
        /// Whether the list should be displayed in descending order instead of
        /// ascending.
        reversed

        /// An integer to start counting from for the list items.
        start

        /// The numbering type of the list marker.
        r#type
    }

    /// Represents a paragraph. Paragraphs are usually represented in visual
    /// media as blocks of text separated from adjacent blocks by blank lines
    /// and/or first-line indentation, but HTML paragraphs can be any structural
    /// grouping of related content, such as images or form fields.
    p

    /// Represents preformatted text which is to be presented exactly as written
    /// in the HTML file. The text is typically rendered using a
    /// non-proportional, or monospaced, font. Whitespace inside this element is
    /// displayed as written.
    pre

    /// Represents an unordered list of items, typically rendered as a bulleted
    /// list.
    ul

    /// Together with its [`href`](Self::href) attribute, creates a hyperlink to
    /// web pages, files, email addresses, locations within the current page, or
    /// anything else a URL can address.
    a {
        /// The URL that the hyperlink points to.
        href

        /// Where to display the linked URL, as the name for a browsing context.
        target

        /// Causes the browser to treat the linked URL as a download.
        download

        /// A space-separated list of URLs to ping when the hyperlink is
        /// followed.
        ping

        /// The relationship of the linked URL as space-separated link types.
        rel

        /// Hints at the human language of the linked URL.
        hreflang

        /// Hints at the MIME type of the linked URL.
        r#type

        /// How much of the referrer to send when following the link.
        referrerpolicy
    }

    /// Represents an abbreviation or acronym.
    abbr

    /// Used to draw the reader's attention to the element's contents, which
    /// are not otherwise granted special importance. This was formerly known as
    /// the Boldface element, and most browsers still draw the text in boldface.
    /// However, you should not use [`b`] for styling text or granting
    /// importance. If you wish to create boldface text, you should use the CSS
    /// `font-weight` property. If you wish to indicate an element is of special
    /// importance, you should use the [`strong`] element.
    b

    /// Tells the browser's bidirectional algorithm to treat the text it
    /// contains in isolation from its surrounding text. It's particularly
    /// useful when a website dynamically inserts some text and doesn't know the
    /// directionality of the text being inserted.
    bdi

    /// Overrides the current directionality of text, so that the text within is
    /// rendered in a different direction.
    bdo

    /// Used to mark up the title of a cited creative work. The reference may be
    /// in an abbreviated form according to context-appropriate conventions
    /// related to citation metadata.
    cite

    /// Displays its contents styled in a fashion intended to indicate that the
    /// text is a short fragment of computer code. By default, the content text
    /// is displayed using the user agent's default monospace font.
    code

    /// Links a given piece of content with a machine-readable translation. If
    /// the content is time- or date-related, the [`time`] element must be used.
    data {
        /// The machine-readable translation of the element's content.
        value
    }

    /// Used to indicate the term being defined within the context of a
    /// definition phrase or sentence. The ancestor [`p`] element, the
    /// [`dt`]/[`dd`] pairing, or the nearest section ancestor of the [`dfn`]
    /// element, is considered to be the definition of the term.
    dfn

    /// Marks text that has stress emphasis. The [`em`] element can be nested,
    /// with each nesting level indicating a greater degree of emphasis.
    em

    /// Represents a range of text that is set off from the normal text for some
    /// reason, such as idiomatic text, technical terms, and taxonomical
    /// designations, among others. Historically, these have been presented using
    /// italicized type, which is the original source of the [`i`] naming of
    /// this element.
    i

    /// Represents a span of inline text denoting textual user input from a
    /// keyboard, voice input, or any other text entry device. By convention,
    /// the user agent defaults to rendering the contents of a [`kbd`] element
    /// using its default monospace font, although this is not mandated by the
    /// HTML standard.
    kbd

    /// Represents text which is marked or highlighted for reference or notation
    /// purposes due to the marked passage's relevance in the enclosing context.
    mark

    /// Indicates that the enclosed text is a short inline quotation. Most
    /// modern browsers implement this by surrounding the text in quotation
    /// marks. This element is intended for short quotations that don't require
    /// paragraph breaks; for long quotations use the [`blockquote`] element.
    q {
        /// A URL that designates a source document or message for the
        /// information quoted.
        cite
    }

    /// Used to provide fall-back parentheses for browsers that do not support
    /// the display of ruby annotations using the [`ruby`] element. One [`rp`]
    /// element should enclose each of the opening and closing parentheses that
    /// wrap the [`rt`] element that contains the annotation's text.
    rp

    /// Specifies the ruby text component of a ruby annotation, which is used to
    /// provide pronunciation, translation, or transliteration information for
    /// East Asian typography. The [`rt`] element must always be contained within
    /// a [`ruby`] element.
    rt

    /// Represents small annotations that are rendered above, below, or next to
    /// base text, usually used for showing the pronunciation of East Asian
    /// characters. It can also be used for annotating other kinds of text, but
    /// this usage is less common.
    ruby

    /// Renders text with a strikethrough, or a line through it. Use the [`s`]
    /// element to represent things that are no longer relevant or no longer
    /// accurate. However, [`s`] is not appropriate when indicating document
    /// edits; for that, use the [`del`] and [`ins`] elements, as appropriate.
    s

    /// Used to enclose inline text which represents sample (or quoted) output
    /// from a computer program. Its contents are typically rendered using the
    /// browser's default monospaced font.
    samp

    /// Represents side-comments and small print, like copyright and legal text,
    /// independent of its styled presentation. By default, it renders text
    /// within it one font size smaller, such as from `small` to `x-small`.
    small

    /// A generic inline container for phrasing content, which does not
    /// inherently represent anything. It can be used to group elements for
    /// styling purposes (using the `class` or `id` attributes), or because they
    /// share attribute values, such as `lang`. It should be used only when no
    /// other semantic element is appropriate. [`span`] is very much like a
    /// [`div`] element, but [`div`] is a block-level element whereas a [`span`]
    /// is an inline-level element.
    span

    /// Indicates that its contents have strong importance, seriousness, or
    /// urgency. Browsers typically render the contents in bold type.
    strong

    /// Specifies inline text which should be displayed as subscript for solely
    /// typographical reasons. Subscripts are typically rendered with a lowered
    /// baseline using smaller text.
    sub

    /// Specifies inline text which is to be displayed as superscript for solely
    /// typographical reasons. Superscripts are usually rendered with a raised
    /// baseline using smaller text.
    sup

    /// Represents a specific period in time. It may include the
    /// [`datetime`](Self::datetime) attribute to translate dates into
    /// machine-readable format, allowing for better search engine results or
    /// custom features such as reminders.
    time {
        /// A machine-readable date/time value for the element's content.
        datetime
    }

    /// Represents a span of inline text which should be rendered in a way that
    /// indicates that it has a non-textual annotation. This is rendered by
    /// default as a single solid underline but may be altered using CSS.
    u

    /// Represents the name of a variable in a mathematical expression or a
    /// programming context. It's typically presented using an italicized version
    /// of the current typeface, although that behavior is browser-dependent.
    var

    /// Represents a range of text that has been added to a document. You can
    /// use the [`del`] element to similarly represent a range of text that has
    /// been deleted from the document.
    ins {
        /// A URL to a resource that explains the change.
        cite

        /// The date and (optionally) time of the change in a
        /// machine-readable format.
        datetime
    }

    /// Represents a range of text that has been deleted from a document. This
    /// can be used when rendering "track changes" or source code diff
    /// information, for example. The [`ins`] element can be used for the
    /// opposite purpose: to indicate text that has been added to the document.
    del {
        /// A URL to a resource that explains the change.
        cite

        /// The date and (optionally) time of the change in a
        /// machine-readable format.
        datetime
    }

    /// Contains zero or more [`source`] elements and one [`img`] element to
    /// offer alternative versions of an image for different display/device
    /// scenarios.
    picture

    /// Represents a nested browsing context, embedding another HTML page into
    /// the current one.
    iframe {
        /// The URL of the page to embed.
        src

        /// Inline HTML to embed, overriding the [`src`](Self::src) attribute.
        srcdoc

        /// A targetable name for the embedded browsing context.
        name

        /// Controls the restrictions applied to the content embedded in the
        /// [`iframe`].
        sandbox

        /// Specifies a [Permissions Policy] for the [`iframe`].
        ///
        /// [Permissions Policy]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Permissions_Policy
        allow

        /// Set to `true` if the [`iframe`] can activate fullscreen mode by
        /// calling the `requestFullscreen()` method.
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

    /// Represents an external resource, which can be treated as an image, a
    /// nested browsing context, or a resource to be handled by a plugin.
    object {
        /// The address of the resource as a valid URL.
        data

        /// The content type of the resource specified by
        /// [`data`](Self::data).
        r#type

        /// The name of a valid browsing context.
        name

        /// Associates the element with a [`form`] element.
        form

        /// The width of the display resource in CSS pixels.
        width

        /// The height of the display resource in CSS pixels.
        height
    }

    /// Embeds a media player which supports video playback into the document.
    /// You can also use [`video`] for audio content, but the [`audio`] element
    /// may provide a more appropriate user experience.
    video {
        /// The URL of the video to embed.
        src

        /// How the element handles crossorigin requests.
        crossorigin

        /// A URL for an image to be shown while the video is downloading.
        poster

        /// Provides a hint to the browser about what the author thinks will
        /// lead to the best user experience regarding what content is loaded
        /// before the video is played.
        preload

        /// A Boolean attribute; if specified, the video automatically begins
        /// to play back as soon as it can without stopping to finish loading
        /// the data.
        autoplay

        /// A Boolean attribute indicating that the video is to be played
        /// inline, that is within the element's playback area.
        playsinline

        /// A Boolean attribute; if specified, the browser will automatically
        /// seek back to the start upon reaching the end of the video.
        r#loop

        /// A Boolean attribute that indicates the default setting of the
        /// audio contained in the video.
        muted

        /// If this attribute is present, the browser will offer controls to
        /// allow the user to control video playback.
        controls

        /// The width of the video's display area in CSS pixels.
        width

        /// The height of the video's display area in CSS pixels.
        height
    }

    /// Used to embed sound content in documents. It may contain one or more
    /// audio sources, represented using the [`src`](Self::src) attribute or the
    /// [`source`] element: the browser will choose the most suitable one. It
    /// can also be the destination for streamed media, using a `MediaStream`.
    audio {
        /// The URL of the audio to embed.
        src

        /// How the element handles crossorigin requests.
        crossorigin

        /// Provides a hint to the browser about what the author thinks will
        /// lead to the best user experience regarding what content is loaded
        /// before the audio is played.
        preload

        /// A Boolean attribute; if specified, the audio will automatically
        /// begin playback as soon as it can without waiting for the entire
        /// audio file to finish downloading.
        autoplay

        /// If specified, the audio player will automatically seek back to the
        /// start upon reaching the end of the audio.
        r#loop

        /// A Boolean attribute that indicates whether the audio will be
        /// initially silenced.
        muted

        /// If this attribute is present, the browser will offer controls to
        /// allow the user to control audio playback.
        controls
    }

    /// Used with [`area`] elements to define an image map (a clickable link
    /// area).
    map {
        /// The name of the map to reference from the [`usemap`](img::usemap)
        /// attribute.
        name
    }

    /// Represents tabular data — that is, information presented in a
    /// two-dimensional table comprised of rows and columns of cells containing
    /// data.
    table

    /// Specifies the caption (or title) of a table.
    caption

    /// Defines a group of columns within a table.
    colgroup {
        /// The number of consecutive columns the element spans.
        span
    }

    /// Encapsulates a set of table rows ([`tr`] elements), indicating that they
    /// comprise the body of a table's (main) data.
    tbody

    /// Encapsulates a set of table rows ([`tr`] elements), indicating that they
    /// comprise the head of a table with information about the table's columns.
    /// This is usually in the form of column headers ([`th`] elements).
    thead

    /// Encapsulates a set of table rows ([`tr`] elements), indicating that they
    /// comprise the foot of a table with information about the table's columns.
    /// This is usually a summary of the columns, e.g., a sum of the given
    /// numbers in a column.
    tfoot

    /// Defines a row of cells in a table. The row's cells can then be
    /// established using a mix of [`td`] (data cell) and [`th`] (header cell)
    /// elements.
    tr

    /// A child of the [`tr`] element, it defines a cell of a table that
    /// contains data.
    td {
        /// The number of columns the cell extends.
        colspan

        /// The number of rows the cell extends.
        rowspan

        /// A list of space-separated strings, each corresponding to the `id`
        /// attribute of the [`th`] elements that apply to this element.
        headers
    }

    /// A child of the [`tr`] element, it defines a cell as the header of a
    /// group of table cells. The nature of this group can be explicitly defined
    /// by the [`scope`](Self::scope) and [`headers`](Self::headers) attributes.
    th {
        /// The number of columns the header cell extends.
        colspan

        /// The number of rows the header cell extends.
        rowspan

        /// A list of space-separated strings, each corresponding to the `id`
        /// attribute of the [`th`] elements that apply to this element.
        headers

        /// Defines the cells that the header (defined in the [`th`]) element
        /// relates to.
        scope

        /// A short, abbreviated description of the header cell's content
        /// provided as an alternative label to use for the header cell.
        abbr
    }

    /// Represents a document section containing interactive controls for
    /// submitting information.
    form {
        /// Space-separated character encodings the server accepts. The browser
        /// uses them in the order in which they are listed.
        accept_charset

        /// The URL that processes the form submission.
        action

        /// Indicates whether input elements can by default have their values
        /// automatically completed by the browser.
        autocomplete

        /// The MIME type of the form submission.
        enctype

        /// The HTTP method to submit the form with.
        method

        /// The name of the form.
        name

        /// Indicates that the form shouldn't be validated when submitted.
        novalidate

        /// Indicates where to display the response after submitting the form.
        target

        /// Annotations and the relationship of the link to the document
        /// containing it.
        rel
    }

    /// Represents a caption for an item in a user interface.
    label {
        /// The value of the `id` attribute of the form-related element in the
        /// same document to which the [`label`] is associated.
        r#for
    }

    /// An interactive element activated by a user with a mouse, keyboard,
    /// finger, voice command, or other assistive technology. Once activated, it
    /// performs an action, such as submitting a [`form`] or opening a dialog.
    button {
        /// Whether the button is disabled.
        disabled

        /// Associates the [`button`] element with a [`form`] element.
        form

        /// The URL that processes the information submitted by the button.
        formaction

        /// If the button is a submit button (it's inside/associated with a
        /// [`form`] and doesn't have `type="button"`), specifies how to encode
        /// the form data that is submitted.
        formenctype

        /// If the button is a submit button (it's inside/associated with a
        /// [`form`] and doesn't have `type="button"`), this attribute specifies
        /// the HTTP method used to submit the form.
        formmethod

        /// If the button is a submit button, this Boolean attribute specifies
        /// that the form is not to be validated when it is submitted.
        formnovalidate

        /// If the button is a submit button, this attribute is an
        /// author-defined name or standardized, underscore-prefixed keyword
        /// indicating where to display the response from submitting the form.
        formtarget

        /// The name of the button, submitted as a pair with the button's
        /// [`value`](Self::value) as part of the form data, when that button
        /// is used to submit the form.
        name

        /// Turns a [`button`] element into a popover control button; takes the
        /// `id` of the popover element to control as its value.
        popovertarget

        /// Specifies the action to be performed on a popover element being
        /// controlled by a control [`button`].
        popovertargetaction

        /// The default behavior of the button.
        r#type

        /// Defines the value associated with the button's
        /// [`name`](Self::name) when it's submitted with the form data.
        value
    }

    /// Represents a control that provides a menu of options.
    select {
        /// Hint for form autofill feature.
        autocomplete

        /// Whether the form control is disabled.
        disabled

        /// Associates the [`select`] element with a [`form`] element.
        form

        /// Indicates that multiple options can be selected in the list.
        multiple

        /// The name of the control.
        name

        /// Indicates that an option with a non-empty string value must be
        /// selected.
        required

        /// If the control is presented as a scrolling list box (e.g. when
        /// `multiple` is specified), this attribute represents the number of
        /// rows in the list that should be visible at one time.
        size
    }

    /// Contains a set of [`option`] elements that represent the permissible or
    /// recommended options available to choose from within other controls.
    datalist

    /// Creates a grouping of options within a [`select`] element.
    optgroup {
        /// If this Boolean attribute is set, none of the items in this option
        /// group is selectable.
        disabled

        /// The name of the group of options, which the browser can use when
        /// labeling the options in the user interface.
        label
    }

    /// Used to define an item contained in a [`select`], an [`optgroup`], or a
    /// [`datalist`] element. As such, [`option`] can represent menu items in
    /// popups and other lists of items in an HTML document.
    option {
        /// If this Boolean attribute is set, this option is not checkable.
        disabled

        /// This attribute is text for the label indicating the meaning of the
        /// option. If the [`label`](Self::label) attribute isn't defined, its
        /// value is that of the element text content.
        label

        /// If present, this Boolean attribute indicates that the option is
        /// initially selected.
        selected

        /// The content of this attribute represents the value to be submitted
        /// with the form, should this option be selected.
        value
    }

    /// Represents a multi-line plain-text editing control, useful when you want
    /// to allow users to enter a sizeable amount of free-form text, for
    /// example, a comment on a review or feedback form.
    textarea {
        /// Hint for form autofill feature.
        autocomplete

        /// The visible width of the text control, in average character widths.
        cols

        /// Indicates how the control's directionality will be submitted in a
        /// form.
        dirname

        /// Whether the text control is disabled.
        disabled

        /// Associates the [`textarea`] element with a [`form`] element.
        form

        /// The maximum string length (measured in UTF-16 code units) that the
        /// user can enter.
        maxlength

        /// The minimum string length (measured in UTF-16 code units) required
        /// that the user should enter.
        minlength

        /// The name of the control.
        name

        /// A hint to the user of what can be entered in the control.
        placeholder

        /// Whether the control's value cannot be changed.
        readonly

        /// Indicates that the user must fill in a value before submitting a
        /// form.
        required

        /// The number of visible text lines for the control.
        rows

        /// Indicates how the control's value is to be wrapped for form
        /// submission.
        wrap
    }

    /// Container element into which a site or app can inject the results of a
    /// calculation or the outcome of a user action.
    output {
        /// A space-separated list of other elements' `id`s, indicating that
        /// those elements contributed input values to (or otherwise affected)
        /// the calculation.
        r#for

        /// Associates the [`output`] element with a [`form`] element.
        form

        /// The name of the element.
        name
    }

    /// Displays an indicator showing the completion progress of a task,
    /// typically displayed as a progress bar.
    progress {
        /// How much work the task indicated by the [`progress`] element
        /// requires.
        max

        /// Specifies how much of the task that has been completed.
        value
    }

    /// Represents either a scalar value within a known range or a fractional
    /// value.
    meter {
        /// The current numeric value.
        value

        /// The lower numeric bound of the measured range.
        min

        /// The upper numeric bound of the measured range.
        max

        /// The upper numeric bound of the low end of the measured range.
        low

        /// The lower numeric bound of the high end of the measured range.
        high

        /// This attribute indicates the optimal numeric value.
        optimum
    }

    /// Used to group several controls as well as labels ([`label`]) within a
    /// web form.
    fieldset {
        /// If this Boolean attribute is set, all form controls that are
        /// descendants of the [`fieldset`] are disabled.
        disabled

        /// Associates the [`fieldset`] element with a [`form`] element.
        form

        /// The name associated with the group.
        name
    }

    /// Represents a caption for the content of its parent [`fieldset`].
    legend

    /// Creates a disclosure widget in which information is visible only when
    /// the widget is toggled into an "open" state. A summary or label must be
    /// provided using the [`summary`] element.
    details {
        /// This attribute enables multiple [`details`] elements to be
        /// connected, with only one open at a time.
        name

        /// This Boolean attribute indicates whether the details — that is, the
        /// contents of the [`details`] element — are currently visible.
        open
    }

    /// Specifies a summary, caption, or legend for a [`details`] element's
    /// disclosure box. Clicking the [`summary`] element toggles the state of
    /// the parent [`details`] element open and closed.
    summary

    /// Represents a dialog box or other interactive component, such as a
    /// dismissible alert, inspector, or subwindow.
    dialog {
        /// Indicates that the dialog box is active and can be interacted with.
        open
    }

    /// Used to embed executable code or data; this is typically used to embed
    /// or refer to JavaScript code. The [`script`] element can also be used
    /// with other languages, such as WebGL's GLSL shader programming language
    /// and JSON.
    script {
        /// This attribute specifies the URI of an external script.
        src

        /// This attribute indicates the type of script represented.
        r#type

        /// This Boolean attribute prevents a script from being executed in
        /// browsers that support ES modules.
        nomodule

        /// For classic scripts, if the `async` attribute is present, then the
        /// classic script will be fetched in parallel to parsing and evaluated
        /// as soon as it is available.
        r#async

        /// This Boolean attribute indicates that the browser should not execute
        /// the script and the script's fetch should be deferred.
        defer

        /// How the element handles crossorigin requests.
        crossorigin

        /// Contains inline metadata that a user agent can use to verify that a
        /// fetched resource has been delivered without unexpected manipulation.
        integrity

        /// Indicates which referrer to send when fetching the script, or
        /// resources fetched by the script.
        referrerpolicy

        /// Whether the element is potentially render-blocking.
        blocking

        /// Provides a hint of the relative priority to use when fetching an
        /// external script.
        fetchpriority
    }

    /// Defines a section of HTML to be inserted if a script type on the page
    /// is unsupported or if scripting is currently turned off in the browser.
    noscript

    /// A mechanism for holding HTML that is not to be rendered immediately when
    /// a page is loaded but may be instantiated subsequently during runtime
    /// using JavaScript.
    template {
        /// Determines whether or not a shadow root should be created for the
        /// parent element.
        shadowrootmode

        /// Enables setting of the [`delegatesFocus`] property on a
        /// declarative shadow root.
        ///
        /// [`delegatesFocus`]: https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/delegatesFocus
        shadowrootdelegatesfocus
    }

    /// Part of the [Web Components] technology suite, this element is a
    /// placeholder inside a web component that you can fill with your own
    /// markup, which lets you create separate DOM trees and present them
    /// together.
    ///
    /// [Web Components]: https://developer.mozilla.org/en-US/docs/Web/API/Web_components
    slot {
        /// The slot's name.
        name
    }

    /// Container element to use with either the [canvas scripting API] or the
    /// [WebGL API] to draw graphics and animations.
    ///
    /// [canvas scripting API]: https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API
    /// [WebGL API]: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API
    canvas {
        /// The width of the coordinate space in CSS pixels.
        width

        /// The height of the coordinate space in CSS pixels.
        height
    }
}

define_void_elements! {
    /// Defines an area inside an image map that has predefined clickable areas.
    /// An *image map* allows geometric areas on an image to be associated with
    /// a hyperlink.
    area {
        /// Defines the text that a browser will present if the image is
        /// missing, or the user's browser cannot display images.
        alt

        /// The `coords` attribute details the coordinates of the
        /// [`shape`](Self::shape) attribute in size, shape, and placement of an
        /// [`area`].
        coords

        /// The shape of the associated hot spot.
        shape

        /// The hyperlink target for the area.
        href

        /// Where to display the linked URL.
        target

        /// Causes the browser to treat the linked URL as a download.
        download

        /// A space-separated list of URLs to ping when the hyperlink is
        /// followed.
        ping

        /// The relationship of the linked URL as space-separated link types.
        rel

        /// Indicates which referrer is sent when fetching the resource.
        referrerpolicy
    }

    /// Specifies the base URL to use for all relative URLs in a document.
    /// There can be only one [`base`] element in a document.
    base {
        /// The base URL to be used throughout the document for relative URLs.
        href

        /// A keyword or author-defined name of the default browsing context to
        /// show the results of navigation from [`a`], [`area`], or [`form`]
        /// elements without explicit `target` attributes.
        target
    }

    /// Produces a line break in text (carriage-return). It is useful for
    /// writing a poem or an address, where the division of lines is
    /// significant.
    br

    /// Defines one or more columns in a column group represented by its
    /// implicit or explicit parent [`colgroup`] element. The [`col`] element
    /// is only valid as a child of a [`colgroup`] element that has no
    /// [`span`](colgroup::span) attribute defined.
    col {
        /// The number of consecutive columns the [`col`] element spans.
        span
    }

    /// Embeds external content at the specified point in the document. This
    /// content is provided by an external application or other source of
    /// interactive content such as a browser plug-in.
    embed {
        /// The URL of the resource being embedded.
        src

        /// The MIME type to use to select the plug-in to instantiate.
        r#type

        /// The display width of the resource in CSS pixels.
        width

        /// The display height of the resource in CSS pixels.
        height
    }

    /// Represents a thematic break between paragraph-level elements: for
    /// example, a change of scene in a story, or a shift of topic within a
    /// section.
    hr

    /// Embeds an image into the document.
    img {
        /// Defines text that can replace the image in the page.
        alt

        /// The image URL.
        src

        /// One or more strings separated by commas, indicating possible image
        /// sources for the user agent to use.
        srcset

        /// One or more strings separated by commas, indicating a set of source
        /// sizes.
        sizes

        /// Indicates if the fetching of the image must be done using a CORS
        /// request.
        crossorigin

        /// Specifies the partial URL (starting with `#`) of an image map
        /// associated with the element.
        usemap

        /// Indicates that the image is part of a server-side map.
        ismap

        /// The intrinsic width of the image in pixels.
        width

        /// The intrinsic height of the image in pixels.
        height

        /// Indicates which referrer to use when fetching the resource.
        referrerpolicy

        /// A hint to the browser as to whether it should perform image
        /// decoding along with rendering the other DOM content in a single
        /// presentation step that looks more "correct".
        decoding

        /// Indicates how the browser should load the image.
        loading

        /// Provides a hint of the relative priority to use when fetching the
        /// image.
        fetchpriority
    }

    /// Used to create interactive controls for web-based forms in order to
    /// accept data from the user; a wide variety of types of input data and
    /// control widgets are available, depending on the device and user agent.
    /// The [`input`] element is one of the most powerful and complex in all of
    /// HTML due to the sheer number of combinations of input types and
    /// attributes.
    input {
        /// Valid for the `file` input type only, the `accept` attribute defines
        /// which file types are selectable in a `file` upload control.
        accept

        /// Valid for the `image` input type only, the `alt` attribute provides
        /// alternative text for the image, displaying the value of the
        /// attribute if the image [`src`](Self::src) is missing or otherwise
        /// fails to load.
        alt

        /// Hint for form autofill feature.
        autocomplete

        /// Introduced in the HTML Media Capture specification and valid for the
        /// `file` input type only, the `capture` attribute defines which media
        /// (microphone, video, or camera) should be used to capture a new file
        /// for upload.
        capture

        /// Valid for `checkbox` and `radio` types, `checked` is a Boolean
        /// attribute. If present on a `checkbox` type, it indicates that the
        /// checkbox is checked by default. If present on a `radio` type, it
        /// indicates that the radio button is the currently selected one.
        checked

        /// Valid for `text` and `search` input types only, the `dirname`
        /// attribute enables the submission of the directionality of the
        /// element.
        dirname

        /// Indicates that the user cannot interact with the input.
        disabled

        /// Associates the [`input`] element with a [`form`] element.
        form

        /// Valid for the `image` and `submit` input types only. See the
        /// `submit` input type for more information.
        formaction

        /// Valid for the `image` and `submit` input types only. See the
        /// `submit` input type for more information.
        formenctype

        /// Valid for the `image` and `submit` input types only. See the
        /// `submit` input type for more information.
        formmethod

        /// Valid for the `image` and `submit` input types only. See the
        /// `submit` input type for more information.
        formnovalidate

        /// Valid for the `image` and `submit` input types only. See the
        /// `submit` input type for more information.
        formtarget

        /// Valid for the `image` input type only, the `height` is the height,
        /// in CSS pixels, of the image displayed to represent the graphical
        /// submit button.
        height

        /// Identifies a [`datalist`] element whose contents represent
        /// pre-defined suggested values to be suggested to the user for this
        /// input control.
        list

        /// Valid for `date`, `month`, `week`, `time`, `datetime-local`,
        /// `number`, and `range`, it defines the greatest value in the range
        /// of permitted values.
        max

        /// Defines the maximum string length (measured in UTF-16 code units)
        /// that the user can enter into an `email`, `password`, `search`,
        /// `tel`, `text`, or `url` input.
        maxlength

        /// Valid for `date`, `month`, `week`, `time`, `datetime-local`,
        /// `number`, and `range`, it defines the most negative value in the
        /// range of permitted values.
        min

        /// Defines the minimum string length (measured in UTF-16 code units)
        /// that the user can enter into an `email`, `password`, `search`,
        /// `tel`, `text`, or `url` input.
        minlength

        /// The Boolean `multiple` attribute, if set, means the user can enter
        /// comma separated email addresses in the `email` widget or can choose
        /// more than one file with the `file` input.
        multiple

        /// A string specifying a name for the input control.
        name

        /// The `pattern` attribute, when specified, is a regular expression
        /// that the input's [`value`](Self::value) must match for the value to
        /// pass constraint validation.
        pattern

        /// The `placeholder` attribute is a string that provides a brief hint
        /// to the user as to what kind of information is expected in the field.
        placeholder

        /// Turns an [`input`] element into a popover control button; takes the
        /// `id` of the popover element to control as its value.
        popovertarget

        /// Specifies the action to be performed on a popover element being
        /// controlled by a control [`input`].
        popovertargetaction

        /// A Boolean attribute which, if present, indicates that the user
        /// should not be able to edit the value of the input.
        readonly

        /// `required` is a Boolean attribute which, if present, indicates that
        /// the user must specify a value for the input before the owning form
        /// can be submitted.
        required

        /// The `size` attribute is a numeric value indicating how many
        /// characters wide the input field should be.
        size

        /// Valid for `email`, `password`, `tel`, `url`, and `text` input
        /// types, the `src` attribute specifies the location of the image for
        /// the `image` input type.
        src

        /// Valid for `date`, `month`, `week`, `time`, `datetime-local`,
        /// `number`, and `range`, the `step` attribute is a number that
        /// specifies the granularity that the value must adhere to.
        step

        /// Indicates the type of control to render.
        r#type

        /// The input control's value.
        value

        /// Valid for the `image` input type only, the `width` is the width,
        /// in CSS pixels, of the image displayed to represent the graphical
        /// submit button.
        width
    }

    /// Specifies relationships between the current document and an external
    /// resource. This element is most commonly used to link to CSS but is also
    /// used to establish site icons (both "favicon" style icons and icons for
    /// the home screen and apps on mobile devices) among other things.
    link {
        /// This attribute specifies the URL of the linked resource.
        href

        /// This enumerated attribute indicates whether CORS must be used when
        /// fetching the resource.
        crossorigin

        /// This attribute names a relationship of the linked document to the
        /// current document.
        rel

        /// This attribute specifies the media that the linked resource applies
        /// to.
        media

        /// Contains inline metadata — a base64-encoded cryptographic hash of
        /// the resource (file) you're telling the browser to fetch.
        integrity

        /// This attribute indicates the language of the linked resource.
        hreflang

        /// This attribute is used to define the type of the content linked to.
        r#type

        /// A string indicating which referrer to use when fetching the
        /// resource.
        referrerpolicy

        /// This attribute defines the sizes of the icons for visual media
        /// contained in the resource.
        sizes

        /// For `rel="preload"` and `as="image"` only, the `imagesrcset`
        /// attribute has similar syntax and semantics as the `srcset` attribute
        /// for [`img`] elements.
        imagesrcset

        /// For `rel="preload"` and `as="image"` only, the `imagesizes`
        /// attribute has similar syntax and semantics as the `sizes` attribute
        /// for [`img`] elements.
        imagesizes

        /// For `rel="preload"` and `rel="modulepreload"`, the `as` attribute
        /// is required.
        r#as

        /// Whether the element is potentially render-blocking.
        blocking

        /// This attribute is used to specify a color to be used by the browser
        /// for the display of a mask icon for a pinned tab in Safari.
        color

        /// Whether the linked stylesheet is disabled.
        disabled

        /// Provides a hint of the relative priority to use when fetching a
        /// preloaded resource.
        fetchpriority
    }

    /// Represents metadata that cannot be represented by other HTML
    /// meta-related elements, like [`base`], [`link`], [`script`], [`style`]
    /// and [`title`].
    meta {
        /// The `name` and `content` attributes can be used together to provide
        /// document metadata in terms of name-value pairs, with the
        /// [`name`](Self::name) attribute giving the metadata name, and the
        /// [`content`](Self::content) attribute giving the value.
        name

        /// Defines a pragma directive.
        http_equiv

        /// This attribute contains the value for the
        /// [`http-equiv`](Self::http_equiv) or [`name`](Self::name) attribute,
        /// depending on which is used.
        content

        /// This attribute declares the document's character encoding.
        charset

        /// The `media` attribute defines which media the theme color defined
        /// in the `content` attribute should be applied to.
        media
    }

    /// Specifies multiple media resources for the [`picture`], the [`audio`]
    /// element, or the [`video`] element. It is a void element, meaning that
    /// it has no content and does not have a closing tag. It is commonly used
    /// to offer the same media content in multiple file formats in order to
    /// provide compatibility with a broad range of browsers given their
    /// differing support for image file formats and media file formats.
    source {
        /// The MIME media type of the resource, optionally with a codecs
        /// parameter.
        r#type

        /// Media query of the resource's intended media.
        media

        /// Required if [`source`] is the last or sole source element within a
        /// [`picture`] element, but not permitted if [`source`] is within
        /// [`audio`] or [`video`] elements.
        src

        /// A list of one or more strings, separated by commas, indicating a
        /// set of possible images represented by the source for the browser to
        /// use.
        srcset

        /// A list of source sizes that describes the final rendered width of
        /// the image represented by the source.
        sizes

        /// The intrinsic width of the image in pixels.
        width

        /// The intrinsic height of the image in pixels.
        height
    }

    /// Used as a child of the media elements, [`audio`] and [`video`]. It lets
    /// you specify timed text tracks (or time-based data), for example to
    /// automatically handle subtitles. The tracks are formatted in [WebVTT
    /// format] (`.vtt` files) — Web Video Text Tracks.
    ///
    /// [WebVTT format]: https://developer.mozilla.org/en-US/docs/Web/API/WebVTT_API
    track {
        /// How the text track is meant to be used.
        kind

        /// Address of the track (`.vtt` file).
        src

        /// Language of the track text data. It must be a valid [BCP 47]
        /// language tag. If the [`kind`](Self::kind) attribute is set to
        /// `subtitles`, then [`srclang`](Self::srclang) must be defined.
        ///
        /// [BCP 47]: https://r12a.github.io/app-subtags/
        srclang

        /// A user-readable title of the text track which is used by the
        /// browser when listing available text tracks.
        label

        /// Indicates that the track should be enabled unless the user's
        /// preferences indicate that another track is more appropriate.
        default
    }

    /// Represents a word break opportunity — a position within text where the
    /// browser may optionally break a line, though its line-breaking rules
    /// would not otherwise create a break at that location.
    wbr
}
