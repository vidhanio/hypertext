# Hypertext HTMX RSX Example

## Setup

First, install `npm` packages (Tailwind CSS CLI)

```sh
npm i
```

Next, install [air](https://github.com/air-verse/air) for automatic reload (make sure you have [Go installed](https://go.dev/doc/install)):

```sh
go install github.com/air-verse/air@latest
```

Start the server:

```sh
air
```

Open `localhost:3001` in your browser!

## Design

The `views` folder contains any HTML templates.
The `handlers` folder contains any `axum` handlers used for routing.

### Components

With `hypertext` you can use Rust functions as re-usable HTML components!  Simply set the return type to `impl Renderable` and you can
reference that function to call your component.

```rust
use hypertext::prelude::*;

use crate::views::nav;

pub fn about(nav_oob: bool) -> impl Renderable {
    rsx! {
        @if nav_oob {
            { nav("/", true) }
        } 
        <div class="flex flex-col items-center">
            <h1 class="text-4xl font-bold">"About HTMX-RSX"</h1>
            <p class="mt-4">"HTMX-RSX is a simple example of using HTMX with RSX."</p>
            <p class="mt-2">"This project demonstrates how to use HTMX for dynamic content loading in a Rust web application."</p>
        </div>
    }
}
```

You can even pass a component into another component as a parameter!

In this example we are setting a parameter `page` so that any component can be passed into this one.

```rust
pub fn index(selected: &str, page: impl Renderable) -> impl Renderable {
    // ...
}
```
