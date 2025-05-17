use axum::response::IntoResponse;
use axum_htmx::HxRequest;
use hypertext::prelude::*;

use crate::views::{about, home, index, list};

pub async fn handle_home(HxRequest(is_hx_request): HxRequest) -> impl IntoResponse {
    if is_hx_request {
        home(true).render()
    } else {
        index(String::from("/"), home(false)).render()
    }
}

pub async fn handle_about(HxRequest(is_hx_request): HxRequest) -> impl IntoResponse {
    if is_hx_request {
        about(true).render()
    } else {
        index(String::from("/about"), about(false)).render()
    }
}

pub async fn handle_list(HxRequest(is_hx_request): HxRequest) -> impl IntoResponse {
    if is_hx_request {
        list(true).render()
    } else {
        index(String::from("/list"), list(false)).render()
    }
}
