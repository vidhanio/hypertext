#![cfg(feature = "alloc")]

use hypertext::prelude::*;

#[test]
fn string_literal() {
    let maud_result = maud! { "hello" }.render();
    let rsx_result = rsx! { hello }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "hello");
    }
}

#[test]
fn adjacent_string_literals() {
    let maud_result = maud! { "hello" " " "world" }.render();
    let rsx_result = rsx! { hello " " world }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "hello world");
    }
}

#[test]
fn empty_string_literal() {
    let result = maud! { "" }.render();
    assert_eq!(result.as_inner(), "");
}

#[test]
fn string_in_element() {
    let maud_result = maud! { p { "Hello, world!" } }.render();
    let rsx_result = rsx! { <p>"Hello, world!"</p> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<p>Hello, world!</p>");
    }
}

#[test]
fn dynamic_str_expression() {
    let name = "Alice";

    let maud_result = maud! { p { "Hello, " (name) "!" } }.render();
    let rsx_result = rsx! { <p>"Hello, " (name) "!"</p> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<p>Hello, Alice!</p>");
    }
}

#[test]
fn dynamic_string_expression_maud() {
    let name = String::from("Bob");
    let result = maud! { span { (name) } }.render();
    assert_eq!(result.as_inner(), "<span>Bob</span>");
}

#[test]
fn dynamic_string_expression_rsx() {
    let name = String::from("Bob");
    let result = rsx! { <span>(name)</span> }.render();
    assert_eq!(result.as_inner(), "<span>Bob</span>");
}

#[test]
fn integer_content() {
    let val: i32 = 42;

    let maud_result = maud! { span { (val) } }.render();
    let rsx_result = rsx! { <span>(val)</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>42</span>");
    }
}

#[test]
fn negative_integer_content() {
    let val: i32 = -7;

    let maud_result = maud! { span { (val) } }.render();
    let rsx_result = rsx! { <span>(val)</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>-7</span>");
    }
}

#[test]
fn unsigned_integer_content() {
    let val: u64 = 100;

    let maud_result = maud! { span { (val) } }.render();
    let rsx_result = rsx! { <span>(val)</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>100</span>");
    }
}

#[test]
fn i8_content() {
    let val: i8 = -128;
    let result = maud! { (val) }.render();
    assert_eq!(result.as_inner(), "-128");
}

#[test]
fn u8_content() {
    let val: u8 = 255;
    let result = maud! { (val) }.render();
    assert_eq!(result.as_inner(), "255");
}

#[test]
fn i16_content() {
    let val: i16 = -32768;
    let result = maud! { (val) }.render();
    assert_eq!(result.as_inner(), "-32768");
}

#[test]
fn u16_content() {
    let val: u16 = 65535;
    let result = maud! { (val) }.render();
    assert_eq!(result.as_inner(), "65535");
}

#[test]
fn i64_content() {
    let val: i64 = -9_000_000_000;
    let result = maud! { (val) }.render();
    assert_eq!(result.as_inner(), "-9000000000");
}

#[test]
fn i128_content() {
    let val: i128 = -170_141_183_460_469;
    let result = maud! { (val) }.render();
    assert_eq!(result.as_inner(), "-170141183460469");
}

#[test]
fn u128_content() {
    let val: u128 = 340_282_366_920_938;
    let result = maud! { (val) }.render();
    assert_eq!(result.as_inner(), "340282366920938");
}

#[test]
fn isize_content() {
    let val: isize = -42;
    let result = maud! { (val) }.render();
    assert_eq!(result.as_inner(), "-42");
}

#[test]
fn usize_content() {
    let val: usize = 42;
    let result = maud! { (val) }.render();
    assert_eq!(result.as_inner(), "42");
}

#[test]
fn f32_content() {
    let val: f32 = 3.14;

    let maud_result = maud! { span { (val) } }.render();
    let rsx_result = rsx! { <span>(val)</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>3.14</span>");
    }
}

#[test]
fn f64_content() {
    let val: f64 = 2.718281828;

    let maud_result = maud! { span { (val) } }.render();
    let rsx_result = rsx! { <span>(val)</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>2.718281828</span>");
    }
}

#[test]
fn negative_float_content() {
    let val: f64 = -0.5;
    let result = maud! { (val) }.render();
    assert_eq!(result.as_inner(), "-0.5");
}

#[test]
fn bool_true_content() {
    let val = true;

    let maud_result = maud! { span { (val) } }.render();
    let rsx_result = rsx! { <span>(val)</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>true</span>");
    }
}

#[test]
fn bool_false_content() {
    let val = false;

    let maud_result = maud! { span { (val) } }.render();
    let rsx_result = rsx! { <span>(val)</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>false</span>");
    }
}

#[test]
fn char_content_normal() {
    let c = 'A';

    let maud_result = maud! { span { (c) } }.render();
    let rsx_result = rsx! { <span>(c)</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>A</span>");
    }
}

#[test]
fn char_content_ampersand_escaped() {
    let c = '&';
    let result = maud! { (c) }.render();
    assert_eq!(result.as_inner(), "&amp;");
}

