#![cfg(feature = "alloc")]

use hypertext::prelude::*;

#[test]
fn if_true() {
    let show = true;

    let maud_result = maud! {
        @if show {
            p { "visible" }
        }
    }
    .render();

    let rsx_result = rsx! {
        @if show {
            <p>visible</p>
        }
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<p>visible</p>");
    }
}

#[test]
fn if_false() {
    let show = false;

    let maud_result = maud! {
        @if show {
            p { "visible" }
        }
    }
    .render();

    let rsx_result = rsx! {
        @if show {
            <p>visible</p>
        }
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "");
    }
}

#[test]
fn if_else_true_branch() {
    let logged_in = true;

    let maud_result = maud! {
        @if logged_in {
            p { "Welcome!" }
        } @else {
            p { "Please log in." }
        }
    }
    .render();

    let rsx_result = rsx! {
        @if logged_in {
            <p>"Welcome!"</p>
        } @else {
            <p>"Please log in."</p>
        }
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<p>Welcome!</p>");
    }
}

#[test]
fn if_else_false_branch() {
    let logged_in = false;

    let maud_result = maud! {
        @if logged_in {
            p { "Welcome!" }
        } @else {
            p { "Please log in." }
        }
    }
    .render();

    let rsx_result = rsx! {
        @if logged_in {
            <p>"Welcome!"</p>
        } @else {
            <p>"Please log in."</p>
        }
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<p>Please log in.</p>");
    }
}

#[test]
fn if_else_if_else_chain() {
    let score = 75;

    let maud_result = maud! {
        @if score >= 90 {
            span { "A" }
        } @else if score >= 70 {
            span { "B" }
        } @else {
            span { "C" }
        }
    }
    .render();

    let rsx_result = rsx! {
        @if score >= 90 {
            <span>A</span>
        } @else if score >= 70 {
            <span>B</span>
        } @else {
            <span>C</span>
        }
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>B</span>");
    }
}

#[test]
fn if_let_some() {
    let name = Some("Alice");

    let maud_result = maud! {
        @if let Some(n) = name {
            p { "Hello, " (n) }
        }
    }
    .render();

    let rsx_result = rsx! {
        @if let Some(n) = name {
            <p>"Hello, " (n)</p>
        }
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<p>Hello, Alice</p>");
    }
}

#[test]
fn if_let_none() {
    let name = None::<&str>;

    let maud_result = maud! {
        @if let Some(n) = name {
            p { "Hello, " (n) }
        }
    }
    .render();

    let rsx_result = rsx! {
        @if let Some(n) = name {
            <p>"Hello, " (n)</p>
        }
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "");
    }
}

#[test]
fn if_let_with_else() {
    let value = None::<&str>;

    let maud_result = maud! {
        @if let Some(v) = value {
            span { (v) }
        } @else {
            span { "nothing" }
        }
    }
    .render();

    let rsx_result = rsx! {
        @if let Some(v) = value {
            <span>(v)</span>
        } @else {
            <span>nothing</span>
        }
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>nothing</span>");
    }
}

#[test]
fn match_basic() {
    let status = "active";

    let maud_result = maud! {
        @match status {
            "active" => {
                span { "Active" }
            },
            "inactive" => {
                span { "Inactive" }
            },
            _ => {
                span { "Unknown" }
            },
        }
    }
    .render();

    let rsx_result = rsx! {
        @match status {
            "active" => {
                <span>Active</span>
            },
            "inactive" => {
                <span>Inactive</span>
            },
            _ => {
                <span>Unknown</span>
            },
        }
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>Active</span>");
    }
}

#[test]
fn match_wildcard_arm() {
    let status = "pending";

    let maud_result = maud! {
        @match status {
            "active" => {
                span { "Active" }
            },
            _ => {
                span { "Other" }
            },
        }
    }
    .render();

    let rsx_result = rsx! {
        @match status {
            "active" => {
                <span>Active</span>
            },
            _ => {
                <span>Other</span>
            },
        }
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>Other</span>");
    }
}

#[test]
fn match_with_guard() {
    let value = 42;

    let maud_result = maud! {
        @match value {
            x if x > 100 => {
                span { "big" }
            },
            x if x > 10 => {
                span { "medium" }
            },
            _ => {
                span { "small" }
            },
        }
    }
    .render();

    let rsx_result = rsx! {
        @match value {
            x if x > 100 => {
                <span>big</span>
            },
            x if x > 10 => {
                <span>medium</span>
            },
            _ => {
                <span>small</span>
            },
        }
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>medium</span>");
    }
}

#[test]
fn match_enum() {
    #[derive(Clone, Copy)]
    #[allow(dead_code)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    let color = Color::Green;

    let maud_result = maud! {
        @match color {
            Color::Red => {
                span { "red" }
            },
            Color::Green => {
                span { "green" }
            },
            Color::Blue => {
                span { "blue" }
            },
        }
    }
    .render();

    let rsx_result = rsx! {
        @match color {
            Color::Red => {
                <span>red</span>
            },
            Color::Green => {
                <span>green</span>
            },
            Color::Blue => {
                <span>blue</span>
            },
        }
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>green</span>");
    }
}

