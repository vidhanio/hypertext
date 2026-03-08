#![expect(missing_docs)]

use hypertext::{Raw, Rendered, prelude::*};

#[test]
fn maud_simple_static() {
    const RESULT: Raw<&str> = maud::simple! {
        div #profile title="Profile" {
            h1 { "Hello, world!" }
        }
    };

    assert_eq!(
        RESULT.into_inner(),
        r#"<div id="profile" title="Profile"><h1>Hello, world!</h1></div>"#
    );
}

#[test]
fn rsx_simple_static() {
    const RESULT: Raw<&str> = rsx::simple! {
        <div id="profile" title="Profile">
            <h1>"Hello, world!"</h1>
        </div>
    };

    assert_eq!(
        RESULT.into_inner(),
        r#"<div id="profile" title="Profile"><h1>Hello, world!</h1></div>"#
    );
}

#[test]
fn maud_and_rsx_simple_parity() {
    const MAUD_RAW_RESULT: Raw<&str> = maud::simple! {
        div #profile title="Profile" {
            h1 { "Hello, world!" }
        }
    };
    const RSX_RAW_RESULT: Raw<&str> = rsx::simple! {
        <div id="profile" title="Profile">
            <h1>"Hello, world!"</h1>
        </div>
    };

    const MAUD_RENDERED_RESULT: Rendered<&str> = MAUD_RAW_RESULT.rendered();
    const RSX_RENDERED_RESULT: Rendered<&str> = RSX_RAW_RESULT.rendered();

    const EXPECTED: &str = r#"<div id="profile" title="Profile"><h1>Hello, world!</h1></div>"#;

    for result in [MAUD_RAW_RESULT, RSX_RAW_RESULT] {
        assert_eq!(result.into_inner(), EXPECTED);
    }

    for result in [MAUD_RENDERED_RESULT, RSX_RENDERED_RESULT] {
        assert_eq!(result.into_inner(), EXPECTED);
    }
}

#[test]
fn simple_const_context() {
    const HTML: Raw<&str> = maud::simple! {
        p { "constant" }
    };

    const _: &str = HTML.into_inner();
    assert_eq!(HTML.into_inner(), "<p>constant</p>");
}

#[test]
fn simple_empty() {
    const EMPTY: Raw<&str> = maud::simple! {};
    assert_eq!(EMPTY.as_str(), "");
}

#[test]
fn svg_simple() {
    const SVG: Raw<&str> = svg::maud::simple! {
        svg width="100" height="100" {
            circle cx="50" cy="50" r="40";
        }
    };

    assert_eq!(
        SVG.as_str(),
        r#"<svg width="100" height="100"><circle cx="50" cy="50" r="40"/></svg>"#
    );
}

#[test]
fn mathml_simple() {
    const MATH: Raw<&str> = mathml::maud::simple! {
        math {
            mi { "x" }
            mo { "+" }
            mn { "1" }
        }
    };

    assert_eq!(MATH.as_str(), "<math><mi>x</mi><mo>+</mo><mn>1</mn></math>");
}
