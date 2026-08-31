#![allow(dead_code)]

use hypertext::{Builder, prelude::*};

#[derive(Builder, Renderable)]
#[maud(article .card { h1 { (self.title) } (self.children) })]
struct Card<R: Renderable> {
    title: String,
    children: R,
}

fn main() {
    let children = maud::simple! { p { "body" } };

    let _ = maud! {
        Card title=("Hello".to_owned()) children=(children);
    }
    .render();
    let _ = rsx! {
        <Card title=("Hello".to_owned()) children=(children)>
    }
    .render();
    let _: hypertext::Raw<&str> = maud::simple! { main { "static" } };
    let _: hypertext::Raw<&str> = rsx::simple! { <main>static</main> };
    let _: hypertext::RawAttribute<&str> = attribute::simple! { "value" };
    let _ = svg::rsx! { <svg><circle cx="1" cy="2" r="1" /></svg> }.render();
    let _ = mathml::maud! { math { mi { "x" } } }.render();
}
