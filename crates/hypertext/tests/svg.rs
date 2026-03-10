//! Tests for SVG element and attribute rendering.
#![cfg(feature = "alloc")]

use hypertext::prelude::*;

#[test]
fn basic_svg() {
    let maud_result = svg::maud! {
        svg width="100" height="100" {
            circle cx="50" cy="50" r="40" fill="red";
        }
    }
    .render();

    let rsx_result = svg::rsx! {
        <svg width="100" height="100">
            <circle cx="50" cy="50" r="40" fill="red" />
        </svg>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<svg width="100" height="100"><circle cx="50" cy="50" r="40" fill="red"/></svg>"#,
        );
    }
}

#[test]
fn self_closing_vs_explicit() {
    // `;` in maud = self-closing `/>`, `{}` = explicit close tag
    let maud_self_close = svg::maud! { rect width="10" height="10"; }.render();
    let maud_explicit = svg::maud! { rect width="10" height="10" {} }.render();

    assert_eq!(
        maud_self_close.as_inner(),
        r#"<rect width="10" height="10"/>"#,
    );
    assert_eq!(
        maud_explicit.as_inner(),
        r#"<rect width="10" height="10"></rect>"#,
    );

    // `<foo/>` in rsx = self-closing, `<foo></foo>` = explicit close
    let rsx_self_close = svg::rsx! { <rect width="10" height="10" /> }.render();
    let rsx_explicit = svg::rsx! { <rect width="10" height="10"></rect> }.render();

    assert_eq!(
        rsx_self_close.as_inner(),
        r#"<rect width="10" height="10"/>"#,
    );
    assert_eq!(
        rsx_explicit.as_inner(),
        r#"<rect width="10" height="10"></rect>"#,
    );
}

#[test]
fn path_element() {
    let maud_result = svg::maud! {
        svg viewBox="0 0 100 100" {
            path d="M 10 10 H 90 V 90 H 10 Z" fill="none" stroke="black";
        }
    }
    .render();

    let rsx_result = svg::rsx! {
        <svg viewBox="0 0 100 100">
            <path d="M 10 10 H 90 V 90 H 10 Z" fill="none" stroke="black" />
        </svg>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<svg viewBox="0 0 100 100"><path d="M 10 10 H 90 V 90 H 10 Z" fill="none" stroke="black"/></svg>"#,
        );
    }
}

#[test]
fn nested_elements() {
    let maud_result = svg::maud! {
        svg viewBox="0 0 200 200" {
            defs {
                linearGradient id="grad" {
                    stop offset="0%" stop_color="red";
                    stop offset="100%" stop_color="blue";
                }
            }
            rect x="10" y="10" width="180" height="180" fill="url(#grad)";
        }
    }
    .render();

    let rsx_result = svg::rsx! {
        <svg viewBox="0 0 200 200">
            <defs>
                <linearGradient id="grad">
                    <stop offset="0%" stop_color="red" />
                    <stop offset="100%" stop_color="blue" />
                </linearGradient>
            </defs>
            <rect x="10" y="10" width="180" height="180" fill="url(#grad)" />
        </svg>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<svg viewBox="0 0 200 200"><defs><linearGradient id="grad"><stop offset="0%" stop_color="red"/><stop offset="100%" stop_color="blue"/></linearGradient></defs><rect x="10" y="10" width="180" height="180" fill="url(#grad)"/></svg>"#,
        );
    }
}

#[test]
fn with_text() {
    let maud_result = svg::maud! {
        svg viewBox="0 0 200 50" {
            text x="10" y="30" font_size="20" { "Hello SVG" }
        }
    }
    .render();

    let rsx_result = svg::rsx! {
        <svg viewBox="0 0 200 50">
            <text x="10" y="30" font_size="20">Hello SVG</text>
        </svg>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<svg viewBox="0 0 200 50"><text x="10" y="30" font_size="20">Hello SVG</text></svg>"#,
        );
    }
}

#[test]
fn empty_svg_element() {
    let maud_result = svg::maud! { svg {} }.render();
    let rsx_result = svg::rsx! { <svg></svg> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<svg></svg>");
    }
}

#[test]
fn svg_with_multiple_shapes() {
    let maud_result = svg::maud! {
        svg width="200" height="200" {
            circle cx="50" cy="50" r="30" fill="red";
            rect x="100" y="20" width="60" height="60" fill="blue";
            line x1="0" y1="0" x2="200" y2="200" stroke="green";
        }
    }
    .render();

    let rsx_result = svg::rsx! {
        <svg width="200" height="200">
            <circle cx="50" cy="50" r="30" fill="red" />
            <rect x="100" y="20" width="60" height="60" fill="blue" />
            <line x1="0" y1="0" x2="200" y2="200" stroke="green" />
        </svg>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<svg width="200" height="200"><circle cx="50" cy="50" r="30" fill="red"/><rect x="100" y="20" width="60" height="60" fill="blue"/><line x1="0" y1="0" x2="200" y2="200" stroke="green"/></svg>"#,
        );
    }
}

#[test]
fn dynamic_attributes() {
    let r = 25;
    let color = "blue";

    let maud_result = svg::maud! {
        svg {
            circle cx="50" cy="50" r=(r) fill=(color);
        }
    }
    .render();

    let rsx_result = svg::rsx! {
        <svg>
            <circle cx="50" cy="50" r=(r) fill=(color) />
        </svg>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<svg><circle cx="50" cy="50" r="25" fill="blue"/></svg>"#,
        );
    }
}

#[test]
fn mixed_static_and_dynamic_attributes() {
    let opacity = 0.5_f64;

    let maud_result = svg::maud! {
        svg {
            rect width="100" height="100" fill="red" opacity=(opacity);
        }
    }
    .render();

    let rsx_result = svg::rsx! {
        <svg>
            <rect width="100" height="100" fill="red" opacity=(opacity) />
        </svg>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<svg><rect width="100" height="100" fill="red" opacity="0.5"/></svg>"#,
        );
    }
}

#[test]
fn xml_declaration_maud() {
    let result = svg::maud! {
        !xml
        svg viewBox="0 0 100 100" {
            circle cx="50" cy="50" r="40";
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<?xml version="1.0" encoding="UTF-8"?><svg viewBox="0 0 100 100"><circle cx="50" cy="50" r="40"/></svg>"#,
    );
}

#[test]
fn xml_declaration_rsx() {
    let result = svg::rsx! {
        <?xml?>
        <svg viewBox="0 0 100 100">
            <circle cx="50" cy="50" r="40" />
        </svg>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<?xml version="1.0" encoding="UTF-8"?><svg viewBox="0 0 100 100"><circle cx="50" cy="50" r="40"/></svg>"#,
    );
}

#[test]
fn xml_declaration_parity() {
    let maud_result = svg::maud! {
        !xml
        svg { circle cx="50" cy="50" r="40"; }
    }
    .render();

    let rsx_result = svg::rsx! {
        <?xml?>
        <svg><circle cx="50" cy="50" r="40" /></svg>
    }
    .render();

    assert_eq!(maud_result.as_inner(), rsx_result.as_inner());
}