#[test]
fn char_content_lt_escaped() {
    let c = '<';
    let result = maud! { (c) }.render();
    assert_eq!(result.as_inner(), "&lt;");
}

#[test]
fn char_content_gt_escaped() {
    let c = '>';
    let result = maud! { (c) }.render();
    assert_eq!(result.as_inner(), "&gt;");
}

#[test]
fn escapes_ampersand() {
    let text = "A & B";

    let maud_result = maud! { p { (text) } }.render();
    let rsx_result = rsx! { <p>(text)</p> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<p>A &amp; B</p>");
    }
}

#[test]
fn escapes_less_than() {
    let text = "a < b";

    let maud_result = maud! { p { (text) } }.render();
    let rsx_result = rsx! { <p>(text)</p> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<p>a &lt; b</p>");
    }
}

#[test]
fn escapes_greater_than() {
    let text = "a > b";

    let maud_result = maud! { p { (text) } }.render();
    let rsx_result = rsx! { <p>(text)</p> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<p>a &gt; b</p>");
    }
}

#[test]
fn escapes_script_tag() {
    let text = "<script>alert('xss')</script>";

    let maud_result = maud! { p { (text) } }.render();
    let rsx_result = rsx! { <p>(text)</p> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<p>&lt;script&gt;alert('xss')&lt;/script&gt;</p>"
        );
    }
}

#[test]
fn escapes_multiple_special_chars() {
    let text = "a & b < c > d";

    let result = maud! { (text) }.render();
    assert_eq!(result.as_inner(), "a &amp; b &lt; c &gt; d");
}

#[test]
fn quotes_not_escaped_in_node_context() {
    let text = r#"She said "hello""#;

    let result = maud! { p { (text) } }.render();
    assert_eq!(result.as_inner(), r#"<p>She said "hello"</p>"#);
}

#[test]
fn display_rendering() {
    use core::fmt;

    struct Greeting(&'static str);

    impl fmt::Display for Greeting {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Hello, {}!", self.0)
        }
    }

    let maud_result = maud! { div { %(Greeting("Alice")) } }.render();
    let rsx_result = rsx! { <div>%(Greeting("Alice"))</div> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<div>Hello, Alice!</div>");
    }
}

#[test]
fn display_rendering_with_html_escaping() {
    use core::fmt;

    struct Dangerous(&'static str);

    impl fmt::Display for Dangerous {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "<script>{}</script>", self.0)
        }
    }

    let result = maud! { div { %(Dangerous("alert('xss')")) } }.render();
    assert_eq!(
        result.as_inner(),
        "<div>&lt;script&gt;alert('xss')&lt;/script&gt;</div>"
    );
}

#[test]
fn debug_rendering() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point(i32, i32);

    let maud_result = maud! { div { ?(Point(1, 2)) } }.render();
    let rsx_result = rsx! { <div>?(Point(1, 2))</div> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<div>Point(1, 2)</div>");
    }
}

#[test]
fn debug_rendering_with_escaping() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Tag(&'static str);

    let result = maud! { div { ?(Tag("<b>bold</b>")) } }.render();
    assert_eq!(
        result.as_inner(),
        r#"<div>Tag("&lt;b&gt;bold&lt;/b&gt;")</div>"#
    );
}

#[test]
fn debug_in_attribute() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Greeting(&'static str);

    let result = maud! {
        div title=?(Greeting("Alice")) {
            ?(Greeting("Alice"))
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div title="Greeting(&quot;Alice&quot;)">Greeting("Alice")</div>"#
    );
}

#[test]
fn multiline_string_unindent() {
    let maud_result = maud! {
        pre {
            "line 1\n"
            "line 2\n"
            "line 3"
        }
    }
    .render();

    let rsx_result = rsx! {
        <pre>
            "line 1\n"
            "line 2\n"
            "line 3"
        </pre>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<pre>line 1\nline 2\nline 3</pre>");
    }
}

#[test]
fn mixed_static_and_dynamic_content() {
    let count = 5;
    let name = "widgets";

    let maud_result = maud! {
        p { "There are " (count) " " (name) " available." }
    }
    .render();

    let rsx_result = rsx! {
        <p>"There are " (count) " " (name) " available."</p>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<p>There are 5 widgets available.</p>");
    }
}

#[test]
fn expression_with_method_call() {
    let items = vec!["a", "b", "c"];
    let len = items.len();

    let maud_result = maud! { span { (len) } }.render();
    let rsx_result = rsx! { <span>(len)</span> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>3</span>");
    }
}

#[test]
fn raw_html_content() {
    use hypertext::Raw;

    // XSS SAFETY: test content
    let raw = Raw::dangerously_create("<b>bold</b>");

    let maud_result = maud! { div { (raw) } }.render();
    let rsx_result = rsx! { <div>(raw)</div> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<div><b>bold</b></div>");
    }
}

#[test]
fn raw_string_not_escaped() {
    use hypertext::Raw;

    // XSS SAFETY: test content
    let raw = Raw::dangerously_create("a & b < c > d");

    let result = maud! { (raw) }.render();
    assert_eq!(result.as_inner(), "a & b < c > d");
}
