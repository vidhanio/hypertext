//! Runtime rendering, buffers, and pre-rendered values.
#![cfg(feature = "alloc")]

extern crate alloc;

use alloc::{borrow::Cow, boxed::Box, format, rc::Rc, string::String, sync::Arc, vec};
use core::fmt;

use hypertext::{
    Buffer, Debugged, Lazy, LazyAttribute, LazyMathMl, LazySvg, Raw, RawAttribute, RawMathMl,
    RawSvg, Renderable, RenderableExt, Rendered,
    context::{AttributeValue, MathMlNode, Node, SvgNode},
    maud,
    prelude::{GlobalAttributes, hypertext_elements},
};

fn assert_node<T: Renderable<Node>>(value: T, expected: &str) {
    assert_eq!(value.to_buffer().into_inner(), expected);

    let mut buffer = Buffer::<Node>::new();
    value.render_to(&mut buffer);
    assert_eq!(buffer.into_inner(), expected);

    assert_eq!(value.render().as_inner(), expected);
}

fn assert_attribute<T: Renderable<AttributeValue>>(value: T, expected: &str) {
    assert_eq!(value.to_buffer().into_inner(), expected);

    let mut buffer = Buffer::<AttributeValue>::new();
    value.render_to(&mut buffer);
    assert_eq!(buffer.into_inner(), expected);
}

#[test]
fn primitive_renderables_cover_each_formatter() {
    assert_node("hello", "hello");
    assert_node(String::from("owned"), "owned");
    assert_node(true, "true");
    assert_node(false, "false");
    assert_node(-42_i8, "-42");
    assert_node(200_u8, "200");
    assert_node(-1_000_i16, "-1000");
    assert_node(60_000_u16, "60000");
    assert_node(-100_000_i32, "-100000");
    assert_node(4_000_000_u32, "4000000");
    assert_node(-9_000_000_000_i64, "-9000000000");
    assert_node(18_000_000_000_u64, "18000000000");
    assert_node(-170_141_183_460_i128, "-170141183460");
    assert_node(340_282_366_920_u128, "340282366920");
    assert_node(-99_isize, "-99");
    assert_node(1_024_usize, "1024");

    #[expect(clippy::approx_constant)]
    let float32 = 3.14_f32;
    assert_node(float32, "3.14");
    assert_node(1.25_f64, "1.25");

    assert_node('X', "X");
    assert_node('<', "&lt;");
}

#[test]
fn composite_renderables_flatten_in_order() {
    let mut number = 42;
    let reference: &i32 = &number;
    assert_node(reference, "42");
    let mutable_reference: &mut i32 = &mut number;
    assert_node(mutable_reference, "42");
    assert_node(Box::new("boxed"), "boxed");
    assert_node(Rc::new("rc"), "rc");
    assert_node(Arc::new("arc"), "arc");
    assert_node(Cow::<str>::Borrowed("borrowed"), "borrowed");
    assert_node(Cow::<str>::Owned(String::from("owned")), "owned");

    assert_node(Some("present"), "present");
    assert_node(None::<&str>, "");
    assert_node(Ok::<_, &str>("success"), "success");
    assert_node(Err::<&str, _>("error"), "error");
    assert_node(vec!["x", "y", "z"], "xyz");
    assert_node(Vec::<&str>::new(), "");

    let array = ["a", "b", "c"];
    assert_node(array, "abc");
    assert_node(&array[..], "abc");
    let empty: [&str; 0] = [];
    assert_node(empty, "");

    assert_node((), "");
    assert_node(("hello",), "hello");
    assert_node(("a", "b", "c"), "abc");
    assert_node(
        (
            1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8,
        ),
        "123456789101112",
    );
}

#[test]
fn node_and_attribute_contexts_escape_their_own_delimiters() {
    let value = "<tag attr=\"x\"> & '";

    assert_node(value, "&lt;tag attr=\"x\"&gt; &amp; '");
    assert_attribute(value, "&lt;tag attr=&quot;x&quot;&gt; &amp; '");
    assert_node('&', "&amp;");
    assert_node('>', "&gt;");
    assert_attribute('"', "&quot;");

    assert_node(format_args!("{}:{}", "<", "&"), "&lt;:&amp;");
    assert_attribute(format_args!("{}:{}", "<", "\""), "&lt;:&quot;");
}

