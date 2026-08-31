//! Control-flow coverage for nodes and attribute values.
#![cfg(feature = "alloc")]

use hypertext::prelude::*;

macro_rules! assert_html_pair {
    ($maud:expr, $rsx:expr, $expected:expr $(,)?) => {{
        let maud = ($maud).render();
        let rsx = ($rsx).render();
        assert_eq!(maud.as_inner(), $expected);
        assert_eq!(rsx.as_inner(), $expected);
    }};
}

#[test]
fn if_else_and_else_if_are_parity_cases() {
    for show in [true, false] {
        let expected = if show {
            "<p>visible</p>"
        } else {
            "<p>hidden</p>"
        };

        assert_html_pair!(
            maud! {
                @if show {
                    p { "visible" }
                } @else {
                    p { "hidden" }
                }
            },
            rsx! {
                @if show {
                    <p>visible</p>
                } @else {
                    <p>hidden</p>
                }
            },
            expected,
        );
    }

    for (score, expected) in [
        (95, "<span>A</span>"),
        (75, "<span>B</span>"),
        (50, "<span>C</span>"),
    ] {
        assert_html_pair!(
            maud! {
                @if score >= 90 { span { "A" } }
                @else if score >= 70 { span { "B" } }
                @else { span { "C" } }
            },
            rsx! {
                @if score >= 90 { <span>A</span> }
                @else if score >= 70 { <span>B</span> }
                @else { <span>C</span> }
            },
            expected,
        );
    }
}

#[test]
fn if_let_binds_values_and_handles_none() {
    for (name, expected) in [
        (Some("Alice"), "<span>Hello, Alice</span>"),
        (Some("Bob"), "<span>Hello, Bob</span>"),
        (None, "<span>anonymous</span>"),
    ] {
        assert_html_pair!(
            maud! {
                @if let Some(name) = name {
                    span { "Hello, " (name) }
                } @else {
                    span { "anonymous" }
                }
            },
            rsx! {
                @if let Some(name) = name {
                    <span>"Hello, " (name)</span>
                } @else {
                    <span>anonymous</span>
                }
            },
            expected,
        );
    }
}

#[test]
fn for_patterns_cover_ranges_enumeration_nesting_and_empty_input() {
    let rows = [["a", "b"], ["c", "d"]];
    assert_html_pair!(
        maud! {
            table {
                @for (row_index, row) in rows.iter().enumerate() {
                    tr data-row=(row_index) {
                        @for cell in row {
                            td { (cell) }
                        }
                    }
                }
            }
        },
        rsx! {
            <table>
                @for (row_index, row) in rows.iter().enumerate() {
                    <tr data-row=(row_index)>
                        @for cell in row {
                            <td>(cell)</td>
                        }
                    </tr>
                }
            </table>
        },
        r#"<table><tr data-row="0"><td>a</td><td>b</td></tr><tr data-row="1"><td>c</td><td>d</td></tr></table>"#,
    );

    assert_html_pair!(
        maud! { ul { @for i in 1..=3 { li { (i) } } } },
        rsx! { <ul>@for i in 1..=3 { <li>(i)</li> }</ul> },
        "<ul><li>1</li><li>2</li><li>3</li></ul>",
    );

    let empty: &[&str] = &[];
    assert_html_pair!(
        maud! { ul { @for item in empty { li { (item) } } } },
        rsx! { <ul>@for item in empty { <li>(item)</li> }</ul> },
        "<ul></ul>",
    );
}

#[test]
fn match_supports_bindings_guards_or_patterns_and_child_bodies() {
    for (value, expected) in [
        (0, "<span>zero</span>"),
        (1, "<span>small</span>"),
        (2, "<span>small</span>"),
        (12, "<span>large: 12</span>"),
        (3, "<span>other</span>"),
    ] {
        assert_html_pair!(
            maud! {
                @match value {
                    0 => span { "zero" },
                    1 | 2 => { span { "small" } },
                    n if n > 10 => { span { "large: " (n) } },
                    _ => { span { "other" } },
                }
            },
            rsx! {
                @match value {
                    0 => <span>zero</span>,
                    1 | 2 => { <span>small</span> },
                    n if n > 10 => { <span>"large: " (n)</span> },
                    _ => { <span>other</span> },
                }
            },
            expected,
        );
    }

    let values = [Some(1), None, Some(3)];
    assert_html_pair!(
        maud! {
            ul {
                @for value in &values {
                    @match value {
                        Some(value) => li { (value) },
                        None => li { "-" },
                    }
                }
            }
        },
        rsx! {
            <ul>
                @for value in &values {
                    @match value {
                        Some(value) => <li>(value)</li>,
                        None => <li>"-"</li>,
                    }
                }
            </ul>
        },
        "<ul><li>1</li><li>-</li><li>3</li></ul>",
    );
}

#[test]
fn let_bindings_are_scoped_and_parity_safe() {
    let pair = (42, "answer");
    assert_html_pair!(
        maud! {
            @let (number, label) = pair;
            div {
                h1 { (label) }
                p { (number) }
            }
        },
        rsx! {
            @let (number, label) = pair;
            <div><h1>(label)</h1><p>(number)</p></div>
        },
        "<div><h1>answer</h1><p>42</p></div>",
    );
}

#[test]
fn while_let_borrows_and_consumes_an_iterator() {
    use core::cell::RefCell;

    let maud_iter = RefCell::new(["a", "b", "c"].into_iter());
    let rsx_iter = RefCell::new(["a", "b", "c"].into_iter());
    assert_html_pair!(
        maud::borrow! {
            ul {
                @while let Some(item) = maud_iter.borrow_mut().next() {
                    li { (item) }
                }
            }
        },
        rsx::borrow! {
            <ul>
                @while let Some(item) = rsx_iter.borrow_mut().next() {
                    <li>(item)</li>
                }
            </ul>
        },
        "<ul><li>a</li><li>b</li><li>c</li></ul>",
    );
}

#[test]
fn attribute_values_accept_the_same_control_flow() {
    let enabled = true;
    let value = attribute! {
        "base"
        @if enabled { "-active" }
        @for index in 0..3 { (index) }
    }
    .to_buffer()
    .into_inner();
    assert_eq!(value, "base-active012");

    let disabled = false;
    let value = attribute! {
        "base"
        @if disabled { "-active" }
    }
    .to_buffer()
    .into_inner();
    assert_eq!(value, "base");
}
