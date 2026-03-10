//! Tests for the `#[renderable]` attribute macro.
#![cfg(feature = "alloc")]

use hypertext::prelude::*;

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
