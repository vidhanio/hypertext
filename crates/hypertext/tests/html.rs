#![cfg(feature = "alloc")]
#![expect(missing_docs)]

use std::fmt::{self, Display, Formatter};

use hypertext::prelude::*;

#[test]
fn void_elements() {
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
fn basic_element() {
    let maud_result = maud! { div { "hello" } }.render();
    let rsx_result = rsx! { <div>hello</div> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<div>hello</div>");
    }
}

#[test]
fn nested_elements() {
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
fn empty_element() {
    let maud_result = maud! { div {} }.render();
    let rsx_result = rsx! { <div></div> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<div></div>");
    }
}

#[test]
fn deeply_nested_elements() {
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
fn sibling_elements() {
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
fn multiple_void_elements() {
    let maud_result = maud! {
        br;
        hr;
        br;
    }
    .render();

    let rsx_result = rsx! {
        <br>
        <hr>
        <br>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<br><hr><br>");
    }
}

#[test]
fn mixed_void_and_normal_elements() {
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
fn img_void_element() {
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
fn meta_and_link_void_elements() {
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
                }
            }}
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
#[allow(clippy::too_many_lines)]
fn alpine_attributes() {
    let tests = [
        (
            maud! { div x-data="{ open: false }" { "Hello, world!" } }.render(),
            r#"<div x-data="{ open: false }">Hello, world!</div>"#,
        ),
        (
            rsx! { <div x-data="{ open: false }">"Hello, world!"</div> }.render(),
            r#"<div x-data="{ open: false }">Hello, world!</div>"#,
        ),
        (
            maud! { div x-bind:class="! open ? 'hidden' : ''" { "Hello, world!" } }.render(),
            r#"<div x-bind:class="! open ? 'hidden' : ''">Hello, world!</div>"#,
        ),
        (
            rsx! { <div x-bind:class="! open ? 'hidden' : ''">"Hello, world!"</div> }.render(),
            r#"<div x-bind:class="! open ? 'hidden' : ''">Hello, world!</div>"#,
        ),
        (
            maud! { div :class="! open ? 'hidden' : ''" { "Hello, world!" } }.render(),
            r#"<div :class="! open ? 'hidden' : ''">Hello, world!</div>"#,
        ),
        (
            rsx! { <div :class="! open ? 'hidden' : ''">"Hello, world!"</div> }.render(),
            r#"<div :class="! open ? 'hidden' : ''">Hello, world!</div>"#,
        ),
        (
            maud! { div x-on:click="open = ! open" { "Hello, world!" } }.render(),
            r#"<div x-on:click="open = ! open">Hello, world!</div>"#,
        ),
        (
            rsx! { <div x-on:click="open = ! open">"Hello, world!"</div> }.render(),
            r#"<div x-on:click="open = ! open">Hello, world!</div>"#,
        ),
        (
            maud! { div @click="open = ! open" { "Hello, world!" } }.render(),
            r#"<div @click="open = ! open">Hello, world!</div>"#,
        ),
        (
            rsx! { <div @click="open = ! open">"Hello, world!"</div> }.render(),
            r#"<div @click="open = ! open">Hello, world!</div>"#,
        ),
        (
            maud! { div @click.shift="open = ! open" { "Hello, world!" } }.render(),
            r#"<div @click.shift="open = ! open">Hello, world!</div>"#,
        ),
        (
            rsx! { <div @click.shift="open = ! open">"Hello, world!"</div> }.render(),
            r#"<div @click.shift="open = ! open">Hello, world!</div>"#,
        ),
        (
            maud! { input type="text" @keyup.enter="alert('Submitted!')"; }.render(),
            r#"<input type="text" @keyup.enter="alert('Submitted!')">"#,
        ),
        (
            rsx! { <input type="text" @keyup.enter="alert('Submitted!')" /> }.render(),
            r#"<input type="text" @keyup.enter="alert('Submitted!')">"#,
        ),
        (
            maud! { input type="text" @keyup.shift.enter="alert('Submitted!')"; }.render(),
            r#"<input type="text" @keyup.shift.enter="alert('Submitted!')">"#,
        ),
        (
            rsx! { <input type="text" @keyup.shift.enter="alert('Submitted!')" /> }.render(),
            r#"<input type="text" @keyup.shift.enter="alert('Submitted!')">"#,
        ),
        (
            maud! { div x-text="new Date().getFullYear()" { "Hello, world!" } }.render(),
            r#"<div x-text="new Date().getFullYear()">Hello, world!</div>"#,
        ),
        (
            rsx! { <div x-text="new Date().getFullYear()">"Hello, world!"</div> }.render(),
            r#"<div x-text="new Date().getFullYear()">Hello, world!</div>"#,
        ),
        (
            maud! { div x-html="(await axios.get('/some/html/partial')).data" { "Hello, world!" } }.render(),
            r#"<div x-html="(await axios.get('/some/html/partial')).data">Hello, world!</div>"#,
        ),
        (
            rsx! { <div x-html="(await axios.get('/some/html/partial')).data">"Hello, world!"</div> }.render(),
            r#"<div x-html="(await axios.get('/some/html/partial')).data">Hello, world!</div>"#,
        ),
        (
            maud! { input type="text" x-model="search"; }.render(),
            r#"<input type="text" x-model="search">"#,
        ),
        (
            rsx! { <input type="text" x-model="search" /> }.render(),
            r#"<input type="text" x-model="search">"#,
        ),
        (
            maud! { div x-show="open" { "Hello, world!" } }.render(),
            r#"<div x-show="open">Hello, world!</div>"#,
        ),
        (
            rsx! { <div x-show="open">"Hello, world!"</div> }.render(),
            r#"<div x-show="open">Hello, world!</div>"#,
        ),
        (
            maud! { div x-show="open" x-transition { "Hello, world!" } }.render(),
            r#"<div x-show="open" x-transition>Hello, world!</div>"#,
        ),
        (
            rsx! { <div x-show="open" x-transition>"Hello, world!"</div> }.render(),
            r#"<div x-show="open" x-transition>Hello, world!</div>"#,
        ),
        (
            maud! {
                template x-for="post in posts" {
                    h2 x-text="post.title" {}
                }
            }.render(),
            r#"<template x-for="post in posts"><h2 x-text="post.title"></h2></template>"#,
        ),
        (
            rsx! {
                <template x-for="post in posts">
                    <h2 x-text="post.title"></h2>
                </template>
            }.render(),
            r#"<template x-for="post in posts"><h2 x-text="post.title"></h2></template>"#,
        ),
        (
            maud! {
                template x-if="open" {
                    h2 x-text="post.title" {}
                }
            }.render(),
            r#"<template x-if="open"><h2 x-text="post.title"></h2></template>"#,
        ),
        (
            rsx! {
                <template x-if="open">
                    <h2 x-text="post.title"></h2>
                </template>
            }.render(),
            r#"<template x-if="open"><h2 x-text="post.title"></h2></template>"#,
        ),
        (
            maud! { div x-init="date = new Date()" {} }.render(),
            r#"<div x-init="date = new Date()"></div>"#,
        ),
        (
            rsx! { <div x-init="date = new Date()"></div> }.render(),
            r#"<div x-init="date = new Date()"></div>"#,
        ),
        (
            maud! { div x-effect="console.log('Count is '+count)" {} }.render(),
            r#"<div x-effect="console.log('Count is '+count)"></div>"#,
        ),
        (
            rsx! { <div x-effect="console.log('Count is '+count)"></div> }.render(),
            r#"<div x-effect="console.log('Count is '+count)"></div>"#,
        ),
        (
            maud! { input type="text" x-ref="content"; }.render(),
            r#"<input type="text" x-ref="content">"#,
        ),
        (
            rsx! { <input type="text" x-ref="content" /> }.render(),
            r#"<input type="text" x-ref="content">"#,
        ),
        (
            maud! { div x-cloak {} }.render(),
            r"<div x-cloak></div>",
        ),
        (
            rsx! { <div x-cloak></div> }.render(),
            r"<div x-cloak></div>",
        ),
        (
            maud! { div x-ignore {} }.render(),
            r"<div x-ignore></div>",
        ),
        (
            rsx! { <div x-ignore></div> }.render(),
            r"<div x-ignore></div>",
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
fn attribute_escaping() {
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
fn text_escaping_html_entities() {
    let text = "<script>alert('xss')</script>";

    let maud_result = maud! { div { (text) } }.render();
    let rsx_result = rsx! { <div>(text)</div> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<div>&lt;script&gt;alert('xss')&lt;/script&gt;</div>"
        );
    }
}

#[test]
fn text_escaping_ampersand() {
    let text = "Tom & Jerry";

    let maud_result = maud! { p { (text) } }.render();
    let rsx_result = rsx! { <p>(text)</p> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<p>Tom &amp; Jerry</p>");
    }
}

#[test]
fn text_escaping_quotes() {
    let text = r#"He said "hello""#;

    let maud_result = maud! { p { (text) } }.render();
    let rsx_result = rsx! { <p>(text)</p> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), r#"<p>He said "hello"</p>"#);
    }
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
fn static_text_not_escaped() {
    let maud_result = maud! {
        p { "Safe <b>text</b>" }
    }
    .render();

    // Static string literals in maud are NOT treated as HTML — they are literal
    // text. The angle brackets in a literal string ARE escaped.
    assert_eq!(
        maud_result.as_inner(),
        "<p>Safe &lt;b&gt;text&lt;/b&gt;</p>"
    );
}

#[test]
fn multiple_special_chars_in_text() {
    let text = "<>&\"'";

    let maud_result = maud! { span { (text) } }.render();
    let rsx_result = rsx! { <span>(text)</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>&lt;&gt;&amp;\"'</span>");
    }
}

#[test]
fn multiline_unindent() {
    let result = maud! {
        div title="
        multiline
        title
        " {
            "
            in
                out
            in
            "
        }
        "\n"
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div title="multiline
title">in
    out
in</div>
"#
    );
}

#[test]
fn string_literal() {
    let maud_result = maud! { span { "hello world" } }.render();
    let rsx_result = rsx! { <span>hello world</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>hello world</span>");
    }
}

#[test]
fn integer_literal() {
    let maud_result = maud! { span { (42) } }.render();
    let rsx_result = rsx! { <span>(42)</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>42</span>");
    }
}

#[test]
#[expect(clippy::approx_constant)]
fn float_literal() {
    let maud_result = maud! { span { (3.14_f64) } }.render();
    let rsx_result = rsx! { <span>(3.14_f64)</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>3.14</span>");
    }
}

#[test]
fn bool_literal() {
    let maud_result = maud! { span { (true) } }.render();
    let rsx_result = rsx! { <span>(true)</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>true</span>");
    }
}

#[test]
fn char_literal() {
    let maud_result = maud! { span { ('A') } }.render();
    let rsx_result = rsx! { <span>('A')</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>A</span>");
    }
}

#[test]
fn empty_string_literal() {
    let maud_result = maud! { span { "" } }.render();
    let rsx_result = rsx! { <span></span> }.render();

    assert_eq!(maud_result.as_inner(), "<span></span>");
    assert_eq!(rsx_result.as_inner(), "<span></span>");
}

#[test]
fn adjacent_string_literals() {
    let maud_result = maud! {
        span { "hello " "world" }
    }
    .render();

    assert_eq!(maud_result.as_inner(), "<span>hello world</span>");
}

#[test]
fn displayed_debugged() {
    #[derive(Debug)]
    struct Greeting(&'static str);

    impl Display for Greeting {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "Hello, {}! <script>", self.0)
        }
    }

    let maud_result = maud! {
        div {
            %(Greeting("World"))
        }
        div {
            ?(Greeting("World"))
        }
        div {
            (format_args!("{:#X}", 3_735_928_559_u32))
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            %(Greeting("World"))
        </div>
        <div>
            ?(Greeting("World"))
        </div>
        <div>
            (format_args!("{:#X}", 3_735_928_559_u32))
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<div>Hello, World! &lt;script&gt;</div><div>Greeting("World")</div><div>0xDEADBEEF</div>"#
        );
    }
}

#[test]
fn dynamic_expression() {
    let name = "World";

    let maud_result = maud! { span { "Hello, " (name) "!" } }.render();
    let rsx_result = rsx! { <span>"Hello, " (name) "!"</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>Hello, World!</span>");
    }
}

#[test]
fn format_args_expression() {
    let x = 255;

    let maud_result = maud! {
        span { (format_args!("{:#X}", x)) }
    }
    .render();

    let rsx_result = rsx! {
        <span>(format_args!("{:#X}", x))</span>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>0xFF</span>");
    }
}

#[test]
fn displayed_escapes_html() {
    struct Dangerous;
    impl Display for Dangerous {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "<img src=x onerror=alert(1)>")
        }
    }

    let maud_result = maud! { div { %(Dangerous) } }.render();
    let rsx_result = rsx! { <div>%(Dangerous)</div> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<div>&lt;img src=x onerror=alert(1)&gt;</div>"
        );
    }
}

#[test]
fn expression_with_method_call() {
    let items = ["a", "b", "c"];
    let len = items.len();

    let maud_result = maud! {
        span { (len) }
    }
    .render();

    let rsx_result = rsx! {
        <span>(len)</span>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>3</span>");
    }
}

#[test]
fn expression_with_block() {
    let maud_result = maud! {
        span { ({
            let x = 2;
            let y = 3;
            x + y
        }) }
    }
    .render();

    assert_eq!(maud_result.as_inner(), "<span>5</span>");
}

#[test]
fn if_else_match_for_while_let() {
    let cond = true;

    let maud_result = maud! {
        div {
            @if cond {
                span { "branch 1" }
            } @else {
                span { "branch 2" }
            }


            @match !cond {
                true => span { "branch 1" },
                false => span { "branch 2" },
            }

            @for i in 0..3 {
                span { (i) }
            }

            @let mut i = 3;

            @while i < 6 {
                span { (i) }
                (i += 1)
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            @if cond {
                <span>branch 1</span>
            } @else {
                <span>branch 2</span>
            }

            @match !cond {
                true => {
                    <span>branch 1</span>
                },
                false => <span>branch 2</span>,
            }

            @for i in 0..3 {
                <span>(i)</span>
            }

            @let mut i = 3;

            @while i < 6 {
                <span>(i)</span>
                (i += 1)
            }
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<div><span>branch 1</span><span>branch 2</span><span>0</span><span>1</span><span>2</span><span>3</span><span>4</span><span>5</span></div>"
        );
    }
}

#[test]
fn if_without_else() {
    let show = true;

    let maud_result = maud! {
        div {
            @if show {
                span { "visible" }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            @if show {
                <span>visible</span>
            }
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<div><span>visible</span></div>");
    }
}

#[test]
fn if_false_branch() {
    let show = false;

    let maud_result = maud! {
        div {
            @if show {
                span { "visible" }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            @if show {
                <span>visible</span>
            }
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<div></div>");
    }
}

#[test]
fn if_let() {
    let value = Some("hello");

    let maud_result = maud! {
        div {
            @if let Some(v) = value {
                span { (v) }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            @if let Some(v) = value {
                <span>(v)</span>
            }
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<div><span>hello</span></div>");
    }
}

#[test]
fn for_with_empty_iterator() {
    let items: Vec<&str> = vec![];

    let maud_result = maud::borrow! {
        ul {
            @for item in &items {
                li { (item) }
            }
        }
    }
    .render();

    let rsx_result = rsx::borrow! {
        <ul>
            @for item in &items {
                <li>(item)</li>
            }
        </ul>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<ul></ul>");
    }
}

#[test]
#[expect(clippy::manual_flatten)]
fn nested_control_flow() {
    let items = vec![Some(1), None, Some(3)];

    let maud_result = maud::borrow! {
        ul {
            @for item in &items {
                @if let Some(val) = item {
                    li { (val) }
                }
            }
        }
    }
    .render();

    let rsx_result = rsx::borrow! {
        <ul>
            @for item in &items {
                @if let Some(val) = item {
                    <li>(val)</li>
                }
            }
        </ul>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<ul><li>1</li><li>3</li></ul>");
    }
}

#[test]
fn match_with_multiple_arms() {
    let status = 404;

    let maud_result = maud! {
        div {
            @match status {
                200 => span { "OK" },
                404 => span { "Not Found" },
                500 => span { "Server Error" },
                _ => span { "Unknown" },
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            @match status {
                200 => <span>OK</span>,
                404 => <span>Not Found</span>,
                500 => <span>Server Error</span>,
                _ => <span>Unknown</span>,
            }
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<div><span>Not Found</span></div>");
    }
}

#[test]
fn for_with_enumerate() {
    let items = ["a", "b", "c"];

    let maud_result = maud! {
        ol {
            @for (i, item) in items.iter().enumerate() {
                li { (i) ": " (item) }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ol>
            @for (i, item) in items.iter().enumerate() {
                <li>(i) ": " (item)</li>
            }
        </ol>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<ol><li>0: a</li><li>1: b</li><li>2: c</li></ol>"
        );
    }
}

#[test]
fn option_toggle_some_and_none() {
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
fn option_toggle_none_omits_attribute() {
    let val = None::<&str>;

    let maud_result = maud! { div title=[val] {} }.render();
    let rsx_result = rsx! { <div title=[val]></div> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<div></div>");
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
fn boolean_toggle_true() {
    let is_active = true;

    let maud_result = maud! {
        div .active[is_active] { "content" }
    }
    .render();

    assert_eq!(
        maud_result.as_inner(),
        r#"<div class="active">content</div>"#
    );
}

#[test]
fn boolean_toggle_false() {
    let is_active = false;

    let maud_result = maud! {
        div .active[is_active] { "content" }
    }
    .render();

    assert_eq!(maud_result.as_inner(), r#"<div class="">content</div>"#);
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
fn doctype_maud() {
    let result = maud! {
        !DOCTYPE
        html {
            head { title { "Test" } }
            body { "Hello" }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<!DOCTYPE html><html><head><title>Test</title></head><body>Hello</body></html>"
    );
}

#[test]
fn doctype_rsx() {
    let result = rsx! {
        <!DOCTYPE html>
        <html>
            <head><title>Test</title></head>
            <body>Hello</body>
        </html>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<!DOCTYPE html><html><head><title>Test</title></head><body>Hello</body></html>"
    );
}

#[test]
fn doctype_both_syntaxes_match() {
    let maud_result = maud! {
        !DOCTYPE
        html {
            body { "content" }
        }
    }
    .render();

    let rsx_result = rsx! {
        <!DOCTYPE html>
        <html>
            <body>content</body>
        </html>
    }
    .render();

    assert_eq!(maud_result.as_inner(), rsx_result.as_inner());
}
