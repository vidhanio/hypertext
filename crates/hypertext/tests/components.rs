//! Component and derive macro tests.
#![cfg(feature = "alloc")]

use hypertext::{Builder, prelude::*};

#[derive(Builder, Renderable)]
#[maud(span { "Hello, " (self.name) "!" })]
struct MaudGreeting {
    name: String,
}

#[test]
fn derive_renderable_maud_basic() {
    let greeting = MaudGreeting {
        name: "Alice".into(),
    };

    let result = maud! { div { (greeting) } }.render();
    assert_eq!(result.as_inner(), "<div><span>Hello, Alice!</span></div>");
}

#[test]
fn derive_renderable_maud_invoked_as_component() {
    let maud_result = maud! {
        div {
            MaudGreeting name=("Bob".into());
        }
    }
    .render();

    assert_eq!(
        maud_result.as_inner(),
        "<div><span>Hello, Bob!</span></div>"
    );
}

#[derive(Builder, Renderable)]
#[rsx(<span>"Hello, " (self.name) "!"</span>)]
struct RsxGreeting {
    name: String,
}

#[test]
fn derive_renderable_rsx_basic() {
    let greeting = RsxGreeting {
        name: "Carol".into(),
    };

    let result = rsx! { <div>(greeting)</div> }.render();
    assert_eq!(result.as_inner(), "<div><span>Hello, Carol!</span></div>");
}

#[test]
fn derive_renderable_rsx_invoked_as_component() {
    let rsx_result = rsx! {
        <div>
            <RsxGreeting name=("Dave".into())>
        </div>
    }
    .render();

    assert_eq!(
        rsx_result.as_inner(),
        "<div><span>Hello, Dave!</span></div>"
    );
}

#[derive(Builder, Renderable)]
#[attribute((self.x) "," (self.y))]
struct Coordinates {
    x: i32,
    y: i32,
}

#[test]
fn derive_renderable_attribute() {
    let coords = Coordinates { x: 10, y: 20 };

    let maud_result = maud! { div title=(coords) { "Location" } }.render();
    let rsx_result = rsx! { <div title=(Coordinates { x: 10, y: 20 })>"Location"</div> }.render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), r#"<div title="10,20">Location</div>"#);
    }
}

#[derive(Builder, Renderable)]
#[maud(span .badge { (self.label) })]
#[attribute((self.label))]
struct Badge {
    label: String,
}

#[test]
fn derive_renderable_node_and_attribute() {
    let badge = Badge {
        label: "new".into(),
    };
    let node_result = maud! { div { (badge) } }.render();
    assert_eq!(
        node_result.as_inner(),
        r#"<div><span class="badge">new</span></div>"#
    );

    let attr_badge = Badge {
        label: "hot".into(),
    };
    let attr_result = maud! { div title=(attr_badge) {} }.render();
    assert_eq!(attr_result.as_inner(), r#"<div title="hot"></div>"#);
}

#[derive(Builder, Renderable)]
#[maud(
    div .card {
        h2 { (self.title) }
        p { (self.body) }
    }
)]
struct Card {
    title: String,
    body: String,
}

#[test]
fn card_component_maud_invocation() {
    let result = maud! {
        main {
            Card title=("My Title".into()) body=("My Body".into());
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<main><div class="card"><h2>My Title</h2><p>My Body</p></div></main>"#,
    );
}

#[derive(Builder, Renderable)]
#[rsx(
    <div class="card">
        <h2>(self.title)</h2>
        <p>(self.body)</p>
    </div>
)]
struct RsxCard {
    title: String,
    body: String,
}

#[test]
fn card_component_rsx_invocation() {
    let result = rsx! {
        <main>
            <RsxCard title=("My Title".into()) body=("My Body".into())>
        </main>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<main><div class="card"><h2>My Title</h2><p>My Body</p></div></main>"#,
    );
}

#[derive(Builder, Renderable)]
#[maud(
    div {
        h1 { (self.title) }
        @if let Some(subtitle) = &self.subtitle {
            h2 { (subtitle) }
        }
    }
)]
struct Header {
    title: String,
    subtitle: Option<String>,
}

#[test]
fn component_optional_field_absent() {
    let result = maud! {
        Header title=("Hello".into());
    }
    .render();

    assert_eq!(result.as_inner(), "<div><h1>Hello</h1></div>");
}

#[test]
fn component_optional_field_present() {
    let result = maud! {
        Header title=("Hello".into()) subtitle=("World".into());
    }
    .render();

    assert_eq!(result.as_inner(), "<div><h1>Hello</h1><h2>World</h2></div>");
}

#[test]
fn component_optional_field_both_variants() {
    let result = maud! {
        Header title=("Hello".into());
        Header title=("Hello".into()) subtitle=("World".into());
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<div><h1>Hello</h1></div><div><h1>Hello</h1><h2>World</h2></div>"
    );
}

