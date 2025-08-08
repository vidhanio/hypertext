//! Tests for the `hypertext` crate.
#![cfg(feature = "alloc")]

use std::fmt::{self, Display, Formatter};

use hypertext::{Buffer, Raw, maud_borrow, maud_static, prelude::*, rsx_borrow, rsx_static};

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
fn class_id_maud() {
    let result = maud! {
        div #profile .class:colon-dash {
            h1 { "Hello, world!" }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div id="profile" class="class:colon-dash"><h1>Hello, world!</h1></div>"#
    );
}

#[test]
#[cfg(feature = "htmx")]
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
                        <input type="text" name="username">
                        <input type="password" name="password">
                        <input type="submit" value="Login">
                    </form>
                </div>
            }
            .render(),
            r#"<div><form hx-post="/login" hx-on::after-request="this.reset()"><input type="text" name="username"><input type="password" name="password"><input type="submit" value="Login"></form></div>"#,
        ),
    ];

    for (result, expected) in tests {
        assert_eq!(result.as_inner(), expected);
    }
}

#[test]
#[cfg(feature = "alpine")]
#[allow(clippy::too_many_lines)]
fn alpine() {
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
        (
            maud! { div :class="! open ? 'hidden' : ''" { "Hello, world!" } }.render(),
            r#"<div :class="! open ? 'hidden' : ''">Hello, world!</div>"#,
        ),
        (
            rsx! { <div :class="! open ? 'hidden' : ''">"Hello, world!"</div> }.render(),
            r#"<div :class="! open ? 'hidden' : ''">Hello, world!</div>"#,
        ),
        (
            maud! { div x-on:click="open = ! open" { "Hello, world!" } }.render(),
            r#"<div x-on:click="open = ! open">Hello, world!</div>"#,
        ),
        (
            rsx! { <div x-on:click="open = ! open">"Hello, world!"</div> }.render(),
            r#"<div x-on:click="open = ! open">Hello, world!</div>"#,
        ),
        (
            maud! { div @click="open = ! open" { "Hello, world!" } }.render(),
            r#"<div @click="open = ! open">Hello, world!</div>"#,
        ),
        (
            rsx! { <div @click="open = ! open">"Hello, world!"</div> }.render(),
            r#"<div @click="open = ! open">Hello, world!</div>"#,
        ),
        (
            maud! { div @click.shift="open = ! open" { "Hello, world!" } }.render(),
            r#"<div @click.shift="open = ! open">Hello, world!</div>"#,
        ),
        (
            rsx! { <div @click.shift="open = ! open">"Hello, world!"</div> }.render(),
            r#"<div @click.shift="open = ! open">Hello, world!</div>"#,
        ),
        (
            maud! { input type="text" @keyup.enter="alert('Submitted!')"; }.render(),
            r#"<input type="text" @keyup.enter="alert('Submitted!')">"#,
        ),
        (
            rsx! { <input type="text" @keyup.enter="alert('Submitted!')" /> }.render(),
            r#"<input type="text" @keyup.enter="alert('Submitted!')">"#,
        ),
        (
            maud! { input type="text" @keyup.shift.enter="alert('Submitted!')"; }.render(),
            r#"<input type="text" @keyup.shift.enter="alert('Submitted!')">"#,
        ),
        (
            rsx! { <input type="text" @keyup.shift.enter="alert('Submitted!')" /> }.render(),
            r#"<input type="text" @keyup.shift.enter="alert('Submitted!')">"#,
        ),
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
        (
            maud! { input type="text" x-model="search"; }.render(),
            r#"<input type="text" x-model="search">"#,
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
        (
            maud! { input type="text" x-ref="content"; }.render(),
            r#"<input type="text" x-ref="content">"#,
        ),
        (
            rsx! { <input type="text" x-ref="content" /> }.render(),
            r#"<input type="text" x-ref="content">"#,
        ),
        (
            maud! { div x-cloak {} }.render(),
            r"<div x-cloak></div>",
        ),
        (
            rsx! { <div x-cloak></div> }.render(),
            r"<div x-cloak></div>",
        ),
        (
            maud! { div x-ignore {} }.render(),
            r"<div x-ignore></div>",
        ),
        (
            rsx! { <div x-ignore></div> }.render(),
            r"<div x-ignore></div>",
        ),
    ];

    for (test, expected) in tests {
        assert_eq!(test.as_inner(), expected);
    }
}

#[test]
#[cfg(feature = "hyperscript")]
fn hyperscript() {
    let results = [
        maud! {
            button _="on click increment :x then put result into the next <output/>" {
                "Click Me"
            }
            output { "--" }
        }
        .render(),
        rsx! {
            <button _="on click increment :x then put result into the next <output/>">
                Click Me
            </button>
            <output>"--"</output>
        }
        .render(),
    ];

    for result in results {
        assert_eq!(
            result.as_inner(),
            r#"<button _="on click increment :x then put result into the next &lt;output/&gt;">Click Me</button><output>--</output>"#,
        );
    }
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
        result.as_inner(),
        "<ul><li>milk</li><li>eggs</li><li>bread</li></ul>"
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
        result.as_inner(),
        r#"<div data-code="&quot;alert('XSS')"></div>"#
    );
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
        assert_eq!(result.into_inner(), EXPECTED);
    }

    for result in [MAUD_RENDERED_RESULT, RSX_RENDERED_RESULT] {
        assert_eq!(result.into_inner(), EXPECTED);
    }
}

