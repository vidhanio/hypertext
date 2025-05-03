# `hypertext`

A blazing fast type-checked HTML macro crate.

## Features

- Type checking for element names/attributes
  - Completely extensible for use with non-standard elements/attributes
- `#![no_std]` support
- Automatic escaping
- Lazy rendering by default to avoid multiple allocations
  - Results in outstanding performance in cases of nested documents, which other libraries may falter in

## Projects Using `hypertext`

- [vidhan.io](https://github.com/vidhanio/site), my website

Make a pull request to list your project here!

## Example

```rust
use hypertext::{html_elements, GlobalAttributes, RenderIterator, Renderable};

let shopping_list = ["milk", "eggs", "bread"];

let shopping_list_maud = hypertext::maud! {
    div {
        h1 { "Shopping List" }
        ul {
            @for (&item, i) in shopping_list.iter().zip(1..) {
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

let shopping_list_rsx = hypertext::rsx! {
    <div>
        <h1>Shopping List</h1>
        <ul>
            @for (&item, i) in shopping_list.iter().zip(1..) {
                <li class="item">
                    <input id={ format!("item-{i}") } type="checkbox" />
                    <label for={ format!("item-{i}") }>{ item }</label>
                </li>
            }
        </ul>
    </div>
}
.render();
```
