//! Element validation and rendering across HTML, SVG, and MathML contexts.
#![cfg(feature = "alloc")]

use hypertext::prelude::*;

mod hypertext_elements {
    pub use hypertext::validation::hypertext_elements::*;
    use hypertext::{define_elements, define_void_elements};

    define_elements! {
        status_badge { severity }
        widget { label }
    }

    define_void_elements! {
        icon_element { name size }
    }
}

mod hypertext_svg_elements {
    use hypertext::define_svg_elements;
    pub use hypertext::validation::hypertext_svg_elements::*;

    define_svg_elements! {
        widget { radius }
        custom_shape { sides }
    }
}

mod hypertext_mathml_elements {
    use hypertext::define_mathml_elements;
    pub use hypertext::validation::hypertext_mathml_elements::*;

    define_mathml_elements! {
        widget { notation }
    }
}

macro_rules! assert_pair {
    ($maud:expr, $rsx:expr, $expected:expr $(,)?) => {{
        let maud = ($maud).render();
        let rsx = ($rsx).render();
        assert_eq!(maud, rsx);
        assert_eq!(maud.as_inner(), $expected);
    }};
}

#[test]
fn html_normal_void_and_document_nodes() {
    assert_pair!(
        maud! {
            !DOCTYPE
            main {
                h1 { "Hello" }
                img src="photo.png" alt="A & B";
                input disabled;
            }
        },
        rsx! {
            <!DOCTYPE html>
            <main>
                <h1>"Hello"</h1>
                <img src="photo.png" alt="A & B">
                <input disabled>
            </main>
        },
        r#"<!DOCTYPE html><main><h1>Hello</h1><img src="photo.png" alt="A &amp; B"><input disabled></main>"#,
    );
}

#[test]
fn html_empty_and_sibling_nodes_keep_their_boundaries() {
    assert_pair!(
        maud! { div {} p {} br; },
        rsx! { <div></div><p></p><br> },
        "<div></div><p></p><br>",
    );
}

#[test]
fn custom_html_elements_validate_attributes_and_voidness() {
    assert_pair!(
        maud! {
            status-badge severity="info" { "Ready" }
            icon-element name="check" size=16;
        },
        rsx! {
            <status-badge severity="info">Ready</status-badge>
            <icon-element name="check" size=16>
        },
        r#"<status-badge severity="info">Ready</status-badge><icon-element name="check" size="16">"#,
    );
}

#[test]
fn svg_supports_self_closing_and_nested_elements() {
    assert_pair!(
        svg::maud! {
            svg viewBox="0 0 10 10" {
                custom-shape sides=3 fill="red";
                g transform="translate(1 1)" {
                    circle cx=4 cy=4 r=2;
                }
            }
        },
        svg::rsx! {
            <svg viewBox="0 0 10 10">
                <custom-shape sides=3 fill="red" />
                <g transform="translate(1 1)">
                    <circle cx=4 cy=4 r=2 />
                </g>
            </svg>
        },
        r#"<svg viewBox="0 0 10 10"><custom-shape sides="3" fill="red"/><g transform="translate(1 1)"><circle cx="4" cy="4" r="2"/></g></svg>"#,
    );
}

#[test]
fn svg_distinguishes_self_closing_from_empty_elements() {
    assert_eq!(
        svg::maud! { rect width="10" height="10"; }
            .render()
            .as_inner(),
        r#"<rect width="10" height="10"/>"#,
    );
    assert_eq!(
        svg::maud! { rect width="10" height="10" {} }
            .render()
            .as_inner(),
        r#"<rect width="10" height="10"></rect>"#,
    );

    assert_eq!(
        svg::rsx! { <rect width="10" height="10" /> }
            .render()
            .as_inner(),
        r#"<rect width="10" height="10"/>"#,
    );
    assert_eq!(
        svg::rsx! { <rect width="10" height="10"></rect> }
            .render()
            .as_inner(),
        r#"<rect width="10" height="10"></rect>"#,
    );
}

#[test]
fn svg_dynamic_attributes_and_text_are_escaped() {
    let label = "<&";
    let size = 8;

    assert_pair!(
        svg::maud! { svg data-label=(label) { text x=(size) { (label) } } },
        svg::rsx! { <svg data-label=(label)><text x=(size)>(label)</text></svg> },
        r#"<svg data-label="&lt;&amp;"><text x="8">&lt;&amp;</text></svg>"#,
    );
}

#[test]
fn mathml_elements_render_with_xml_boundaries() {
    assert_pair!(
        mathml::maud! {
            math display="block" {
                mfrac {
                    mi { "x" }
                    msqrt { mn { 2 } }
                }
            }
        },
        mathml::rsx! {
            <math display="block">
                <mfrac>
                    <mi>x</mi>
                    <msqrt><mn>2</mn></msqrt>
                </mfrac>
            </math>
        },
        r#"<math display="block"><mfrac><mi>x</mi><msqrt><mn>2</mn></msqrt></mfrac></math>"#,
    );
}

#[test]
fn same_name_elements_resolve_from_the_active_context() {
    assert_pair!(
        maud! {
            div {
                widget label="html" { "HTML" }
                svg {
                    widget radius=5;
                    foreignObject width=10 height=10 {
                        widget label="nested" { "HTML again" }
                    }
                }
                math {
                    widget notation="math" { mn { 1 } }
                }
            }
        },
        rsx! {
            <div>
                <widget label="html">HTML</widget>
                <svg>
                    <widget radius=5 />
                    <foreignObject width=10 height=10>
                        <widget label="nested">HTML again</widget>
                    </foreignObject>
                </svg>
                <math>
                    <widget notation="math"><mn>1</mn></widget>
                </math>
            </div>
        },
        r#"<div><widget label="html">HTML</widget><svg><widget radius="5"/><foreignObject width="10" height="10"><widget label="nested">HTML again</widget></foreignObject></svg><math><widget notation="math"><mn>1</mn></widget></math></div>"#,
    );
}

#[test]
fn custom_elements_work_inside_foreign_object_and_svg() {
    assert_pair!(
        svg::maud! {
            svg {
                custom-shape sides=3;
                foreignObject {
                    status-badge severity="ok" { "inside" }
                }
            }
        },
        svg::rsx! {
            <svg>
                <custom-shape sides=3 />
                <foreignObject>
                    <status-badge severity="ok">inside</status-badge>
                </foreignObject>
            </svg>
        },
        r#"<svg><custom-shape sides="3"/><foreignObject><status-badge severity="ok">inside</status-badge></foreignObject></svg>"#,
    );
}
