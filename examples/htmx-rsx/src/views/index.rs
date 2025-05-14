use hypertext::prelude::*;

use crate::views::nav;

pub fn index(selected: &str, page: impl Renderable) -> impl Renderable {
    rsx! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Hypertext - HTMX with RSX"</title>
                <meta charset="UTF-8">
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1.0"
                >
                <script
                    src="https://unpkg.com/htmx.org@2.0.4"
                    integrity="sha384-HGfztofotfshcF7+8n44JQL2oJmowVChPTg48S+jvZoztPfvwD79OC/LTtG6dMp+"
                    crossorigin="anonymous"
                ></script>
                <link rel="stylesheet" href="/static/output.css">
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
