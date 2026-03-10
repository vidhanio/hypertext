//! Renderable trait and buffer type tests.
#![cfg(feature = "alloc")]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, string::String, sync::Arc, vec};

use hypertext::{prelude::*, Buffer, Lazy, Raw, Rendered};

#[test]
fn renderable_str() {
    let result = maud! { ("hello") }.render();
    assert_eq!(result.as_inner(), "hello");
}

#[test]
fn renderable_string() {
    let s = String::from("world");
    let result = maud! { (s) }.render();
    assert_eq!(result.as_inner(), "world");
}

#[test]
fn renderable_bool_true() {
    let result = maud! { (true) }.render();
    assert_eq!(result.as_inner(), "true");
}

#[test]
fn renderable_bool_false() {
    let result = maud! { (false) }.render();
    assert_eq!(result.as_inner(), "false");
}

#[test]
fn renderable_i8() {
    let v: i8 = -42;
    let result = maud! { (v) }.render();
    assert_eq!(result.as_inner(), "-42");
}

#[test]
fn renderable_u8() {
    let v: u8 = 200;
    let result = maud! { (v) }.render();
    assert_eq!(result.as_inner(), "200");
}

#[test]
fn renderable_i16() {
    let v: i16 = -1000;
    let result = maud! { (v) }.render();
    assert_eq!(result.as_inner(), "-1000");
}

#[test]
fn renderable_u16() {
    let v: u16 = 60000;
    let result = maud! { (v) }.render();
    assert_eq!(result.as_inner(), "60000");
}

#[test]
fn renderable_i32() {
    let v: i32 = -100_000;
    let result = maud! { (v) }.render();
    assert_eq!(result.as_inner(), "-100000");
}

#[test]
fn renderable_u32() {
    let v: u32 = 4_000_000;
    let result = maud! { (v) }.render();
    assert_eq!(result.as_inner(), "4000000");
}

#[test]
fn renderable_i64() {
    let v: i64 = -9_000_000_000;
    let result = maud! { (v) }.render();
    assert_eq!(result.as_inner(), "-9000000000");
}

#[test]
fn renderable_u64() {
    let v: u64 = 18_000_000_000;
    let result = maud! { (v) }.render();
    assert_eq!(result.as_inner(), "18000000000");
}

#[test]
fn renderable_i128() {
    let v: i128 = -170_141_183_460;
    let result = maud! { (v) }.render();
    assert_eq!(result.as_inner(), "-170141183460");
}

#[test]
fn renderable_u128() {
    let v: u128 = 340_282_366_920;
    let result = maud! { (v) }.render();
    assert_eq!(result.as_inner(), "340282366920");
}

#[test]
fn renderable_isize() {
    let v: isize = -99;
    let result = maud! { (v) }.render();
    assert_eq!(result.as_inner(), "-99");
}

#[test]
fn renderable_usize() {
    let v: usize = 1024;
    let result = maud! { (v) }.render();
    assert_eq!(result.as_inner(), "1024");
}

#[test]
fn renderable_f32() {
    let v: f32 = 3.14;
    let result = maud! { (v) }.render();
    assert_eq!(result.as_inner(), "3.14");
}

#[test]
fn renderable_f64() {
    let v: f64 = 2.718281828;
    let result = maud! { (v) }.render();
    assert_eq!(result.as_inner(), "2.718281828");
}

#[test]
fn renderable_char() {
    let c = 'X';
    let result = maud! { (c) }.render();
    assert_eq!(result.as_inner(), "X");
}

#[test]
fn renderable_char_escaping() {
    let c = '<';
    let result = maud! { (c) }.render();
    assert_eq!(result.as_inner(), "&lt;");
}

#[test]
fn renderable_ref() {
    let v = 42;
    let r = &v;
    let result = maud! { (r) }.render();
    assert_eq!(result.as_inner(), "42");
}

#[test]
fn renderable_mut_ref() {
    let mut v = 42;
    let r = &mut v;
    let result = maud! { (*r) }.render();
    assert_eq!(result.as_inner(), "42");
}

#[test]
fn renderable_box() {
    let b = Box::new("boxed");
    let result = maud! { (b) }.render();
    assert_eq!(result.as_inner(), "boxed");
}

#[test]
fn renderable_rc() {
    let r = Rc::new("rc'd");
    let result = maud! { (r) }.render();
    assert_eq!(result.as_inner(), "rc'd");
}

#[test]
fn renderable_arc() {
    let a = Arc::new("arc'd");
    let result = maud! { (a) }.render();
    assert_eq!(result.as_inner(), "arc'd");
}

#[test]
fn renderable_cow_borrowed() {
    use alloc::borrow::Cow;
    let c: Cow<str> = Cow::Borrowed("borrowed");
    let result = maud! { (c) }.render();
    assert_eq!(result.as_inner(), "borrowed");
}

