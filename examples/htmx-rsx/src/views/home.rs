use crate::views::nav;
use hypertext::{GlobalAttributes, Renderable, html_elements, rsx_move};

pub fn home(nav_oob: bool) -> impl Renderable {
    rsx_move! {
        @if nav_oob {
            { nav("/", true) }
        }
        <div class="flex flex-col items-center">
            <h1 class="text-4xl font-bold">"Welcome to HTMX-RSX"</h1>
            <p class="mt-4">"This is a simple example of using HTMX with RSX."</p>
            <p class="mt-2">"Click the links in the navigation bar to explore."</p>
        </div>
    }
}
