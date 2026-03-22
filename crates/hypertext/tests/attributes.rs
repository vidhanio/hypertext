//! Attribute rendering tests.
#![cfg(feature = "alloc")]

use hypertext::prelude::*;

#[test]
fn static_string_attribute() {
    let maud_result = maud! {
        div title="hello" { "content" }
    }
    .render();

    let rsx_result = rsx! {
        <div title="hello">content</div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), r#"<div title="hello">content</div>"#);
    }
}

#[test]
fn style_attribute() {
    let maud_result = maud! {
        div style="color: red; font-size: 14px;" { "styled" }
    }
    .render();

    let rsx_result = rsx! {
        <div style="color: red; font-size: 14px;">styled</div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<div style="color: red; font-size: 14px;">styled</div>"#
        );
    }
}

#[test]
fn dynamic_attribute_values() {
    let name = "world";
    let size = 42;

    let maud_result = maud! {
        div title=(name) data-size=(size) { "hi" }
    }
    .render();

    let rsx_result = rsx! {
        <div title=(name) data-size=(size)>hi</div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<div title="world" data-size="42">hi</div>"#
        );
    }
}

#[test]
fn concatenated_attribute_values() {
    let id = 5;

    let maud_result = maud! {
        div id={ "item-" (id) } { "content" }
    }
    .render();

    let rsx_result = rsx! {
        <div id={ "item-" (id) }>content</div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), r#"<div id="item-5">content</div>"#);
    }
}

#[test]
fn boolean_attributes() {
    let maud_result = maud! {
        input type="checkbox" checked disabled;
    }
    .render();

    let rsx_result = rsx! {
        <input type="checkbox" checked disabled>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<input type="checkbox" checked disabled>"#
        );
    }
}

#[test]
fn option_toggle_some_includes_attribute() {
    let val = Some("tip");

    let maud_result = maud! { div title=[val] {} }.render();
    let rsx_result = rsx! { <div title=[val]></div> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), r#"<div title="tip"></div>"#);
    }
}

#[test]
fn option_toggle_none_omits_attribute() {
    let val = None::<&str>;

    let maud_result = maud! { div title=[val] {} }.render();
    let rsx_result = rsx! { <div title=[val]></div> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<div></div>");
    }
}

