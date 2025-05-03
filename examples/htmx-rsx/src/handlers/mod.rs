use axum::response::IntoResponse;
use axum_htmx::HxRequest;
use hypertext::Renderable;

use crate::views::{about, home, index, list};

pub async fn handle_index(HxRequest(is_hx_request): HxRequest) -> impl IntoResponse {
    if is_hx_request {
        home(true).render().into_response()
    } else {
        index("/", home(false)).render().into_response()
    }
}

pub async fn handle_about(HxRequest(is_hx_request): HxRequest) -> impl IntoResponse {
    if is_hx_request {
        about(true).render().into_response()
    } else {
        index("/about", about(false)).render().into_response()
    }
}

pub async fn handle_list(HxRequest(is_hx_request): HxRequest) -> impl IntoResponse {
    if is_hx_request {
        list(true).render().into_response()
    } else {
        index("/list", list(false)).render().into_response()
    }
}