#[renderable]
fn nav_bar<'a>(title: &'a str, subtitle: &String, add_smiley: bool) -> impl Renderable {
    maud! {
        nav {
            h1 { (title) }
            h2 { (subtitle) }
            @if add_smiley {
                span { ":)" }
            }
        }
    }
}

#[test]
fn renderable_function_maud_invocation() {
    let result = maud! {
        div {
            NavBar title="My Nav Bar" subtitle=("My Subtitle".to_owned()) add_smiley=true;
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<div><nav><h1>My Nav Bar</h1><h2>My Subtitle</h2><span>:)</span></nav></div>"
    );
}

#[test]
fn renderable_function_rsx_invocation() {
    let result = rsx! {
        <div>
            <NavBar title="My Nav Bar" subtitle=("My Subtitle".to_owned()) add_smiley=true>
        </div>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<div><nav><h1>My Nav Bar</h1><h2>My Subtitle</h2><span>:)</span></nav></div>"
    );
}

#[test]
fn renderable_function_smiley_off() {
    let result = maud! {
        NavBar title="Hello" subtitle=("World".to_owned()) add_smiley=false;
    }
    .render();

    assert_eq!(result.as_inner(), "<nav><h1>Hello</h1><h2>World</h2></nav>");
}

#[renderable(MyCustomBanner)]
fn _my_banner_internal<'a>(text: &'a str) -> impl Renderable {
    maud! {
        div .banner { (text) }
    }
}

#[test]
fn renderable_custom_name() {
    let result = maud! {
        MyCustomBanner text="Welcome!";
    }
    .render();

    assert_eq!(result.as_inner(), r#"<div class="banner">Welcome!</div>"#);
}

#[renderable]
fn static_footer() -> impl Renderable {
    maud! {
        footer {
            p { "Copyright 2025" }
        }
    }
}

#[test]
fn renderable_no_params_maud() {
    let result = maud! {
        StaticFooter;
    }
    .render();

    assert_eq!(result.as_inner(), "<footer><p>Copyright 2025</p></footer>");
}

#[test]
fn renderable_no_params_rsx() {
    let result = rsx! {
        <StaticFooter>
    }
    .render();

    assert_eq!(result.as_inner(), "<footer><p>Copyright 2025</p></footer>");
}

#[renderable]
fn simple_wrapper<R: Renderable>(children: &R) -> impl Renderable {
    maud! {
        div .wrapper {
            (children)
        }
    }
}

#[test]
fn component_with_children_maud() {
    let result = maud! {
        SimpleWrapper {
            p { "Child content" }
            span { "More children" }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div class="wrapper"><p>Child content</p><span>More children</span></div>"#
    );
}

#[test]
fn component_with_children_rsx() {
    let result = rsx! {
        <SimpleWrapper>
            <p>Child content</p>
            <span>More children</span>
        </SimpleWrapper>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div class="wrapper"><p>Child content</p><span>More children</span></div>"#
    );
}

#[renderable]
fn panel<R: Renderable>(heading: &String, children: &R) -> impl Renderable {
    maud! {
        section .panel {
            h3 { (heading) }
            div .panel-body {
                (children)
            }
        }
    }
}

#[test]
fn component_children_and_props_maud() {
    let result = maud! {
        Panel heading=("Settings".into()) {
            p { "Configure your preferences." }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<section class="panel"><h3>Settings</h3><div class="panel-body"><p>Configure your preferences.</p></div></section>"#,
    );
}

#[test]
fn component_children_and_props_rsx() {
    let result = rsx! {
        <Panel heading=("Settings".into())>
            <p>"Configure your preferences."</p>
        </Panel>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<section class="panel"><h3>Settings</h3><div class="panel-body"><p>Configure your preferences.</p></div></section>"#,
    );
}

#[renderable]
fn layout<'a, R: Renderable>(title: &'a str, children: &R) -> impl Renderable {
    maud! {
        html {
            head { title { (title) } }
            body { (children) }
        }
    }
}

#[test]
fn renderable_function_with_children_maud() {
    let result = maud! {
        Layout title="My Page" {
            h1 { "Welcome" }
            p { "Content" }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<html><head><title>My Page</title></head><body><h1>Welcome</h1><p>Content</p></body></html>"
    );
}

#[test]
fn renderable_function_with_children_rsx() {
    let result = rsx! {
        <Layout title="My Page">
            <h1>Welcome</h1>
            <p>Content</p>
        </Layout>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<html><head><title>My Page</title></head><body><h1>Welcome</h1><p>Content</p></body></html>"
    );
}

#[renderable(builder = hypertext::DefaultBuilder)]
#[derive(Default)]
fn default_widget<'a>(label: &'a str, count: u32) -> impl Renderable {
    maud! {
        div {
            span { (label) }
            span { (count) }
        }
    }
}

#[test]
fn default_builder_component_maud() {
    let result = maud! {
        DefaultWidget label="Items" count=42;
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<div><span>Items</span><span>42</span></div>"
    );
}

#[test]
fn default_builder_component_rsx() {
    let result = rsx! {
        <DefaultWidget label="Items" count=42>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<div><span>Items</span><span>42</span></div>"
    );
}

#[test]
fn default_builder_component_defaults() {
    let result = maud! {
        DefaultWidget;
    }
    .render();

    assert_eq!(result.as_inner(), "<div><span></span><span>0</span></div>");
}

#[renderable(builder = false)]
fn manual_widget<'a>(label: &'a str) -> impl Renderable {
    maud! { span { (label) } }
}

impl<'a> ManualWidget<'a> {
    const fn builder() -> Self {
        Self { label: "default" }
    }

    const fn build(self) -> Self {
        self
    }

    const fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }
}

