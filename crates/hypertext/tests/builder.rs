//! Tests for the `hypertext` crate.
#![cfg(feature = "alloc")]

use hypertext::{Buffer, Builder, DefaultBuilder, Lazy, prelude::*, renderable};

#[test]
#[allow(clippy::too_many_lines)]
fn default() {
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

    #[renderable(builder = Builder, attrs = [builder])]
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
#[allow(clippy::too_many_lines)]
fn custom() {
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
