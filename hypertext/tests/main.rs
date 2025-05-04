//! Tests for the `hypertext` crate.

use hypertext::{
    html_elements, maud, maud_dyn, maud_move, maud_static, rsx, rsx_dyn, rsx_move, rsx_static,
    AlpineJsAttributes, GlobalAttributes, HtmxAttributes, Raw, Renderable, Rendered,
};

#[test]
fn readme() {
    let shopping_list = ["milk", "eggs", "bread"];

    let shopping_list_maud = hypertext::maud! {
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

    let shopping_list_rsx = hypertext::rsx! {
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
            "<div><h1>Shopping List</h1><ul><li class=\"item\"><input id=\"item-1\" type=\"checkbox\"><label for=\"item-1\">milk</label></li><li class=\"item\"><input id=\"item-2\" type=\"checkbox\"><label for=\"item-2\">eggs</label></li><li class=\"item\"><input id=\"item-3\" type=\"checkbox\"><label for=\"item-3\">bread</label></li></ul></div>"
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

    for (test, expected) in tests {
        assert_eq!(test, Rendered(expected.to_string()));
    }
}

#[test]
fn alpinejs() {
    let tests = [
        (
            maud! { div x-data="{ open: false }" { "Hello, world!" } }.render(),
            r#"<div x-data="{ open: false }">Hello, world!</div>"#,
        ),
        (
            rsx! { <div x-data="{ open: false }">"Hello, world!"</div> }.render(),
            r#"<div x-data="{ open: false }">Hello, world!</div>"#,
        ),
        (
            maud! { div x-bind:class="! open ? 'hidden' : ''" { "Hello, world!" } }.render(),
            r#"<div x-bind:class="! open ? 'hidden' : ''">Hello, world!</div>"#,
        ),
        (
            rsx! { <div x-bind:class="! open ? 'hidden' : ''">"Hello, world!"</div> }.render(),
            r#"<div x-bind:class="! open ? 'hidden' : ''">Hello, world!</div>"#,
        ),
        // WARNING: the next two tests do not compile!
        // Test the shorthand syntax for `x:bind`: `:`
        // (
        //     maud! { div :class="! open ? 'hidden' : ''" { "Hello, world!" } }.render(),
        //     r#"<div x-bind:class="! open ? 'hidden' : ''">Hello, world!</div>"#,
        // ),
        // (
        //     rsx! { <div :class="! open ? 'hidden' : ''">"Hello, world!"</div> }.render(),
        //     r#"<div x-bind:class="! open ? 'hidden' : ''">Hello, world!</div>"#,
        // ),
        (
            maud! { div x-on:click="open = ! open" { "Hello, world!" } }.render(),
            r#"<div x-on:click="open = ! open">Hello, world!</div>"#,
        ),
        (
            rsx! { <div x-on:click="open = ! open">"Hello, world!"</div> }.render(),
            r#"<div x-on:click="open = ! open">Hello, world!</div>"#,
        ),
        // WARNING: the next two tests do not compile!
        // Test the shorthand syntax for `x:on`: `@`
        // (
        //     maud! { div @click="open = ! open" { "Hello, world!" } }.render(),
        //     r#"<div x-on:click="open = ! open">Hello, world!</div>"#,
        // ),
        // (
        //     rsx! { <div @click="open = ! open">"Hello, world!"</div> }.render(),
        //     r#"<div x-on:click="open = ! open">Hello, world!</div>"#,
        // ),
        (
            maud! { div x-text="new Date().getFullYear()" { "Hello, world!" } }.render(),
            r#"<div x-text="new Date().getFullYear()">Hello, world!</div>"#,
        ),
        (
            rsx! { <div x-text="new Date().getFullYear()">"Hello, world!"</div> }.render(),
            r#"<div x-text="new Date().getFullYear()">Hello, world!</div>"#,
        ),
        (
            maud! { div x-html="(await axios.get('/some/html/partial')).data" { "Hello, world!" } }.render(),
            r#"<div x-html="(await axios.get('/some/html/partial')).data">Hello, world!</div>"#,
        ),
        (
            rsx! { <div x-html="(await axios.get('/some/html/partial')).data">"Hello, world!"</div> }.render(),
            r#"<div x-html="(await axios.get('/some/html/partial')).data">Hello, world!</div>"#,
        ),
        // WARNING: It seems the input element doesn't render consistently and doesn't auto-close
        (
            maud! { input type="text" x-model="search" {} }.render(),
            r#"<input type="text" x-model="search"></input>"#,
        ),
        (
            rsx! { <input type="text" x-model="search" /> }.render(),
            r#"<input type="text" x-model="search">"#,
        ),
        (
            maud! { div x-show="open" { "Hello, world!" } }.render(),
            r#"<div x-show="open">Hello, world!</div>"#,
        ),
        (
            rsx! { <div x-show="open">"Hello, world!"</div> }.render(),
            r#"<div x-show="open">Hello, world!</div>"#,
        ),
        (
            maud! { div x-show="open" x-transition { "Hello, world!" } }.render(),
            r#"<div x-show="open" x-transition>Hello, world!</div>"#,
        ),
        (
            rsx! { <div x-show="open" x-transition>"Hello, world!"</div> }.render(),
            r#"<div x-show="open" x-transition>Hello, world!</div>"#,
        ),
        (
            maud! { 
                template x-for="post in posts" {
                    h2 x-text="post.title" {}
                }
            }.render(),
            r#"<template x-for="post in posts"><h2 x-text="post.title"></h2></template>"#,
        ),
        (
            rsx! { 
                <template x-for="post in posts">
                    <h2 x-text="post.title"></h2>
                </template>
            }.render(),
            r#"<template x-for="post in posts"><h2 x-text="post.title"></h2></template>"#,
        ),
        (
            maud! { 
                template x-if="open" {
                    h2 x-text="post.title" {}
                }
            }.render(),
            r#"<template x-if="open"><h2 x-text="post.title"></h2></template>"#,
        ),
        (
            rsx! { 
                <template x-if="open">
                    <h2 x-text="post.title"></h2>
                </template>
            }.render(),
            r#"<template x-if="open"><h2 x-text="post.title"></h2></template>"#,
        ),
        (
            maud! { div x-init="date = new Date()" {} }.render(),
            r#"<div x-init="date = new Date()"></div>"#,
        ),
        (
            rsx! { <div x-init="date = new Date()"></div> }.render(),
            r#"<div x-init="date = new Date()"></div>"#,
        ),
        (
            maud! { div x-effect="console.log('Count is '+count)" {} }.render(),
            r#"<div x-effect="console.log('Count is '+count)"></div>"#,
        ),
        (
            rsx! { <div x-effect="console.log('Count is '+count)"></div> }.render(),
            r#"<div x-effect="console.log('Count is '+count)"></div>"#,
        ),
        // WARNING: It seems the input element doesn't render consistently and doesn't auto-close
        (
            maud! { input type="text" x-ref="content" {} }.render(),
            r#"<input type="text" x-ref="content"></input>"#,
        ),
        (
            rsx! { <input type="text" x-ref="content" /> }.render(),
            r#"<input type="text" x-ref="content">"#,
        ),
        (
            maud! { div x-cloak {} }.render(),
            r#"<div x-cloak></div>"#,
        ),
        (
            rsx! { <div x-cloak></div> }.render(),
            r#"<div x-cloak></div>"#,
        ),
        (
            maud! { div x-ignore {} }.render(),
            r#"<div x-ignore></div>"#,
        ),
        (
            rsx! { <div x-ignore></div> }.render(),
            r#"<div x-ignore></div>"#,
        ),
    ];

    for (test, expected) in tests {
        assert_eq!(test, Rendered(expected.to_string()));
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
        r#"<div><my_element my_attribute="test">Hello, world!</my_element></div>"#
    );
}

#[test]
fn can_render_arc() {
    let value = std::sync::Arc::new("arc");
    let result = maud!(span { (value) }).render();

    assert_eq!(result, "<span>arc</span>");
}

#[test]
fn can_render_box() {
    let value = Box::new("box");
    let result = maud!(span { (value) }).render();

    assert_eq!(result, "<span>box</span>");
}

#[test]
fn can_render_rc() {
    let value = std::rc::Rc::new("rc");
    let result = maud!(span { (value) }).render();

    assert_eq!(result, "<span>rc</span>");
}

#[test]
fn can_render_cow() {
    let value = std::borrow::Cow::from("cow");
    let result = maud!(span { (value) }).render();

    assert_eq!(result, "<span>cow</span>");
}

#[test]
fn can_render_vec() {
    let groceries = ["milk", "eggs", "bread"]
        .into_iter()
        .map(|s| maud_move! { li { (s) } })
        .collect::<Vec<_>>();

    let result = maud! {
        ul { (groceries) }
    }
    .render();

    assert_eq!(result, "<ul><li>milk</li><li>eggs</li><li>bread</li></ul>");
}

#[test]
fn correct_attr_escape() {
    let xss = r#""alert('XSS')"#;

    let result = maud! {
        div data-code=(xss) {}
    }
    .render();

    assert_eq!(result, r#"<div data-code="&quot;alert('XSS')"></div>"#);
}

#[test]
fn maud_dyn() {
    let cond = true;
    let result = maud! {
        div {
            (if cond {
                maud_dyn! { span { "closure 1" } }
            } else {
                maud_dyn! { span { "closure 2" } }
            })
        }
    }
    .render();

    assert_eq!(result, "<div><span>closure 1</span></div>");
}

#[test]
fn rsx_dyn() {
    let cond = true;
    let result = rsx! {
        <div>
            {
                if cond {
                    rsx_dyn! { <span>"closure 1"</span> }
                } else {
                    rsx_dyn! { <span>"closure 2"</span> }
                }
            }
        </div>
    }
    .render();

    assert_eq!(result, "<div><span>closure 1</span></div>");
}

#[test]
fn statics() {
    const MAUD_RESULT: Raw<&str> = maud_static! {
        div #profile title="Profile" {
            h1 { "Hello, world!" }
        }
    };

    const RSX_RESULT: Raw<&str> = rsx_static! {
        <div id="profile" title="Profile">
            <h1>"Hello, world!"</h1>
        </div>
    };

    for result in [MAUD_RESULT, RSX_RESULT] {
        assert_eq!(
            result,
            r#"<div id="profile" title="Profile"><h1>Hello, world!</h1></div>"#
        );
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
            "<div><span>branch 1</span><span>branch 2</span><span>0</span><span>1</span><span>2</span><span>3</span><span>4</span><span>5</span></div>"
        );
    }
}

#[test]
fn components() {
    fn component() -> impl Renderable {
        maud! { span { "Hello, world!" } }
    }

    fn wrapping_component_maud(c: impl Renderable) -> impl Renderable {
        maud_move! { div { (c) } }
    }

    fn wrapping_component_rsx(c: impl Renderable) -> impl Renderable {
        rsx_move! { <div>{ c }</div> }
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
        "<div><span>Hello, world!</span><div><span>Hello, world!</span></div><div><span>Hello, world!</span></div></div>"
    );
}
