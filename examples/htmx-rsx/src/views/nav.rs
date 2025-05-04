use hypertext::{
    GlobalAttributes, Renderable, frameworks::HtmxAttributes, html_elements, rsx_move,
};

pub fn nav(selected: &str, oob: bool) -> impl Renderable {
    let routes = [("Home", "/"), ("About", "/about"), ("List", "/list")];

    let unselected_class =
        "flex items-center justify-center text-gray-100 w-16 hover:bg-gray-700 p-2";
    let selected_class = format!("{unselected_class} bg-gray-800");

    let class = move |path| {
        if path == selected {
            selected_class.clone()
        } else {
            unselected_class.to_owned()
        }
    };

    rsx_move! {
        <nav id="nav" class="text-gray-100 border-b border-b-gray-700" hx-swap-oob=oob>
            <ul class="flex flex-row items-center justify-center">
                @for (name, path) in routes {
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
