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

#[cfg(feature = "actix")]
mod actix_support {
    use actix_web::{HttpRequest, HttpResponse, Responder};
    use actix_web::body::EitherBody;

    use crate::Rendered;

    impl<T> Responder for Rendered<T>
        where
            T: Responder,
    {
        type Body = EitherBody<T::Body>;

        #[inline]
        fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
            self.0
                .customize()
                .insert_header(("content-type", "text/html"))
                .respond_to(req)
        }
    }
}
