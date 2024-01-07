use std::collections::HashMap;

use proc_macro2::{Delimiter, Ident, Literal, Spacing, Span, TokenStream, TokenTree};
use proc_macro2_diagnostics::{Diagnostic, SpanDiagnosticExt};
use rstml::ParsingResult;
use syn::Lit;

use super::ast::{self, Block, ElementBody, Markup, MatchArm};

pub fn parse(input: TokenStream) -> ParsingResult<Vec<Markup>> {
    Parser::new(input).markups()
}

#[derive(Clone)]
struct Parser {
    /// If we're inside an attribute, then this contains the attribute name.
    current_attr: Option<String>,
    input: <TokenStream as IntoIterator>::IntoIter,
}

impl Iterator for Parser {
    type Item = TokenTree;

    fn next(&mut self) -> Option<TokenTree> {
        self.input.next()
    }
}

impl Parser {
    fn new(input: TokenStream) -> Parser {
        Parser {
            current_attr: None,
            input: input.into_iter(),
        }
    }

    fn with_input(&self, input: TokenStream) -> Parser {
        Parser {
            current_attr: self.current_attr.clone(),
            input: input.into_iter(),
        }
    }

    /// Returns the next token in the stream without consuming it.
    fn peek(&mut self) -> Option<TokenTree> {
        self.clone().next()
    }

    /// Returns the next two tokens in the stream without consuming them.
    fn peek2(&mut self) -> Option<(TokenTree, Option<TokenTree>)> {
        let mut clone = self.clone();
        clone.next().map(|first| (first, clone.next()))
    }

    /// Advances the cursor by one step.
    fn advance(&mut self) {
        self.next();
    }

    /// Advances the cursor by two steps.
    fn advance2(&mut self) {
        self.next();
        self.next();
    }

    /// Parses multiple blocks of markup.
    fn markups(&mut self) -> ParsingResult<Vec<Markup>> {
        let mut markups = Vec::new();
        let mut diagnostics = Vec::new();

        while let Some(tt) = self.peek2() {
            match tt {
                (TokenTree::Punct(ref punct), _) if punct.as_char() == ';' => self.advance(),
                (TokenTree::Punct(ref punct), Some(TokenTree::Ident(ref ident)))
                    if punct.as_char() == '@' && *ident == "let" =>
                {
                    self.advance2();
                    let keyword = TokenTree::Ident(ident.clone());
                    let (markup, diags) = self.let_expr(punct.span(), keyword).split();
                    if let Some(markup) = markup {
                        markups.push(markup);
                    }
                    diagnostics.extend(diags);
                }
                _ => {
                    let (markup, diags) = self.markup().split();
                    if let Some(markup) = markup {
                        markups.push(markup);
                    }
                    diagnostics.extend(diags);
                }
            }
        }

        ParsingResult::from_parts(Some(markups), diagnostics)
    }

