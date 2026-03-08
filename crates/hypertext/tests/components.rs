#![cfg(feature = "alloc")]
#![expect(missing_docs)]

use hypertext::{Buffer, Builder, DefaultBuilder, Lazy, Renderable, prelude::*};

#[test]
fn function_components() {
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
fn struct_component_with_builder() {
    #[derive(Builder)]
    struct Repeater<R> {
        count: usize,
        children: R,
    }

    impl<R: Renderable> Renderable for Repeater<R> {
        fn render_to(&self, buffer: &mut Buffer) {
            buffer.push(maud! {
                @for _ in 0..self.count {
                    (self.children)
                }
            });
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
fn component_without_children() {
    #[derive(Builder)]
    struct Greeting {
        name: String,
    }

    impl Renderable for Greeting {
        fn render_to(&self, buffer: &mut Buffer) {
            buffer.push(maud! {
                span { "Hello, " (self.name) "!" }
            });
        }
    }

    let maud_result = maud! {
        Greeting name=("World".to_owned());
    }
    .render();

    let rsx_result = rsx! {
        <Greeting name=("World".to_owned()) />
    }
    .render();

    for result in [maud_result, rsx_result] {
        assert_eq!(result.as_inner(), "<span>Hello, World!</span>");
    }
}

#[test]
fn renderable_attr_basic() {
    #[renderable(HelloWorld)]
    fn my_component() -> impl Renderable {
        maud! { span { "Hi!" } }
    }

    assert_eq!(
        maud! { HelloWorld; }.render().as_inner(),
        "<span>Hi!</span>"
    );
    assert_eq!(
        rsx! { <HelloWorld /> }.render().as_inner(),
        "<span>Hi!</span>"
    );
}

#[test]
fn renderable_attr_custom_visibility() {
    mod component_module {
        use hypertext::prelude::*;

        #[renderable(pub)]
        fn private_component() -> impl Renderable {
            maud! { span { "secret..." } }
        }
    }

    use component_module::PrivateComponent;

    assert_eq!(
        maud! { PrivateComponent; }.render().as_inner(),
        "<span>secret...</span>"
    );
}

#[test]
fn renderable_attr_combined() {
    #[renderable(HelloWorld)]
    fn my_component() -> impl Renderable {
        maud! { span { "Hi!" } }
    }

    mod component_module {
        use hypertext::prelude::*;

        #[renderable(pub)]
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
fn renderable_attr_with_params() {
    #[renderable]
    fn greeting<'a>(name: &'a str) -> impl Renderable + 'a {
        maud! { span { "Hello, " (name) "!" } }
    }

    let result = maud! {
        Greeting name="World";
    }
    .render();

    assert_eq!(result.as_inner(), "<span>Hello, World!</span>");
}

#[test]
fn derive_renderable_with_maud() {
    #[derive(Builder, Renderable)]
    #[maud(
        div {
            h1 { (self.title) }
            p { (self.body) }
        }
    )]
    struct CardMaud {
        title: String,
        body: String,
    }

    let maud_result = maud! {
        main {
            CardMaud title=("My Title".to_owned()) body=("My Body".to_owned());
        }
    }
    .render();

    let rsx_result = rsx! {
        <main>
            <CardMaud title=("My Title".to_owned()) body=("My Body".to_owned())>
        </main>
    }
    .render();

    let expected = "<main><div><h1>My Title</h1><p>My Body</p></div></main>";
    assert_eq!(maud_result.as_inner(), expected);
    assert_eq!(rsx_result.as_inner(), expected);
}

#[test]
fn derive_renderable_with_rsx() {
    #[derive(Builder, Renderable)]
    #[rsx(
        <div>
            <h1>(self.title)</h1>
            <p>(self.body)</p>
        </div>
    )]
    struct CardRsx {
        title: String,
        body: String,
    }

    let maud_result = maud! {
        main {
            CardRsx title=("My Title".to_owned()) body=("My Body".to_owned());
        }
    }
    .render();

    let rsx_result = rsx! {
        <main>
            <CardRsx title=("My Title".to_owned()) body=("My Body".to_owned())>
        </main>
    }
    .render();

    let expected = "<main><div><h1>My Title</h1><p>My Body</p></div></main>";
    assert_eq!(maud_result.as_inner(), expected);
    assert_eq!(rsx_result.as_inner(), expected);
}

#[test]
fn derive_renderable_with_optional_field() {
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

    let maud_result = maud! {
        Header title=("Hello".to_owned());
        Header title=("Hello".to_owned()) subtitle=("World".to_owned());
    }
    .render();

    let rsx_result = rsx! {
        <Header title=("Hello".to_owned())>
        <Header title=("Hello".to_owned()) subtitle=("World".to_owned())>
    }
    .render();

    let expected = "<div><h1>Hello</h1></div><div><h1>Hello</h1><h2>World</h2></div>";
    assert_eq!(maud_result.as_inner(), expected);
    assert_eq!(rsx_result.as_inner(), expected);
}

