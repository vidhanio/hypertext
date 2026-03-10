//! Macro variant and syntax tests.
#![cfg(feature = "alloc")]

use hypertext::prelude::*;

#[test]
fn maud_rsx_parity_basic() {
    let maud_result = maud! { div { p { "hello" } } }.render();
    let rsx_result = rsx! { <div><p>hello</p></div> }.render();
    assert_eq!(maud_result, rsx_result);
}

#[test]
fn maud_rsx_parity_with_attributes() {
    let maud_result = maud! { a href="https://example.com" { "link" } }.render();
    let rsx_result = rsx! { <a href="https://example.com">link</a> }.render();
    assert_eq!(maud_result, rsx_result);
}

#[test]
fn maud_rsx_parity_void_elements() {
    let maud_result = maud! { br; hr; input type="text"; }.render();
    let rsx_result = rsx! { <br><hr><input type="text"> }.render();
    assert_eq!(maud_result, rsx_result);
}

#[test]
fn maud_rsx_parity_dynamic_content() {
    let name = "world";

    let maud_result = maud! { p { "hello " (name) } }.render();
    let rsx_result = rsx! { <p>"hello " (name)</p> }.render();
    assert_eq!(maud_result, rsx_result);
}

#[test]
fn maud_rsx_parity_nested_control_flow() {
    let items = [1, 2, 3];

    let maud_result = maud! {
        ul {
            @for item in items {
                li { (item) }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ul>
            @for item in items {
                <li>(item)</li>
            }
        </ul>
    }
    .render();

    assert_eq!(maud_result, rsx_result);
}

#[test]
fn maud_borrow_captures_by_reference() {
    let name = String::from("Alice");

    let result = maud::borrow! { p { (name) } }.render();
    assert_eq!(result.as_inner(), "<p>Alice</p>");

    assert_eq!(name, "Alice");
}

#[test]
fn rsx_borrow_captures_by_reference() {
    let name = String::from("Bob");

    let result = rsx::borrow! { <p>(name)</p> }.render();
    assert_eq!(result.as_inner(), "<p>Bob</p>");

    assert_eq!(name, "Bob");
}

#[test]
fn borrow_parity() {
    let value = String::from("test");

    let maud_result = maud::borrow! { span { (value) } }.render();
    let rsx_result = rsx::borrow! { <span>(value)</span> }.render();
    assert_eq!(maud_result, rsx_result);

    assert_eq!(value, "test");
}

#[test]
fn borrow_with_multiple_references() {
    let a = String::from("hello");
    let b = String::from("world");

    let result = maud::borrow! {
        div {
            span { (a) }
            span { (b) }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<div><span>hello</span><span>world</span></div>"
    );

    assert_eq!(a, "hello");
    assert_eq!(b, "world");
}

#[test]
fn maud_simple_basic() {
    let result = maud::simple! { div { p { "hello" } } };
    assert_eq!(*result.as_inner(), "<div><p>hello</p></div>");
}

#[test]
fn rsx_simple_basic() {
    let result = rsx::simple! { <div><p>hello</p></div> };
    assert_eq!(*result.as_inner(), "<div><p>hello</p></div>");
}

#[test]
fn simple_parity() {
    let maud_result = maud::simple! { h1 { "Title" } };
    let rsx_result = rsx::simple! { <h1>Title</h1> };
    assert_eq!(maud_result.as_inner(), rsx_result.as_inner());
}

#[test]
fn simple_in_const_context() {
    const MAUD_HTML: hypertext::Raw<&str> = maud::simple! {
        div .container {
            h1 { "Static Page" }
            p { "No dynamic content here." }
        }
    };

    const RSX_HTML: hypertext::Raw<&str> = rsx::simple! {
        <div class="container">
            <h1>"Static Page"</h1>
            <p>"No dynamic content here."</p>
        </div>
    };

    assert_eq!(MAUD_HTML.as_inner(), RSX_HTML.as_inner());
    assert_eq!(
        *MAUD_HTML.as_inner(),
        r#"<div class="container"><h1>Static Page</h1><p>No dynamic content here.</p></div>"#
    );
}

#[test]
fn simple_in_static_context() {
    static HTML: hypertext::Raw<&str> = maud::simple! {
        footer { "Copyright 2025" }
    };

    assert_eq!(*HTML.as_inner(), "<footer>Copyright 2025</footer>");
}

#[test]
fn simple_void_elements() {
    let result = maud::simple! { br; hr; br; };
    assert_eq!(*result.as_inner(), "<br><hr><br>");

    let rsx_result = rsx::simple! { <br><hr><br> };
    assert_eq!(*rsx_result.as_inner(), "<br><hr><br>");
}

#[test]
fn svg_maud_basic() {
    let result = svg::maud! {
        svg width="100" height="100" {
            circle cx="50" cy="50" r="40";
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<svg width="100" height="100"><circle cx="50" cy="50" r="40"/></svg>"#
    );
}

#[test]
fn svg_rsx_basic() {
    let result = svg::rsx! {
        <svg width="100" height="100">
            <circle cx="50" cy="50" r="40" />
        </svg>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<svg width="100" height="100"><circle cx="50" cy="50" r="40"/></svg>"#
    );
}

#[test]
fn svg_maud_rsx_parity() {
    let maud_result = svg::maud! {
        svg viewBox="0 0 100 100" {
            rect x="10" y="10" width="80" height="80" fill="blue";
        }
    }
    .render();

    let rsx_result = svg::rsx! {
        <svg viewBox="0 0 100 100">
            <rect x="10" y="10" width="80" height="80" fill="blue" />
        </svg>
    }
    .render();

    assert_eq!(maud_result, rsx_result);
}

#[test]
fn svg_borrow() {
    let color = String::from("red");

    let result = svg::maud::borrow! {
        svg {
            circle cx="50" cy="50" r="40" fill=(color);
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<svg><circle cx="50" cy="50" r="40" fill="red"/></svg>"#
    );

    assert_eq!(color, "red");
}

#[test]
fn svg_simple() {
    const SVG: hypertext::Raw<&str> = svg::maud::simple! {
        svg width="50" height="50" {
            rect width="50" height="50" fill="green";
        }
    };

    assert_eq!(
        *SVG.as_inner(),
        r#"<svg width="50" height="50"><rect width="50" height="50" fill="green"/></svg>"#
    );
}

#[test]
fn svg_rsx_simple() {
    const SVG: hypertext::Raw<&str> = svg::rsx::simple! {
        <svg width="50" height="50">
            <rect width="50" height="50" fill="green" />
        </svg>
    };

    assert_eq!(
        *SVG.as_inner(),
        r#"<svg width="50" height="50"><rect width="50" height="50" fill="green"/></svg>"#
    );
}

#[test]
fn mathml_maud_basic() {
    let result = mathml::maud! {
        math {
            mi { "x" }
            mo { "+" }
            mn { "1" }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<math><mi>x</mi><mo>+</mo><mn>1</mn></math>"
    );
}

#[test]
fn mathml_rsx_basic() {
    let result = mathml::rsx! {
        <math>
            <mi>x</mi>
            <mo>"+"</mo>
            <mn>1</mn>
        </math>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<math><mi>x</mi><mo>+</mo><mn>1</mn></math>"
    );
}

#[test]
fn mathml_maud_rsx_parity() {
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

    assert_eq!(maud_result, rsx_result);
}

#[test]
fn mathml_borrow() {
    let var_name = String::from("y");

    let result = mathml::maud::borrow! {
        math {
            mi { (var_name) }
        }
    }
    .render();

    assert_eq!(result.as_inner(), "<math><mi>y</mi></math>");
    assert_eq!(var_name, "y");
}

#[test]
fn mathml_simple() {
    const MATH: hypertext::Raw<&str> = mathml::maud::simple! {
        math {
            mi { "e" }
            mo { "=" }
            mi { "m" }
            msup {
                mi { "c" }
                mn { "2" }
            }
        }
    };

    assert_eq!(
        *MATH.as_inner(),
        "<math><mi>e</mi><mo>=</mo><mi>m</mi><msup><mi>c</mi><mn>2</mn></msup></math>"
    );
}

#[test]
fn attribute_basic() {
    let attr = attribute! { "hello world" };

    let result = maud! { div title=attr {} }.render();
    assert_eq!(result.as_inner(), r#"<div title="hello world"></div>"#);
}

#[test]
fn attribute_with_dynamic_content() {
    let name = "Alice";
    let attr = attribute! { "Hello, " (name) "!" };

    let result = maud! { div title=attr {} }.render();
    assert_eq!(result.as_inner(), r#"<div title="Hello, Alice!"></div>"#);
}

#[test]
fn attribute_with_control_flow() {
    let attr = attribute! { "x" @for i in 0..5 { (i) } };

    let result = maud! { div title=attr { "Hi!" } }.render();
    assert_eq!(result.as_inner(), r#"<div title="x01234">Hi!</div>"#);
}

#[test]
fn attribute_borrow() {
    let name = String::from("world");
    let attr = attribute::borrow! { "hello " (name) };

    let result = maud! { div title=attr {} }.render();
    assert_eq!(result.as_inner(), r#"<div title="hello world"></div>"#);

    assert_eq!(name, "world");
}

#[test]
fn attribute_simple() {
    const ATTR: hypertext::RawAttribute<&str> = attribute::simple! { "static-value" };

    let result = maud! { div title=ATTR {} }.render();
    assert_eq!(result.as_inner(), r#"<div title="static-value"></div>"#);
}

#[test]
fn attribute_simple_in_const() {
    const CLASS_VALUE: hypertext::RawAttribute<&str> = attribute::simple! { "btn btn-primary" };

    let result = maud! { button class=CLASS_VALUE { "Click" } }.render();
    assert_eq!(
        result.as_inner(),
        r#"<button class="btn btn-primary">Click</button>"#
    );
}

#[test]
fn maud_doctype() {
    let result = maud! {
        !DOCTYPE
        html {
            head {}
            body { "Hello" }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<!DOCTYPE html><html><head></head><body>Hello</body></html>"
    );
}

#[test]
fn rsx_doctype() {
    let result = rsx! {
        <!DOCTYPE html>
        <html>
            <head></head>
            <body>Hello</body>
        </html>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<!DOCTYPE html><html><head></head><body>Hello</body></html>"
    );
}

#[test]
fn doctype_parity() {
    let maud_result = maud! {
        !DOCTYPE
        html {
            head { title { "Test" } }
            body { p { "Content" } }
        }
    }
    .render();

    let rsx_result = rsx! {
        <!DOCTYPE html>
        <html>
            <head><title>Test</title></head>
            <body><p>Content</p></body>
        </html>
    }
    .render();

    assert_eq!(maud_result, rsx_result);
}

#[test]
fn simple_doctype() {
    const HTML: hypertext::Raw<&str> = maud::simple! {
        !DOCTYPE
        html {
            head {}
            body { "Static" }
        }
    };

    assert_eq!(
        *HTML.as_inner(),
        "<!DOCTYPE html><html><head></head><body>Static</body></html>"
    );
}

#[test]
fn svg_maud_xml_declaration() {
    let result = svg::maud! {
        !xml
        svg width="100" height="100" {}
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<?xml version="1.0" encoding="UTF-8"?><svg width="100" height="100"></svg>"#,
    );
}

#[test]
fn svg_rsx_xml_declaration() {
    let result = svg::rsx! {
        <?xml?>
        <svg width="100" height="100"></svg>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<?xml version="1.0" encoding="UTF-8"?><svg width="100" height="100"></svg>"#,
    );
}

#[test]
fn mathml_maud_xml_declaration() {
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
fn mathml_rsx_xml_declaration() {
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
fn xml_declaration_parity() {
    let maud_result = svg::maud! {
        !xml
        svg viewBox="0 0 100 100" {
            circle cx="50" cy="50" r="40";
        }
    }
    .render();

    let rsx_result = svg::rsx! {
        <?xml?>
        <svg viewBox="0 0 100 100">
            <circle cx="50" cy="50" r="40" />
        </svg>
    }
    .render();

    assert_eq!(maud_result, rsx_result);
}

#[test]
fn svg_simple_xml_declaration() {
    const SVG: hypertext::Raw<&str> = svg::maud::simple! {
        !xml
        svg width="50" height="50" {
            rect width="50" height="50" fill="red";
        }
    };

    assert_eq!(
        *SVG.as_inner(),
        r#"<?xml version="1.0" encoding="UTF-8"?><svg width="50" height="50"><rect width="50" height="50" fill="red"/></svg>"#,
    );
}

#[test]
fn maud_moves_by_default() {
    let name = String::from("moved");

    let _result = maud! { p { (name) } };

    // `name` has been moved into the closure — this should NOT compile:
    // let _ = name;
}

#[test]
fn rsx_moves_by_default() {
    let name = String::from("moved");

    let _result = rsx! { <p>(name)</p> };

    // `name` has been moved into the closure — this should NOT compile:
    // let _ = name;
}

#[test]
fn maud_empty() {
    let result = maud! {}.render();
    assert_eq!(result.as_inner(), "");
}

#[test]
fn rsx_empty() {
    let result = rsx! {}.render();
    assert_eq!(result.as_inner(), "");
}

#[test]
fn simple_empty() {
    const EMPTY: hypertext::Raw<&str> = maud::simple! {};
    assert_eq!(*EMPTY.as_inner(), "");
}

#[test]
fn render_returns_rendered_buffer() {
    let lazy = maud! { div { "test" } };
    let rendered = lazy.render();
    assert_eq!(rendered.as_inner(), "<div>test</div>");
}

#[test]
fn render_to_pushes_to_existing_buffer() {
    use hypertext::Buffer;

    let mut buf = Buffer::new();
    let lazy1 = maud! { p { "first" } };
    let lazy2 = maud! { p { "second" } };

    buf.push(lazy1);
    buf.push(lazy2);

    let rendered = buf.rendered();
    assert_eq!(rendered.as_inner(), "<p>first</p><p>second</p>");
}

#[test]
fn svg_rsx_borrow() {
    let fill = String::from("purple");

    let result = svg::rsx::borrow! {
        <svg>
            <circle cx="25" cy="25" r="20" fill=(fill) />
        </svg>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<svg><circle cx="25" cy="25" r="20" fill="purple"/></svg>"#,
    );

    assert_eq!(fill, "purple");
}

#[test]
fn mathml_rsx_borrow() {
    let var = String::from("z");

    let result = mathml::rsx::borrow! {
        <math>
            <mi>(var)</mi>
        </math>
    }
    .render();

    assert_eq!(result.as_inner(), "<math><mi>z</mi></math>");
    assert_eq!(var, "z");
}

#[test]
fn mathml_rsx_simple() {
    const MATH: hypertext::Raw<&str> = mathml::rsx::simple! {
        <math>
            <mi>a</mi>
            <mo>"+"</mo>
            <mi>b</mi>
        </math>
    };

    assert_eq!(
        *MATH.as_inner(),
        "<math><mi>a</mi><mo>+</mo><mi>b</mi></math>"
    );
}
