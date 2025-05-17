use crate::views::nav::Nav;
use hypertext::prelude::*;

#[component]
pub fn about(nav_oob: bool) -> impl Renderable {
    rsx! {
        @if nav_oob {
            <Nav selected=(String::from("/about")) oob=true />
        }
        <div class="flex flex-col items-center">
            <h1 class="text-4xl font-bold">"About HTMX-RSX"</h1>
            <p class="mt-4">"HTMX-RSX is a simple example of using HTMX with RSX."</p>
            <p class="mt-2">"This project demonstrates how to use HTMX for dynamic content loading in a Rust web application."</p>
        </div>
    }
}