#[test]
fn builder_false_with_manual_builder() {
    let result = maud! {
        ManualWidget label="custom";
    }
    .render();

    assert_eq!(result.as_inner(), "<span>custom</span>");
}

#[test]
fn builder_false_with_manual_default() {
    let result = maud! {
        ManualWidget;
    }
    .render();

    assert_eq!(result.as_inner(), "<span>default</span>");
}

#[derive(Builder, Renderable)]
#[maud(
    li .item { (self.text) }
)]
struct ListItem {
    text: String,
}

#[renderable]
fn list<R: Renderable>(children: &R) -> impl Renderable {
    maud! {
        ul .list {
            (children)
        }
    }
}

#[test]
fn nested_components_maud() {
    let result = maud! {
        List {
            ListItem text=("First".into());
            ListItem text=("Second".into());
            ListItem text=("Third".into());
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<ul class="list"><li class="item">First</li><li class="item">Second</li><li class="item">Third</li></ul>"#
    );
}

#[test]
fn nested_components_rsx() {
    let result = rsx! {
        <List>
            <ListItem text=("First".into())>
            <ListItem text=("Second".into())>
            <ListItem text=("Third".into())>
        </List>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<ul class="list"><li class="item">First</li><li class="item">Second</li><li class="item">Third</li></ul>"#
    );
}

#[derive(Builder, Renderable)]
#[maud(
    div .alert {
        @if self.dismissible {
            button .close { "x" }
        }
        p { (self.message) }
    }
)]
struct Alert {
    message: String,
    dismissible: bool,
}

#[test]
fn component_with_conditional_rendering() {
    let dismissible = maud! {
        Alert message=("Warning!".into()) dismissible=true;
    }
    .render();

    assert_eq!(
        dismissible.as_inner(),
        r#"<div class="alert"><button class="close">x</button><p>Warning!</p></div>"#
    );

    let not_dismissible = maud! {
        Alert message=("Info".into()) dismissible=false;
    }
    .render();

    assert_eq!(
        not_dismissible.as_inner(),
        r#"<div class="alert"><p>Info</p></div>"#
    );
}

#[test]
fn component_used_inline_with_expression() {
    let card = Card {
        title: "Direct".into(),
        body: "Construction".into(),
    };

    let result = maud! { main { (card) } }.render();
    assert_eq!(
        result.as_inner(),
        r#"<main><div class="card"><h2>Direct</h2><p>Construction</p></div></main>"#
    );
}

#[test]
fn multiple_components_in_sequence_maud() {
    let result = maud! {
        Card title=("First".into()) body=("Card 1".into());
        Card title=("Second".into()) body=("Card 2".into());
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div class="card"><h2>First</h2><p>Card 1</p></div><div class="card"><h2>Second</h2><p>Card 2</p></div>"#,
    );
}

#[renderable]
fn typed_item<'a, T: core::fmt::Display>(label: &'a str, value: &T) -> impl Renderable {
    maud! {
        div {
            strong { (label) }
            ": "
            span { (value.to_string()) }
        }
    }
}

#[test]
fn generic_renderable_function_integer() {
    let result = maud! {
        TypedItem label="Count" value=(42_i32);
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<div><strong>Count</strong>: <span>42</span></div>"
    );
}

#[test]
fn generic_renderable_function_float() {
    let result = maud! {
        TypedItem label="Price" value=(9.99_f64);
    }
    .render();

    assert_eq!(
        result.as_inner(),
        "<div><strong>Price</strong>: <span>9.99</span></div>"
    );
}

#[derive(Builder, Renderable)]
#[maud(
    nav {
        @for link in &self.links {
            a href=(link) { (link) }
        }
    }
)]
struct NavLinks {
    links: Vec<String>,
}

#[test]
fn component_with_loop_over_field() {
    let result = maud! {
        NavLinks links=(vec![
            "/home".into(),
            "/about".into(),
            "/contact".into(),
        ]);
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<nav><a href="/home">/home</a><a href="/about">/about</a><a href="/contact">/contact</a></nav>"#,
    );
}
