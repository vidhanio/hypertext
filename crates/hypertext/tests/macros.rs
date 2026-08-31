//! Macro variants, templates, and non-HTML contexts.
#![cfg(feature = "alloc")]

use hypertext::prelude::*;

macro_rules! assert_html_pair {
    ($left:expr, $right:expr, $expected:expr $(,)?) => {{
        let left = ($left).render();
        let right = ($right).render();
        assert_eq!(left.as_inner(), $expected);
        assert_eq!(right.as_inner(), $expected);
    }};
}

macro_rules! assert_context_pair {
    ($kind:ty, $left:expr, $right:expr, $expected:expr $(,)?) => {{
        let left = ($left).render::<$kind>();
        let right = ($right).render::<$kind>();
        assert_eq!(left.as_inner(), $expected);
        assert_eq!(right.as_inner(), $expected);
    }};
}

#[test]
fn maud_and_rsx_share_the_same_rendering_contract() {
    let name = "Ada";
    assert_html_pair!(
        maud! {
            div #profile .card data-kind="person" {
                h1 { "Hello, " (name) }
                input type="text" disabled;
            }
        },
        rsx! {
            <div id="profile" class="card" data-kind="person">
                <h1>"Hello, " (name)</h1>
                <input type="text" disabled>
            </div>
        },
        r#"<div id="profile" class="card" data-kind="person"><h1>Hello, Ada</h1><input type="text" disabled></div>"#,
    );
}

#[test]
fn borrow_variants_and_html_alias_preserve_references() {
    let name = String::from("Grace");
    let maud = maud::borrow! { p { (name) } }.render();
    let rsx = rsx::borrow! { <p>(name)</p> }.render();
    let html = html::borrow! { <p>(name)</p> }.render();

    assert_eq!(maud.as_inner(), "<p>Grace</p>");
    assert_eq!(rsx.as_inner(), maud.as_inner());
    assert_eq!(html.as_inner(), maud.as_inner());
    assert_eq!(name, "Grace");
}

#[test]
fn simple_variants_are_static_and_cross_syntax_parity() {
    const MAUD: hypertext::Raw<&str> = maud::simple! {
        main .page {
            h1 { "Static" }
            br;
        }
    };
    const RSX: hypertext::Raw<&str> = rsx::simple! {
        <main class="page"><h1>Static</h1><br></main>
    };
    const HTML: hypertext::Raw<&str> = html::simple! {
        <main class="page"><h1>Static</h1><br></main>
    };
    const ATTR: hypertext::RawAttribute<&str> = attribute::simple! {
        "btn btn-primary"
    };

    assert_eq!(
        MAUD.as_str(),
        r#"<main class="page"><h1>Static</h1><br></main>"#
    );
    assert_eq!(RSX.as_str(), MAUD.as_str());
    assert_eq!(HTML.as_str(), MAUD.as_str());
    assert_eq!(ATTR.as_str(), "btn btn-primary");
}

#[test]
fn file_variants_parse_external_rsx_and_track_borrowing() {
    let name = "Ada";
    let rsx = rsx::file!("tests/templates/variables.html").render();
    let html = html::file!("tests/templates/variables.html").render();
    assert_eq!(rsx.as_inner(), html.as_inner());
    assert_eq!(
        rsx.as_inner(),
        r#"<article data-kind="greeting"><h1>Ada</h1><p>Static body</p></article>"#,
    );

    let name = String::from("Grace");
    let borrowed = rsx::file_borrow!("tests/templates/variables.html").render();
    let borrowed_alias = html::file_borrow!("tests/templates/variables.html").render();
    assert_eq!(borrowed.as_inner(), borrowed_alias.as_inner());
    assert_eq!(
        borrowed.as_inner(),
        "<article data-kind=\"greeting\"><h1>Grace</h1><p>Static body</p></article>"
    );
    assert_eq!(name, "Grace");
}

#[test]
fn attribute_variants_render_dynamic_and_static_values() {
    const STATIC: hypertext::RawAttribute<&str> = attribute::simple! { "static" };

    let name = String::from("Ada");
    let dynamic_name = name.clone();
    let dynamic = attribute! {
        "Hello, " (dynamic_name) "!"
        @if true { " active" }
    };
    let borrowed = attribute::borrow! { "Hello, " (name) };

    assert_eq!(dynamic.to_buffer().into_inner(), "Hello, Ada! active");
    assert_eq!(borrowed.to_buffer().into_inner(), "Hello, Ada");
    assert_eq!(STATIC.as_str(), "static");
    assert_eq!(name, "Ada");
}

