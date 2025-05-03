use hypertext::{GlobalAttributes, HtmxAttributes, Renderable, html_elements, rsx_move};

pub fn nav(selected: &str, oob: bool) -> impl Renderable {
    let routes = [("Home", "/"), ("About", "/about"), ("List", "/list")];

    let list_base_class = "flex items-center justify-center text-gray-100 w-16";
    let list_selected_class = "bg-gray-800";
    let list_unselected_class = "hover:bg-gray-700";
    let class = move |path: &str| {
        if path == selected {
            format!("{list_base_class} {list_selected_class} p-2")
        } else {
            format!("{list_base_class} {list_unselected_class} p-2")
        }
    };

    rsx_move! {
        <nav id="nav" class="text-gray-100 border-b border-b-gray-700" hx-swap-oob=oob>
            <ul class="flex flex-row items-center justify-center">
                @for (name, path) in routes.iter() {
                  <a href={path} class={class(path)} hx-get={path} hx-target="#page" hx-swap="innerHTML" hx-push-url="true">
                    <li>
                        { name }
                    </li>
                  </a>
                }
            </ul>
        </nav>
    }
}