#[test]
fn renderable_cow_owned() {
    use alloc::borrow::Cow;
    let c: Cow<str> = Cow::Owned(String::from("owned"));
    let result = maud! { (c) }.render();
    assert_eq!(result.as_inner(), "owned");
}

#[test]
fn renderable_option_some() {
    let o = Some("present");
    let result = maud! { (o) }.render();
    assert_eq!(result.as_inner(), "present");
}

#[test]
fn renderable_option_none() {
    let o = None::<&str>;
    let result = maud! { (o) }.render();
    assert_eq!(result.as_inner(), "");
}

#[test]
fn renderable_result_ok() {
    let r: Result<&str, &str> = Ok("success");
    let result = maud! { (r) }.render();
    assert_eq!(result.as_inner(), "success");
}

#[test]
fn renderable_result_err() {
    let r: Result<&str, &str> = Err("error");
    let result = maud! { (r) }.render();
    assert_eq!(result.as_inner(), "error");
}

#[test]
fn renderable_vec() {
    let v: alloc::vec::Vec<&str> = vec!["x", "y", "z"];
    let result = maud! { (v) }.render();
    assert_eq!(result.as_inner(), "xyz");
}

#[test]
fn renderable_array() {
    let arr: [&str; 3] = ["a", "b", "c"];
    let result = maud! { (arr) }.render();
    assert_eq!(result.as_inner(), "abc");
}

#[test]
fn renderable_slice() {
    let arr = ["x", "y"];
    let s: &[&str] = &arr;
    let result = maud! { (s) }.render();
    assert_eq!(result.as_inner(), "xy");
}

#[test]
fn renderable_unit_tuple() {
    let result = maud! { (()) }.render();
    assert_eq!(result.as_inner(), "");
}

#[test]
fn renderable_single_tuple() {
    let t = ("hello",);
    let result = maud! { (t) }.render();
    assert_eq!(result.as_inner(), "hello");
}

#[test]
fn renderable_pair_tuple() {
    let t = ("a", "b");
    let result = maud! { (t) }.render();
    assert_eq!(result.as_inner(), "ab");
}

#[test]
fn renderable_triple_tuple() {
    let t = ("x", "y", "z");
    let result = maud! { (t) }.render();
    assert_eq!(result.as_inner(), "xyz");
}

#[test]
fn buffer_new_is_empty() {
    let buffer: Buffer = Buffer::new();
    assert_eq!(buffer.into_inner(), "");
}

#[test]
fn buffer_default_is_empty() {
    let buffer: Buffer = Buffer::default();
    assert_eq!(buffer.into_inner(), "");
}

#[test]
fn buffer_push() {
    let mut buffer: Buffer = Buffer::new();
    buffer.push("hello");
    buffer.push(" ");
    buffer.push("world");
    assert_eq!(buffer.into_inner(), "hello world");
}

#[test]
fn buffer_push_escapes() {
    let mut buffer: Buffer = Buffer::new();
    buffer.push("<script>");
    assert_eq!(buffer.into_inner(), "&lt;script&gt;");
}

#[test]
fn buffer_dangerously_from_string() {
    let buffer: Buffer = Buffer::dangerously_from_string("<b>bold</b>".into());
    assert_eq!(buffer.into_inner(), "<b>bold</b>");
}

#[test]
fn buffer_dangerously_get_string() {
    let mut buffer: Buffer = Buffer::new();
    buffer.dangerously_get_string().push_str("<raw>");
    assert_eq!(buffer.into_inner(), "<raw>");
}

#[test]
fn buffer_rendered() {
    let mut buffer: Buffer = Buffer::new();
    buffer.push("hello");
    let rendered = buffer.rendered();
    assert_eq!(rendered.as_inner(), "hello");
}

#[test]
fn buffer_with_context() {
    use hypertext::context::AttributeValue;

    let mut buffer: Buffer = Buffer::new();
    let attr_buf: &mut Buffer<AttributeValue> = buffer.with_context();
    attr_buf.push("a\"b");
    assert_eq!(buffer.into_inner(), "a&quot;b");
}

#[test]
fn raw_dangerously_create() {
    let raw: Raw<&str> = Raw::dangerously_create("<b>bold</b>");
    assert_eq!(raw.as_str(), "<b>bold</b>");
}

#[test]
fn raw_into_inner() {
    let raw: Raw<&str> = Raw::dangerously_create("test");
    assert_eq!(raw.into_inner(), "test");
}

#[test]
fn raw_as_inner() {
    let raw: Raw<&str> = Raw::dangerously_create("test");
    assert_eq!(*raw.as_inner(), "test");
}

#[test]
fn raw_as_str() {
    let raw: Raw<String> = Raw::dangerously_create(String::from("hello"));
    assert_eq!(raw.as_str(), "hello");
}

#[test]
fn raw_rendered() {
    let raw: Raw<&str> = Raw::dangerously_create("<em>italic</em>");
    let rendered = raw.rendered();
    assert_eq!(*rendered.as_inner(), "<em>italic</em>");
}

