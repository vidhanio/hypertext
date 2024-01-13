#[cfg(feature = "axum")]
mod axum_support {
    extern crate alloc;

    use axum_core::{
        body::Body,
        response::{IntoResponse, Response},
    };
    use http::{header, HeaderValue};

    use crate::Rendered;

    impl<T: Into<Body>> IntoResponse for Rendered<T> {
        #[inline]
        fn into_response(self) -> Response {
            (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("text/html; charset=utf-8"),
                )],
                self.0.into(),
            )
                .into_response()
        }
    }
}
