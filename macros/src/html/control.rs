use std::convert::Infallible;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Expr, Pat, PatType, Token, braced,
    parse::{Parse, ParseStream},
    token::Brace,
};

use super::{AnyBlock, Context, Generate, Generator, Many};

pub enum Control<C: Context> {
    Let(Let),
    If(If<C>),
    For(For<C>),
    While(While<C>),
    Match(Match<C>),
}

impl<C: Context + Parse> Parse for Control<C> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![@]>()?;

        let lookahead = input.lookahead1();

        if lookahead.peek(Token![let]) {
            input.parse().map(Self::Let)
        } else if lookahead.peek(Token![if]) {
            input.parse().map(Self::If)
        } else if lookahead.peek(Token![for]) {
            input.parse().map(Self::For)
        } else if lookahead.peek(Token![while]) {
            input.parse().map(Self::While)
        } else if lookahead.peek(Token![match]) {
            input.parse().map(Self::Match)
        } else {
            Err(lookahead.error())
        }
    }
}

impl<C: Context> Generate for Control<C> {
    type Context = C;

    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Let(let_) => g.push(let_),
            Self::If(if_) => g.push(if_),
            Self::For(for_) => g.push(for_),
            Self::While(while_) => g.push(while_),
            Self::Match(match_) => g.push(match_),
        }
    }
}

pub struct Let {
    let_token: Token![let],
    pat: Pat,
    init: Option<(Token![=], Expr)>,
    semi_token: Token![;],
}

impl Parse for Let {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            let_token: input.parse()?,
            pat: {
                let pat = input.call(Pat::parse_single)?;
                if input.peek(Token![:]) {
                    Pat::Type(PatType {
                        attrs: Vec::new(),
                        pat: Box::new(pat),
                        colon_token: input.parse()?,
                        ty: input.parse()?,
                    })
                } else {
                    pat
                }
            },
            init: if input.peek(Token![=]) {
                Some((input.parse()?, input.parse()?))
            } else {
                None
            },
            semi_token: input.parse()?,
        })
    }
}

impl Generate for Let {
    type Context = Infallible;

    fn generate(&self, g: &mut Generator) {
        let let_token = self.let_token;
        let pat = &self.pat;
        let (eq_token, expr) = self
            .init
            .as_ref()
            .map(|(eq_token, expr)| (eq_token, expr))
            .unzip();
        let semi_token = self.semi_token;

        g.push_stmt(quote! {
            #let_token #pat #eq_token #expr #semi_token
        });
    }
}

pub struct ControlBlock<C: Context> {
    brace_token: Brace,
    children: Many<C>,
}

impl<C: Context> ControlBlock<C> {
    fn block(&self, g: &mut Generator) -> AnyBlock {
        self.children.block(g, self.brace_token)
    }
}

impl<C: Context + Parse> Parse for ControlBlock<C> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            brace_token: braced!(content in input),
            children: content.parse()?,
        })
    }
}

pub struct If<C: Context> {
    if_token: Token![if],
    cond: Expr,
    then_block: ControlBlock<C>,
    else_branch: Option<(Token![else], Box<ControlIfOrBlock<C>>)>,
}

impl<C: Context + Parse> Parse for If<C> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            if_token: input.parse()?,
            cond: input.call(Expr::parse_without_eager_brace)?,
            then_block: input.parse()?,
            else_branch: if input.peek(Token![@]) && input.peek2(Token![else]) {
                input.parse::<Token![@]>()?;

                Some((input.parse()?, input.parse()?))
            } else {
                None
            },
        })
    }
}

impl<C: Context> Generate for If<C> {
    type Context = C;

    fn generate(&self, g: &mut Generator) {
        fn to_expr<C: Context>(if_: &If<C>, g: &mut Generator) -> TokenStream {
            let if_token = if_.if_token;
            let cond = &if_.cond;
            let then_block = if_.then_block.block(g);
            let else_branch = if_.else_branch.as_ref().map(|(else_token, if_or_block)| {
                let else_block = match &**if_or_block {
                    ControlIfOrBlock::If(if_) => to_expr(if_, g),
                    ControlIfOrBlock::Block(block) => block.block(g).to_token_stream(),
                };

                quote! {
                    #else_token #else_block
                }
            });

            quote! {
                #if_token #cond
                    #then_block
                #else_branch
            }
        }

        let expr = to_expr(self, g);

        g.push_stmt(expr);
    }
}

