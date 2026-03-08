#![expect(missing_docs)]

mod derive;
mod html;
mod renderable;

use html::{AttributeValue, Many};
use proc_macro::TokenStream;
use syn::{parse::Parse, parse_macro_input};

use self::html::{Document, Maud, Rsx};
use crate::html::generate::{Config, Generate, NodeFlavour, Semantics, XmlFlavour};

fn generate<T: Parse + Generate>(config: Config, tokens: TokenStream) -> TokenStream {
    config
        .generate::<T>(tokens.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

macro_rules! create_variants {
    {
        $($Ty:ty {
            $lazy_move:ident
            $lazy_borrow:ident
            $simple:ident
        })*
    } => {
        $(#[proc_macro]
        pub fn $lazy_move(tokens: TokenStream) -> TokenStream {
            generate::<$Ty>(Config { lazy: Some(Semantics::Move), flavour: NodeFlavour::Html }, tokens)
        }

        #[proc_macro]
        pub fn $lazy_borrow(tokens: TokenStream) -> TokenStream {
            generate::<$Ty>(Config { lazy: Some(Semantics::Borrow), flavour: NodeFlavour::Html }, tokens)
        }

        #[proc_macro]
        pub fn $simple(tokens: TokenStream) -> TokenStream {
            generate::<$Ty>(Config { lazy: None, flavour: NodeFlavour::Html }, tokens)
        })*
    };
}

macro_rules! create_xml_variants {
    {
        flavour = $flavour:expr;
        $($Ty:ty {
            $lazy_move:ident
            $lazy_borrow:ident
            $simple:ident
        })*
    } => {
        $(#[proc_macro]
        pub fn $lazy_move(tokens: TokenStream) -> TokenStream {
            generate::<$Ty>(Config { lazy: Some(Semantics::Move), flavour: NodeFlavour::Xml($flavour) }, tokens)
        }

        #[proc_macro]
        pub fn $lazy_borrow(tokens: TokenStream) -> TokenStream {
            generate::<$Ty>(Config { lazy: Some(Semantics::Borrow), flavour: NodeFlavour::Xml($flavour) }, tokens)
        }

        #[proc_macro]
        pub fn $simple(tokens: TokenStream) -> TokenStream {
            generate::<$Ty>(Config { lazy: None, flavour: NodeFlavour::Xml($flavour) }, tokens)
        })*
    };
}

create_variants! {
    Document<Maud> {
        maud
        maud_borrow
        maud_simple
    }

    Document<Rsx> {
        rsx
        rsx_borrow
        rsx_simple
    }

    Many<AttributeValue> {
        attribute
        attribute_borrow
        attribute_simple
    }
}

create_xml_variants! {
    flavour = XmlFlavour::Svg;

    Document<Maud> {
        svg_maud
        svg_maud_borrow
        svg_maud_simple
    }

    Document<Rsx> {
        svg_rsx
        svg_rsx_borrow
        svg_rsx_simple
    }
}

create_xml_variants! {
    flavour = XmlFlavour::MathMl;

    Document<Maud> {
        mathml_maud
        mathml_maud_borrow
        mathml_maud_simple
    }

    Document<Rsx> {
        mathml_rsx
        mathml_rsx_borrow
        mathml_rsx_simple
    }
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