#[test]
fn match_or_patterns() {
    let n = 2;

    let maud_result = maud! {
        @match n {
            1 | 2 => {
                span { "one or two" }
            },
            _ => {
                span { "other" }
            },
        }
    }
    .render();

    let rsx_result = rsx! {
        @match n {
            1 | 2 => {
                <span>"one or two"</span>
            },
            _ => {
                <span>other</span>
            },
        }
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>one or two</span>");
    }
}

#[test]
fn for_range() {
    let maud_result = maud! {
        ul {
            @for i in 1..=3 {
                li { (i) }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ul>
            @for i in 1..=3 {
                <li>(i)</li>
            }
        </ul>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<ul><li>1</li><li>2</li><li>3</li></ul>");
    }
}

#[test]
fn for_slice() {
    let items = ["apple", "banana", "cherry"];

    let maud_result = maud! {
        ul {
            @for item in &items {
                li { (item) }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ul>
            @for item in &items {
                <li>(item)</li>
            }
        </ul>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<ul><li>apple</li><li>banana</li><li>cherry</li></ul>"
        );
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
fn for_empty_iterator() {
    let items: &[&str] = &[];

    let maud_result = maud! {
        ul {
            @for item in items {
                li { (item) }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ul>
            @for item in items {
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
fn while_let_with_iterator() {
    use core::cell::RefCell;

    let items = vec!["a", "b", "c"];
    let iter = RefCell::new(items.into_iter());

    let result = maud::borrow! {
        ul {
            @while let Some(item) = iter.borrow_mut().next() {
                li { (item) }
            }
        }
    }
    .render();

    assert_eq!(result.as_inner(), "<ul><li>a</li><li>b</li><li>c</li></ul>");
}

#[test]
fn let_binding() {
    let maud_result = maud! {
        @let greeting = "Hello";
        p { (greeting) }
    }
    .render();

    let rsx_result = rsx! {
        @let greeting = "Hello";
        <p>(greeting)</p>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<p>Hello</p>");
    }
}

#[test]
fn let_destructuring() {
    let pair = (42, "answer");

    let maud_result = maud! {
        @let (num, label) = pair;
        span { (label) ": " (num) }
    }
    .render();

    let rsx_result = rsx! {
        @let (num, label) = pair;
        <span>(label) ": " (num)</span>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>answer: 42</span>");
    }
}

#[test]
fn nested_if_in_for() {
    let items = [1, 2, 3, 4, 5];

    let maud_result = maud! {
        ul {
            @for item in &items {
                @if *item % 2 == 0 {
                    li { (item) " (even)" }
                }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ul>
            @for item in &items {
                @if *item % 2 == 0 {
                    <li>(item) " (even)"</li>
                }
            }
        </ul>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<ul><li>2 (even)</li><li>4 (even)</li></ul>"
        );
    }
}

#[test]
fn nested_for_in_for() {
    let rows = [1, 2];
    let cols = ["a", "b"];

    let maud_result = maud! {
        table {
            @for row in &rows {
                tr {
                    @for col in &cols {
                        td { (row) (col) }
                    }
                }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <table>
            @for row in &rows {
                <tr>
                    @for col in &cols {
                        <td>(row) (col)</td>
                    }
                </tr>
            }
        </table>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<table><tr><td>1a</td><td>1b</td></tr><tr><td>2a</td><td>2b</td></tr></table>"
        );
    }
}

#[test]
fn nested_match_in_for() {
    let values = [Some(1), None, Some(3)];

    let maud_result = maud! {
        ul {
            @for v in &values {
                @match v {
                    Some(n) => {
                        li { (n) }
                    },
                    None => {
                        li { "-" }
                    },
                }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ul>
            @for v in &values {
                @match v {
                    Some(n) => {
                        <li>(n)</li>
                    },
                    None => {
                        <li>"-"</li>
                    },
                }
            }
        </ul>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<ul><li>1</li><li>-</li><li>3</li></ul>");
    }
}

#[test]
fn if_with_surrounding_content() {
    let show = true;

    let maud_result = maud! {
        div {
            h1 { "Title" }
            @if show {
                p { "Conditional content" }
            }
            footer { "Always here" }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            <h1>Title</h1>
            @if show {
                <p>"Conditional content"</p>
            }
            <footer>"Always here"</footer>
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<div><h1>Title</h1><p>Conditional content</p><footer>Always here</footer></div>"
        );
    }
}

#[test]
fn for_in_attribute_value() {
    let result = attribute! { "x" @for i in 0..5 { (i) } }
        .to_buffer()
        .into_inner();
    assert_eq!(result, "x01234");
}

#[test]
fn if_in_attribute_value() {
    let show = true;
    let result = attribute! { "base" @if show { "-active" } }
        .to_buffer()
        .into_inner();
    assert_eq!(result, "base-active");
}
