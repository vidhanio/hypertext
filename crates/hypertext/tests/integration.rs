//! End-to-end rendering scenarios.
#![cfg(feature = "alloc")]

use hypertext::prelude::*;

macro_rules! assert_html_pair {
    ($maud:expr, $rsx:expr, $expected:expr $(,)?) => {{
        let maud = ($maud).render();
        let rsx = ($rsx).render();
        assert_eq!(maud.as_inner(), $expected);
        assert_eq!(rsx.as_inner(), $expected);
    }};
}

#[test]
fn page_skeleton_parity_includes_doctype_voids_and_escaping() {
    let title = "My & Website";
    let description = "A <sample> page";

    assert_html_pair!(
        maud! {
            !DOCTYPE
            html lang="en" {
                head {
                    meta charset="utf-8";
                    meta name="description" content=(description);
                    title { (title) }
                    link rel="stylesheet" href="/styles.css";
                }
                body {
                    header {
                        nav {
                            a href="/" { "Home" }
                            a href="/about" { "About" }
                        }
                    }
                    main {
                        h1 { (title) }
                        p { "Welcome." }
                    }
                }
            }
        },
        rsx! {
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8">
                    <meta name="description" content=(description)>
                    <title>(title)</title>
                    <link rel="stylesheet" href="/styles.css">
                </head>
                <body>
                    <header>
                        <nav>
                            <a href="/">Home</a>
                            <a href="/about">About</a>
                        </nav>
                    </header>
                    <main>
                        <h1>(title)</h1>
                        <p>"Welcome."</p>
                    </main>
                </body>
            </html>
        },
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="description" content="A &lt;sample&gt; page"><title>My &amp; Website</title><link rel="stylesheet" href="/styles.css"></head><body><header><nav><a href="/">Home</a><a href="/about">About</a></nav></header><main><h1>My &amp; Website</h1><p>Welcome.</p></main></body></html>"#,
    );
}

fn render_shopping_list(items: &[&str]) -> (Rendered<String>, Rendered<String>) {
    (
        maud! {
            div #shopping-list {
                h1 { "Shopping List" }
                ul {
                    @for item in items {
                        li { (item) }
                    }
                }
            }
        }
        .render(),
        rsx! {
            <div id="shopping-list">
                <h1>"Shopping List"</h1>
                <ul>
                    @for item in items {
                        <li>(item)</li>
                    }
                </ul>
            </div>
        }
        .render(),
    )
}

#[test]
fn collection_rendering_matrix_handles_values_and_empty_input() {
    for (items, expected) in [
        (
            &(["Eggs", "Milk", "Bread"] as [&str; 3])[..],
            "<div id=\"shopping-list\"><h1>Shopping List</h1><ul><li>Eggs</li><li>Milk</li><li>Bread</li></ul></div>",
        ),
        (
            (&[] as &[&str]),
            "<div id=\"shopping-list\"><h1>Shopping List</h1><ul></ul></div>",
        ),
    ] {
        let (maud, rsx) = render_shopping_list(items);
        assert_eq!(maud, rsx);
        assert_eq!(maud.as_inner(), expected);
    }
}

fn render_blog_post(
    title: &str,
    author: &str,
    body: &str,
    tags: &[&str],
    published: bool,
) -> (Rendered<String>, Rendered<String>) {
    (
        maud! {
            article .post {
                header {
                    h1 { (title) }
                    span .author { "By " (author) }
                    @if !published {
                        span .draft { " [DRAFT]" }
                    }
                }
                p .content { (body) }
                @if !tags.is_empty() {
                    footer .tags {
                        "Tags: "
                        @for (index, tag) in tags.iter().enumerate() {
                            @if index > 0 { ", " }
                            a href={ "/tags/" (tag) } { (tag) }
                        }
                    }
                }
            }
        }
        .render(),
        rsx! {
            <article class="post">
                <header>
                    <h1>(title)</h1>
                    <span class="author">"By " (author)</span>
                    @if !published {
                        <span class="draft">" [DRAFT]"</span>
                    }
                </header>
                <p class="content">(body)</p>
                @if !tags.is_empty() {
                    <footer class="tags">
                        "Tags: "
                        @for (index, tag) in tags.iter().enumerate() {
                            @if index > 0 { ", " }
                            <a href={ "/tags/" (tag) }>(tag)</a>
                        }
                    </footer>
                }
            </article>
        }
        .render(),
    )
}

