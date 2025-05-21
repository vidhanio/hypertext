use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Expr, Local, Pat, Stmt, Token, braced,
    parse::{Parse, ParseStream},
    token::Brace,
};

use super::{AnyBlock, Generate, Generator, Node, Nodes};

pub struct Control<N: Node>(ControlKind<N>);

impl<N: Node + Parse> Parse for Control<N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![@]>()?;

        Ok(Self(input.parse()?))
    }
}

impl<N: Node> Generate for Control<N> {
    fn generate(&self, g: &mut Generator) {
        g.push(&self.0);
    }
}

pub enum ControlKind<N: Node> {
    Let(Let),
    If(If<N>),
    For(For<N>),
    While(While<N>),
    Match(Match<N>),
}

impl<N: Node + Parse> Parse for ControlKind<N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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

impl<N: Node> Generate for ControlKind<N> {
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

pub struct Let(Local);

impl Parse for Let {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let local = match input.parse()? {
            Stmt::Local(local) => local,
            stmt => return Err(syn::Error::new_spanned(stmt, "expected `let` statement")),
        };

        Ok(Self(local))
    }
}

impl Generate for Let {
    fn generate(&self, g: &mut Generator) {
        g.push_stmt(&self.0);
    }
}

pub struct ControlBlock<N: Node> {
    brace_token: Brace,
    nodes: Nodes<N>,
}

impl<N: Node> ControlBlock<N> {
    fn block(&self, g: &mut Generator) -> AnyBlock {
        self.nodes.block(g, self.brace_token)
    }
}

impl<N: Node + Parse> Parse for ControlBlock<N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            brace_token: braced!(content in input),
            nodes: content.parse()?,
        })
    }
}

pub struct If<N: Node> {
    if_token: Token![if],
    cond: Expr,
    then_block: ControlBlock<N>,
    else_branch: Option<(Token![else], Box<ControlIfOrBlock<N>>)>,
}

impl<N: Node + Parse> Parse for If<N> {
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

impl<N: Node> Generate for If<N> {
    fn generate(&self, g: &mut Generator) {
        fn to_expr<N: Node>(if_: &If<N>, g: &mut Generator) -> TokenStream {
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

pub enum ControlIfOrBlock<N: Node> {
    If(If<N>),
    Block(ControlBlock<N>),
}

impl<N: Node + Parse> Parse for ControlIfOrBlock<N> {
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

pub struct For<N: Node> {
    for_token: Token![for],
    pat: Pat,
    in_token: Token![in],
    expr: Expr,
    block: ControlBlock<N>,
}

impl<N: Node + Parse> Parse for For<N> {
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

impl<N: Node> Generate for For<N> {
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

pub struct While<N: Node> {
    while_token: Token![while],
    cond: Expr,
    block: ControlBlock<N>,
}

impl<N: Node + Parse> Parse for While<N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            while_token: input.parse()?,
            cond: input.call(Expr::parse_without_eager_brace)?,
            block: input.parse()?,
        })
    }
}

impl<N: Node> Generate for While<N> {
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

pub struct Match<N: Node> {
    match_token: Token![match],
    expr: Expr,
    brace_token: Brace,
    arms: Vec<MatchNodeArm<N>>,
}

impl<N: Node + Parse> Parse for Match<N> {
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

impl<N: Node> Generate for Match<N> {
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
                    MatchNodeArmBody::Node(node) => {
                        g.block_with(Brace::default(), |g| g.push(node))
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

pub struct MatchNodeArm<N: Node> {
    pat: Pat,
    guard: Option<(Token![if], Expr)>,
    fat_arrow_token: Token![=>],
    body: MatchNodeArmBody<N>,
    comma_token: Option<Token![,]>,
}

impl<N: Node + Parse> Parse for MatchNodeArm<N> {
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

pub enum MatchNodeArmBody<N: Node> {
    Block(ControlBlock<N>),
    Node(N),
}

impl<N: Node + Parse> Parse for MatchNodeArmBody<N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Brace) {
            input.parse().map(Self::Block)
        } else {
            input.parse().map(Self::Node)
        }
    }
}
