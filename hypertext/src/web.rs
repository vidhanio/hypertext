const HTML_MIME_TYPE: &str = "text/html; charset=utf-8";

#[cfg(feature = "axum")]
mod axum_support {
    extern crate alloc;

    use axum_core::{
        body::Body,
        response::{IntoResponse, Response},
    };
    use http::{HeaderValue, header};

    use super::HTML_MIME_TYPE;
    use crate::Rendered;

    impl<T: Into<Body>> IntoResponse for Rendered<T> {
        #[inline]
        fn into_response(self) -> Response {
            (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(HTML_MIME_TYPE),
                )],
                self.0.into(),
            )
                .into_response()
        }
    }
}

#[cfg(feature = "actix")]
mod actix_support {
    extern crate alloc;

    use alloc::string::String;

    use actix_web::{HttpRequest, HttpResponse, Responder, web::Html};

    use crate::Rendered;

    impl<T: Into<String>> Responder for Rendered<T> {
        type Body = <Html as Responder>::Body;

        #[inline]
        fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
            Html::new(self.0).respond_to(req)
        }
    }
}

#[cfg(feature = "poem")]
mod poem_support {
    extern crate alloc;

    use alloc::string::String;
    use core::marker::Send;

    use poem::{IntoResponse, Response, web::Html};

    use crate::Rendered;

    impl<T: Into<String> + Send> IntoResponse for Rendered<T> {
        #[inline]
        fn into_response(self) -> Response {
            Html(self.0).into_response()
        }
    }
}
