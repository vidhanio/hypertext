#![cfg(feature = "alloc")]
#![expect(missing_docs)]

use hypertext::prelude::*;

#[test]
fn readme_shopping_list() {
    let shopping_list = ["milk", "eggs", "bread"];

    let shopping_list_maud = maud! {
        div {
            h1 { "Shopping List" }
            ul {
                @for (i, item) in (1..).zip(shopping_list) {
                    li.item {
                        input #{ "item-" (i) } type="checkbox";
                        label for={ "item-" (i) } { (item) }
                    }
                }
            }
        }
    }
    .render();

    // or, alternatively:

    let shopping_list_rsx = rsx! {
        <div>
            <h1>Shopping List</h1>
            <ul>
                @for (i, item) in (1..).zip(shopping_list) {
                    <li class="item">
                        <input id={ "item-" (i) } type="checkbox">
                        <label for={ "item-" (i) }>(item)</label>
                    </li>
                }
            </ul>
        </div>
    }
    .render();

    for result in [shopping_list_maud, shopping_list_rsx] {
        assert_eq!(
            result.as_inner(),
            r#"<div><h1>Shopping List</h1><ul><li class="item"><input id="item-1" type="checkbox"><label for="item-1">milk</label></li><li class="item"><input id="item-2" type="checkbox"><label for="item-2">eggs</label></li><li class="item"><input id="item-3" type="checkbox"><label for="item-3">bread</label></li></ul></div>"#
        );
    }
}

#[test]
fn full_page_rendering() {
    let title = "My Page";
    let items = ["Home", "About", "Contact"];

    let maud_result = maud! {
        !DOCTYPE
        html {
            head {
                meta charset="utf-8";
                title { (title) }
            }
            body {
                nav {
                    ul {
                        @for item in items {
                            li { a href="#" { (item) } }
                        }
                    }
                }
                main {
                    h1 { "Welcome" }
                    p { "This is a test page." }
                }
            }
        }
    }
    .render();

    assert!(maud_result.as_inner().starts_with("<!DOCTYPE html><html>"));
    assert!(maud_result.as_inner().contains("<title>My Page</title>"));
    assert!(
        maud_result
            .as_inner()
            .contains("<li><a href=\"#\">Home</a></li>")
    );
    assert!(maud_result.as_inner().ends_with("</html>"));
}

#[test]
fn mixed_static_and_dynamic_content() {
    let user_name = "Alice";
    let is_admin = true;
    let messages = vec!["Hello", "World"];

    let result = maud! {
        div .dashboard {
            h1 { "Welcome, " (user_name) }
            @if is_admin {
                span .badge { "Admin" }
            }
            ul {
                @for msg in &messages {
                    li { (msg) }
                }
            }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div class="dashboard"><h1>Welcome, Alice</h1><span class="badge">Admin</span><ul><li>Hello</li><li>World</li></ul></div>"#
    );
}
