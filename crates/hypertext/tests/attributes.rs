//! Attribute rendering and escaping.
#![cfg(feature = "alloc")]

use hypertext::{AttributeBuffer, Buffer, RawAttribute, prelude::*};

fn assert_pair(maud: &Rendered<String>, rsx: &Rendered<String>, expected: &str) {
    assert_eq!(maud, rsx);
    assert_eq!(maud.as_inner(), expected);
    assert_eq!(rsx.as_inner(), expected);
}

fn assert_attribute<T: Renderable<hypertext::context::AttributeValue>>(value: T, expected: &str) {
    assert_eq!(value.to_buffer().into_inner(), expected);

    let mut buffer = AttributeBuffer::new();
    value.render_to(&mut buffer);
    assert_eq!(buffer.into_inner(), expected);
}

#[test]
fn standard_attributes_render_in_both_macro_syntaxes() {
    let id = 5;
    let title = "A \"quoted\" title & note";

    let maud = maud! {
        div id={ "item-" (id) }
            title=(title)
            style="color: red; font-size: 14px;"
            aria-label="Example"
            data-id=(id) {
            "content"
        }
    }
    .render();
    let rsx = rsx! {
        <div id={ "item-" (id) }
            title=(title)
            style="color: red; font-size: 14px;"
            aria-label="Example"
            data-id=(id)>
            content
        </div>
    }
    .render();

    assert_pair(
        &maud,
        &rsx,
        r#"<div id="item-5" title="A &quot;quoted&quot; title &amp; note" style="color: red; font-size: 14px;" aria-label="Example" data-id="5">content</div>"#,
    );
}

fn render_optional_attributes(
    enabled: bool,
    title: Option<&str>,
) -> (Rendered<String>, Rendered<String>) {
    (
        maud! {
            input type="checkbox" checked[enabled] disabled[!enabled] title=[title];
        }
        .render(),
        rsx! {
            <input type="checkbox" checked[enabled] disabled[!enabled] title=[title]>
        }
        .render(),
    )
}

#[test]
fn boolean_and_option_attributes_form_a_presence_matrix() {
    for (enabled, title, expected) in [
        (
            true,
            Some("<tip>"),
            r#"<input type="checkbox" checked title="&lt;tip&gt;">"#,
        ),
        (false, None, r#"<input type="checkbox" disabled>"#),
    ] {
        let (maud, rsx) = render_optional_attributes(enabled, title);
        assert_pair(&maud, &rsx, expected);
    }
}

fn render_shorthand(active: bool, disabled: bool, extra: Option<&str>) -> Rendered<String> {
    maud! {
        button #post/1 .base .w-1/2 .active[active] .disabled[disabled] .[extra] {
            "Click"
        }
    }
    .render()
}

#[test]
fn maud_shorthands_cover_ids_classes_and_toggles() {
    for (active, disabled, extra, expected) in [
        (
            true,
            false,
            Some("highlight"),
            r#"<button id="post/1" class="base w-1/2 active highlight">Click</button>"#,
        ),
        (
            false,
            true,
            None,
            r#"<button id="post/1" class="base w-1/2 disabled">Click</button>"#,
        ),
    ] {
        assert_eq!(
            render_shorthand(active, disabled, extra).as_inner(),
            expected
        );
    }

    assert_eq!(
        maud! { div .active[false] { "content" } }
            .render()
            .as_inner(),
        r#"<div class="">content</div>"#,
    );
}

#[test]
fn node_and_attribute_escaping_protect_delimiters() {
    let value = r#"<script src="x"> & '"#;
    let maud = maud! { p title=(value) { (value) } }.render();
    let rsx = rsx! { <p title=(value)>(value)</p> }.render();

    assert_pair(
        &maud,
        &rsx,
        r#"<p title="&lt;script src=&quot;x&quot;&gt; &amp; '">&lt;script src="x"&gt; &amp; '</p>"#,
    );
    assert_attribute(value, "&lt;script src=&quot;x&quot;&gt; &amp; '");
}

