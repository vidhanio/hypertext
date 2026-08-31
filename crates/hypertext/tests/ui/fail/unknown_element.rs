#![allow(dead_code)]

use hypertext::prelude::*;

fn main() {
    let _ = rsx! { <not_an_element /> }.render();
}
