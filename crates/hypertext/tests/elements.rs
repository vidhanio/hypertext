#![cfg(feature = "alloc")]

use hypertext::prelude::*;

mod hypertext_elements {
    pub use hypertext::validation::hypertext_elements::*;
    use hypertext::{define_elements, define_void_elements};

    define_elements! {
        status_badge {
            severity
        }

        widget {
            label
        }
    }

    define_void_elements! {
        icon_element {
            name
            size
        }
    }
}

mod hypertext_svg_elements {
    use hypertext::define_svg_elements;
    pub use hypertext::validation::hypertext_svg_elements::*;

    define_svg_elements! {
        widget {
            radius
        }

        custom_shape {
            sides
        }
    }
}

mod hypertext_mathml_elements {
    use hypertext::define_mathml_elements;
    pub use hypertext::validation::hypertext_mathml_elements::*;

    define_mathml_elements! {
        widget {
            notation
        }
    }
}

#[test]
fn html_basic_element() {
    let maud_result = maud! { div { "hello" } }.render();
    let rsx_result = rsx! { <div>hello</div> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<div>hello</div>");
    }
}

#[test]
fn html_empty_element() {
    let maud_result = maud! { div {} }.render();
    let rsx_result = rsx! { <div></div> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<div></div>");
    }
}

#[test]
fn html_nested_elements() {
    let maud_result = maud! {
        div {
            p { span { "text" } }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            <p><span>text</span></p>
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<div><p><span>text</span></p></div>");
    }
}

#[test]
fn html_deeply_nested_elements() {
    let maud_result = maud! {
        div {
            section {
                article {
                    header {
                        h1 { "Deep" }
                    }
                }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            <section>
                <article>
                    <header>
                        <h1>Deep</h1>
                    </header>
                </article>
            </section>
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<div><section><article><header><h1>Deep</h1></header></article></section></div>"
        );
    }
}

#[test]
fn html_sibling_elements() {
    let maud_result = maud! {
        h1 { "Title" }
        p { "Paragraph" }
        footer { "Footer" }
    }
    .render();

    let rsx_result = rsx! {
        <h1>Title</h1>
        <p>Paragraph</p>
        <footer>Footer</footer>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<h1>Title</h1><p>Paragraph</p><footer>Footer</footer>"
        );
    }
}

#[test]
fn html_void_elements_in_form() {
    let maud_result = maud! {
        div {
            input type="text" name="username";
            input type="password" name="password";
            input type="submit" value="Login";
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            <input type="text" name="username">
            <input type="password" name="password">
            <input type="submit" value="Login">
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<div><input type="text" name="username"><input type="password" name="password"><input type="submit" value="Login"></div>"#
        );
    }
}

#[test]
fn html_multiple_void_elements() {
    let maud_result = maud! { br; hr; br; }.render();
    let rsx_result = rsx! { <br><hr><br> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<br><hr><br>");
    }
}

#[test]
fn html_img_void_element() {
    let maud_result = maud! {
        img src="photo.jpg" alt="A photo" width="200";
    }
    .render();

    let rsx_result = rsx! {
        <img src="photo.jpg" alt="A photo" width="200">
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<img src="photo.jpg" alt="A photo" width="200">"#
        );
    }
}

#[test]
fn html_meta_and_link_void_elements() {
    let maud_result = maud! {
        head {
            meta charset="utf-8";
            link rel="stylesheet" href="style.css";
        }
    }
    .render();

    let rsx_result = rsx! {
        <head>
            <meta charset="utf-8">
            <link rel="stylesheet" href="style.css">
        </head>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<head><meta charset="utf-8"><link rel="stylesheet" href="style.css"></head>"#
        );
    }
}

#[test]
fn html_mixed_void_and_normal_elements() {
    let maud_result = maud! {
        form {
            label for="email" { "Email:" }
            input type="email" name="email";
            label for="name" { "Name:" }
            input type="text" name="name";
            button type="submit" { "Submit" }
        }
    }
    .render();

    let rsx_result = rsx! {
        <form>
            <label for="email">"Email:"</label>
            <input type="email" name="email">
            <label for="name">"Name:"</label>
            <input type="text" name="name">
            <button type="submit">"Submit"</button>
        </form>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<form><label for="email">Email:</label><input type="email" name="email"><label for="name">Name:</label><input type="text" name="name"><button type="submit">Submit</button></form>"#
        );
    }
}