#[test]
fn control() {
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
                <span>(i)</span>
            }

            @let mut i = 3;

            @while i < 6 {
                <span>(i)</span>
                (i += 1)
            }
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<div><span>branch 1</span><span>branch 2</span><span>0</span><span>1</span><span>2</span><span>3</span><span>4</span><span>5</span></div>"
        );
    }
}

#[test]
fn component_fns() {
    fn component() -> impl Renderable {
        maud! { span { "Hello, world!" } }
    }

    fn wrapping_component_maud(c: impl Renderable) -> impl Renderable {
        maud! { div { (c) } }
    }

    fn wrapping_component_rsx(c: impl Renderable) -> impl Renderable {
        rsx! { <div>(c)</div> }
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
        result.as_inner(),
        r"<div><span>Hello, world!</span><div><span>Hello, world!</span></div><div><span>Hello, world!</span></div></div>"
    );
}

#[test]
fn borrow() {
    let s = "Hello, world!".to_owned();
    let maud_result = maud_borrow! { span { (s) } };
    let rsx_result = rsx_borrow! { <span>(s)</span> };
    // still able to use `s` after the borrow, as we use `maud_borrow!` and
    // `rsx_borrow!`
    let expected = format!("<span>{s}</span>");

    assert_eq!(maud_result.render().into_inner(), expected);
    assert_eq!(rsx_result.render().into_inner(), expected);
}

#[test]
fn void_elements() {
    let maud_result = maud! {
        div {
            input type="text" name="username";
            input type="password" name="password";
            input type="submit" value="Login";
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            <input type="text" name="username">
            <input type="password" name="password">
            <input type="submit" value="Login">
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<div><input type="text" name="username"><input type="password" name="password"><input type="submit" value="Login"></div>"#
        );
    }
}

#[test]
fn component() {
    struct Repeater<R> {
        count: usize,
        children: R,
    }

    impl<R: Renderable> Renderable for Repeater<R> {
        fn render_to(&self, buffer: &mut Buffer) {
            maud! {
                @for _ in 0..self.count {
                    (self.children)
                }
            }
            .render_to(buffer);
        }
    }

    let maud_result = maud! {
        div {
            Repeater count=3 {
                span { "Hello, world!" }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            <Repeater count=3>
                <span>"Hello, world!"</span>
            </Repeater>
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<div><span>Hello, world!</span><span>Hello, world!</span><span>Hello, world!</span></div>"
        );
    }
}

#[test]
fn unindent() {
    let result = maud! {
        div title="
        multiline
        title
        " {
            "
            in
                out
            in
            "
        }
        "\n"
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<div title=\"multiline\ntitle\">in\n    out\nin</div>\n"
    );
}

#[test]
fn displayed_debugged() {
    #[derive(Debug)]
    struct Greeting(&'static str);

    impl Display for Greeting {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "Hello, {}! <script>", self.0)
        }
    }

    let maud_result = maud! {
        div {
            %(Greeting("World"))
        }
        div {
            ?(Greeting("World"))
        }
        div {
            (format_args!("{:#X}", 3_735_928_559_u32))
        }
    }
    .render();

    let rsx_result = rsx! {
        <div>
            %(Greeting("World"))
        </div>
        <div>
            ?(Greeting("World"))
        </div>
        <div>
            (format_args!("{:#X}", 3_735_928_559_u32))
        </div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<div>Hello, World! &lt;script&gt;</div><div>Greeting(\"World\")</div><div>0xDEADBEEF</div>"
        );
    }
}

#[test]
fn aria() {
    let maud_result = maud! {
        div aria-label="Hello, world!" {
            "Hello, world!"
        }
    }
    .render();

    let rsx_result = rsx! {
        <div aria-label="Hello, world!">"Hello, world!"</div>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            r#"<div aria-label="Hello, world!">Hello, world!</div>"#
        );
    }
}

#[test]
#[cfg(feature = "mathml")]
fn mathml() {
    let maud_result = maud! {
        math  {
            mi { "x" }
            mo { "+" }
            mn { "1" }
        }
    }
    .render();

    let rsx_result = rsx! {
        <math>
            <mi>x</mi>
            <mo>"+"</mo>
            <mn>1</mn>
        </math>
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(
            result.as_inner(),
            "<math><mi>x</mi><mo>+</mo><mn>1</mn></math>"
        );
    }
}

#[test]
fn component_attr() {
    #[component(HelloWorld)]
    fn my_component() -> impl Renderable {
        maud! { span { "Hi!" } }
    }

    mod component_module {
        use hypertext::prelude::*;

        #[component(pub)]
        fn private_component() -> impl Renderable {
            maud! { span { "secret..." } }
        }
    }

    use component_module::PrivateComponent;

    assert_eq!(
        maud! { HelloWorld; PrivateComponent; }.render().as_inner(),
        "<span>Hi!</span><span>secret...</span>"
    );
}

#[test]
fn toggles() {
    let option_some = Some("value");
    let option_none = None::<&str>;

    let maud_result = maud! {
        input id=[option_some] type="checkbox" checked;
    }
    .render();

    let rsx_result = rsx! {
        <input id=[option_none] type="checkbox" checked />
    }
    .render();

    assert_eq!(
        maud_result.as_inner(),
        r#"<input id="value" type="checkbox" checked>"#
    );
    assert_eq!(rsx_result.as_inner(), r#"<input type="checkbox" checked>"#);
}