#[test]
fn option_toggle_both_variants() {
    let option_some = Some("value");
    let option_none = None::<&str>;

    let maud_result = maud! {
        input id=[option_some] type="checkbox" checked;
    }
    .render();

    let rsx_result = rsx! {
        <input id=[option_none] type="checkbox" checked />
    }
    .render();

    assert_eq!(
        maud_result.as_inner(),
        r#"<input id="value" type="checkbox" checked>"#
    );
    assert_eq!(rsx_result.as_inner(), r#"<input type="checkbox" checked>"#);
}

#[test]
fn class_id_shorthand_maud() {
    let result = maud! {
        div #profile .class:colon-dash {
            h1 { "Hello, world!" }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div id="profile" class="class:colon-dash"><h1>Hello, world!</h1></div>"#
    );
}

#[test]
fn class_shorthand_supports_slashes_maud() {
    let result = maud! {
        div .w-1/2 .sm:w-1/3 { "sized" }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div class="w-1/2 sm:w-1/3">sized</div>"#
    );
}

#[test]
fn id_shorthand_supports_slashes_maud() {
    let result = maud! {
        div #post/1 { "entry" }
    }
    .render();

    assert_eq!(result.as_inner(), r#"<div id="post/1">entry</div>"#);
}

#[test]
fn multiple_classes_maud() {
    let result = maud! {
        div .foo .bar .baz { "multi-class" }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div class="foo bar baz">multi-class</div>"#
    );
}

#[test]
fn boolean_class_toggle_true() {
    let is_active = true;

    let result = maud! {
        div .active[is_active] { "content" }
    }
    .render();

    assert_eq!(result.as_inner(), r#"<div class="active">content</div>"#);
}

#[test]
fn boolean_class_toggle_with_slash_true() {
    let is_half = true;

    let result = maud! {
        div .w-1/2[is_half] { "content" }
    }
    .render();

    assert_eq!(result.as_inner(), r#"<div class="w-1/2">content</div>"#);
}

#[test]
fn boolean_class_toggle_with_slash_false() {
    let is_half = false;

    let result = maud! {
        div .w-1/2[is_half] { "content" }
    }
    .render();

    assert_eq!(result.as_inner(), r#"<div class="">content</div>"#);
}

#[test]
fn boolean_class_toggle_false() {
    let is_active = false;

    let result = maud! {
        div .active[is_active] { "content" }
    }
    .render();

    assert_eq!(result.as_inner(), r#"<div class="">content</div>"#);
}

#[test]
fn multiple_class_toggles() {
    let primary = true;
    let disabled = false;
    let large = true;

    let result = maud! {
        button .primary[primary] .disabled[disabled] .large[large] { "Click" }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<button class="primary large">Click</button>"#
    );
}

#[test]
fn option_class_toggle() {
    let class_some = Some("highlight");
    let class_none = None::<&str>;

    let result = maud! {
        div .[class_some] .[class_none] { "content" }
    }
    .render();

    assert_eq!(result.as_inner(), r#"<div class="highlight">content</div>"#);
}

#[test]
fn option_class_toggle_with_slash_value() {
    let class_some = Some("w-1/2");
    let class_none = None::<&str>;

    let result = maud! {
        div .[class_some] .[class_none] { "content" }
    }
    .render();

    assert_eq!(result.as_inner(), r#"<div class="w-1/2">content</div>"#);
}

#[test]
fn aria_attributes() {
    let maud_result = maud! {
        div aria-label="Hello, world!" {
            "Hello, world!"
        }
    }
    .render();

    let rsx_result = rsx! {
        <div aria-label="Hello, world!">"Hello, world!"</div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<div aria-label="Hello, world!">Hello, world!</div>"#
        );
    }
}

#[test]
fn data_attributes() {
    let maud_result = maud! {
        div data-id="42" data-name="test" { "Content" }
    }
    .render();

    let rsx_result = rsx! {
        <div data-id="42" data-name="test">Content</div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<div data-id="42" data-name="test">Content</div>"#
        );
    }
}

#[test]
#[cfg(feature = "opengraph")]
fn opengraph_meta_property_attribute() {
    let maud_result = maud! {
        meta property="og:title" content="Hypertext";
    }
    .render();

    let rsx_result = rsx! {
        <meta property="og:title" content="Hypertext">
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<meta property="og:title" content="Hypertext">"#
        );
    }
}

#[test]
fn attribute_escaping_quotes() {
    let xss = r#""alert('XSS')"#;

    let result = maud! {
        div data-code=(xss) {}
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div data-code="&quot;alert('XSS')"></div>"#
    );
}

#[test]
fn attribute_escaping_quotes_and_ampersand() {
    let val = r#"a&b"c"#;

    let result = maud! {
        div title=(val) {}
    }
    .render();

    assert_eq!(result.as_inner(), r#"<div title="a&amp;b&quot;c"></div>"#);
}

#[test]
fn xss_prevention_in_attributes() {
    let xss = r#"" onmouseover="alert(1)"#;

    let result = maud! {
        a href=(xss) { "click" }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<a href="&quot; onmouseover=&quot;alert(1)">click</a>"#
    );
}

#[test]
fn attribute_macro_basic() {
    let class = "highlight";
    let result = attribute! { "container " (class) }.to_buffer().into_inner();
    assert_eq!(result, "container highlight");
}

#[test]
fn attribute_simple_const() {
    const ATTR: hypertext::RawAttribute<&str> = attribute::simple! { "static-value" };
    assert_eq!(ATTR.as_str(), "static-value");
}

#[test]
fn attribute_borrow() {
    let val = "dynamic".to_owned();
    let lazy = attribute::borrow! { "prefix-" (val) };
    assert_eq!(val, "dynamic");
    assert_eq!(lazy.to_buffer().into_inner(), "prefix-dynamic");
}

#[test]
fn attribute_escaping() {
    let val = r#"a"b&c"#;
    let result = attribute! { (val) }.to_buffer().into_inner();
    assert_eq!(result, "a&quot;b&amp;c");
}

#[test]
#[cfg(feature = "htmx")]
fn htmx_attributes() {
    let tests = [
        (
            maud! { div hx-get="/api/endpoint" { "Hello, world!" } }.render(),
            r#"<div hx-get="/api/endpoint">Hello, world!</div>"#,
        ),
        (
            rsx! { <div hx-get="/api/endpoint">"Hello, world!"</div> }.render(),
            r#"<div hx-get="/api/endpoint">Hello, world!</div>"#,
        ),
        (
            maud! { div hx-post="/api/endpoint" { "Hello, world!" } }.render(),
            r#"<div hx-post="/api/endpoint">Hello, world!</div>"#,
        ),
        (
            rsx! { <div hx-post="/api/endpoint">"Hello, world!"</div> }.render(),
            r#"<div hx-post="/api/endpoint">Hello, world!</div>"#,
        ),
        (
            maud! { div hx-on:click="this.classList.toggle('active')" { "Hello, world!" } }
                .render(),
            r#"<div hx-on:click="this.classList.toggle('active')">Hello, world!</div>"#,
        ),
        (
            rsx! { <div hx-on:click="this.classList.toggle('active')">"Hello, world!"</div> }
                .render(),
            r#"<div hx-on:click="this.classList.toggle('active')">Hello, world!</div>"#,
        ),
        (
            maud! {
                div {
                    form hx-post="/login" hx-on::after-request="this.reset()" {
                        input type="text" name="username";
                        input type="password" name="password";
                        input type="submit" value="Login";
                }}
            }
            .render(),
            r#"<div><form hx-post="/login" hx-on::after-request="this.reset()"><input type="text" name="username"><input type="password" name="password"><input type="submit" value="Login"></form></div>"#,
        ),
        (
            rsx! {
                <div>
                    <form hx-post="/login" hx-on::after-request="this.reset()">
                        <input type="text" name="username">
                        <input type="password" name="password">
                        <input type="submit" value="Login">
                    </form>
                </div>
            }
            .render(),
            r#"<div><form hx-post="/login" hx-on::after-request="this.reset()"><input type="text" name="username"><input type="password" name="password"><input type="submit" value="Login"></form></div>"#,
        ),
    ];

    for (result, expected) in tests {
        assert_eq!(result.as_inner(), expected);
    }
}

#[test]
#[cfg(feature = "alpine")]
fn alpine_attributes() {
    let tests = [
        (
            maud! { div x-data="{ open: false }" {} }.render(),
            r#"<div x-data="{ open: false }"></div>"#,
        ),
        (
            rsx! { <div x-data="{ open: false }"></div> }.render(),
            r#"<div x-data="{ open: false }"></div>"#,
        ),
        (
            maud! { div x-on:click="open = ! open" {} }.render(),
            r#"<div x-on:click="open = ! open"></div>"#,
        ),
        (
            rsx! { <div x-on:click="open = ! open"></div> }.render(),
            r#"<div x-on:click="open = ! open"></div>"#,
        ),
        (
            maud! { div @click="open = ! open" {} }.render(),
            r#"<div @click="open = ! open"></div>"#,
        ),
        (
            rsx! { <div @click="open = ! open"></div> }.render(),
            r#"<div @click="open = ! open"></div>"#,
        ),
        (
            maud! { div :class="! open ? 'hidden' : ''" {} }.render(),
            r#"<div :class="! open ? 'hidden' : ''"></div>"#,
        ),
        (
            rsx! { <div :class="! open ? 'hidden' : ''"></div> }.render(),
            r#"<div :class="! open ? 'hidden' : ''"></div>"#,
        ),
        (maud! { div x-cloak {} }.render(), r"<div x-cloak></div>"),
        (
            rsx! { <div x-cloak></div> }.render(),
            r"<div x-cloak></div>",
        ),
    ];

    for (test, expected) in tests {
        assert_eq!(test.as_inner(), expected);
    }
}

#[test]
#[cfg(feature = "hyperscript")]
fn hyperscript_attributes() {
    let results = [
        maud! {
            button _="on click increment :x then put result into the next <output/>" {
                "Click Me"
            }
            output { "--" }
        }
        .render(),
        rsx! {
            <button _="on click increment :x then put result into the next <output/>">
                Click Me
            </button>
            <output>"--"</output>
        }
        .render(),
    ];

    for result in results {
        assert_eq!(
            result.as_inner(),
            r#"<button _="on click increment :x then put result into the next &lt;output/&gt;">Click Me</button><output>--</output>"#,
        );
    }
}
