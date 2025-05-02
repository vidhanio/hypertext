use crate::views::nav;
use hypertext::{RenderIterator, Renderable, html_elements, rsx, rsx_move};

pub fn list(nav_oob: bool) -> impl Renderable {
    let list_items = vec!["Hypertext", "is", "fun!"];
    rsx_move! {
        { if nav_oob {
            rsx! {
                { nav("/", true) }
            }
        } else {
            rsx! {}
        }}
        <div class="flex flex-col items-center">
            <h1 class="text-4xl font-bold">"Loop through items using Rust code!"</h1>
            <ul class="mt-4 list-disc">
                { list_items.into_iter().map(|item| rsx_move! {
                    <li class="mt-2">{ item }</li>
                }).render_all()}
            </ul>
        </div>
    }
}