    /// Parses a single block of markup.
    fn markup(&mut self) -> ParsingResult<Markup> {
        let token = match self.peek() {
            Some(token) => token,
            None => {
                return ParsingResult::Failed(vec![
                    Span::call_site().error("unexpected end of input")
                ])
            }
        };

        match token {
            // Literal
            TokenTree::Literal(literal) => {
                self.advance();
                self.literal(literal)
            }
            // Special form
            TokenTree::Punct(ref punct) if punct.as_char() == '@' => {
                self.advance();
                let at_span = punct.span();
                match self.next() {
                    Some(TokenTree::Ident(ident)) => {
                        let keyword = TokenTree::Ident(ident.clone());
                        match ident.to_string().as_str() {
                            "if" => {
                                let mut segments = Vec::new();
                                self.if_expr(at_span, vec![keyword], &mut segments);
                                ParsingResult::Ok(Markup::Special { segments })
                            }
                            "while" => self.while_expr(at_span, keyword),
                            "for" => self.for_expr(at_span, keyword),
                            "match" => self.match_expr(at_span, keyword),
                            "let" => ParsingResult::Partial(
                                Markup::ParseError {
                                    span: keyword.span(),
                                },
                                vec![keyword.span().error("`@let` only works inside a block")],
                            ),
                            other => ParsingResult::Partial(
                                Markup::ParseError {
                                    span: keyword.span(),
                                },
                                vec![keyword.span().error(format!("unknown keyword `@{other}`"))],
                            ),
                        }
                    }
                    _ => ParsingResult::Partial(
                        Markup::ParseError { span: at_span },
                        vec![at_span.error("expected keyword after `@`")],
                    ),
                }
            }
            // Element
            TokenTree::Ident(ident) => {
                let ident_string = ident.to_string();
                match ident_string.as_str() {
                    "if" | "while" | "for" | "match" | "let" => {
                        return ParsingResult::Partial(
                            Markup::ParseError { span: ident.span() },
                            vec![ident
                                .span()
                                .error(format!("found keyword `{ident_string}`"))
                                .help(format!("should this be a `@{ident_string}`?"))],
                        );
                    }
                    "true" | "false" => {
                        if let Some(attr_name) = &self.current_attr {
                            return ParsingResult::Partial(
                                Markup::ParseError { span: ident.span() },
                                vec![ident
                                .span()
                                .error("attribute value must be a string")
                                .help(format!(
                                    "to declare an empty attribute, omit the equals sign: `{attr_name}`"
                                ))
                                .help(format!(
                                    "to toggle the attribute, use square brackets: `{attr_name}[some_boolean_flag]`"
                                ))],
                            );
                        }
                    }
                    _ => {}
                }

                // `.try_namespaced_name()` should never fail as we've
                // already seen an `Ident`
                let name = self.try_namespaced_name().expect("identifier");
                self.element(name)
            }
            // Div element shorthand
            TokenTree::Punct(ref punct) if punct.as_char() == '.' || punct.as_char() == '#' => {
                let name = TokenTree::Ident(Ident::new("div", punct.span()));
                self.element(name.into())
            }
            // Splice
            TokenTree::Group(ref group) if group.delimiter() == Delimiter::Parenthesis => {
                self.advance();
                ParsingResult::Ok(Markup::Splice {
                    expr: group.stream(),
                    outer_span: group.span(),
                })
            }
            // Block
            TokenTree::Group(ref group) if group.delimiter() == Delimiter::Brace => {
                self.advance();

                match self.block(group.stream(), group.span()) {
                    ParsingResult::Ok(block) => ParsingResult::Ok(Markup::Block(block)),
                    ParsingResult::Failed(diags) => ParsingResult::Failed(diags),
                    ParsingResult::Partial(block, diags) => {
                        ParsingResult::Partial(Markup::Block(block), diags)
                    }
                }
            }
            // ???
            token => ParsingResult::Partial(
                Markup::ParseError { span: token.span() },
                vec![token.span().error("invalid syntax")],
            ),
        }
    }

