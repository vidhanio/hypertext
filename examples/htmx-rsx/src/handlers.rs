use axum::response::IntoResponse;
use axum_htmx::HxRequest;
use hypertext::prelude::*;

use crate::views::{about, home, index, list};

pub async fn handle_home(HxRequest(is_hx_request): HxRequest) -> impl IntoResponse {
    rsx! {
        @if is_hx_request {
            (home(true))
        } @else {
            (index("/", home(false)))
        }
    }
}

pub async fn handle_about(HxRequest(is_hx_request): HxRequest) -> impl IntoResponse {
    rsx! {
        @if is_hx_request {
            (about(true))
        } @else {
            (index("/about", about(false)))
        }
    }
}

pub async fn handle_list(HxRequest(is_hx_request): HxRequest) -> impl IntoResponse {
    rsx! {
        @if is_hx_request {
            (list(true))
        } @else {
            (index("/list", list(false)))
        }
    }
}