#[test]
fn conditional_blog_post_matrix_covers_published_and_draft_states() {
    let tags = ["rust", "web"];
    for (published, tags, expected) in [
        (
            true,
            &tags[..],
            "<article class=\"post\"><header><h1>Hello World</h1><span class=\"author\">By Alice</span></header><p class=\"content\">Welcome.</p><footer class=\"tags\">Tags: <a href=\"/tags/rust\">rust</a>, <a href=\"/tags/web\">web</a></footer></article>",
        ),
        (
            false,
            &[] as &[&str],
            "<article class=\"post\"><header><h1>Draft</h1><span class=\"author\">By Bob</span><span class=\"draft\"> [DRAFT]</span></header><p class=\"content\">Work in progress.</p></article>",
        ),
    ] {
        let (title, author, body) = if published {
            ("Hello World", "Alice", "Welcome.")
        } else {
            ("Draft", "Bob", "Work in progress.")
        };
        let (maud, rsx) = render_blog_post(title, author, body, tags, published);
        assert_eq!(maud, rsx);
        assert_eq!(maud.as_inner(), expected);
    }
}

fn render_form(
    username_error: Option<&str>,
    email_error: Option<&str>,
) -> (Rendered<String>, Rendered<String>) {
    (
        maud! {
            form method="post" action="/register" {
                div .form-group {
                    label for="username" { "Username" }
                    input type="text" id="username" class={
                        @if username_error.is_some() { "form-control is-invalid" }
                        @else { "form-control" }
                    };
                    @if let Some(error) = username_error {
                        span .error { (error) }
                    }
                }
                div .form-group {
                    label for="email" { "Email" }
                    input type="email" id="email" class={
                        @if email_error.is_some() { "form-control is-invalid" }
                        @else { "form-control" }
                    };
                    @if let Some(error) = email_error {
                        span .error { (error) }
                    }
                }
                button type="submit" { "Register" }
            }
        }
        .render(),
        rsx! {
            <form method="post" action="/register">
                <div class="form-group">
                    <label for="username">Username</label>
                    <input type="text" id="username" class={
                        @if username_error.is_some() { "form-control is-invalid" }
                        @else { "form-control" }
                    }>
                    @if let Some(error) = username_error {
                        <span class="error">(error)</span>
                    }
                </div>
                <div class="form-group">
                    <label for="email">Email</label>
                    <input type="email" id="email" class={
                        @if email_error.is_some() { "form-control is-invalid" }
                        @else { "form-control" }
                    }>
                    @if let Some(error) = email_error {
                        <span class="error">(error)</span>
                    }
                </div>
                <button type="submit">Register</button>
            </form>
        }
        .render(),
    )
}

#[test]
fn form_validation_matrix_matches_optional_errors() {
    for (username_error, email_error, expected) in [
        (
            Some("Username is required"),
            None,
            "<form method=\"post\" action=\"/register\"><div class=\"form-group\"><label for=\"username\">Username</label><input type=\"text\" id=\"username\" class=\"form-control is-invalid\"><span class=\"error\">Username is required</span></div><div class=\"form-group\"><label for=\"email\">Email</label><input type=\"email\" id=\"email\" class=\"form-control\"></div><button type=\"submit\">Register</button></form>",
        ),
        (
            None,
            Some("Invalid email"),
            "<form method=\"post\" action=\"/register\"><div class=\"form-group\"><label for=\"username\">Username</label><input type=\"text\" id=\"username\" class=\"form-control\"></div><div class=\"form-group\"><label for=\"email\">Email</label><input type=\"email\" id=\"email\" class=\"form-control is-invalid\"><span class=\"error\">Invalid email</span></div><button type=\"submit\">Register</button></form>",
        ),
    ] {
        let (maud, rsx) = render_form(username_error, email_error);
        assert_eq!(maud, rsx);
        assert_eq!(maud.as_inner(), expected);
    }
}

#[test]
fn repeated_sections_preserve_order_in_both_syntaxes() {
    let sections = [
        ("intro", "Welcome"),
        ("features", "Features"),
        ("faq", "FAQ"),
    ];
    assert_html_pair!(
        maud! {
            @for (id, title) in sections {
                section #(id) { h2 { (title) } }
            }
        },
        rsx! {
            @for (id, title) in sections {
                <section id=(id)><h2>(title)</h2></section>
            }
        },
        "<section id=\"intro\"><h2>Welcome</h2></section><section id=\"features\"><h2>Features</h2></section><section id=\"faq\"><h2>FAQ</h2></section>",
    );
}