#[test]
fn displayed_and_debugged_values_are_escaped() {
    struct Greeting(&'static str);

    impl fmt::Display for Greeting {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Hello, {}! <script>", self.0)
        }
    }

    #[derive(Debug)]
    #[expect(dead_code)]
    struct Tag(&'static str);

    assert_node(
        hypertext::Displayed(Greeting("World")),
        "Hello, World! &lt;script&gt;",
    );
    assert_attribute(
        hypertext::Displayed("<b> & \"quoted\""),
        "&lt;b&gt; &amp; &quot;quoted&quot;",
    );
    assert_node(Debugged(Tag("<b>")), r#"Tag("&lt;b&gt;")"#);
    assert_attribute(Debugged(Tag("<b>")), "Tag(&quot;&lt;b&gt;&quot;)");
}

#[test]
fn buffer_push_and_context_conversion_share_storage() {
    let mut buffer = Buffer::<Node>::default();
    assert_eq!(format!("{buffer:?}"), "Buffer(\"\")");
    buffer.push("<safe text> &");
    buffer.push(42);

    {
        let attributes: &mut Buffer<AttributeValue> = buffer.with_context();
        attributes.push("a\"b");
    }

    assert_eq!(buffer.into_inner(), "&lt;safe text&gt; &amp;42a&quot;b");

    let mut source = String::from("prefix:");
    {
        // XSS SAFETY: this test intentionally starts with trusted source text.
        let mutable = Buffer::<Node>::dangerously_from_string_mut(&mut source);
        mutable.push("<user>");
    }
    assert_eq!(source, "prefix:&lt;user&gt;");
}

#[test]
fn dangerous_buffer_writes_are_preserved_and_rendered() {
    // XSS SAFETY: these literals are deliberately pre-escaped/trusted test
    // data.
    let buffer = Buffer::<Node>::dangerously_from_string(String::from("<b>raw</b>"));
    assert_eq!(buffer.clone().into_inner(), "<b>raw</b>");
    assert_eq!(format!("{buffer:?}"), "Buffer(\"<b>raw</b>\")");
    assert_eq!(buffer.rendered().as_inner(), "<b>raw</b>");
}

#[test]
fn svg_and_mathml_buffers_keep_their_node_context() {
    let mut svg = Buffer::<SvgNode>::new();
    svg.push("<path/>");
    assert_eq!(svg.rendered().as_inner(), "&lt;path/&gt;");

    let mut mathml = Buffer::<MathMlNode>::new();
    mathml.push("<mi>x</mi>");
    assert_eq!(mathml.rendered().as_inner(), "&lt;mi&gt;x&lt;/mi&gt;");

    let mut svg = Buffer::<SvgNode>::new();
    let html: &mut Buffer<Node> = svg.with_context();
    html.push("text");
    assert_eq!(svg.into_inner(), "text");
}

#[test]
fn raw_values_preserve_preescaped_data_in_each_context() {
    const EMPTY: Raw<&str> = Raw::dangerously_create("");
    assert_eq!(EMPTY.as_str(), "");
    assert_eq!(EMPTY.as_inner(), &"");

    let raw = Raw::<&str>::dangerously_create("&lt;b&gt;");
    assert_node(raw, "&lt;b&gt;");
    assert_eq!(raw, Raw::dangerously_create("&lt;b&gt;"));
    assert_ne!(raw, Raw::dangerously_create("other"));
    assert_eq!(format!("{raw:?}"), r#"Raw("&lt;b&gt;")"#);

    let attr = RawAttribute::dangerously_create("&lt;quoted&gt;&quot;");
    assert_attribute(attr, "&lt;quoted&gt;&quot;");
    assert_eq!(attr.as_str(), "&lt;quoted&gt;&quot;");

    let owned: Raw<String> = Raw::dangerously_create(String::from("owned"));
    assert_eq!(owned.as_inner(), "owned");
    assert_eq!(owned.clone().into_inner(), "owned");
    assert_eq!(owned.rendered().into_inner(), "owned");

    let svg = RawSvg::dangerously_create(String::from("<svg/>")).rendered();
    assert_eq!(svg.as_inner(), "<svg/>");
    let mathml = RawMathMl::dangerously_create(String::from("<math/>")).rendered();
    assert_eq!(mathml.as_inner(), "<math/>");
}

#[test]
fn rendered_values_have_small_value_semantics() {
    let rendered = maud! { p { "hello" } }.render();
    assert_eq!(rendered.as_inner(), "<p>hello</p>");
    assert_eq!(rendered.clone(), rendered);
    assert_ne!(rendered, maud! { p { "other" } }.render());
    assert_eq!(format!("{rendered:?}"), r#"Rendered("<p>hello</p>")"#);
    assert_eq!(Rendered::<String>::default().into_inner(), "");

    let svg: hypertext::RenderedSvg<String> =
        hypertext::RawSvg::dangerously_create(String::from("<svg/>")).rendered();
    assert_eq!(svg.into_inner(), "<svg/>");
}

#[test]
fn lazy_values_render_later_and_expose_their_closure() {
    let lazy = Lazy::<_, Node>::dangerously_create(|buffer: &mut Buffer| {
        buffer.push("<later>");
    });
    assert_node(lazy, "&lt;later&gt;");

    let lazy = Lazy::<_, Node>::dangerously_create(|buffer: &mut Buffer| {
        buffer.push("value");
    });
    let function = lazy.as_inner();
    let mut buffer = Buffer::new();
    function(&mut buffer);
    assert_eq!(buffer.into_inner(), "value");
    let _ = lazy.into_inner();

    let empty = Lazy::<fn(&mut Buffer), Node>::default();
    assert_node(empty, "");
    assert_eq!(
        format!("{:?}", Lazy::<fn(&mut Buffer), Node>::default()),
        "Lazy(..)"
    );
}

#[test]
fn lazy_aliases_and_memoize_keep_context_specific_escaping() {
    let attribute = LazyAttribute::dangerously_create(|buffer: &mut Buffer<AttributeValue>| {
        buffer.push("\"<&");
    });
    let memoized = attribute.memoize();
    assert_eq!(memoized.as_str(), "&quot;&lt;&amp;");
    assert_eq!(
        maud! { div title=(memoized) {} }.render().as_inner(),
        r#"<div title="&quot;&lt;&amp;"></div>"#,
    );

    let svg = LazySvg::dangerously_create(|buffer: &mut Buffer<SvgNode>| {
        buffer.push("<circle/>");
    });
    assert_eq!(
        svg.render::<hypertext::context::Svg>().as_inner(),
        "&lt;circle/&gt;",
    );

    let mathml = LazyMathMl::dangerously_create(|buffer: &mut Buffer<MathMlNode>| {
        buffer.push("<mi>x</mi>");
    });
    assert_eq!(
        mathml.render::<hypertext::context::MathMl>().as_inner(),
        "&lt;mi&gt;x&lt;/mi&gt;",
    );
}

#[test]
fn memoize_pre_renders_once_for_reuse() {
    use core::cell::Cell;

    struct Counted<'a>(&'a Cell<u8>);

    impl Renderable for Counted<'_> {
        fn render_to(&self, buffer: &mut Buffer) {
            self.0.set(self.0.get() + 1);
            buffer.push("counted");
        }
    }

    let calls = Cell::new(0);
    let counted = Counted(&calls);
    let counted_memoized = counted.memoize();
    assert_eq!(calls.get(), 1);
    assert_eq!(counted_memoized.as_str(), "counted");

    let dangerous = "<script>alert(1)</script>";
    let memoized = maud! { div { (dangerous) } }.memoize();
    assert_eq!(
        memoized.as_str(),
        "<div>&lt;script&gt;alert(1)&lt;/script&gt;</div>"
    );

    let first = maud::borrow! { (memoized) }.render();
    let second = maud::borrow! { section { (memoized) } }.render();
    assert_eq!(
        first.as_inner(),
        "<div>&lt;script&gt;alert(1)&lt;/script&gt;</div>"
    );
    assert_eq!(
        second.as_inner(),
        "<section><div>&lt;script&gt;alert(1)&lt;/script&gt;</div></section>",
    );
    assert_eq!(calls.get(), 1);
}
