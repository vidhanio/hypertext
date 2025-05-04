const HTML_MIME_TYPE: &str = "text/html; charset=utf-8";

#[cfg(feature = "actix-web")]
mod actix_web {
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

#[cfg(feature = "axum")]
mod axum {
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

#[cfg(feature = "poem")]
mod poem {
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

#[cfg(feature = "rocket")]
mod rocket {
    extern crate alloc;

    use rocket::{
        Request,
        response::{self, Responder, content::RawHtml},
    };

    use crate::Rendered;

    impl<'r, 'o: 'r, R: Responder<'r, 'o>> Responder<'r, 'o> for Rendered<R> {
        #[inline]
        fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
            RawHtml(self.0).respond_to(req)
        }
    }
}

#[cfg(feature = "salvo")]
mod salvo {
    use salvo_core::{Response, Scribe, writing::Text};

    use crate::Rendered;

    impl<T> Scribe for Rendered<T>
    where
        Text<T>: Scribe,
    {
        #[inline]
        fn render(self, res: &mut Response) {
            Text::Html(self.0).render(res);
        }
    }
}

#[cfg(feature = "tide")]
mod tide {
    use tide::{Body, Response, http::mime};

    use crate::Rendered;

    impl<T: Into<Body>> From<Rendered<T>> for Response {
        #[inline]
        fn from(rendered: Rendered<T>) -> Self {
            let body = rendered.0.into();
            let mut resp = Self::from(body);
            resp.set_content_type(mime::HTML);
            resp
        }
    }
}

#[cfg(feature = "warp")]
mod warp {
    use hyper::Body;
    use warp::reply::{Reply, Response};

    use crate::Rendered;

    impl<T: Send> Reply for Rendered<T>
    where
        Body: From<T>,
    {
        #[inline]
        fn into_response(self) -> Response {
            warp::reply::html(self.0).into_response()
        }
    }
}
