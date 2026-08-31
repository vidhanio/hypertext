//! Component and derive-macro coverage.
#![cfg(feature = "alloc")]

use hypertext::{Builder, DefaultBuilder, prelude::*};

macro_rules! assert_html_pair {
    ($maud:expr, $rsx:expr, $expected:expr $(,)?) => {{
        let maud = ($maud).render();
        let rsx = ($rsx).render();
        assert_eq!(maud.as_inner(), $expected);
        assert_eq!(rsx.as_inner(), $expected);
    }};
}

macro_rules! assert_context_pair {
    ($kind:ty, $maud:expr, $rsx:expr, $expected:expr $(,)?) => {{
        let maud = ($maud).render::<$kind>();
        let rsx = ($rsx).render::<$kind>();
        assert_eq!(maud.as_inner(), $expected);
        assert_eq!(rsx.as_inner(), $expected);
    }};
}

#[derive(Builder, Renderable)]
#[maud(
    article .card {
        h1 { (self.title) }
        p { (self.body) }
    }
)]
struct MaudCard {
    title: String,
    body: String,
}

#[derive(Builder, Renderable)]
#[rsx(
    <article class="card">
        <h1>(self.title)</h1>
        <p>(self.body)</p>
    </article>
)]
struct RsxCard {
    title: String,
    body: String,
}

#[derive(Builder, Renderable)]
#[maud(
    header {
        h1 { (self.title) }
        @if let Some(subtitle) = &self.subtitle {
            p .subtitle { (subtitle) }
        }
    }
)]
struct Header {
    title: String,
    subtitle: Option<String>,
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

#[derive(Builder, Renderable)]
#[maud(span .badge { (self.label) })]
#[attribute((self.label))]
struct Badge {
    label: String,
}

#[derive(Renderable)]
#[renderable(node = svg)]
#[maud(circle cx=(self.cx) cy=(self.cy) r=(self.radius);)]
struct SvgCircle {
    cx: u32,
    cy: u32,
    radius: u32,
}

#[derive(Renderable)]
#[renderable(node = mathml)]
#[rsx(<mi>(self.value)</mi>)]
struct MathIdentifier {
    value: String,
}

#[renderable]
fn panel<'a, R: Renderable>(title: &'a str, expanded: bool, children: &R) -> impl Renderable {
    maud! {
        section .panel {
            h2 { (title) }
            @if expanded {
                (children)
            }
        }
    }
}

#[renderable]
fn wrapper<R: Renderable>(children: &R) -> impl Renderable {
    maud! { div .wrapper { (children) } }
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

#[renderable]
fn static_footer() -> impl Renderable {
    maud! { footer { "done" } }
}

#[renderable(MyBanner)]
fn banner<'a>(text: &'a str) -> impl Renderable {
    maud! { aside .banner { (text) } }
}

#[renderable(builder = DefaultBuilder)]
#[derive(Default)]
fn counter<'a>(label: &'a str, count: u32) -> impl Renderable {
    maud! { output { (label) ":" (count) } }
}

#[renderable(builder = false)]
fn manual_widget<'a>(label: &'a str) -> impl Renderable {
    maud! { span .manual { (label) } }
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

#[renderable]
fn typed<'a, T: core::fmt::Display>(label: &'a str, value: &T) -> impl Renderable {
    maud! { p { (label) ": " (value.to_string()) } }
}

#[derive(Default, DefaultBuilder)]
struct Defaults {
    label: String,
    #[builder(skip)]
    hidden: u8,
}

#[test]
fn derived_components_render_in_both_syntaxes() {
    assert_html_pair!(
        maud! {
            main {
                MaudCard title=("Title".into()) body=("Body".into());
            }
        },
        rsx! {
            <main>
                <RsxCard title=("Title".into()) body=("Body".into())>
            </main>
        },
        r#"<main><article class="card"><h1>Title</h1><p>Body</p></article></main>"#,
    );

    let card = MaudCard {
        title: "Direct".into(),
        body: "value".into(),
    };
    assert_eq!(
        maud! { (card) }.render().as_inner(),
        "<article class=\"card\"><h1>Direct</h1><p>value</p></article>",
    );
}

#[test]
fn derive_builder_handles_optional_fields_and_attribute_context() {
    let rendered = maud! {
        Header title=("Hello".into());
        Header title=("Hello".into()) subtitle=("World".into());
        Badge label=("new".into());
    }
    .render();
    assert_eq!(
        rendered.as_inner(),
        "<header><h1>Hello</h1></header><header><h1>Hello</h1><p class=\"subtitle\">World</p></header><span class=\"badge\">new</span>",
    );

    assert_eq!(
        rsx! { <div title=(Badge { label: "tip".into() })>ok</div> }
            .render()
            .as_inner(),
        r#"<div title="tip">ok</div>"#,
    );
}

