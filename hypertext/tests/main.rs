//! Tests for the `hypertext` crate.

use hypertext::{Raw, Rendered, maud_borrow, maud_static, prelude::*, rsx_borrow, rsx_static};

#[test]
fn readme() {
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
                        <input id={ format!("item-{i}") } type="checkbox" />
                        <label for={ format!("item-{i}") }>{ item }</label>
                    </li>
                }
            </ul>
        </div>
    }
    .render();

    for result in [shopping_list_maud, shopping_list_rsx] {
        assert_eq!(
            result,
            Rendered(
                r#"<div><h1>Shopping List</h1><ul><li class="item"><input id="item-1" type="checkbox"><label for="item-1">milk</label></li><li class="item"><input id="item-2" type="checkbox"><label for="item-2">eggs</label></li><li class="item"><input id="item-3" type="checkbox"><label for="item-3">bread</label></li></ul></div>"#
            )
        );
    }
}

#[test]
fn htmx() {
    let tests = [
        (
            maud! { div hx-get="/api/endpoint" { "Hello, world!" } }.render(),
            r#"<div hx-get="/api/endpoint">Hello, world!</div>"#,
        ),
        (
            rsx! { <div hx-get="/api/endpoint">"Hello, world!"</div> }.render(),
            r#"<div hx-get="/api/endpoint">Hello, world!</div>"#,
        ),
        (
            maud! { div hx-post="/api/endpoint" { "Hello, world!" } }.render(),
            r#"<div hx-post="/api/endpoint">Hello, world!</div>"#,
        ),
        (
            rsx! { <div hx-post="/api/endpoint">"Hello, world!"</div> }.render(),
            r#"<div hx-post="/api/endpoint">Hello, world!</div>"#,
        ),
        (
            maud! { div hx-on:click="this.classList.toggle('active')" { "Hello, world!" } }
                .render(),
            r#"<div hx-on:click="this.classList.toggle('active')">Hello, world!</div>"#,
        ),
        (
            rsx! { <div hx-on:click="this.classList.toggle('active')">"Hello, world!"</div> }
                .render(),
            r#"<div hx-on:click="this.classList.toggle('active')">Hello, world!</div>"#,
        ),
        (
            maud! {
                div {
                    form hx-post="/login" hx-on::after-request="this.reset()" {
                        input type="text" name="username";
                        input type="password" name="password";
                        input type="submit" value="Login";
                }
            }}
            .render(),
            r#"<div><form hx-post="/login" hx-on::after-request="this.reset()"><input type="text" name="username"><input type="password" name="password"><input type="submit" value="Login"></form></div>"#,
        ),
        (
            rsx! {
                <div>
                    <form hx-post="/login" hx-on::after-request="this.reset()">
                        <input type="text" name="username" />
                        <input type="password" name="password" />
                        <input type="submit" value="Login" />
                    </form>
                </div>
            }
            .render(),
            r#"<div><form hx-post="/login" hx-on::after-request="this.reset()"><input type="text" name="username"><input type="password" name="password"><input type="submit" value="Login"></form></div>"#,
        ),
    ];

    for (result, expected) in tests {
        assert_eq!(result, Rendered(expected));
    }
}

