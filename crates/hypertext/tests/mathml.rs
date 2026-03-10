//! Tests for MathML element and attribute rendering.
#![cfg(feature = "alloc")]

use hypertext::prelude::*;

#[test]
fn basic_mathml() {
    let maud_result = mathml::maud! {
        math {
            mi { "x" }
            mo { "+" }
            mn { "1" }
        }
    }
    .render();

    let rsx_result = mathml::rsx! {
        <math>
            <mi>x</mi>
            <mo>"+"</mo>
            <mn>1</mn>
        </math>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<math><mi>x</mi><mo>+</mo><mn>1</mn></math>"
        );
    }
}

#[test]
fn mathml_fraction() {
    let maud_result = mathml::maud! {
        math {
            mfrac {
                mi { "a" }
                mi { "b" }
            }
        }
    }
    .render();

    let rsx_result = mathml::rsx! {
        <math>
            <mfrac>
                <mi>a</mi>
                <mi>b</mi>
            </mfrac>
        </math>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<math><mfrac><mi>a</mi><mi>b</mi></mfrac></math>"
        );
    }
}

#[test]
fn mathml_sqrt() {
    let maud_result = mathml::maud! {
        math {
            msqrt {
                mi { "x" }
            }
        }
    }
    .render();

    let rsx_result = mathml::rsx! {
        <math>
            <msqrt>
                <mi>x</mi>
            </msqrt>
        </math>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<math><msqrt><mi>x</mi></msqrt></math>");
    }
}

#[test]
fn mathml_superscript() {
    let maud_result = mathml::maud! {
        math {
            msup {
                mi { "x" }
                mn { "2" }
            }
        }
    }
    .render();

    let rsx_result = mathml::rsx! {
        <math>
            <msup>
                <mi>x</mi>
                <mn>2</mn>
            </msup>
        </math>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<math><msup><mi>x</mi><mn>2</mn></msup></math>"
        );
    }
}

#[test]
fn mathml_complex_expression() {
    let maud_result = mathml::maud! {
        math {
            mrow {
                mi { "a" }
                mo { "+" }
                mfrac {
                    mi { "b" }
                    mi { "c" }
                }
            }
        }
    }
    .render();

    let rsx_result = mathml::rsx! {
        <math>
            <mrow>
                <mi>a</mi>
                <mo>"+"</mo>
                <mfrac>
                    <mi>b</mi>
                    <mi>c</mi>
                </mfrac>
            </mrow>
        </math>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<math><mrow><mi>a</mi><mo>+</mo><mfrac><mi>b</mi><mi>c</mi></mfrac></mrow></math>"
        );
    }
}

#[test]
fn xml_declaration_mathml_maud() {
    let result = mathml::maud! {
        !xml
        math {
            mi { "x" }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<?xml version="1.0" encoding="UTF-8"?><math><mi>x</mi></math>"#,
    );
}

#[test]
fn xml_declaration_mathml_rsx() {
    let result = mathml::rsx! {
        <?xml?>
        <math>
            <mi>x</mi>
        </math>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<?xml version="1.0" encoding="UTF-8"?><math><mi>x</mi></math>"#,
    );
}

#[test]
fn xml_declaration_mathml_parity() {
    let maud_result = mathml::maud! {
        !xml
        math { mi { "x" } }
    }
    .render();

    let rsx_result = mathml::rsx! {
        <?xml?>
        <math><mi>x</mi></math>
    }
    .render();

    assert_eq!(maud_result.as_inner(), rsx_result.as_inner());
}
