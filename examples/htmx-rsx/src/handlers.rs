use axum::response::IntoResponse;
use axum_htmx::HxRequest;
use hypertext::prelude::*;

use crate::views::{Document, Nav, about, home, list};

fn maybe_document<R: Renderable>(
    HxRequest(is_hx_request): HxRequest,
    selected: &str,
    children: R,
) -> impl IntoResponse {
    rsx! {
        @if is_hx_request {
            <Nav selected=selected oob=true />
            (children)
        } @else {
            <Document selected=selected>
                (children)
            </Document>
        }
    }
}

pub async fn handle_home(hx_request: HxRequest) -> impl IntoResponse {
    maybe_document(hx_request, "/", home())
}

pub async fn handle_about(hx_request: HxRequest) -> impl IntoResponse {
    maybe_document(hx_request, "/about", about())
}

pub async fn handle_list(hx_request: HxRequest) -> impl IntoResponse {
    maybe_document(hx_request, "/list", list())
}
