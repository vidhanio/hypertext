//! Tests for the `hypertext` crate.
#![cfg(feature = "alloc")]

use hypertext::{Buffer, DefaultBuilder, Lazy, TypedBuilder, prelude::*};

#[test]
fn default() {
    #[component]
    fn element_a<'a>(
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

    #[component(builder = TypedBuilder, attrs = [builder])]
    fn element_b<'a>(
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

    #[component(builder = DefaultBuilder)]
    fn element_c<'a>(
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

    impl<'a> Default for ElementC<'a> {
        fn default() -> Self {
            Self {
                id: Default::default(),
                tabindex: 1,
                children: Default::default(),
            }
        }
    }

    #[component(builder = DefaultBuilder)]
    #[derive(Default)]
    fn element_d<'a>(
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
        ElementA;
        ElementB;
        ElementC;
        ElementD;
    }
    .render();

    let rsx_result = rsx! {
        <ElementA />
        <ElementB />
        <ElementC />
        <ElementD />
    }
    .render();

    let element_html = r#"<div id="" tabindex="1"></div>"#;
    let expected_result = format!(
        r#"{}<div id="" tabindex="0"></div>"#,
        element_html.repeat(3)
    );
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ElementA id="test";
        ElementB id="test";
        ElementC id="test";
        ElementD id="test";
    }
    .render();

    let rsx_result = rsx! {
        <ElementA id="test" />
        <ElementB id="test" />
        <ElementC id="test" />
        <ElementD id="test" />
    }
    .render();

    let element_html = r#"<div id="test" tabindex="1"></div>"#;
    let expected_result = format!(
        r#"{}<div id="test" tabindex="0"></div>"#,
        element_html.repeat(3)
    );
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ElementA {
            h1 { "hello" }
        }
        ElementB {
            h1 { "hello" }
        }
        ElementC {
            h1 { "hello" }
        }
        ElementD {
            h1 { "hello" }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ElementA>
            <h1>hello</h1>
        </ElementA>
        <ElementB>
            <h1>hello</h1>
        </ElementB>
        <ElementC>
            <h1>hello</h1>
        </ElementC>
        <ElementD>
            <h1>hello</h1>
        </ElementD>
    }
    .render();

    let element_html = r#"<div id="" tabindex="1"><h1>hello</h1></div>"#;
    let expected_result = format!(
        r#"{}<div id="" tabindex="0"><h1>hello</h1></div>"#,
        element_html.repeat(3)
    );
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ElementA tabindex=2 id="element" {
            h1 { "hello" }
        }
        ElementB tabindex=2 id="element" {
            h1 { "hello" }
        }
        ElementC tabindex=2 id="element" {
            h1 { "hello" }
        }
        ElementD tabindex=2 id="element" {
            h1 { "hello" }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ElementA tabindex=2 id="element">
            <h1>hello</h1>
        </ElementA>
        <ElementB tabindex=2 id="element">
            <h1>hello</h1>
        </ElementB>
        <ElementC tabindex=2 id="element">
            <h1>hello</h1>
        </ElementC>
        <ElementD tabindex=2 id="element">
            <h1>hello</h1>
        </ElementD>
    }
    .render();

    let element_html = r#"<div id="element" tabindex="2"><h1>hello</h1></div>"#;
    let expected_result = element_html.repeat(4);
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ElementA {
            ElementA id="nested" {
                h1 { "hello" }
            }
        }
        ElementB {
            ElementB id="nested" {
                h1 { "hello" }
            }
        }
        ElementC {
            ElementC id="nested" {
                h1 { "hello" }
            }
        }
        ElementD {
            ElementD id="nested" {
                h1 { "hello" }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ElementA>
            <ElementA id="nested">
                <h1>"hello"</h1>
            </ElementA>
        </ElementA>
        <ElementB>
            <ElementB id="nested">
                <h1>"hello"</h1>
            </ElementB>
        </ElementB>
        <ElementC>
            <ElementC id="nested">
                <h1>"hello"</h1>
            </ElementC>
        </ElementC>
        <ElementD>
            <ElementD id="nested">
                <h1>"hello"</h1>
            </ElementD>
        </ElementD>
    }
    .render();

    let element_html =
        r#"<div id="" tabindex="1"><div id="nested" tabindex="1"><h1>hello</h1></div></div>"#;
    let expected_result = format!(
        r#"{}<div id="" tabindex="0"><div id="nested" tabindex="0"><h1>hello</h1></div></div>"#,
        element_html.repeat(3)
    );
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);
}

#[test]
fn custom() {
    #[component(builder = false)]
    fn element_a<'a>(
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

    impl<'a> ElementA<'a> {
        fn builder() -> Self {
            Self {
                id: "custom",
                tabindex: 2,
                children: Default::default(),
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

    #[derive(TypedBuilder)]
    struct ElementB<'a> {
        #[builder(default = "custom")]
        id: &'a str,

        #[builder(default = 2)]
        tabindex: u32,

        #[builder(default)]
        children: Lazy<fn(&mut Buffer)>,
    }

    impl<'a> Renderable for ElementB<'a> {
        fn render_to(&self, buf: &mut Buffer) {
            rsx! {
                <div id=(self.id) tabindex=(self.tabindex)>
                    (self.children)
                </div>
            }
            .render_to(buf)
        }
    }

    let maud_result = maud! {
        ElementA;
        ElementB;
    }
    .render();

    let rsx_result = rsx! {
        <ElementA />
        <ElementB />
    }
    .render();

    let element_html = r#"<div id="custom" tabindex="2"></div>"#;
    let expected_result = element_html.repeat(2);
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ElementA id="test";
        ElementB id="test";
    }
    .render();

    let rsx_result = rsx! {
        <ElementA id="test" />
        <ElementB id="test" />
    }
    .render();

    let element_html = r#"<div id="test" tabindex="2"></div>"#;
    let expected_result = element_html.repeat(2);
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ElementA {
            h1 { "hello" }
        }
        ElementB {
            h1 { "hello" }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ElementA>
            <h1>hello</h1>
        </ElementA>
        <ElementB>
            <h1>hello</h1>
        </ElementB>
    }
    .render();

    let element_html = r#"<div id="custom" tabindex="2"><h1>hello</h1></div>"#;
    let expected_result = element_html.repeat(2);
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ElementA tabindex=1 id="element" {
            h1 { "hello" }
        }
        ElementB tabindex=1 id="element" {
            h1 { "hello" }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ElementA tabindex=1 id="element">
            <h1>hello</h1>
        </ElementA>
        <ElementB tabindex=1 id="element">
            <h1>hello</h1>
        </ElementB>
    }
    .render();

    let element_html = r#"<div id="element" tabindex="1"><h1>hello</h1></div>"#;
    let expected_result = element_html.repeat(2);
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);

    let maud_result = maud! {
        ElementA {
            ElementA id="nested" {
                h1 { "hello" }
            }
        }
        ElementB {
            ElementB id="nested" {
                h1 { "hello" }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        <ElementA>
            <ElementA id="nested">
                <h1>"hello"</h1>
            </ElementA>
        </ElementA>
        <ElementB>
            <ElementB id="nested">
                <h1>"hello"</h1>
            </ElementB>
        </ElementB>
    }
    .render();

    let element_html =
        r#"<div id="custom" tabindex="2"><div id="nested" tabindex="2"><h1>hello</h1></div></div>"#;
    let expected_result = element_html.repeat(2);
    assert_eq!(maud_result.as_inner(), &expected_result);
    assert_eq!(rsx_result.as_inner(), &expected_result);
}
