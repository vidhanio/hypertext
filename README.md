# `hypertext`

A blazing fast type-checked HTML macro crate.

## Features

- Type checking for element names/attributes
  - Completely extensible for use with non-standard elements/attributes, such as
    those used by [htmx](https://htmx.org/) and [Alpine.js](https://alpinejs.dev/)
- `#![no_std]` support
- Automatic escaping
- [Extremely fast](https://github.com/askama-rs/template-benchmark#benchmark-results),
  using lazy rendering by default to avoid unnecessary allocations
- Support for two well-known HTML macro syntaxes, `maud` and `rsx`
- `#![forbid(unsafe_code)]` across the entire codebase
- Integration with all major web frameworks, enabled by their respective feature flags
  - [`actix-web`](https://actix.rs/)
  - [`axum`](https://github.com/tokio-rs/axum)
  - [`poem`](https://github.com/poem-web/poem)
  - [`rocket`](https://rocket.rs/)
  - [`salvo`](https://github.com/salvo-rs/salvo)
  - [`tide`](https://github.com/http-rs/tide)
  - [`warp`](https://github.com/seanmonstar/warp)

## Projects Using `hypertext`

- [vidhan.io](https://github.com/vidhanio/site) (my website!)

Make a pull request to list your project here!

## Example

```rust
use hypertext::prelude::*;

let shopping_list = ["milk", "eggs", "bread"];

let shopping_list_maud = maud! {
    div {
        h1 { "Shopping List" }
        ul {
            @for (i, item) in (1..).zip(shopping_list) {
                li.item {
                    input #{ "item-" (i) } type="checkbox";
                    label for={ "item-" (i) } { (item) }
                }
            }
        }
    }
}
.render();

// or, alternatively:

let shopping_list_rsx = rsx! {
    <div>
        <h1>Shopping List</h1>
        <ul>
            @for (i, item) in (1..).zip(shopping_list) {
                <li class="item">
                    <input id={ format!("item-{i}") } type="checkbox">
                    <label for={ format!("item-{i}") }>{ item }</label>
                </li>
            }
        </ul>
    </div>
}
.render();
```