#[test]
fn raw_partial_eq() {
    let a = Raw::<&str>::dangerously_create("test");
    let b = Raw::<&str>::dangerously_create("test");
    let c = Raw::<&str>::dangerously_create("other");

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn raw_debug() {
    let raw: Raw<&str> = Raw::dangerously_create("hello");
    let debug = alloc::format!("{raw:?}");
    assert_eq!(debug, r#"Raw("hello")"#);
}

#[test]
fn raw_clone() {
    let raw: Raw<&str> = Raw::dangerously_create("test");
    let cloned = raw.clone();
    assert_eq!(raw, cloned);
}

#[test]
fn raw_default() {
    let raw = Raw::<&str>::default();
    assert_eq!(raw.as_str(), "");
}

#[test]
fn raw_attribute_create_and_read() {
    let attr = hypertext::RawAttribute::dangerously_create("my-value");
    assert_eq!(attr.as_str(), "my-value");
}

#[test]
fn rendered_into_inner() {
    let rendered = maud! { "hello" }.render();
    assert_eq!(rendered.into_inner(), "hello");
}

#[test]
fn rendered_as_inner() {
    let rendered = maud! { "hello" }.render();
    assert_eq!(*rendered.as_inner(), "hello");
}

#[test]
fn rendered_partial_eq() {
    let a = maud! { "test" }.render();
    let b = maud! { "test" }.render();
    let c = maud! { "other" }.render();

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn rendered_debug() {
    let rendered = maud! { "hello" }.render();
    let debug = alloc::format!("{rendered:?}");
    assert_eq!(debug, r#"Rendered("hello")"#);
}

#[test]
fn rendered_default() {
    let rendered: Rendered<String> = Rendered::default();
    assert_eq!(rendered.into_inner(), "");
}

#[test]
fn lazy_dangerously_create() {
    let lazy = Lazy::dangerously_create(|buffer: &mut Buffer| {
        buffer.dangerously_get_string().push_str("hello");
    });
    let result = lazy.render();
    assert_eq!(result.as_inner(), "hello");
}

#[test]
fn lazy_into_inner() {
    let lazy = Lazy::dangerously_create(|_: &mut Buffer| {});
    let _f = lazy.into_inner();
}

#[test]
fn lazy_as_inner() {
    let lazy = Lazy::dangerously_create(|buffer: &mut Buffer| {
        buffer.dangerously_get_string().push_str("test");
    });
    let f = lazy.as_inner();
    let mut buffer = Buffer::new();
    f(&mut buffer);
    assert_eq!(buffer.into_inner(), "test");
}

#[test]
fn lazy_default() {
    let lazy = Lazy::<fn(&mut Buffer)>::default();
    let result = lazy.render();
    assert_eq!(result.as_inner(), "");
}

#[test]
fn lazy_debug() {
    let lazy = Lazy::dangerously_create(|_: &mut Buffer| {});
    let debug = alloc::format!("{lazy:?}");
    assert_eq!(debug, "Lazy(..)");
}

#[test]
fn memoize_returns_raw() {
    let lazy = maud! {
        div { "hello" }
    };

    let memoized = lazy.memoize();
    assert_eq!(memoized.as_str(), "<div>hello</div>");

    let r1 = maud::borrow! { (memoized) }.render();
    let r2 = maud::borrow! { section { (memoized) } }.render();
    assert_eq!(r1.as_inner(), "<div>hello</div>");
    assert_eq!(r2.as_inner(), "<section><div>hello</div></section>");
}

#[test]
fn displayed_directly() {
    use core::fmt;

    use hypertext::Displayed;

    struct Greeting(&'static str);

    impl fmt::Display for Greeting {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Hello, {}!", self.0)
        }
    }

    let d = Displayed(Greeting("World"));
    let result = maud! { (d) }.render();
    assert_eq!(result.as_inner(), "Hello, World!");
}

#[test]
fn displayed_escapes_html() {
    use hypertext::Displayed;

    let d = Displayed("<script>alert('xss')</script>");
    let result = maud! { (d) }.render();
    assert_eq!(
        result.as_inner(),
        "&lt;script&gt;alert('xss')&lt;/script&gt;"
    );
}

#[test]
fn debugged_directly() {
    use hypertext::Debugged;

    let d = Debugged(vec![1, 2, 3]);
    let result = maud! { (d) }.render();
    assert_eq!(result.as_inner(), "[1, 2, 3]");
}

#[test]
fn debugged_escapes_html() {
    use hypertext::Debugged;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Tag(&'static str);

    let d = Debugged(Tag("<b>"));
    let result = maud! { (d) }.render();
    assert_eq!(result.as_inner(), r#"Tag("&lt;b&gt;")"#);
}

#[test]
fn format_args_via_display_syntax() {
    let name = "Alice";
    let age = 30;

    let result = maud! {
        p { %(format_args!("{name} is {age} years old")) }
    }
    .render();

    assert_eq!(result.as_inner(), "<p>Alice is 30 years old</p>");
}