#[test]
fn elements_macro() {
    mod html_elements {
        use hypertext::elements;
        pub use hypertext::html_elements::*;

        elements! {
            /// This is a test element
            my_element {
                /// This is a test attribute
                my_attribute
            }
        }
    }

    let custom_maud = maud! {
        div {
            my_element my_attribute="test" {
                "Hello, world!"
            }
        }
    }
    .render();

    assert_eq!(
        custom_maud,
        Rendered(r#"<div><my_element my_attribute="test">Hello, world!</my_element></div>"#)
    );
}

#[test]
fn can_render_vec() {
    let groceries = ["milk", "eggs", "bread"]
        .into_iter()
        .map(|s| maud! { li { (s) } })
        .collect::<Vec<_>>();

    let result = maud! {
        ul { (groceries) }
    }
    .render();

    assert_eq!(
        result,
        Rendered("<ul><li>milk</li><li>eggs</li><li>bread</li></ul>")
    );
}

#[test]
fn correct_attr_escape() {
    let xss = r#""alert('XSS')"#;

    let result = maud! {
        div data-code=(xss) {}
    }
    .render();

    assert_eq!(
        result,
        Rendered(r#"<div data-code="&quot;alert('XSS')"></div>"#)
    );
}

#[test]
fn dynamic() {
    let cond = true;

    let maud_result = if cond {
        maud! { span { "closure 1" } }.dyn_renderable()
    } else {
        maud! { span { "closure 2" } }.dyn_renderable()
    }
    .render();

    let rsx_result = if cond {
        rsx! { <span>closure 1</span> }.dyn_renderable()
    } else {
        rsx! { <span>closure 2</span> }.dyn_renderable()
    }
    .render();

    assert_eq!(maud_result, Rendered("<span>closure 1</span>"));
    assert_eq!(rsx_result, Rendered("<span>closure 1</span>"));
}

#[test]
fn statics() {
    const MAUD_RAW_RESULT: Raw<&str> = maud_static! {
        div #profile title="Profile" {
            h1 { "Hello, world!" }
        }
    };
    const RSX_RAW_RESULT: Raw<&str> = rsx_static! {
        <div id="profile" title="Profile">
            <h1>"Hello, world!"</h1>
        </div>
    };

    const MAUD_RENDERED_RESULT: Rendered<&str> = MAUD_RAW_RESULT.rendered();
    const RSX_RENDERED_RESULT: Rendered<&str> = RSX_RAW_RESULT.rendered();

    const EXPECTED: &str = r#"<div id="profile" title="Profile"><h1>Hello, world!</h1></div>"#;

    for result in [MAUD_RAW_RESULT, RSX_RAW_RESULT] {
        assert_eq!(result, Raw(EXPECTED));
    }

    for result in [MAUD_RENDERED_RESULT, RSX_RENDERED_RESULT] {
        assert_eq!(result, Rendered(EXPECTED));
    }
}

#[test]
fn keywords() {
    let cond = true;

    let maud_result = maud! {
        div {
            @if cond {
                span { "branch 1" }
            } @else {
                span { "branch 2" }
            }


            @match !cond {
                true => span { "branch 1" },
                false => span { "branch 2" },
            }

            @for i in 0..3 {
                span { (i) }
            }

            @let mut i = 3;

            @while i < 6 {
                span { (i) }
                (i += 1)
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            @if cond {
                <span>branch 1</span>
            } @else {
                <span>branch 2</span>
            }

            @match !cond {
                true => {
                    <span>branch 1</span>
                },
                false => <span>branch 2</span>,
            }

            @for i in 0..3 {
                <span>{ i }</span>
            }

            @let mut i = 3;

            @while i < 6 {
                <span>{ i }</span>
                {i += 1}
            }
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result,
            Rendered(
                r"<div><span>branch 1</span><span>branch 2</span><span>0</span><span>1</span><span>2</span><span>3</span><span>4</span><span>5</span></div>"
            )
        );
    }
}

#[test]
fn components() {
    fn component() -> impl Renderable {
        maud! { span { "Hello, world!" } }
    }

    fn wrapping_component_maud(c: impl Renderable) -> impl Renderable {
        maud! { div { (c) } }
    }

    fn wrapping_component_rsx(c: impl Renderable) -> impl Renderable {
        rsx! { <div>{ c }</div> }
    }

    let result = maud! {
        div {
            (component())
            (wrapping_component_maud(component()))
            (wrapping_component_rsx(component()))
        }
    }
    .render();

    assert_eq!(
        result,
        Rendered(
            r"<div><span>Hello, world!</span><div><span>Hello, world!</span></div><div><span>Hello, world!</span></div></div>"
        )
    );
}

#[test]
fn borrow() {
    let s = "Hello, world!".to_owned();
    let maud_result = maud_borrow! { span { (s) } };
    let rsx_result = rsx_borrow! { <span>{ s }</span> };
    // still able to use `s` after the borrow, as we use `maud_borrow!` and
    // `rsx_borrow!`
    let expected = Rendered(format!("<span>{s}</span>"));

    assert_eq!(maud_result.render(), expected);
    assert_eq!(rsx_result.render(), expected);
}
