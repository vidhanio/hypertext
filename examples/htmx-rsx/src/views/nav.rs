use hypertext::prelude::*;

#[component]
pub fn nav<'a>(selected: &'a str, oob: bool) -> impl Renderable {
    let routes = [("Home", "/"), ("About", "/about"), ("List", "/list")];

    rsx! {
        <nav id="nav" class="text-gray-100 border-b border-b-gray-700" hx-swap-oob=(oob)>
            <ul class="flex flex-row items-center justify-center">
                @for (name, path) in routes {
                    <a
                        href=(path)
                        class={
                            "flex items-center justify-center text-gray-100 w-16 hover:bg-gray-700 p-2"
                            @if path == selected {
                                " bg-gray-800"
                            }
                        }
                        hx-get=(path)
                        hx-target="#page"
                        hx-swap="innerHTML"
                        hx-push-url="true"
                    >
                        <li>
                            (name)
                        </li>
                    </a>
                }
            </ul>
        </nav>
    }
}