#[test]
#[expect(clippy::too_many_lines)]
fn default_builder() {
    #[renderable]
    fn component_a<'a>(
        #[builder(default)] id: &'a str,
        #[builder(default = 1)] tabindex: u32,
        #[builder(default)] children: Lazy<fn(&mut Buffer)>,
    ) -> impl Renderable {
        rsx! {
            <div id=(id) tabindex=(tabindex)>
                (children)
            </div>
        }
    }

    #[renderable(builder = Builder)]
    fn component_b<'a>(
        #[builder(default)] id: &'a str,
        #[builder(default = 1)] tabindex: u32,
        #[builder(default)] children: Lazy<fn(&mut Buffer)>,
    ) -> impl Renderable {
        rsx! {
            <div id=(id) tabindex=(tabindex)>
                (children)
            </div>
        }
    }

    #[renderable(builder = DefaultBuilder)]
    fn component_c<'a>(
        id: &'a str,
        tabindex: u32,
        children: Lazy<fn(&mut Buffer)>,
    ) -> impl Renderable {
        rsx! {
            <div id=(id) tabindex=(tabindex)>
                (children)
            </div>
        }
    }

    impl Default for ComponentC<'_> {
        fn default() -> Self {
            Self {
                id: Default::default(),
                tabindex: 1,
                children: Lazy::default(),
            }
        }
    }

    #[renderable(builder = DefaultBuilder)]
    #[derive(Default)]
    fn component_d<'a>(
        id: &'a str,
        tabindex: u32,
        children: Lazy<fn(&mut Buffer)>,
    ) -> impl Renderable {
        rsx! {
            <div id=(id) tabindex=(tabindex)>
                (children)
            </div>
        }
    }

    let maud_result = maud! {
        ComponentA;
        ComponentB;
        ComponentC;
        ComponentD;
    }
    .render();

    let rsx_result = rsx! {
        <ComponentA />
        <ComponentB />
        <ComponentC />
        <ComponentD />
    }
    .render();

    let component_html = r#"<div id="" tabindex="1"></div>"#;
    let expected_result = format!(
        r#"{}<div id="" tabindex="0"></div>"#,
        component_html.repeat(3)
    );
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ComponentA id="test";
        ComponentB id="test";
        ComponentC id="test";
        ComponentD id="test";
    }
    .render();

    let rsx_result = rsx! {
        <ComponentA id="test" />
        <ComponentB id="test" />
        <ComponentC id="test" />
        <ComponentD id="test" />
    }
    .render();

    let component_html = r#"<div id="test" tabindex="1"></div>"#;
    let expected_result = format!(
        r#"{}<div id="test" tabindex="0"></div>"#,
        component_html.repeat(3)
    );
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ComponentA {
            h1 { "hello" }
        }
        ComponentB {
            h1 { "hello" }
        }
        ComponentC {
            h1 { "hello" }
        }
        ComponentD {
            h1 { "hello" }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ComponentA>
            <h1>hello</h1>
        </ComponentA>
        <ComponentB>
            <h1>hello</h1>
        </ComponentB>
        <ComponentC>
            <h1>hello</h1>
        </ComponentC>
        <ComponentD>
            <h1>hello</h1>
        </ComponentD>
    }
    .render();

    let component_html = r#"<div id="" tabindex="1"><h1>hello</h1></div>"#;
    let expected_result = format!(
        r#"{}<div id="" tabindex="0"><h1>hello</h1></div>"#,
        component_html.repeat(3)
    );
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ComponentA tabindex=2 id="component" {
            h1 { "hello" }
        }
        ComponentB tabindex=2 id="component" {
            h1 { "hello" }
        }
        ComponentC tabindex=2 id="component" {
            h1 { "hello" }
        }
        ComponentD tabindex=2 id="component" {
            h1 { "hello" }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ComponentA tabindex=2 id="component">
            <h1>hello</h1>
        </ComponentA>
        <ComponentB tabindex=2 id="component">
            <h1>hello</h1>
        </ComponentB>
        <ComponentC tabindex=2 id="component">
            <h1>hello</h1>
        </ComponentC>
        <ComponentD tabindex=2 id="component">
            <h1>hello</h1>
        </ComponentD>
    }
    .render();

    let component_html = r#"<div id="component" tabindex="2"><h1>hello</h1></div>"#;
    let expected_result = component_html.repeat(4);
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ComponentA {
            ComponentA id="nested" {
                h1 { "hello" }
            }
        }
        ComponentB {
            ComponentB id="nested" {
                h1 { "hello" }
            }
        }
        ComponentC {
            ComponentC id="nested" {
                h1 { "hello" }
            }
        }
        ComponentD {
            ComponentD id="nested" {
                h1 { "hello" }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ComponentA>
            <ComponentA id="nested">
                <h1>"hello"</h1>
            </ComponentA>
        </ComponentA>
        <ComponentB>
            <ComponentB id="nested">
                <h1>"hello"</h1>
            </ComponentB>
        </ComponentB>
        <ComponentC>
            <ComponentC id="nested">
                <h1>"hello"</h1>
            </ComponentC>
        </ComponentC>
        <ComponentD>
            <ComponentD id="nested">
                <h1>"hello"</h1>
            </ComponentD>
        </ComponentD>
    }
    .render();

    let component_html =
        r#"<div id="" tabindex="1"><div id="nested" tabindex="1"><h1>hello</h1></div></div>"#;
    let expected_result = format!(
        r#"{}<div id="" tabindex="0"><div id="nested" tabindex="0"><h1>hello</h1></div></div>"#,
        component_html.repeat(3)
    );
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);
}

