#![expect(missing_docs)]

mod derive;
mod html;
mod renderable;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use self::html::{Maud, Rsx};
use crate::html::generate::{Config, NodeFlavour, Semantics, XmlFlavour};

fn generate_nodes<S: html::Syntax>(
    config: Config,
    flavour: NodeFlavour,
    tokens: TokenStream,
) -> TokenStream
where
    html::Document<S>: syn::parse::Parse,
{
    config
        .generate_nodes::<S>(flavour, tokens.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn generate_attrs(config: Config, tokens: TokenStream) -> TokenStream {
    config
        .generate_attrs(tokens.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn generate_file<S: html::Syntax>(
    config: Config,
    flavour: NodeFlavour,
    tokens: TokenStream,
) -> TokenStream
where
    html::Document<S>: syn::parse::Parse,
{
    config
        .generate_file::<html::Document<S>>(flavour, tokens.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

macro_rules! create_node_variants {
    {
        syntax = $S:ty;
        flavour = $flavour:expr;
        $($lazy_move:ident $lazy_borrow:ident $simple:ident)*
    } => {
        $(#[proc_macro]
        pub fn $lazy_move(tokens: TokenStream) -> TokenStream {
            generate_nodes::<$S>(Config { lazy: Some(Semantics::Move) }, $flavour, tokens)
        }

        #[proc_macro]
        pub fn $lazy_borrow(tokens: TokenStream) -> TokenStream {
            generate_nodes::<$S>(Config { lazy: Some(Semantics::Borrow) }, $flavour, tokens)
        }

        #[proc_macro]
        pub fn $simple(tokens: TokenStream) -> TokenStream {
            generate_nodes::<$S>(Config { lazy: None }, $flavour, tokens)
        })*
    };
}

macro_rules! create_attr_variants {
    {
        $($lazy_move:ident $lazy_borrow:ident $simple:ident)*
    } => {
        $(#[proc_macro]
        pub fn $lazy_move(tokens: TokenStream) -> TokenStream {
            generate_attrs(Config { lazy: Some(Semantics::Move) }, tokens)
        }

        #[proc_macro]
        pub fn $lazy_borrow(tokens: TokenStream) -> TokenStream {
            generate_attrs(Config { lazy: Some(Semantics::Borrow) }, tokens)
        }

        #[proc_macro]
        pub fn $simple(tokens: TokenStream) -> TokenStream {
            generate_attrs(Config { lazy: None }, tokens)
        })*
    };
}

create_node_variants! {
    syntax = Maud;
    flavour = NodeFlavour::Html;
    maud maud_borrow maud_simple
}

create_node_variants! {
    syntax = Rsx;
    flavour = NodeFlavour::Html;
    rsx rsx_borrow rsx_simple
}

create_attr_variants! {
    attribute attribute_borrow attribute_simple
}

create_node_variants! {
    syntax = Maud;
    flavour = NodeFlavour::Xml(XmlFlavour::Svg);
    svg_maud svg_maud_borrow svg_maud_simple
}

create_node_variants! {
    syntax = Rsx;
    flavour = NodeFlavour::Xml(XmlFlavour::Svg);
    svg_rsx svg_rsx_borrow svg_rsx_simple
}

create_node_variants! {
    syntax = Maud;
    flavour = NodeFlavour::Xml(XmlFlavour::MathMl);
    mathml_maud mathml_maud_borrow mathml_maud_simple
}

create_node_variants! {
    syntax = Rsx;
    flavour = NodeFlavour::Xml(XmlFlavour::MathMl);
    mathml_rsx mathml_rsx_borrow mathml_rsx_simple
}

// File-based macros: load RSX from external files.

#[proc_macro]
pub fn rsx_file(tokens: TokenStream) -> TokenStream {
    generate_file::<Rsx>(
        Config {
            lazy: Some(Semantics::Move),
        },
        NodeFlavour::Html,
        tokens,
    )
}

#[proc_macro]
pub fn rsx_file_borrow(tokens: TokenStream) -> TokenStream {
    generate_file::<Rsx>(
        Config {
            lazy: Some(Semantics::Borrow),
        },
        NodeFlavour::Html,
        tokens,
    )
}

// html! aliases: identical to rsx! variants, avoids Dioxus CLI name collision.
// See https://github.com/vidhanio/hypertext/issues/123.

#[proc_macro]
pub fn html(tokens: TokenStream) -> TokenStream {
    generate_nodes::<Rsx>(
        Config {
            lazy: Some(Semantics::Move),
        },
        NodeFlavour::Html,
        tokens,
    )
}

#[proc_macro]
pub fn html_borrow(tokens: TokenStream) -> TokenStream {
    generate_nodes::<Rsx>(
        Config {
            lazy: Some(Semantics::Borrow),
        },
        NodeFlavour::Html,
        tokens,
    )
}

#[proc_macro]
pub fn html_file(tokens: TokenStream) -> TokenStream {
    generate_file::<Rsx>(
        Config {
            lazy: Some(Semantics::Move),
        },
        NodeFlavour::Html,
        tokens,
    )
}

#[proc_macro]
pub fn html_file_borrow(tokens: TokenStream) -> TokenStream {
    generate_file::<Rsx>(
        Config {
            lazy: Some(Semantics::Borrow),
        },
        NodeFlavour::Html,
        tokens,
    )
}

#[proc_macro_derive(Renderable, attributes(maud, rsx, attribute))]
pub fn derive_renderable(input: TokenStream) -> TokenStream {
    derive::renderable(parse_macro_input!(input))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn renderable(attr: TokenStream, item: TokenStream) -> TokenStream {
    renderable::generate(parse_macro_input!(attr), parse_macro_input!(item))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(DefaultBuilder, attributes(builder))]
pub fn derive_default_builder(input: TokenStream) -> TokenStream {
    derive::default_builder(parse_macro_input!(input))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
