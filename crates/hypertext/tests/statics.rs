//! Tests for static and const HTML rendering.

use hypertext::{prelude::*, Raw, Rendered};

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

// ── Context switching in simple! (const/static) ──────────────────────────────

#[test]
fn simple_embedded_svg_maud() {
    // SVG inside HTML via maud::simple! — SVG children must use `/>` self-close.
    const RESULT: Raw<&str> = maud::simple! {
        div {
            svg width="100" height="100" {
                circle cx="50" cy="50" r="40" fill="red";
            }
        }
    };

    assert_eq!(
        RESULT.as_str(),
        r#"<div><svg width="100" height="100"><circle cx="50" cy="50" r="40" fill="red"/></svg></div>"#,
    );
}

#[test]
fn simple_embedded_svg_rsx() {
    // SVG inside HTML via rsx::simple! — SVG children must use `/>` self-close.
    const RESULT: Raw<&str> = rsx::simple! {
        <div>
            <svg width="100" height="100">
                <circle cx="50" cy="50" r="40" fill="red" />
            </svg>
        </div>
    };

    assert_eq!(
        RESULT.as_str(),
        r#"<div><svg width="100" height="100"><circle cx="50" cy="50" r="40" fill="red"/></svg></div>"#,
    );
}

#[test]
fn simple_embedded_svg_with_children_maud() {
    // SVG <g> with children inside HTML maud::simple!
    const RESULT: Raw<&str> = maud::simple! {
        svg viewBox="0 0 200 200" {
            g transform="translate(10,10)" {
                circle cx="50" cy="50" r="40";
            }
        }
    };

    assert_eq!(
        RESULT.as_str(),
        r#"<svg viewBox="0 0 200 200"><g transform="translate(10,10)"><circle cx="50" cy="50" r="40"/></g></svg>"#,
    );
}

#[test]
fn simple_embedded_math_maud() {
    // MathML inside HTML via maud::simple!
    const RESULT: Raw<&str> = maud::simple! {
        p {
            math {
                mi { "x" }
                mo { "+" }
                mn { "1" }
            }
        }
    };

    assert_eq!(
        RESULT.as_str(),
        "<p><math><mi>x</mi><mo>+</mo><mn>1</mn></math></p>"
    );
}

#[test]
fn simple_embedded_math_rsx() {
    // MathML inside HTML via rsx::simple!
    const RESULT: Raw<&str> = rsx::simple! {
        <p>
            <math>
                <mi>"x"</mi>
                <mo>"+"</mo>
                <mn>"1"</mn>
            </math>
        </p>
    };

    assert_eq!(
        RESULT.as_str(),
        "<p><math><mi>x</mi><mo>+</mo><mn>1</mn></math></p>"
    );
}

#[test]
fn simple_embedded_math_fraction_maud() {
    // MathML mfrac inside HTML maud::simple!
    const RESULT: Raw<&str> = maud::simple! {
        div {
            math {
                mfrac {
                    mi { "a" }
                    mi { "b" }
                }
            }
        }
    };

    assert_eq!(
        RESULT.as_str(),
        "<div><math><mfrac><mi>a</mi><mi>b</mi></mfrac></math></div>",
    );
}

#[test]
fn simple_svg_foreign_object_maud() {
    // foreignObject inside SVG switches back to HTML — void elements use `>` not
    // `/>`.
    const RESULT: Raw<&str> = maud::simple! {
        svg width="200" height="200" {
            foreignObject x="10" y="10" width="180" height="180" {
                div {
                    p { "Hello from HTML inside SVG" }
                    input type="text";
                }
            }
        }
    };

    assert_eq!(
        RESULT.as_str(),
        r#"<svg width="200" height="200"><foreignObject x="10" y="10" width="180" height="180"><div><p>Hello from HTML inside SVG</p><input type="text"></div></foreignObject></svg>"#,
    );
}