#[test]
#[expect(clippy::too_many_lines)]
fn custom_builder() {
    #[renderable(builder = false)]
    fn component_a<'a>(
        id: &'a str,
        tabindex: u32,
        children: Lazy<fn(&mut Buffer)>,
    ) -> impl Renderable {
        rsx! {
            <div id=(id) tabindex=(tabindex)>
                (children)
            </div>
        }
    }

    impl<'a> ComponentA<'a> {
        fn builder() -> Self {
            Self {
                id: "custom",
                tabindex: 2,
                children: Lazy::default(),
            }
        }

        fn id(self, id: &'a str) -> Self {
            Self { id, ..self }
        }

        fn tabindex(self, tabindex: u32) -> Self {
            Self { tabindex, ..self }
        }

        fn children(self, children: Lazy<fn(&mut Buffer)>) -> Self {
            Self { children, ..self }
        }

        fn build(self) -> Self {
            self
        }
    }

    #[derive(Builder)]
    struct ComponentB<'a> {
        #[builder(default = "custom")]
        id: &'a str,

        #[builder(default = 2)]
        tabindex: u32,

        #[builder(default)]
        children: Lazy<fn(&mut Buffer)>,
    }

    impl Renderable for ComponentB<'_> {
        fn render_to(&self, buf: &mut Buffer) {
            rsx! {
                <div id=(self.id) tabindex=(self.tabindex)>
                    (self.children)
                </div>
            }
            .render_to(buf);
        }
    }

    let maud_result = maud! {
        ComponentA;
        ComponentB;
    }
    .render();

    let rsx_result = rsx! {
        <ComponentA />
        <ComponentB />
    }
    .render();

    let component_html = r#"<div id="custom" tabindex="2"></div>"#;
    let expected_result = component_html.repeat(2);
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ComponentA id="test";
        ComponentB id="test";
    }
    .render();

    let rsx_result = rsx! {
        <ComponentA id="test" />
        <ComponentB id="test" />
    }
    .render();

    let component_html = r#"<div id="test" tabindex="2"></div>"#;
    let expected_result = component_html.repeat(2);
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ComponentA {
            h1 { "hello" }
        }
        ComponentB {
            h1 { "hello" }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ComponentA>
            <h1>hello</h1>
        </ComponentA>
        <ComponentB>
            <h1>hello</h1>
        </ComponentB>
    }
    .render();

    let component_html = r#"<div id="custom" tabindex="2"><h1>hello</h1></div>"#;
    let expected_result = component_html.repeat(2);
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ComponentA tabindex=1 id="component" {
            h1 { "hello" }
        }
        ComponentB tabindex=1 id="component" {
            h1 { "hello" }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ComponentA tabindex=1 id="component">
            <h1>hello</h1>
        </ComponentA>
        <ComponentB tabindex=1 id="component">
            <h1>hello</h1>
        </ComponentB>
    }
    .render();

    let component_html = r#"<div id="component" tabindex="1"><h1>hello</h1></div>"#;
    let expected_result = component_html.repeat(2);
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ComponentA {
            ComponentA id="nested" {
                h1 { "hello" }
            }
        }
        ComponentB {
            ComponentB id="nested" {
                h1 { "hello" }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ComponentA>
            <ComponentA id="nested">
                <h1>"hello"</h1>
            </ComponentA>
        </ComponentA>
        <ComponentB>
            <ComponentB id="nested">
                <h1>"hello"</h1>
            </ComponentB>
        </ComponentB>
    }
    .render();

    let component_html =
        r#"<div id="custom" tabindex="2"><div id="nested" tabindex="2"><h1>hello</h1></div></div>"#;
    let expected_result = component_html.repeat(2);
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);
}