    /// Parses a literal string.
    fn literal(&mut self, literal: Literal) -> ParsingResult<Markup> {
        match Lit::new(literal.clone()) {
            Lit::Str(lit_str) => ParsingResult::Ok(Markup::Literal {
                content: lit_str.value(),
                span: literal.span(),
            }),
            // Boolean literals are idents, so `Lit::Bool` is handled in
            // `markup`, not here.
            Lit::Int(lit_int) => ParsingResult::Ok(Markup::Literal {
                content: lit_int.to_string(),
                span: literal.span(),
            }),
            Lit::Float(..) => ParsingResult::Partial(
                Markup::ParseError {
                    span: literal.span(),
                },
                vec![literal
                    .span()
                    .error(format!(r#"literal must be double-quoted: `"{literal}"`"#))],
            ),
            Lit::Char(lit_char) => ParsingResult::Partial(
                Markup::ParseError {
                    span: literal.span(),
                },
                vec![literal.span().error(format!(
                    r#"literal must be double-quoted: `"{}"`"#,
                    lit_char.value()
                ))],
            ),
            _ => ParsingResult::Partial(
                Markup::ParseError {
                    span: literal.span(),
                },
                vec![literal.span().error("expected string")],
            ),
        }
    }

    /// Parses an `@if` expression.
    ///
    /// The leading `@if` should already be consumed.
    fn if_expr(
        &mut self,
        at_span: Span,
        prefix: Vec<TokenTree>,
        segments: &mut Vec<ast::Special>,
    ) -> ParsingResult<()> {
        let mut head = prefix;
        let body = loop {
            match self.next() {
                Some(TokenTree::Group(ref block)) if block.delimiter() == Delimiter::Brace => {
                    match self.block(block.stream(), block.span()) {
                        ParsingResult::Ok(block) => break block,
                        ParsingResult::Failed(diags) => {
                            return ParsingResult::Failed(diags);
                        }
                        ParsingResult::Partial(block, diags) => {
                            return ParsingResult::Partial((), diags);
                        }
                    }
                }
                Some(token) => head.push(token),
                None => {
                    return ParsingResult::Partial(
                        (),
                        vec![at_span.error("expected body for this `@if`")],
                    );
                }
            }
        };

        segments.push(ast::Special {
            at_span,
            head: head.into_iter().collect(),
            body,
        });

        self.else_if_expr(segments)
    }

    /// Parses an optional `@else if` or `@else`.
    ///
    /// The leading `@else if` or `@else` should *not* already be consumed.
    fn else_if_expr(&mut self, segments: &mut Vec<ast::Special>) -> ParsingResult<()> {
        match self.peek2() {
            Some((TokenTree::Punct(ref punct), Some(TokenTree::Ident(ref else_keyword))))
                if punct.as_char() == '@' && *else_keyword == "else" =>
            {
                self.advance2();
                let at_span = punct.span();
                let else_keyword = TokenTree::Ident(else_keyword.clone());
                match self.peek() {
                    // `@else if`
                    Some(TokenTree::Ident(ref if_keyword)) if *if_keyword == "if" => {
                        self.advance();
                        let if_keyword = TokenTree::Ident(if_keyword.clone());
                        self.if_expr(at_span, vec![else_keyword, if_keyword], segments)
                    }
                    // Just an `@else`
                    _ => match self.next() {
                        Some(TokenTree::Group(ref group))
                            if group.delimiter() == Delimiter::Brace =>
                        {
                            let body = match self.block(group.stream(), group.span()) {
                                ParsingResult::Ok(block) => block,
                                ParsingResult::Failed(diags) => {
                                    return ParsingResult::Failed(diags);
                                }
                                ParsingResult::Partial(block, diags) => {
                                    return ParsingResult::Partial((), diags);
                                }
                            };

                            segments.push(ast::Special {
                                at_span,
                                head: vec![else_keyword].into_iter().collect(),
                                body,
                            });

                            ParsingResult::Ok(())
                        }
                        _ => ParsingResult::Partial(
                            (),
                            vec![else_keyword.span().error("expected body for this `@else`")],
                        ),
                    },
                }
            }
            // We didn't find an `@else`; stop
            _ => ParsingResult::Ok(()),
        }
    }

    /// Parses an `@while` expression.
    ///
    /// The leading `@while` should already be consumed.
    fn while_expr(&mut self, at_span: Span, keyword: TokenTree) -> ParsingResult<Markup> {
        let keyword_span = keyword.span();
        let mut head = vec![keyword];

        let body = loop {
            match self.next() {
                Some(TokenTree::Group(ref block)) if block.delimiter() == Delimiter::Brace => {
                    match self.block(block.stream(), block.span()) {
                        ParsingResult::Ok(block) => break block,
                        ParsingResult::Failed(diags) => {
                            return ParsingResult::Failed(diags);
                        }
                        ParsingResult::Partial(block, diags) => {
                            return ParsingResult::Partial(
                                Markup::ParseError { span: keyword_span },
                                diags,
                            );
                        }
                    }
                }
                Some(token) => head.push(token),
                None => {
                    return ParsingResult::Partial(
                        Markup::ParseError { span: keyword_span },
                        vec![keyword_span.error("expected body for this `@while`")],
                    );
                }
            }
        };

        ParsingResult::Ok(Markup::Special {
            segments: vec![ast::Special {
                at_span,
                head: head.into_iter().collect(),
                body,
            }],
        })
    }

    /// Parses a `@for` expression.
    ///
    /// The leading `@for` should already be consumed.
    fn for_expr(&mut self, at_span: Span, keyword: TokenTree) -> ParsingResult<Markup> {
        let keyword_span = keyword.span();
        let mut head = vec![keyword];

        loop {
            match self.next() {
                Some(TokenTree::Ident(ref in_keyword)) if *in_keyword == "in" => {
                    head.push(TokenTree::Ident(in_keyword.clone()));
                    break;
                }
                Some(token) => head.push(token),
                None => {
                    return ParsingResult::Partial(
                        Markup::ParseError { span: keyword_span },
                        vec![keyword_span.error("missing `in` in `@for` loop")],
                    );
                }
            }
        }

        let body = loop {
            match self.next() {
                Some(TokenTree::Group(ref block)) if block.delimiter() == Delimiter::Brace => {
                    match self.block(block.stream(), block.span()) {
                        ParsingResult::Ok(block) => break block,
                        ParsingResult::Failed(diags) => {
                            return ParsingResult::Failed(diags);
                        }
                        ParsingResult::Partial(block, diags) => {
                            return ParsingResult::Partial(
                                Markup::ParseError { span: keyword_span },
                                diags,
                            );
                        }
                    }
                }
                Some(token) => head.push(token),
                None => {
                    return ParsingResult::Partial(
                        Markup::ParseError { span: keyword_span },
                        vec![keyword_span.error("expected body for this `@for`")],
                    );
                }
            }
        };

        ParsingResult::Ok(Markup::Special {
            segments: vec![ast::Special {
                at_span,
                head: head.into_iter().collect(),
                body,
            }],
        })
    }

    /// Parses a `@match` expression.
    ///
    /// The leading `@match` should already be consumed.
    fn match_expr(&mut self, at_span: Span, keyword: TokenTree) -> ParsingResult<Markup> {
        let keyword_span = keyword.span();
        let mut head = vec![keyword];

        let (arms, arms_span) = loop {
            match self.next() {
                Some(TokenTree::Group(ref body)) if body.delimiter() == Delimiter::Brace => {
                    let span = body.span();

                    match self.with_input(body.stream()).match_arms() {
                        ParsingResult::Ok(arms) => break (arms, span),
                        ParsingResult::Failed(diagnostics) => {
                            return ParsingResult::Failed(diagnostics)
                        }
                        ParsingResult::Partial(arms, diagnostics) => {
                            return ParsingResult::Partial(Markup::ParseError { span }, diagnostics)
                        }
                    }
                }
                Some(token) => head.push(token),
                None => {
                    return ParsingResult::Partial(
                        Markup::ParseError { span: keyword_span },
                        vec![keyword_span.error("expected body for this `@match`")],
                    );
                }
            }
        };

        ParsingResult::Ok(Markup::Match {
            at_span,
            head: head.into_iter().collect(),
            arms,
            arms_span,
        })
    }

    fn match_arms(&mut self) -> ParsingResult<Vec<MatchArm>> {
        let mut arms = Vec::new();

        while let Some(arm) = match self.match_arm() {
            ParsingResult::Ok(arm) => arm,
            ParsingResult::Failed(diagnostics) => return ParsingResult::Partial(arms, diagnostics),
            ParsingResult::Partial(arm, diagnostics) => {
                arms.extend(arm);
                return ParsingResult::Partial(arms, diagnostics);
            }
        } {
            arms.push(arm);
        }

        ParsingResult::Ok(arms)
    }

    fn match_arm(&mut self) -> ParsingResult<Option<MatchArm>> {
        let mut head = Vec::new();
        loop {
            match self.peek2() {
                Some((TokenTree::Punct(ref eq), Some(TokenTree::Punct(ref gt))))
                    if eq.as_char() == '='
                        && gt.as_char() == '>'
                        && eq.spacing() == Spacing::Joint =>
                {
                    self.advance2();
                    head.push(TokenTree::Punct(eq.clone()));
                    head.push(TokenTree::Punct(gt.clone()));
                    break;
                }
                Some((token, _)) => {
                    self.advance();
                    head.push(token);
                }
                None => {
                    return if head.is_empty() {
                        ParsingResult::Ok(None)
                    } else {
                        let head_span = ast::span_tokens(head);

                        ParsingResult::Partial(
                            None,
                            vec![head_span.error("unexpected end of @match pattern")],
                        )
                    }
                }
            }
        }
        let body = match self.next() {
            // $pat => { $stmts }
            Some(TokenTree::Group(ref body)) if body.delimiter() == Delimiter::Brace => {
                let body = match self.block(body.stream(), body.span()) {
                    ParsingResult::Ok(block) => block,
                    ParsingResult::Failed(diags) => {
                        return ParsingResult::Failed(diags);
                    }
                    ParsingResult::Partial(block, diags) => {
                        return ParsingResult::Partial(None, diags);
                    }
                };

                // Trailing commas are optional if the match arm is a braced block
                if let Some(TokenTree::Punct(ref punct)) = self.peek() {
                    if punct.as_char() == ',' {
                        self.advance();
                    }
                }

                body
            }
            // $pat => $expr
            Some(first_token) => {
                let mut span = first_token.span();
                let mut body = vec![first_token];
                loop {
                    match self.next() {
                        Some(TokenTree::Punct(ref punct)) if punct.as_char() == ',' => break,
                        Some(token) => {
                            body.push(token);
                        }
                        None => break,
                    }
                }
                match self.block(body.into_iter().collect(), span) {
                    ParsingResult::Ok(block) => block,
                    ParsingResult::Failed(diags) => {
                        return ParsingResult::Failed(diags);
                    }
                    ParsingResult::Partial(block, diags) => {
                        return ParsingResult::Partial(None, diags);
                    }
                }
            }
            None => {
                let span = ast::span_tokens(head);

                return ParsingResult::Partial(
                    None,
                    vec![span.error("unexpected end of @match arm")],
                );
            }
        };

        ParsingResult::Ok(Some(MatchArm {
            head: head.into_iter().collect(),
            body,
        }))
    }

    /// Parses a `@let` expression.
    ///
    /// The leading `@let` should already be consumed.
    fn let_expr(&mut self, at_span: Span, keyword: TokenTree) -> ParsingResult<Markup> {
        let mut tokens = vec![keyword];
        loop {
            match self.next() {
                Some(token) => match token {
                    TokenTree::Punct(ref punct) if punct.as_char() == '=' => {
                        tokens.push(token.clone());
                        break;
                    }
                    _ => tokens.push(token),
                },
                None => {
                    let span = ast::span_tokens(tokens);
                    return ParsingResult::Partial(
                        Markup::ParseError { span },
                        vec![span.error("unexpected end of `@let` expression")],
                    );
                }
            }
        }
        loop {
            match self.next() {
                Some(token) => match token {
                    TokenTree::Punct(ref punct) if punct.as_char() == ';' => {
                        tokens.push(token.clone());
                        break;
                    }
                    _ => tokens.push(token),
                },
                None => {
                    let span = ast::span_tokens(tokens);
                    return ParsingResult::Partial(
                        Markup::ParseError { span },
                        vec![span
                            .error("unexpected end of `@let` expression")
                            .help("are you missing a semicolon?")],
                    );
                }
            }
        }

        ParsingResult::Ok(Markup::Let {
            at_span,
            tokens: tokens.into_iter().collect(),
        })
    }

    /// Parses an element node.
    ///
    /// The element name should already be consumed.
    fn element(&mut self, name: TokenStream) -> ParsingResult<Markup> {
        if self.current_attr.is_some() {
            let span = ast::span_tokens(name);
            return ParsingResult::Partial(
                Markup::ParseError { span },
                vec![span.error("unexpected element")],
            );
        }

        let attrs = match self.attrs() {
            ParsingResult::Ok(attrs) => attrs,
            ParsingResult::Failed(diags) => return ParsingResult::Failed(diags),
            ParsingResult::Partial(attrs, diags) => {
                let span = ast::span_tokens(name);
                return ParsingResult::Partial(Markup::ParseError { span }, diags);
            }
        };

        let body = match self.peek() {
            Some(TokenTree::Punct(ref punct))
                if punct.as_char() == ';' || punct.as_char() == '/' =>
            {
                // Void element
                self.advance();
                if punct.as_char() == '/' {
                    return ParsingResult::Partial(
                        Markup::ParseError { span: punct.span() },
                        vec![punct
                            .span()
                            .error("void elements must use `;`, not `/`")
                            .help("change this to `;`")
                            .help("see https://github.com/lambda-fairy/maud/pull/315 for details")],
                    );
                }
                ElementBody::Void {
                    semi_span: punct.span(),
                }
            }
            Some(_) => match self.markup() {
                ParsingResult::Ok(Markup::Block(block)) => ElementBody::Block { block },
                ParsingResult::Ok(markup) => {
                    let markup_span = markup.span();
                    return ParsingResult::Partial(
                        Markup::ParseError { span: markup_span },
                        vec![markup_span
                            .error("element body must be wrapped in braces")
                            .help("see https://github.com/lambda-fairy/maud/pull/137 for details")],
                    );
                }
                failure => return failure,
            },
            None => {
                return ParsingResult::Partial(
                    Markup::ParseError {
                        span: Span::call_site(),
                    },
                    vec![Span::call_site().error("expected `;`, found end of macro")],
                );
            }
        };

        ParsingResult::Ok(Markup::Element { name, attrs, body })
    }

    /// Parses the attributes of an element.
    fn attrs(&mut self) -> ParsingResult<Vec<ast::Attr>> {
        let mut attrs = Vec::new();
        loop {
            if let Some(name) = self.try_namespaced_name() {
                // Attribute
                match self.peek() {
                    // Non-empty attribute
                    Some(TokenTree::Punct(ref punct)) if punct.as_char() == '=' => {
                        self.advance();
                        // Parse a value under an attribute context
                        assert!(self.current_attr.is_none());
                        self.current_attr = Some(ast::name_to_string(name.clone()));
                        let attr_type = match self.attr_toggler() {
                            Some(toggler) => ast::AttrType::Optional { toggler },
                            None => {
                                let value = self.markup();
                                ast::AttrType::Normal { value }
                            }
                        };
                        self.current_attr = None;
                        attrs.push(ast::Attr::Named {
                            named_attr: ast::NamedAttr { name, attr_type },
                        });
                    }
                    // Empty attribute (legacy syntax)
                    Some(TokenTree::Punct(ref punct)) if punct.as_char() == '?' => {
                        self.advance();
                        let toggler = self.attr_toggler();
                        attrs.push(ast::Attr::Named {
                            named_attr: ast::NamedAttr {
                                name: name.clone(),
                                attr_type: ast::AttrType::Empty { toggler },
                            },
                        });
                    }
                    // Empty attribute (new syntax)
                    _ => {
                        let toggler = self.attr_toggler();
                        attrs.push(ast::Attr::Named {
                            named_attr: ast::NamedAttr {
                                name: name.clone(),
                                attr_type: ast::AttrType::Empty { toggler },
                            },
                        });
                    }
                }
            } else {
                match self.peek() {
                    // Class shorthand
                    Some(TokenTree::Punct(ref punct)) if punct.as_char() == '.' => {
                        self.advance();
                        let name = self.class_or_id_name();
                        let toggler = self.attr_toggler();
                        attrs.push(ast::Attr::Class {
                            dot_span: punct.span(),
                            name,
                            toggler,
                        });
                    }
                    // ID shorthand
                    Some(TokenTree::Punct(ref punct)) if punct.as_char() == '#' => {
                        self.advance();
                        let name = self.class_or_id_name();
                        attrs.push(ast::Attr::Id {
                            hash_span: punct.span(),
                            name,
                        });
                    }
                    // If it's not a valid attribute, backtrack and bail out
                    _ => break,
                }
            }
        }

        let mut attr_map: HashMap<String, Vec<Span>> = HashMap::new();
        let mut has_class = false;
        for attr in &attrs {
            let name = match attr {
                ast::Attr::Class { .. } => {
                    if has_class {
                        // Only check the first class to avoid spurious duplicates
                        continue;
                    }
                    has_class = true;
                    "class".to_string()
                }
                ast::Attr::Id { .. } => "id".to_string(),
                ast::Attr::Named { named_attr } => named_attr
                    .name
                    .clone()
                    .into_iter()
                    .map(|token| token.to_string())
                    .collect(),
            };
            let entry = attr_map.entry(name).or_default();
            entry.push(attr.span());
        }

        for (name, spans) in attr_map {
            if spans.len() > 1 {
                let mut spans = spans.into_iter();
                let first_span = spans.next().expect("spans should be non-empty");
                abort!(first_span, "duplicate attribute `{}`", name);
            }
        }

        attrs
    }

    /// Parses the name of a class or ID.
    fn class_or_id_name(&mut self) -> ParsingResult<Markup> {
        if let Some(symbol) = self.try_name() {
            ParsingResult::Ok(Markup::Symbol { symbol })
        } else {
            self.markup()
        }
    }

    /// Parses the `[cond]` syntax after an empty attribute or class shorthand.
    fn attr_toggler(&mut self) -> Option<ast::Toggler> {
        match self.peek() {
            Some(TokenTree::Group(ref group)) if group.delimiter() == Delimiter::Bracket => {
                self.advance();
                Some(ast::Toggler {
                    cond: group.stream(),
                    cond_span: group.span(),
                })
            }
            _ => None,
        }
    }

    /// Parses an identifier, without dealing with namespaces.
    fn try_name(&mut self) -> Option<TokenStream> {
        let mut result = Vec::new();
        let mut expect_ident_or_literal = true;
        loop {
            expect_ident_or_literal = match self.peek() {
                Some(TokenTree::Punct(ref punct)) if punct.as_char() == '-' => {
                    self.advance();
                    result.push(TokenTree::Punct(punct.clone()));
                    true
                }
                Some(token @ TokenTree::Ident(_)) if expect_ident_or_literal => {
                    self.advance();
                    result.push(token);
                    false
                }
                Some(TokenTree::Literal(ref literal)) if expect_ident_or_literal => {
                    self.literal(literal.clone());
                    self.advance();
                    result.push(TokenTree::Literal(literal.clone()));
                    false
                }
                _ => {
                    if result.is_empty() {
                        return None;
                    } else {
                        break;
                    }
                }
            };
        }
        Some(result.into_iter().collect())
    }

    /// Parses a HTML element or attribute name, along with a namespace
    /// if necessary.
    fn try_namespaced_name(&mut self) -> Option<TokenStream> {
        let mut result = vec![self.try_name()?];
        if let Some(TokenTree::Punct(ref punct)) = self.peek() {
            if punct.as_char() == ':' {
                self.advance();
                result.push(TokenStream::from(TokenTree::Punct(punct.clone())));
                result.push(self.try_name()?);
            }
        }
        Some(result.into_iter().collect())
    }

    /// Parses the given token stream as a Maud expression.
    fn block(&mut self, body: TokenStream, outer_span: Span) -> ParsingResult<Block> {
        match self.with_input(body).markups() {
            ParsingResult::Ok(markups) => ParsingResult::Ok(Block {
                markups,
                outer_span,
            }),
            ParsingResult::Failed(diags) => ParsingResult::Failed(diags),
            ParsingResult::Partial(markups, diags) => ParsingResult::Partial(
                Block {
                    markups,
                    outer_span,
                },
                diags,
            ),
        }
    }
}
