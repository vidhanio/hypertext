use crate::views::nav;
use hypertext::{Renderable, html_elements, rsx, rsx_move};

pub fn about(nav_oob: bool) -> impl Renderable {
    rsx_move! {
        { if nav_oob {
            rsx! {
                { nav("/", true) }
            }
        } else {
            rsx! {}
        }}
        <div class="flex flex-col items-center">
            <h1 class="text-4xl font-bold">"About HTMX-RSX"</h1>
            <p class="mt-4">"HTMX-RSX is a simple example of using HTMX with RSX."</p>
            <p class="mt-2">"This project demonstrates how to use HTMX for dynamic content loading in a Rust web application."</p>
        </div>
    }
}
