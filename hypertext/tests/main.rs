//! Tests for the `hypertext` crate.

#[test]
fn readme() {
    use hypertext::{GlobalAttributes, RenderIterator, Renderable, html_elements};

    let shopping_list = ["milk", "eggs", "bread"];

    let shopping_list_maud = hypertext::maud! {
        div {
            h1 { "Shopping List" }
            ul {
                @for (&item, i) in shopping_list.iter().zip(1..) {
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

    let shopping_list_rsx = hypertext::rsx! {
        <div>
            <h1>Shopping List</h1>
            <ul>
                { shopping_list.iter().zip(1..).map(|(&item, i)| hypertext::rsx_move! {
                    <li class="item">
                        <input id=format!("item-{i}") type="checkbox">
                        <label for=format!("item-{i}")>{ item }</label>
                    </li>
                }).render_all() }
            </ul>
        </div>
    }
    .render();

    assert_eq!(shopping_list_maud, shopping_list_rsx);
}

#[test]
#[cfg(feature = "htmx")]
fn htmx() {
    use hypertext::{HtmxAttributes, Renderable, Rendered, html_elements, maud, rsx};

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

    for (test, expected) in tests {
        assert_eq!(test, Rendered(expected.to_string()));
    }
}

#[test]
fn elements_macro() {
    use hypertext::Renderable;

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

    let custom_maud = hypertext::maud! {
        div {
            my_element my_attribute="test" {
                "Hello, world!"
            }
        }
    }
    .render();

    assert_eq!(
        custom_maud,
        r#"<div><my_element my_attribute="test">Hello, world!</my_element></div>"#
    );
}

#[test]
fn can_render_arc() {
    use hypertext::{Renderable, html_elements};

    let value = std::sync::Arc::new("arc");
    let result = hypertext::maud!(span { (value) }).render();

    assert_eq!(result, "<span>arc</span>");
}

#[test]
fn can_render_box() {
    use hypertext::{Renderable, html_elements};

    let value = Box::new("box");
    let result = hypertext::maud!(span { (value) }).render();

    assert_eq!(result, "<span>box</span>");
}

#[test]
fn can_render_rc() {
    use hypertext::{Renderable, html_elements};

    let value = std::rc::Rc::new("rc");
    let result = hypertext::maud!(span { (value) }).render();

    assert_eq!(result, "<span>rc</span>");
}

#[test]
fn can_render_cow() {
    use hypertext::{Renderable, html_elements};

    let value = std::borrow::Cow::from("cow");
    let result = hypertext::maud!(span { (value) }).render();

    assert_eq!(result, "<span>cow</span>");
}

#[test]
fn can_render_vec() {
    use hypertext::{Renderable, html_elements, maud_move};

    let groceries = ["milk", "eggs", "bread"]
        .into_iter()
        .map(|s| maud_move! { li { (s) } })
        .collect::<Vec<_>>();

    let result = hypertext::maud! {
        ul { (groceries) }
    }
    .render();

    assert_eq!(result, "<ul><li>milk</li><li>eggs</li><li>bread</li></ul>");
}

#[test]
fn correct_attr_escape() {
    use hypertext::{Renderable, html_elements, maud};

    let xss = r#""alert('XSS')"#;

    let test = maud! {
        div data-code=(xss) {}
    }
    .render();

    assert_eq!(test, r#"<div data-code="&quot;alert('XSS')"></div>"#);
}
