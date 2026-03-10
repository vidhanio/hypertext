#![cfg(feature = "alloc")]

use hypertext::{prelude::*, Builder};

#[test]
fn shopping_list_maud() {
    let items = vec!["Eggs", "Milk", "Bread"];

    let result = maud! {
        div #shopping-list {
            h1 { "Shopping List" }
            ul {
                @for item in &items {
                    li { (item) }
                }
            }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div id="shopping-list"><h1>Shopping List</h1><ul><li>Eggs</li><li>Milk</li><li>Bread</li></ul></div>"#,
    );
}

#[test]
fn shopping_list_rsx() {
    let items = vec!["Eggs", "Milk", "Bread"];

    let result = rsx! {
        <div id="shopping-list">
            <h1>"Shopping List"</h1>
            <ul>
                @for item in &items {
                    <li>(item)</li>
                }
            </ul>
        </div>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div id="shopping-list"><h1>Shopping List</h1><ul><li>Eggs</li><li>Milk</li><li>Bread</li></ul></div>"#,
    );
}

#[test]
fn full_html_page_maud() {
    let title = "My Website";
    let description = "A sample page";

    let result = maud! {
        !DOCTYPE
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
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
                    p { "Welcome to my website." }
                }
                footer {
                    p { "Copyright 2025" }
                }
            }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><meta name="description" content="A sample page"><title>My Website</title><link rel="stylesheet" href="/styles.css"></head><body><header><nav><a href="/">Home</a><a href="/about">About</a></nav></header><main><h1>My Website</h1><p>Welcome to my website.</p></main><footer><p>Copyright 2025</p></footer></body></html>"#,
    );
}

#[test]
fn full_html_page_rsx() {
    let title = "My Website";
    let description = "A sample page";

    let result = rsx! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
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
                    <p>"Welcome to my website."</p>
                </main>
                <footer>
                    <p>"Copyright 2025"</p>
                </footer>
            </body>
        </html>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><meta name="description" content="A sample page"><title>My Website</title><link rel="stylesheet" href="/styles.css"></head><body><header><nav><a href="/">Home</a><a href="/about">About</a></nav></header><main><h1>My Website</h1><p>Welcome to my website.</p></main><footer><p>Copyright 2025</p></footer></body></html>"#,
    );
}

struct BlogPost {
    title: String,
    author: String,
    content: String,
    tags: Vec<String>,
    published: bool,
}