#[test]
fn attribute_macros_cover_dynamic_borrowed_and_const_values() {
    const STATIC: RawAttribute<&str> = attribute::simple! { "static-value" };

    let value = String::from("a\"&b");
    let dynamic = attribute! { "prefix-" (value) "!" };
    assert_attribute(dynamic, "prefix-a&quot;&amp;b!");

    let borrowed_value = String::from("borrowed");
    let borrowed = attribute::borrow! { "prefix-" (borrowed_value) };
    assert_attribute(borrowed, "prefix-borrowed");
    assert_eq!(borrowed_value, "borrowed");

    assert_eq!(STATIC.as_str(), "static-value");
    assert_attribute(STATIC, "static-value");
}

#[test]
fn attribute_values_compose_with_element_syntax() {
    const STATIC: RawAttribute<&str> = attribute::simple! { "static" };

    let value = String::from("A\" & B");
    let dynamic = attribute! { "prefix-" (value) };
    let maud = maud! { div title=dynamic class=STATIC {} }.render();
    assert_eq!(
        maud.as_inner(),
        r#"<div title="prefix-A&quot; &amp; B" class="static"></div>"#,
    );

    let borrowed_value = String::from("borrowed");
    let borrowed = attribute::borrow! { "value-" (borrowed_value) };
    let rsx = rsx! { <div data-value=borrowed></div> }.render();
    assert_eq!(rsx.as_inner(), r#"<div data-value="value-borrowed"></div>"#);
    assert_eq!(borrowed_value, "borrowed");
}

#[test]
fn raw_attribute_values_are_not_escaped_twice() {
    // XSS SAFETY: the value is intentionally pre-escaped test data.
    let raw = RawAttribute::dangerously_create("&lt;safe&gt;&quot;");
    assert_attribute(raw, "&lt;safe&gt;&quot;");

    let rendered = maud! { div title=(raw) { "ok" } }.render();
    assert_eq!(
        rendered.as_inner(),
        r#"<div title="&lt;safe&gt;&quot;">ok</div>"#
    );
}

#[test]
fn attribute_buffer_uses_attribute_escaping() {
    let mut buffer = Buffer::<hypertext::context::AttributeValue>::new();
    buffer.push("<tag> & \"");
    assert_eq!(buffer.into_inner(), "&lt;tag&gt; &amp; &quot;");
}

#[cfg(feature = "opengraph")]
#[test]
fn opengraph_attributes_are_available_on_meta() {
    let maud = maud! { meta property="og:title" content="Hypertext"; }.render();
    let rsx = rsx! { <meta property="og:title" content="Hypertext"> }.render();
    assert_pair(
        &maud,
        &rsx,
        r#"<meta property="og:title" content="Hypertext">"#,
    );
}

#[cfg(feature = "htmx")]
#[test]
fn htmx_attribute_matrix_renders_in_both_syntaxes() {
    let maud = maud! {
        form hx-get="/search" hx-post="/login"
            hx-on:click="submit()" hx-on::after-request="reset()" {
            input name="query";
        }
    }
    .render();
    let rsx = rsx! {
        <form hx-get="/search" hx-post="/login"
            hx-on:click="submit()" hx-on::after-request="reset()">
            <input name="query">
        </form>
    }
    .render();

    assert_pair(
        &maud,
        &rsx,
        r#"<form hx-get="/search" hx-post="/login" hx-on:click="submit()" hx-on::after-request="reset()"><input name="query"></form>"#,
    );
}

#[cfg(feature = "alpine")]
#[test]
fn alpine_attribute_matrix_renders_in_both_syntaxes() {
    let maud = maud! {
        div x-data="{ open: false }" x-on:click="open = ! open"
            @click="open = ! open" :class="hidden" x-cloak {}
    }
    .render();
    let rsx = rsx! {
        <div x-data="{ open: false }" x-on:click="open = ! open"
            @click="open = ! open" :class="hidden" x-cloak></div>
    }
    .render();

    assert_pair(
        &maud,
        &rsx,
        r#"<div x-data="{ open: false }" x-on:click="open = ! open" @click="open = ! open" :class="hidden" x-cloak></div>"#,
    );
}

#[cfg(feature = "hyperscript")]
#[test]
fn hyperscript_attribute_values_are_escaped() {
    let maud = maud! {
        button _="on click put '<output/>' into me" { "Run" }
        output { "--" }
    }
    .render();
    let rsx = rsx! {
        <button _="on click put '<output/>' into me">Run</button>
        <output>"--"</output>
    }
    .render();

    assert_pair(
        &maud,
        &rsx,
        r#"<button _="on click put '&lt;output/&gt;' into me">Run</button><output>--</output>"#,
    );
}