#[test]
fn document_directives_match_between_maud_and_rsx() {
    assert_html_pair!(
        maud! {
            !DOCTYPE
            html { head {} body { "Hello" } }
        },
        rsx! {
            <!DOCTYPE html>
            <html><head></head><body>Hello</body></html>
        },
        "<!DOCTYPE html><html><head></head><body>Hello</body></html>",
    );

    assert_context_pair!(
        hypertext::context::Svg,
        svg::maud! {
            !xml
            svg { circle cx="1" cy="2" r="1"; }
        },
        svg::rsx! {
            <?xml?>
            <svg><circle cx="1" cy="2" r="1" /></svg>
        },
        r#"<?xml version="1.0" encoding="UTF-8"?><svg><circle cx="1" cy="2" r="1"/></svg>"#,
    );

    assert_context_pair!(
        hypertext::context::MathMl,
        mathml::maud! {
            !xml
            math { mi { "x" } }
        },
        mathml::rsx! {
            <?xml?>
            <math><mi>x</mi></math>
        },
        r#"<?xml version="1.0" encoding="UTF-8"?><math><mi>x</mi></math>"#,
    );
}

#[test]
fn static_variants_preserve_markup_document_directives() {
    const HTML: hypertext::Raw<&str> = maud::simple! {
        !DOCTYPE
        html { body { "Static" } }
    };
    const SVG: hypertext::Raw<&str, hypertext::context::SvgNode> = svg::rsx::simple! {
        <?xml?>
        <svg></svg>
    };
    const MATHML: hypertext::Raw<&str, hypertext::context::MathMlNode> = mathml::maud::simple! {
        !xml
        math { mi { "x" } }
    };

    assert_eq!(
        HTML.as_str(),
        "<!DOCTYPE html><html><body>Static</body></html>",
    );
    assert_eq!(
        SVG.as_str(),
        r#"<?xml version="1.0" encoding="UTF-8"?><svg></svg>"#,
    );
    assert_eq!(
        MATHML.as_str(),
        r#"<?xml version="1.0" encoding="UTF-8"?><math><mi>x</mi></math>"#,
    );
}

#[test]
fn svg_and_mathml_variants_validate_their_own_contexts() {
    assert_context_pair!(
        hypertext::context::Svg,
        svg::maud! {
            svg viewBox="0 0 10 10" {
                rect x="1" y="1" width="8" height="8" fill="red";
            }
        },
        svg::rsx! {
            <svg viewBox="0 0 10 10">
                <rect x="1" y="1" width="8" height="8" fill="red" />
            </svg>
        },
        r#"<svg viewBox="0 0 10 10"><rect x="1" y="1" width="8" height="8" fill="red"/></svg>"#,
    );

    assert_context_pair!(
        hypertext::context::MathMl,
        mathml::maud! {
            math { mfrac { mn { "1" } mn { "2" } } }
        },
        mathml::rsx! {
            <math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>
        },
        "<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>",
    );
}

#[test]
fn empty_documents_are_valid_for_lazy_and_simple_variants() {
    const EMPTY: hypertext::Raw<&str> = maud::simple! {};

    assert_eq!(maud! {}.render().as_inner(), "");
    assert_eq!(rsx! {}.render().as_inner(), "");
    assert_eq!(EMPTY.as_str(), "");
}

#[test]
fn namespace_variants_cover_borrow_and_simple_contexts() {
    const SVG: hypertext::Raw<&str, hypertext::context::SvgNode> = svg::rsx::simple! {
        <svg><circle cx="1" cy="1" r="1" /></svg>
    };
    const MATHML: hypertext::Raw<&str, hypertext::context::MathMlNode> =
        mathml::maud::simple! { math { mi { "x" } } };

    let color = String::from("red");
    let svg_borrowed = svg::rsx::borrow! {
        <svg><circle fill=(color) /></svg>
    }
    .render();
    assert_eq!(
        svg_borrowed.as_inner(),
        r#"<svg><circle fill="red"/></svg>"#,
    );
    assert_eq!(color, "red");

    assert_eq!(SVG.as_str(), r#"<svg><circle cx="1" cy="1" r="1"/></svg>"#);

    let expression = String::from("x");
    let mathml_borrowed = mathml::rsx::borrow! {
        <math><mi>(expression)</mi></math>
    }
    .render();
    assert_eq!(mathml_borrowed.as_inner(), "<math><mi>x</mi></math>");
    assert_eq!(expression, "x");

    assert_eq!(MATHML.as_str(), "<math><mi>x</mi></math>");
}

#[test]
fn html_alias_and_render_to_preserve_macro_output() {
    let rsx = rsx! { <p>"value"</p> };
    let html = html! { <p>"value"</p> };
    assert_eq!(rsx.render().as_inner(), html.render().as_inner());

    let mut buffer = hypertext::Buffer::new();
    rsx.render_to(&mut buffer);
    assert_eq!(buffer.into_inner(), "<p>value</p>");

    let mut buffer = hypertext::Buffer::new();
    html.render_to(&mut buffer);
    assert_eq!(buffer.into_inner(), "<p>value</p>");
}