pub enum ControlIfOrBlock<C: Context> {
    If(If<C>),
    Block(ControlBlock<C>),
}

impl<C: Context + Parse> Parse for ControlIfOrBlock<C> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![if]) {
            input.parse().map(Self::If)
        } else if lookahead.peek(Brace) {
            input.parse().map(Self::Block)
        } else {
            Err(lookahead.error())
        }
    }
}

pub struct For<C: Context> {
    for_token: Token![for],
    pat: Pat,
    in_token: Token![in],
    expr: Expr,
    block: ControlBlock<C>,
}

impl<C: Context + Parse> Parse for For<C> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            for_token: input.parse()?,
            pat: input.call(Pat::parse_multi_with_leading_vert)?,
            in_token: input.parse()?,
            expr: input.call(Expr::parse_without_eager_brace)?,
            block: input.parse()?,
        })
    }
}

impl<C: Context> Generate for For<C> {
    type Context = C;

    fn generate(&self, g: &mut Generator) {
        let for_token = self.for_token;
        let pat = &self.pat;
        let in_token = self.in_token;
        let expr = &self.expr;
        let block = self.block.block(g);

        g.push_stmt(quote! {
            #for_token #pat #in_token #expr
                #block
        });
    }
}

pub struct While<C: Context> {
    while_token: Token![while],
    cond: Expr,
    block: ControlBlock<C>,
}

impl<C: Context + Parse> Parse for While<C> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            while_token: input.parse()?,
            cond: input.call(Expr::parse_without_eager_brace)?,
            block: input.parse()?,
        })
    }
}

impl<C: Context> Generate for While<C> {
    type Context = C;

    fn generate(&self, g: &mut Generator) {
        let while_token = self.while_token;
        let cond = &self.cond;
        let block = self.block.block(g);

        g.push_stmt(quote! {
            #while_token #cond
                #block
        });
    }
}

pub struct Match<C: Context> {
    match_token: Token![match],
    expr: Expr,
    brace_token: Brace,
    arms: Vec<MatchNodeArm<C>>,
}

impl<C: Context + Parse> Parse for Match<C> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            match_token: input.parse()?,
            expr: input.call(Expr::parse_without_eager_brace)?,
            brace_token: braced!(content in input),
            arms: {
                let mut arms = Vec::new();

                while !content.is_empty() {
                    arms.push(content.parse()?);
                }

                arms
            },
        })
    }
}

impl<C: Context> Generate for Match<C> {
    type Context = C;

    fn generate(&self, g: &mut Generator) {
        let arms = self
            .arms
            .iter()
            .map(|arm| {
                let pat = arm.pat.clone();
                let guard = arm
                    .guard
                    .as_ref()
                    .map(|(if_token, guard)| quote!(#if_token #guard));
                let fat_arrow_token = arm.fat_arrow_token;
                let block = match &arm.body {
                    MatchNodeArmBody::Block(block) => block.block(g),
                    MatchNodeArmBody::Child(child) => {
                        g.block_with(Brace::default(), |g| g.push(child))
                    }
                };
                let comma = arm.comma_token;

                quote!(#pat #guard #fat_arrow_token #block #comma)
            })
            .collect::<TokenStream>();

        let match_token = self.match_token;
        let expr = &self.expr;

        let mut stmt = quote!(#match_token #expr);

        self.brace_token
            .surround(&mut stmt, |tokens| tokens.extend(arms));

        g.push_stmt(stmt);
    }
}

pub struct MatchNodeArm<C: Context> {
    pat: Pat,
    guard: Option<(Token![if], Expr)>,
    fat_arrow_token: Token![=>],
    body: MatchNodeArmBody<C>,
    comma_token: Option<Token![,]>,
}

impl<C: Context + Parse> Parse for MatchNodeArm<C> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pat: input.call(Pat::parse_multi_with_leading_vert)?,
            guard: if input.peek(Token![if]) {
                Some((input.parse()?, input.parse()?))
            } else {
                None
            },
            fat_arrow_token: input.parse()?,
            body: input.parse()?,
            comma_token: input.parse()?,
        })
    }
}

pub enum MatchNodeArmBody<C: Context> {
    Block(ControlBlock<C>),
    Child(C),
}

impl<C: Context + Parse> Parse for MatchNodeArmBody<C> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Brace) {
            input.parse().map(Self::Block)
        } else {
            input.parse().map(Self::Child)
        }
    }
}
