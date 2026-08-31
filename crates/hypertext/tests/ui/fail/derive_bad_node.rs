#![allow(dead_code)]

use hypertext::prelude::*;

#[derive(Renderable)]
#[renderable(node = wasm)]
#[maud(div {})]
struct BadNode;

fn main() {}
