//! Tests for borrow-mode rendering macros.
#![cfg(feature = "alloc")]

use hypertext::prelude::*;

#[test]
fn borrow_maud_and_rsx() {
    let s = "Hello, world!".to_owned();
    let maud_result = maud::borrow! { span { (s) } };
    let rsx_result = rsx::borrow! { <span>(s)</span> };
    // still able to use `s` after the borrow, as we use `maud::borrow!` and
    // `rsx::borrow!`
    let expected = format!("<span>{s}</span>");

    assert_eq!(maud_result.render().into_inner(), expected);
    assert_eq!(rsx_result.render().into_inner(), expected);
}

#[test]
#[expect(clippy::useless_vec)]
fn borrow_preserves_ownership() {
    let data = vec![1, 2, 3];
    let _lazy = maud::borrow! {
        span { (data.len()) }
    };
    assert_eq!(data.len(), 3);
}

#[test]
fn borrow_with_multiple_captures() {
    let name = "Alice".to_owned();
    let age = 30;

    let result = maud::borrow! {
        p { (name) " is " (age) }
    }
    .render();

    assert_eq!(name, "Alice");
    assert_eq!(result.as_inner(), "<p>Alice is 30</p>");
}
