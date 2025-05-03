use hypertext::{GlobalAttributes, Renderable, html_elements, rsx_move};

use crate::views::nav;

pub fn index(selected: &str, page: &impl Renderable) -> impl Renderable {
    rsx_move! {
        <!doctype html>
        <html>
            <head>
                <title>"Hypertext - HTMX with RSX"</title>
                <meta charset="UTF-8">
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1.0"
                />
                <link rel="stylesheet" href="/static/output.css" />
                <script src="https://unpkg.com/htmx.org@2.0.4"></script>
            </head>
            <body class="bg-gray-900 text-gray-100">
                <h1 class="flex text-5xl mx-auto font-bold justify-center items-center mb-2">Hypertext</h1>
                { nav(selected, false) }
                <div id="page" class="mt-2">
                    { &page }
                </div>
            </body>
        </html>
    }
}
