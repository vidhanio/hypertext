struct div;

impl div {
    #[inline(always)]
    pub const fn id(_: &impl AsRef<str>) {}

    #[inline(always)]
    pub const fn class(_: &impl AsRef<str>) {}
}

/*
html! {
    <div id="foo" class={x}>
        content
    </div>
}
*/

fn main() {
    let x = "foo";

    format!(r#"<div id="foo" class={}>"#, x);
    {
        div::id(&"foo");
        div::class(&x);
    }
}
