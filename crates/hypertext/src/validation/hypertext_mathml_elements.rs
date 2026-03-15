//! MathML element definitions.
//!
//! This module provides type-checked MathML element definitions for use with
//! [`mathml::maud!`](crate::mathml::maud!) and
//! [`mathml::rsx!`](crate::mathml::rsx!) macros. Each element is defined as a
//! unit struct that implements [`Element<Kind = Xml>`](super::Element) and
//! [`MathMlGlobalAttributes`](super::attributes::MathMlGlobalAttributes).
//!
//! To add custom MathML elements, create a module named
//! `hypertext_mathml_elements` that re-exports this module's contents and adds
//! your own definitions:
//!
//! ```
//! mod hypertext_mathml_elements {
//!     use hypertext::define_mathml_elements;
//!     pub use hypertext::validation::hypertext_mathml_elements::*;
//!
//!     define_mathml_elements! {
//!         /// A custom MathML element.
//!         my_custom_element {
//!             /// A custom attribute.
//!             my_attr
//!         }
//!     }
//! }
//! ```
// Suppress deprecation warnings triggered by the `define_mathml_elements!` macro
// generating code that internally instantiates deprecated element structs.
#![expect(deprecated, clippy::too_long_first_doc_paragraph)]
use crate::define_mathml_elements;

