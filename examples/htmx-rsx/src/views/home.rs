use crate::views::nav::Nav;
use hypertext::prelude::*;

#[component]
pub fn home(nav_oob: bool) -> impl Renderable {
    rsx! {
        @if nav_oob {
            <Nav selected=(String::from("/")) oob=true />
        }
        <div class="flex flex-col items-center">
            <h1 class="text-4xl font-bold">"Welcome to HTMX-RSX"</h1>
            <p class="mt-4">"This is a simple example of using HTMX with RSX."</p>
            <p class="mt-2">"Click the links in the navigation bar to explore."</p>
        </div>
    }
}