#[test]
fn blog_post_page() {
    let post = BlogPost {
        title: "Hello World".into(),
        author: "Alice".into(),
        content: "This is my first blog post.".into(),
        tags: vec!["rust".into(), "web".into(), "hypertext".into()],
        published: true,
    };

    let result = maud! {
        article .post {
            header {
                h1 { (post.title) }
                span .author { "By " (post.author) }
                @if !post.published {
                    span .draft { " [DRAFT]" }
                }
            }
            div .content {
                p { (post.content) }
            }
            @if !post.tags.is_empty() {
                footer .tags {
                    "Tags: "
                    @for (i, tag) in post.tags.iter().enumerate() {
                        @if i > 0 {
                            ", "
                        }
                        a href=(format!("/tags/{tag}")) { (tag) }
                    }
                }
            }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<article class="post"><header><h1>Hello World</h1><span class="author">By Alice</span></header><div class="content"><p>This is my first blog post.</p></div><footer class="tags">Tags: <a href="/tags/rust">rust</a>, <a href="/tags/web">web</a>, <a href="/tags/hypertext">hypertext</a></footer></article>"#,
    );
}

#[test]
fn blog_post_draft() {
    let post = BlogPost {
        title: "Draft Post".into(),
        author: "Bob".into(),
        content: "Work in progress.".into(),
        tags: vec![],
        published: false,
    };

    let result = maud! {
        article .post {
            header {
                h1 { (post.title) }
                span .author { "By " (post.author) }
                @if !post.published {
                    span .draft { " [DRAFT]" }
                }
            }
            div .content {
                p { (post.content) }
            }
            @if !post.tags.is_empty() {
                footer .tags {
                    "Tags: "
                    @for tag in &post.tags {
                        a href=(format!("/tags/{tag}")) { (tag) }
                    }
                }
            }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<article class="post"><header><h1>Draft Post</h1><span class="author">By Bob</span><span class="draft"> [DRAFT]</span></header><div class="content"><p>Work in progress.</p></div></article>"#,
    );
}

#[test]
fn data_table() {
    let headers = ["Name", "Age", "City"];
    let rows: Vec<[&str; 3]> = vec![
        ["Alice", "30", "New York"],
        ["Bob", "25", "San Francisco"],
        ["Carol", "35", "Chicago"],
    ];

    let result = maud! {
        table .data-table {
            thead {
                tr {
                    @for header in headers {
                        th { (header) }
                    }
                }
            }
            tbody {
                @for (i, row) in rows.iter().enumerate() {
                    tr class=(if i % 2 == 0 { "even" } else { "odd" }) {
                        @for cell in row {
                            td { (cell) }
                        }
                    }
                }
            }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<table class="data-table"><thead><tr><th>Name</th><th>Age</th><th>City</th></tr></thead><tbody><tr class="even"><td>Alice</td><td>30</td><td>New York</td></tr><tr class="odd"><td>Bob</td><td>25</td><td>San Francisco</td></tr><tr class="even"><td>Carol</td><td>35</td><td>Chicago</td></tr></tbody></table>"#,
    );
}

#[test]
fn form_with_validation() {
    let username_error: Option<&str> = Some("Username is required");
    let email_error: Option<&str> = None;

    let result = maud! {
        form method="post" action="/register" {
            div .form-group {
                label for="username" { "Username" }
                input
                    type="text"
                    id="username"
                    name="username"
                    class={"form-control" @if username_error.is_some() { " is-invalid" }};
                @if let Some(err) = username_error {
                    span .error { (err) }
                }
            }
            div .form-group {
                label for="email" { "Email" }
                input
                    type="email"
                    id="email"
                    name="email"
                    class={"form-control" @if email_error.is_some() { " is-invalid" }};
                @if let Some(err) = email_error {
                    span .error { (err) }
                }
            }
            button type="submit" class="btn" { "Register" }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<form method="post" action="/register"><div class="form-group"><label for="username">Username</label><input type="text" id="username" name="username" class="form-control is-invalid"><span class="error">Username is required</span></div><div class="form-group"><label for="email">Email</label><input type="email" id="email" name="email" class="form-control"></div><button type="submit" class="btn">Register</button></form>"#,
    );
}

#[test]
fn navigation_with_active_state() {
    let current_path = "/about";
    let links = [("/", "Home"), ("/about", "About"), ("/contact", "Contact")];

    let result = maud! {
        nav .navbar {
            @for (path, label) in links {
                a href=(path) class={"nav-link" @if path == current_path { " active" }} {
                    (label)
                }
            }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<nav class="navbar"><a href="/" class="nav-link">Home</a><a href="/about" class="nav-link active">About</a><a href="/contact" class="nav-link">Contact</a></nav>"#,
    );
}

#[renderable]
fn page_layout<'a, R: Renderable>(title: &'a str, children: &R) -> impl Renderable {
    maud! {
        !DOCTYPE
        html lang="en" {
            head {
                meta charset="utf-8";
                title { (title) }
            }
            body {
                (children)
            }
        }
    }
}

#[derive(Builder, Renderable)]
#[maud(
    div .card {
        h2 { (self.title) }
        p { (self.description) }
        a href=(self.link) { "Read more" }
    }
)]
struct ArticleCard {
    title: String,
    description: String,
    link: String,
}

#[test]
fn full_page_with_components_maud() {
    let result = maud! {
        PageLayout title="Articles" {
            main {
                h1 { "Latest Articles" }
                div .cards {
                    ArticleCard
                        title=("Getting Started".into())
                        description=("Learn the basics.".into())
                        link=("/articles/getting-started".into());
                    ArticleCard
                        title=("Advanced Tips".into())
                        description=("Go deeper.".into())
                        link=("/articles/advanced".into());
                }
            }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><title>Articles</title></head><body><main><h1>Latest Articles</h1><div class="cards"><div class="card"><h2>Getting Started</h2><p>Learn the basics.</p><a href="/articles/getting-started">Read more</a></div><div class="card"><h2>Advanced Tips</h2><p>Go deeper.</p><a href="/articles/advanced">Read more</a></div></div></main></body></html>"#,
    );
}

#[test]
fn full_page_with_components_rsx() {
    let result = rsx! {
        <PageLayout title="Articles">
            <main>
                <h1>Latest Articles</h1>
                <div class="cards">
                    <ArticleCard
                        title=("Getting Started".into())
                        description=("Learn the basics.".into())
                        link=("/articles/getting-started".into())>
                    <ArticleCard
                        title=("Advanced Tips".into())
                        description=("Go deeper.".into())
                        link=("/articles/advanced".into())>
                </div>
            </main>
        </PageLayout>
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><title>Articles</title></head><body><main><h1>Latest Articles</h1><div class="cards"><div class="card"><h2>Getting Started</h2><p>Learn the basics.</p><a href="/articles/getting-started">Read more</a></div><div class="card"><h2>Advanced Tips</h2><p>Go deeper.</p><a href="/articles/advanced">Read more</a></div></div></main></body></html>"#,
    );
}

#[test]
fn page_with_inline_svg_icon() {
    let result = maud! {
        button .icon-button {
            svg viewBox="0 0 24 24" width="24" height="24" {
                path d="M12 2L2 22h20L12 2z" fill="currentColor";
            }
            span { "Warning" }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<button class="icon-button"><svg viewBox="0 0 24 24" width="24" height="24"><path d="M12 2L2 22h20L12 2z" fill="currentColor"/></svg><span>Warning</span></button>"#,
    );
}

#[test]
fn list_with_items() {
    let notifications: Vec<&str> = vec!["New message", "Update available"];

    let result = maud! {
        div .notifications {
            @if notifications.is_empty() {
                p .empty { "No notifications" }
            } @else {
                ul {
                    @for notification in &notifications {
                        li { (notification) }
                    }
                }
                p .count { (notifications.len()) " notification(s)" }
            }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div class="notifications"><ul><li>New message</li><li>Update available</li></ul><p class="count">2 notification(s)</p></div>"#,
    );
}

#[test]
fn list_empty_state() {
    let notifications: Vec<&str> = vec![];

    let result = maud! {
        div .notifications {
            @if notifications.is_empty() {
                p .empty { "No notifications" }
            } @else {
                ul {
                    @for notification in &notifications {
                        li { (notification) }
                    }
                }
            }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div class="notifications"><p class="empty">No notifications</p></div>"#,
    );
}

#[test]
fn memoize_reuse() {
    let expensive = maud! {
        nav {
            a href="/" { "Home" }
            a href="/about" { "About" }
        }
    }
    .memoize();

    let page1 = maud::borrow! {
        div {
            (expensive)
            main { "Page 1" }
        }
    }
    .render();

    let page2 = maud::borrow! {
        div {
            (expensive)
            main { "Page 2" }
        }
    }
    .render();

    assert_eq!(
        page1.as_inner(),
        r#"<div><nav><a href="/">Home</a><a href="/about">About</a></nav><main>Page 1</main></div>"#,
    );
    assert_eq!(
        page2.as_inner(),
        r#"<div><nav><a href="/">Home</a><a href="/about">About</a></nav><main>Page 2</main></div>"#,
    );
}

enum AlertType {
    Success,
    Warning,
    Error,
}

#[test]
fn match_based_alert_rendering() {
    let alerts = [
        (AlertType::Success, "Operation completed"),
        (AlertType::Warning, "Low disk space"),
        (AlertType::Error, "Connection failed"),
    ];

    let result = maud! {
        div .alerts {
            @for (alert_type, message) in &alerts {
                div class={
                    "alert"
                    @match alert_type {
                        AlertType::Success => { " alert-success" },
                        AlertType::Warning => { " alert-warning" },
                        AlertType::Error => { " alert-error" },
                    }
                } {
                    (message)
                }
            }
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div class="alerts"><div class="alert alert-success">Operation completed</div><div class="alert alert-warning">Low disk space</div><div class="alert alert-error">Connection failed</div></div>"#,
    );
}

#[test]
fn dynamic_svg_chart() {
    let data_points = [(10, 90), (30, 50), (50, 70), (70, 30), (90, 60)];

    let result = svg::maud! {
        svg viewBox="0 0 100 100" width="200" height="200" {
            @for (x, y) in data_points {
                circle cx=(x) cy=(y) r="3" fill="blue";
            }
            @for pair in data_points.windows(2) {
                line
                    x1=(pair[0].0)
                    y1=(pair[0].1)
                    x2=(pair[1].0)
                    y2=(pair[1].1)
                    stroke="blue"
                    stroke_width="1";
            }
        }
    }
    .render();

    assert!(result
        .as_inner()
        .starts_with(r#"<svg viewBox="0 0 100 100" width="200" height="200">"#));
    assert!(result
        .as_inner()
        .contains(r#"<circle cx="10" cy="90" r="3" fill="blue"/>"#));
    assert!(result
        .as_inner()
        .contains(r#"<line x1="10" y1="90" x2="30" y2="50" stroke="blue" stroke_width="1"/>"#));
    assert!(result.as_inner().ends_with("</svg>"));
}

#[test]
fn user_content_is_escaped() {
    let user_name = "<script>alert('xss')</script>";
    let user_bio = "I love \"coding\" & <programming>";

    let result = maud! {
        div .profile {
            h2 { (user_name) }
            p { (user_bio) }
            input type="text" value=(user_name);
        }
    }
    .render();

    assert_eq!(
        result.as_inner(),
        r#"<div class="profile"><h2>&lt;script&gt;alert('xss')&lt;/script&gt;</h2><p>I love "coding" &amp; &lt;programming&gt;</p><input type="text" value="&lt;script&gt;alert('xss')&lt;/script&gt;"></div>"#,
    );
}

#[test]
fn multi_section_page_parity() {
    let sections = [
        ("intro", "Welcome"),
        ("features", "Features"),
        ("faq", "FAQ"),
    ];

    let maud_result = maud! {
        @for (id, title) in sections {
            section #(id) {
                h2 { (title) }
            }
        }
    }
    .render();

    let rsx_result = rsx! {
        @for (id, title) in sections {
            <section id=(id)>
                <h2>(title)</h2>
            </section>
        }
    }
    .render();

    assert_eq!(maud_result, rsx_result);
    assert_eq!(
        maud_result.as_inner(),
        r#"<section id="intro"><h2>Welcome</h2></section><section id="features"><h2>Features</h2></section><section id="faq"><h2>FAQ</h2></section>"#,
    );
}