#[test]
fn svg_basic() {
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
fn svg_self_closing_vs_explicit_close() {
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
fn svg_path_element() {
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
fn svg_nested_elements() {
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
fn svg_with_text_content() {
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
fn svg_empty_element() {
    let maud_result = svg::maud! { svg {} }.render();
    let rsx_result = svg::rsx! { <svg></svg> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<svg></svg>");
    }
}

#[test]
fn svg_multiple_shapes() {
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
fn svg_dynamic_attributes() {
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
fn svg_mixed_static_and_dynamic_attributes() {
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
fn mathml_basic() {
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
fn html_embedded_svg_basic() {
    let maud_result = maud! {
        div {
            svg width="100" height="100" {
                circle cx="50" cy="50" r="40" fill="red";
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            <svg width="100" height="100">
                <circle cx="50" cy="50" r="40" fill="red" />
            </svg>
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<div><svg width="100" height="100"><circle cx="50" cy="50" r="40" fill="red"/></svg></div>"#,
        );
    }
}

#[test]
fn html_embedded_svg_self_closing() {
    let maud_result = maud! {
        svg viewBox="0 0 100 100" {
            rect x="10" y="10" width="80" height="80";
            line x1="0" y1="0" x2="100" y2="100" stroke="black";
        }
    }
    .render();

    let rsx_result = rsx! {
        <svg viewBox="0 0 100 100">
            <rect x="10" y="10" width="80" height="80" />
            <line x1="0" y1="0" x2="100" y2="100" stroke="black" />
        </svg>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<svg viewBox="0 0 100 100"><rect x="10" y="10" width="80" height="80"/><line x1="0" y1="0" x2="100" y2="100" stroke="black"/></svg>"#,
        );
    }
}

#[test]
fn html_embedded_svg_with_children() {
    let maud_result = maud! {
        div {
            svg viewBox="0 0 200 200" {
                g transform="translate(10,10)" {
                    circle cx="50" cy="50" r="40";
                }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            <svg viewBox="0 0 200 200">
                <g transform="translate(10,10)">
                    <circle cx="50" cy="50" r="40" />
                </g>
            </svg>
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<div><svg viewBox="0 0 200 200"><g transform="translate(10,10)"><circle cx="50" cy="50" r="40"/></g></svg></div>"#,
        );
    }
}

#[test]
fn html_embedded_svg_path() {
    let maud_result = maud! {
        svg viewBox="0 0 100 100" {
            path d="M 10 10 H 90 V 90 H 10 Z" fill="none" stroke="black";
        }
    }
    .render();

    let rsx_result = rsx! {
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
fn html_embedded_math_basic() {
    let maud_result = maud! {
        p {
            math {
                mi { "x" }
                mo { "+" }
                mn { "1" }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <p>
            <math>
                <mi>x</mi>
                <mo>"+"</mo>
                <mn>1</mn>
            </math>
        </p>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<p><math><mi>x</mi><mo>+</mo><mn>1</mn></math></p>"
        );
    }
}

#[test]
fn html_embedded_math_fraction() {
    let maud_result = maud! {
        div {
            math {
                mfrac {
                    mi { "a" }
                    mi { "b" }
                }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            <math>
                <mfrac>
                    <mi>a</mi>
                    <mi>b</mi>
                </mfrac>
            </math>
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<div><math><mfrac><mi>a</mi><mi>b</mi></mfrac></math></div>"
        );
    }
}

#[test]
fn svg_foreign_object_switches_to_html() {
    let maud_result = maud! {
        svg width="200" height="200" {
            foreignObject x="10" y="10" width="180" height="180" {
                div {
                    p { "Hello from HTML inside SVG" }
                    input type="text";
                }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <svg width="200" height="200">
            <foreignObject x="10" y="10" width="180" height="180">
                <div>
                    <p>Hello from HTML inside SVG</p>
                    <input type="text">
                </div>
            </foreignObject>
        </svg>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<svg width="200" height="200"><foreignObject x="10" y="10" width="180" height="180"><div><p>Hello from HTML inside SVG</p><input type="text"></div></foreignObject></svg>"#,
        );
    }
}

#[test]
fn deeply_nested_context_switches() {
    let maud_result = maud! {
        div {
            svg width="300" height="300" {
                rect x="0" y="0" width="300" height="300" fill="lightgray";
                foreignObject x="10" y="10" width="280" height="280" {
                    p { "Back in HTML!" }
                }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            <svg width="300" height="300">
                <rect x="0" y="0" width="300" height="300" fill="lightgray" />
                <foreignObject x="10" y="10" width="280" height="280">
                    <p>"Back in HTML!"</p>
                </foreignObject>
            </svg>
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<div><svg width="300" height="300"><rect x="0" y="0" width="300" height="300" fill="lightgray"/><foreignObject x="10" y="10" width="280" height="280"><p>Back in HTML!</p></foreignObject></svg></div>"#,
        );
    }
}

#[test]
fn custom_html_element() {
    let maud_result = maud! {
        status-badge severity="error" { "Something went wrong" }
    }
    .render();

    let rsx_result = rsx! {
        <status-badge severity="error">Something went wrong</status-badge>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<status-badge severity="error">Something went wrong</status-badge>"#,
        );
    }
}

#[test]
fn custom_html_element_with_global_attributes() {
    let maud_result = maud! {
        status-badge #my-badge.urgent severity="critical" { "Alert" }
    }
    .render();

    let rsx_result = rsx! {
        <status-badge id="my-badge" class="urgent" severity="critical">Alert</status-badge>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<status-badge id="my-badge" class="urgent" severity="critical">Alert</status-badge>"#,
        );
    }
}

#[test]
fn custom_void_element() {
    let maud_result = maud! {
        div {
            icon-element name="arrow" size="24";
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            <icon-element name="arrow" size="24">
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<div><icon-element name="arrow" size="24"></div>"#,
        );
    }
}

#[test]
fn custom_svg_element() {
    let maud_result = svg::maud! {
        svg viewBox="0 0 100 100" {
            custom-shape sides="6" fill="blue";
        }
    }
    .render();

    let rsx_result = svg::rsx! {
        <svg viewBox="0 0 100 100">
            <custom-shape sides="6" fill="blue" />
        </svg>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<svg viewBox="0 0 100 100"><custom-shape sides="6" fill="blue"/></svg>"#,
        );
    }
}

#[test]
fn custom_mathml_element() {
    let maud_result = mathml::maud! {
        math {
            widget notation="longdiv" {
                mn { "42" }
            }
        }
    }
    .render();

    let rsx_result = mathml::rsx! {
        <math>
            <widget notation="longdiv">
                <mn>42</mn>
            </widget>
        </math>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<math><widget notation="longdiv"><mn>42</mn></widget></math>"#,
        );
    }
}

#[test]
fn same_name_html_vs_embedded_svg() {
    let maud_result = maud! {
        div {
            widget label="html-context" { "HTML widget" }
            svg viewBox="0 0 100 100" {
                widget radius="50" fill="red";
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            <widget label="html-context">"HTML widget"</widget>
            <svg viewBox="0 0 100 100">
                <widget radius="50" fill="red" />
            </svg>
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            concat!(
                r#"<div>"#,
                r#"<widget label="html-context">HTML widget</widget>"#,
                r#"<svg viewBox="0 0 100 100">"#,
                r#"<widget radius="50" fill="red"/>"#,
                r#"</svg>"#,
                r#"</div>"#,
            ),
        );
    }
}

#[test]
fn same_name_html_vs_embedded_mathml() {
    let maud_result = maud! {
        p {
            widget label="text" { "prose" }
            math {
                widget notation="longdiv" {
                    mn { "7" }
                }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <p>
            <widget label="text">prose</widget>
            <math>
                <widget notation="longdiv">
                    <mn>7</mn>
                </widget>
            </math>
        </p>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            concat!(
                "<p>",
                r#"<widget label="text">prose</widget>"#,
                "<math>",
                r#"<widget notation="longdiv"><mn>7</mn></widget>"#,
                "</math>",
                "</p>",
            ),
        );
    }
}

#[test]
fn custom_svg_element_in_foreign_object_with_custom_html() {
    let maud_result = maud! {
        svg viewBox="0 0 200 200" {
            custom-shape sides="3" fill="green";
            foreignObject x="0" y="0" width="200" height="200" {
                status-badge severity="info" { "Inside foreignObject" }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <svg viewBox="0 0 200 200">
            <custom-shape sides="3" fill="green" />
            <foreignObject x="0" y="0" width="200" height="200">
                <status-badge severity="info">Inside foreignObject</status-badge>
            </foreignObject>
        </svg>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            concat!(
                r#"<svg viewBox="0 0 200 200">"#,
                r#"<custom-shape sides="3" fill="green"/>"#,
                r#"<foreignObject x="0" y="0" width="200" height="200">"#,
                r#"<status-badge severity="info">Inside foreignObject</status-badge>"#,
                r#"</foreignObject>"#,
                r#"</svg>"#,
            ),
        );
    }
}

#[test]
fn same_name_context_switch_through_foreign_object() {
    let maud_result = maud! {
        div {
            widget label="outer-html" {}
            svg viewBox="0 0 100 100" {
                widget radius="25" fill="blue";
                foreignObject x="0" y="0" width="100" height="100" {
                    widget label="inner-html" { "Back in HTML" }
                }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            <widget label="outer-html"></widget>
            <svg viewBox="0 0 100 100">
                <widget radius="25" fill="blue" />
                <foreignObject x="0" y="0" width="100" height="100">
                    <widget label="inner-html">Back in HTML</widget>
                </foreignObject>
            </svg>
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            concat!(
                r#"<div>"#,
                r#"<widget label="outer-html"></widget>"#,
                r#"<svg viewBox="0 0 100 100">"#,
                r#"<widget radius="25" fill="blue"/>"#,
                r#"<foreignObject x="0" y="0" width="100" height="100">"#,
                r#"<widget label="inner-html">Back in HTML</widget>"#,
                r#"</foreignObject>"#,
                r#"</svg>"#,
                r#"</div>"#,
            ),
        );
    }
}
