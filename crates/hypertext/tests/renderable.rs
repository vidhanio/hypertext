//! Tests for `Renderable` implementations on standard types.
#![cfg(feature = "alloc")]

use std::{borrow::Cow, rc::Rc, sync::Arc};

use hypertext::{prelude::*, Buffer, Raw, Rendered};

#[test]
fn str_renderable() {
    let result = maud! { span { ("hello") } }.render();
    assert_eq!(result.as_inner(), "<span>hello</span>");
}

#[test]
fn string_renderable() {
    let s = "hello".to_owned();
    let result = maud! { span { (s) } }.render();
    assert_eq!(result.as_inner(), "<span>hello</span>");
}

#[test]
fn bool_renderable() {
    let result_true = maud! { span { (true) } }.render();
    let result_false = maud! { span { (false) } }.render();

    assert_eq!(result_true.as_inner(), "<span>true</span>");
    assert_eq!(result_false.as_inner(), "<span>false</span>");
}

#[test]
#[expect(clippy::similar_names)]
fn integer_renderables() {
    let result_u8 = maud! { span { (42_u8) } }.render();
    let result_i32 = maud! { span { (-7_i32) } }.render();
    let result_u64 = maud! { span { (123_456_789_u64) } }.render();
    let result_usize = maud! { span { (0_usize) } }.render();

    assert_eq!(result_u8.as_inner(), "<span>42</span>");
    assert_eq!(result_i32.as_inner(), "<span>-7</span>");
    assert_eq!(result_u64.as_inner(), "<span>123456789</span>");
    assert_eq!(result_usize.as_inner(), "<span>0</span>");
}

#[test]
#[expect(clippy::approx_constant)]
fn float_renderables() {
    let result_f32 = maud! { span { (3.14_f32) } }.render();
    let result_f64 = maud! { span { (2.718_f64) } }.render();

    assert_eq!(result_f32.as_inner(), "<span>3.14</span>");
    assert_eq!(result_f64.as_inner(), "<span>2.718</span>");
}

#[test]
fn char_renderable() {
    let result = maud! { span { ('X') } }.render();
    assert_eq!(result.as_inner(), "<span>X</span>");
}

#[test]
#[expect(clippy::similar_names)]
fn char_escaping() {
    let result_amp = maud! { span { ('&') } }.render();
    let result_lt = maud! { span { ('<') } }.render();
    let result_gt = maud! { span { ('>') } }.render();

    assert_eq!(result_amp.as_inner(), "<span>&amp;</span>");
    assert_eq!(result_lt.as_inner(), "<span>&lt;</span>");
    assert_eq!(result_gt.as_inner(), "<span>&gt;</span>");
}

#[test]
fn str_escaping() {
    let s = "<script>alert('xss')</script>";
    let result = maud! { span { (s) } }.render();
    assert_eq!(
        result.as_inner(),
        "<span>&lt;script&gt;alert('xss')&lt;/script&gt;</span>"
    );
}

#[test]
fn vec_renderable() {
    let groceries = ["milk", "eggs", "bread"]
        .into_iter()
        .map(|s| maud! { li { (s) } })
        .collect::<Vec<_>>();

    let result = maud! {
        ul { (groceries) }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<ul><li>milk</li><li>eggs</li><li>bread</li></ul>"
    );
}

#[test]
fn option_some_renderable() {
    let val = Some("hello");
    let result = maud! { span { (val) } }.render();
    assert_eq!(result.as_inner(), "<span>hello</span>");
}

#[test]
fn option_none_renderable() {
    let val = None::<&str>;
    let result = maud! { span { (val) } }.render();
    assert_eq!(result.as_inner(), "<span></span>");
}

#[test]
fn array_renderable() {
    let items = [1, 2, 3];
    let result = maud! { span { (items) } }.render();
    assert_eq!(result.as_inner(), "<span>123</span>");
}

#[test]
fn tuple_renderable() {
    let pair = ("hello", " ", "world");
    let result = maud! { span { (pair) } }.render();
    assert_eq!(result.as_inner(), "<span>hello world</span>");
}

#[test]
fn result_ok_renderable() {
    let val: Result<&str, &str> = Ok("success");
    let result = maud! { span { (val) } }.render();
    assert_eq!(result.as_inner(), "<span>success</span>");
}

#[test]
fn result_err_renderable() {
    let val: Result<&str, &str> = Err("failure");
    let result = maud! { span { (val) } }.render();
    assert_eq!(result.as_inner(), "<span>failure</span>");
}

#[test]
fn empty_vec_renderable() {
    let items: Vec<&str> = vec![];
    let result = maud! { span { (items) } }.render();
    assert_eq!(result.as_inner(), "<span></span>");
}

#[test]
fn unit_tuple_renderable() {
    let unit = ();
    let result = maud! { span { (unit) } }.render();
    assert_eq!(result.as_inner(), "<span></span>");
}

#[test]
fn box_renderable() {
    let val: Box<str> = "hello".into();
    let result = maud! { span { (val) } }.render();
    assert_eq!(result.as_inner(), "<span>hello</span>");
}

#[test]
fn rc_renderable() {
    let val: Rc<str> = "hello".into();
    let result = maud! { span { (val) } }.render();
    assert_eq!(result.as_inner(), "<span>hello</span>");
}

#[test]
fn arc_renderable() {
    let val: Arc<str> = "hello".into();
    let result = maud! { span { (val) } }.render();
    assert_eq!(result.as_inner(), "<span>hello</span>");
}

#[test]
fn cow_borrowed_renderable() {
    let val: Cow<'_, str> = Cow::Borrowed("hello");
    let result = maud! { span { (val) } }.render();
    assert_eq!(result.as_inner(), "<span>hello</span>");
}

#[test]
fn cow_owned_renderable() {
    let val: Cow<'_, str> = Cow::Owned("hello".to_owned());
    let result = maud! { span { (val) } }.render();
    assert_eq!(result.as_inner(), "<span>hello</span>");
}

#[test]
fn buffer_new_and_push() {
    let mut buf = Buffer::new();
    buf.push(maud! { span { "hello" } });
    buf.push(maud! { span { "world" } });

    let result = buf.rendered();
    assert_eq!(result.as_inner(), "<span>hello</span><span>world</span>");
}

#[test]
fn buffer_rendered() {
    let mut buf = Buffer::new();
    buf.push(maud! { div { "test" } });
    let rendered: Rendered<String> = buf.rendered();
    assert_eq!(rendered.as_inner(), "<div>test</div>");
}

#[test]
fn raw_bypass_escaping() {
    let raw = Raw::dangerously_create("<b>bold</b>");
    let result = maud! { div { (raw) } }.render();
    assert_eq!(result.as_inner(), "<div><b>bold</b></div>");
}

#[test]
fn raw_into_inner() {
    let raw: Raw<&str> = Raw::dangerously_create("hello");
    assert_eq!(raw.into_inner(), "hello");
}

#[test]
fn rendered_into_inner() {
    let result = maud! { div { "hello" } }.render();
    let inner: String = result.into_inner();
    assert_eq!(inner, "<div>hello</div>");
}

#[test]
fn buffer_dangerously_get_string() {
    let mut buf = Buffer::new();
    buf.push(maud! { p { "test" } });
    let s: &String = buf.dangerously_get_string();
    assert_eq!(s, "<p>test</p>");
}
