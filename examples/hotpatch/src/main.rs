//! # Hot-reload example
//!
//! Demonstrates all three hypertext macro styles (`rsx!`, `html!`, `maud!`)
//! with subsecond hot-patching via the Dioxus CLI.
//!
//! ```sh
//! cargo install dioxus-cli
//! dx serve --hotpatch -p hotpatch
//! ```

use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::pin::Pin;

use axum::response::Html;
use axum::routing::get;
use axum::Router;
use dioxus_devtools::{DevserverMsg, connect};
use hypertext::{html, maud, prelude::*, rsx};
use subsecond::{HotFn, HotFnPanic};

#[tokio::main]
async fn main() {
    connect(|msg| {
        if let DevserverMsg::HotReload(hr) = msg {
            if let Some(jt) = hr.jump_table {
                if hr.for_pid == Some(std::process::id()) {
                    unsafe { subsecond::apply_patch(jt).unwrap() };
                }
            }
        }
    });

    let app = Router::new().route("/", get(index));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<String> {
    call_async(move || async move { Html(page()) }).await
}

fn rsx() -> impl Renderable {
    rsx! {
        <p>"from rsx!"</p>
    }
}

fn html() -> impl Renderable {
    html! {
        <p>"from html!"</p>
    }
}

fn maud() -> impl Renderable {
    maud! {
        p { "from maud!" }
    }
}

fn page() -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
<title>Hot Reload</title>
</head>
<body>
<h1>Hello from hypertext!</h1>
{}{}{}
</body>
</html>
"#,
        rsx().render().into_inner(),
        html().render().into_inner(),
        maud().render().into_inner()
    )
}

/// Wraps an async handler so subsecond can hot-patch it at runtime.
fn call_async<F, Fut, O>(f: F) -> Pin<Box<dyn Future<Output = O> + Send>>
where
    F: FnOnce() -> Fut + 'static,
    Fut: Future<Output = O> + Send + 'static,
    O: 'static,
{
    let mut f_option = Some(f);
    let wrapper = move || -> Pin<Box<dyn Future<Output = O> + Send>> {
        if let Some(closure) = f_option.take() {
            Box::pin(closure())
        } else {
            panic!("hot reload closure already consumed")
        }
    };

    let mut hotfn = HotFn::current(wrapper);
    loop {
        let res = std::panic::catch_unwind(AssertUnwindSafe(|| hotfn.call(())));
        let err = match res {
            Ok(res) => return res,
            Err(err) => err,
        };
        if err.downcast_ref::<HotFnPanic>().is_none() {
            std::panic::resume_unwind(err);
        }
        panic!("hot reload detected but cannot retry FnOnce closure");
    }
}
