# `hypertext`

A blazing fast type-checked HTML macro crate.

## Features

- Type checking for element names/attributes, including extensible support for custom frameworks like [htmx](https://htmx.org/) and [Alpine.js](https://alpinejs.dev/)
- `#![no_std]` support
- [Extremely fast](https://github.com/askama-rs/template-benchmark#benchmark-results),
  using lazy rendering to minimize allocation
- Integration with all major web frameworks

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
                    <input id={ "item-" (i) } type="checkbox">
                    <label for={ "item-" (i) }>(item)</label>
                </li>
            }
        </ul>
    </div>
}
.render();
```

## Projects Using `hypertext`

- [vidhan.io](https://github.com/vidhanio/site) (my website!)
- [The Brainmade Mark](https://github.com/0atman/BrainMade-org)
- [Lipstick on a pig -- a website for hosting volunteer-built tarballs for KISS Linux](https://github.com/kiedtl/loap)

Make a pull request to list your project here!