#[test]
fn derived_components_can_render_borrowed_collection_fields() {
    assert_eq!(
        maud! {
            NavLinks links=(vec![String::from("/home"), String::from("/about")]);
        }
        .render()
        .as_inner(),
        r#"<nav><a href="/home">/home</a><a href="/about">/about</a></nav>"#,
    );

    assert_eq!(
        rsx! {
            <NavLinks links=(vec![String::from("/one"), String::from("/two")])>
        }
        .render()
        .as_inner(),
        r#"<nav><a href="/one">/one</a><a href="/two">/two</a></nav>"#,
    );
}

#[test]
fn derived_contexts_are_restricted_to_svg_and_mathml() {
    assert_context_pair!(
        hypertext::context::Svg,
        svg::maud! {
            svg viewBox="0 0 10 10" {
                (SvgCircle { cx: 5, cy: 5, radius: 4 })
            }
        },
        svg::rsx! {
            <svg viewBox="0 0 10 10">
                (SvgCircle { cx: 5, cy: 5, radius: 4 })
            </svg>
        },
        r#"<svg viewBox="0 0 10 10"><circle cx="5" cy="5" r="4"/></svg>"#,
    );

    assert_eq!(
        mathml::rsx! {
            <math>(MathIdentifier { value: "x".into() })</math>
        }
        .render::<hypertext::context::MathMl>()
        .as_inner(),
        "<math><mi>x</mi></math>",
    );
}

#[test]
fn renderable_functions_cover_props_children_and_branches() {
    assert_html_pair!(
        maud! {
            Panel title="Open" expanded=true { p { "content" } }
            Panel title="Closed" expanded=false { p { "hidden" } }
        },
        rsx! {
            <Panel title="Open" expanded=true><p>content</p></Panel>
            <Panel title="Closed" expanded=false><p>hidden</p></Panel>
        },
        r#"<section class="panel"><h2>Open</h2><p>content</p></section><section class="panel"><h2>Closed</h2></section>"#,
    );

    assert_html_pair!(
        maud! { StaticFooter; },
        rsx! { <StaticFooter> },
        "<footer>done</footer>",
    );

    assert_html_pair!(
        maud! {
            Wrapper {
                p { "first" }
                span { "second" }
            }
        },
        rsx! {
            <Wrapper>
                <p>first</p>
                <span>second</span>
            </Wrapper>
        },
        r#"<div class="wrapper"><p>first</p><span>second</span></div>"#,
    );

    assert_html_pair!(
        maud! {
            Layout title="Page" {
                h1 { "Welcome" }
                p { "Body" }
            }
        },
        rsx! {
            <Layout title="Page">
                <h1>Welcome</h1>
                <p>Body</p>
            </Layout>
        },
        "<html><head><title>Page</title></head><body><h1>Welcome</h1><p>Body</p></body></html>",
    );
}

#[test]
fn renderable_function_custom_name_and_generic_props() {
    assert_eq!(
        maud! { MyBanner text="welcome"; }.render().as_inner(),
        r#"<aside class="banner">welcome</aside>"#,
    );

    assert_eq!(
        rsx! { <Typed label="Count" value=42> }.render().as_inner(),
        "<p>Count: 42</p>",
    );

    assert_eq!(
        maud! { Typed label="Price" value=9.99_f64; }
            .render()
            .as_inner(),
        "<p>Price: 9.99</p>",
    );
}

#[test]
fn renderable_builder_options_preserve_defaults_and_manual_builders() {
    assert_eq!(
        maud! {
            Counter;
            Counter label="Items" count=3;
        }
        .render()
        .as_inner(),
        "<output>:0</output><output>Items:3</output>",
    );

    assert_eq!(
        maud! { ManualWidget; ManualWidget label="custom"; }
            .render()
            .as_inner(),
        r#"<span class="manual">default</span><span class="manual">custom</span>"#,
    );

    assert_eq!(
        rsx! { <Counter><Counter label="Items" count=3> }
            .render()
            .as_inner(),
        "<output>:0</output><output>Items:3</output>",
    );
    assert_eq!(
        rsx! { <ManualWidget><ManualWidget label="custom"> }
            .render()
            .as_inner(),
        r#"<span class="manual">default</span><span class="manual">custom</span>"#,
    );
}

#[test]
fn default_builder_generates_only_non_skipped_setters() {
    let defaults = Defaults::builder().label("set".into()).build();
    assert_eq!(defaults.label, "set");
    assert_eq!(defaults.hidden, 0);
}
