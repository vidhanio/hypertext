#![allow(dead_code)]

use hypertext::prelude::*;

fn main() {
    let _ = rsx! { <div bogus="x"></div> }.render();
}