define_mathml_elements! {
    /// The top-level element in MathML. Every valid MathML instance must be
    /// wrapped in it.
    math {
        /// Specifies the display rendering mode for the `<math>` element.
        /// `block` renders the element in its own block, while `inline`
        /// renders it inline.
        display
    }

    /// Used to attach an annotation to a MathML expression.
    annotation {
        /// Specifies the content type of the annotation (e.g. `application/x-tex`).
        encoding
    }

    /// Used to attach an XML annotation to a MathML expression.
    annotation_xml {
        /// Specifies the content type of the annotation.
        encoding
    }

    /// Indicates that its contents are errors. It wraps incorrect MathML
    /// so that it can still be displayed.
    merror

    /// Displays its argument as a fraction with optional attributes to
    /// control the appearance of the fraction line.
    mfrac {
        /// The thickness of the fraction line.
        linethickness
    }

    /// Represents an identifier such as a function name, variable, or
    /// symbolic constant.
    mi {
        /// Specifies the logical class of the identifier, which varies in
        /// typography.
        mathvariant
    }

    /// Represents a numeric literal.
    mn

    /// Represents an operator in a broad sense. Besides operators in strict
    /// mathematical meaning, this element also includes "operators" like
    /// parentheses, separators such as comma and semicolon, or "absolute
    /// value" bars.
    mo {
        /// Specifies whether the operator is an accent when used as an
        /// underscript or overscript.
        accent

        /// Specifies whether the operator is a fence (such as parentheses).
        fence

        /// Specifies the default value of the operator when it appears in a
        /// context.
        form

        /// Specifies whether the operator should be displayed as a large
        /// operator when in `display` style.
        largeop

        /// Specifies the leading space before the operator.
        lspace

        /// Specifies the maximum size of the operator when it is stretchy.
        maxsize

        /// Specifies the minimum size of the operator when it is stretchy.
        minsize

        /// Specifies whether the operator should move limits to sub/sup
        /// position in `display` mode.
        movablelimits

        /// Specifies the trailing space after the operator.
        rspace

        /// Specifies whether the operator is a separator (such as commas).
        separator

        /// Specifies whether the operator should stretch to the size of
        /// the adjacent elements.
        stretchy

        /// Specifies whether the operator is symmetric about the math axis.
        symmetric
    }

    /// Attaches an accent or a limit over an expression.
    mover {
        /// Specifies whether the overscript should be treated as an accent.
        accent
    }

    /// Adjusts the space around its content. It can change the visible
    /// space, or act as a spacing element.
    mpadded {
        /// Desired depth of the element.
        depth

        /// Desired height of the element.
        height

        /// Horizontal position of the content within the element.
        lspace

        /// Vertical position of the content within the element.
        voffset

        /// Desired width of the element.
        width
    }

    /// Makes its content invisible but keeps the space it would otherwise
    /// occupy. It is typically used to align expressions.
    mphantom

    /// Used as the first child in a [`mmultiscripts`] element to separate
    /// prescripts from postscripts.
    mprescripts

    /// Displays its arguments as a radical with an explicit index.
    mroot

    /// Groups sub-expressions together with no other associated visual
    /// notation. It is used to group elements together so that they can be
    /// treated as a unit (as when writing a sum over a multi-token base).
    mrow

    /// Represents contents that should be rendered as a literal string.
    ms

    /// Represents a blank space of the specified dimensions.
    mspace {
        /// Desired depth of the space.
        depth

        /// Desired height of the space.
        height

        /// Desired width of the space.
        width
    }

    /// Displays its arguments inside a square root, with no index.
    msqrt

    /// Changes the style of its contents.
    mstyle

    /// Attaches a subscript to an expression.
    msub

    /// Attaches both a subscript and a superscript to an expression.
    msubsup

    /// Attaches a superscript to an expression.
    msup

    /// Represents a two-dimensional array of mathematical expressions.
    mtable {
        /// Specifies vertical alignment of the table with respect to its
        /// environment.
        #[doc(alias = "non-standard")]
        align

        /// Specifies the horizontal alignment of the cells in each column.
        #[doc(alias = "non-standard")]
        columnalign

        /// Specifies a style for lines between columns.
        #[doc(alias = "non-standard")]
        columnlines

        /// Specifies the spacing between columns.
        #[doc(alias = "non-standard")]
        columnspacing

        /// Specifies the style for a frame enclosing the table.
        #[doc(alias = "non-standard")]
        frame

        /// Specifies the spacing between the table and its frame.
        #[doc(alias = "non-standard")]
        framespacing

        /// Specifies the vertical alignment of cells in each row.
        #[doc(alias = "non-standard")]
        rowalign

        /// Specifies a style for lines between rows.
        #[doc(alias = "non-standard")]
        rowlines

        /// Specifies the spacing between rows.
        #[doc(alias = "non-standard")]
        rowspacing

        /// Specifies the width of the table.
        #[doc(alias = "non-standard")]
        width
    }

    /// Represents one cell in a table or matrix.
    mtd {
        /// Specifies the number of columns spanned by the cell.
        columnspan

        /// Specifies the number of rows spanned by the cell.
        rowspan

        /// Specifies the horizontal alignment of this cell (non-standard).
        #[doc(alias = "non-standard")]
        columnalign

        /// Specifies the vertical alignment of this cell (non-standard).
        #[doc(alias = "non-standard")]
        rowalign
    }

    /// Represents arbitrary text that should be rendered as text.
    mtext

    /// Represents one row in a table or matrix.
    mtr {
        /// Specifies the horizontal alignment of cells in this row
        /// (non-standard).
        #[doc(alias = "non-standard")]
        columnalign

        /// Specifies the vertical alignment of cells in this row
        /// (non-standard).
        #[doc(alias = "non-standard")]
        rowalign
    }

    /// Attaches an underscript to an expression.
    munder {
        /// Specifies whether the underscript should be treated as an accent.
        accentunder
    }

    /// Attaches both an underscript and an overscript to an expression.
    munderover {
        /// Specifies whether the overscript should be treated as an accent.
        accent

        /// Specifies whether the underscript should be treated as an accent.
        accentunder
    }

    /// Attaches multiple subscripts and superscripts to an expression.
    mmultiscripts

    /// The top-level element used to attach semantic annotations to a
    /// MathML expression.
    semantics

    /// A non-standard element used to enclose a MathML expression with
    /// a symbolic notation.
    #[deprecated = "use other MathML elements instead"]
    menclose {
        /// Specifies the type of enclosure notation.
        notation
    }

    /// A deprecated element that used to attach behavioural semantics to an
    /// expression. Use `<semantics>` instead.
    #[deprecated = "use `semantics` instead"]
    maction {
        /// Specifies the kind of action.
        actiontype

        /// Specifies the selected sub-expression.
        selection
    }

    /// A non-standard and deprecated element that grouped a set of values
    /// and was rendered as a pair of fences.
    #[deprecated = "use `mo` instead"]
    mfenced {
        /// Specifies the opening fence character.
        open

        /// Specifies the closing fence character.
        close

        /// Specifies the separator characters.
        separators
    }
}
