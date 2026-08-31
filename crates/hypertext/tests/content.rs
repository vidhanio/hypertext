//! Text, expression, and raw-node rendering.
#![cfg(feature = "alloc")]

extern crate alloc;

use hypertext::{Raw, prelude::*};

macro_rules! assert_html_pair {
    ($maud:expr, $rsx:expr, $expected:expr $(,)?) => {{
        let maud = ($maud).render();
        let rsx = ($rsx).render();
        assert_eq!(maud.as_inner(), $expected);
        assert_eq!(rsx.as_inner(), $expected);
    }};
}

macro_rules! assert_content {
    ($value:expr => $expected:expr $(,)?) => {{
        let value = $value;
        assert_html_pair!(maud! { (value) }, rsx! { (value) }, $expected);
    }};
}

#[test]
fn literals_and_interpolation_cover_empty_siblings_and_elements() {
    assert_html_pair!(maud! { "hello" }, rsx! { hello }, "hello");
    assert_html_pair!(
        maud! { "hello" " " "world" },
        rsx! { hello " " world },
        "hello world",
    );
    assert_html_pair!(maud! { "" }, rsx! {}, "");

    let name = "Alice";
    assert_html_pair!(
        maud! { p { "Hello, " (name) "!" } },
        rsx! { <p>"Hello, " (name) "!"</p> },
        "<p>Hello, Alice!</p>",
    );

    let owned = String::from("Bob");
    let maud_owned = owned.clone();
    assert_html_pair!(
        maud! { span { (maud_owned.as_str()) } },
        rsx! { <span>(owned.as_str())</span> },
        "<span>Bob</span>",
    );
}

#[test]
fn primitive_expression_matrix_uses_node_rendering() {
    assert_content!(-128_i8 => "-128");
    assert_content!(255_u8 => "255");
    assert_content!(-32_768_i16 => "-32768");
    assert_content!(65_535_u16 => "65535");
    assert_content!(-100_000_i32 => "-100000");
    assert_content!(4_000_000_u32 => "4000000");
    assert_content!(-9_000_000_000_i64 => "-9000000000");
    assert_content!(18_000_000_000_u64 => "18000000000");
    assert_content!(-170_141_183_460_469_i128 => "-170141183460469");
    assert_content!(340_282_366_920_938_u128 => "340282366920938");
    assert_content!(-42_isize => "-42");
    assert_content!(42_usize => "42");

    #[expect(clippy::approx_constant)]
    let float32 = 3.14_f32;
    assert_content!(float32 => "3.14");
    assert_content!(-0.5_f64 => "-0.5");
    assert_content!(true => "true");
    assert_content!(false => "false");
    assert_content!('A' => "A");
    assert_content!('&' => "&amp;");
    assert_content!('<' => "&lt;");
    assert_content!('>' => "&gt;");
}

#[test]
fn text_escaping_matrix_protects_markup_without_touching_quotes() {
    for (text, expected) in [
        ("A & B", "A &amp; B"),
        ("a < b", "a &lt; b"),
        ("a > b", "a &gt; b"),
        (
            "<script>alert('xss')</script>",
            "&lt;script&gt;alert('xss')&lt;/script&gt;",
        ),
        ("a & b < c > d", "a &amp; b &lt; c &gt; d"),
        (r#"She said "hello""#, r#"She said "hello""#),
    ] {
        let expected = alloc::format!("<p>{expected}</p>");
        assert_html_pair!(
            maud! { p { (text) } },
            rsx! { <p>(text)</p> },
            expected.as_str(),
        );
    }
}

#[test]
fn display_and_debug_syntax_escape_values_in_both_contexts() {
    use core::fmt;

    #[derive(Clone, Copy)]
    struct Greeting(&'static str);

    impl fmt::Display for Greeting {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Hello, {}!", self.0)
        }
    }

    #[derive(Debug)]
    #[expect(dead_code)]
    struct Point(i32, i32);

    #[derive(Debug)]
    #[expect(dead_code)]
    struct Tag(&'static str);

    assert_html_pair!(
        maud! { div { %(Greeting("Alice")) } },
        rsx! { <div>%(Greeting("Alice"))</div> },
        "<div>Hello, Alice!</div>",
    );
    assert_html_pair!(
        maud! { div { ?(Point(1, 2)) } },
        rsx! { <div>?(Point(1, 2))</div> },
        "<div>Point(1, 2)</div>",
    );

    assert_html_pair!(
        maud! { div { ?(Tag("<b>bold</b>")) } },
        rsx! { <div>?(Tag("<b>bold</b>"))</div> },
        r#"<div>Tag("&lt;b&gt;bold&lt;/b&gt;")</div>"#,
    );

    let greeting = Greeting("Alice");
    let maud = maud! { div title=%(greeting) { %(greeting) } }.render();
    let rsx = rsx! { <div title=%(greeting)>%(greeting)</div> }.render();
    assert_eq!(
        maud.as_inner(),
        r#"<div title="Hello, Alice!">Hello, Alice!</div>"#
    );
    assert_eq!(
        rsx.as_inner(),
        r#"<div title="Hello, Alice!">Hello, Alice!</div>"#
    );
}

#[test]
fn multiline_and_mixed_content_are_unindented_and_ordered() {
    assert_html_pair!(
        maud! {
            pre {
                "line 1\n"
                "line 2\n"
                "line 3"
            }
        },
        rsx! {
            <pre>
                "line 1\n"
                "line 2\n"
                "line 3"
            </pre>
        },
        "<pre>line 1\nline 2\nline 3</pre>",
    );

    let count = 5;
    let name = "widgets";
    assert_html_pair!(
        maud! { p { "There are " (count) " " (name) " available." } },
        rsx! { <p>"There are " (count) " " (name) " available."</p> },
        "<p>There are 5 widgets available.</p>",
    );

    let items = ["a", "b", "c"];
    assert_html_pair!(
        maud! { span { (items.len()) } },
        rsx! { <span>(items.len())</span> },
        "<span>3</span>",
    );
}

#[test]
fn raw_nodes_bypass_escaping_only_when_explicit() {
    // XSS SAFETY: these values are trusted literals used to exercise `Raw`.
    let raw = Raw::dangerously_create("<b>bold</b>");
    assert_html_pair!(
        maud! { div { (raw) } },
        rsx! { <div>(raw)</div> },
        "<div><b>bold</b></div>",
    );

    let raw = Raw::dangerously_create("a & b < c > d");
    let result = maud! { (raw) }.render();
    assert_eq!(result.as_inner(), "a & b < c > d");

    let escaped = "a & b < c > d";
    assert_eq!(
        maud! { (escaped) }.render().as_inner(),
        "a &amp; b &lt; c &gt; d"
    );
}
