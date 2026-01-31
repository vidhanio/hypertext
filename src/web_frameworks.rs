#[allow(dead_code)]
const HTML_CONTENT_TYPE: &str = "text/html; charset=utf-8";

#[cfg(feature = "actix-web")]
mod actix_web {
    use actix_web::{HttpRequest, HttpResponse, Responder, web::Html};

    use crate::{Buffer, Lazy, alloc::string::String, prelude::*};

    impl<F: Fn(&mut Buffer)> Responder for Lazy<F> {
        type Body = <Rendered<String> as Responder>::Body;

        #[inline]
        fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
            self.render().respond_to(req)
        }
    }

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
    use axum_core::response::{IntoResponse, Response};

    use crate::{Buffer, Lazy, alloc::string::String, prelude::*};

    const CONTENT_TYPE_HEADER: &str = "content-type";

    impl<F: Fn(&mut Buffer)> IntoResponse for Lazy<F> {
        #[inline]
        fn into_response(self) -> Response {
            self.render().into_response()
        }
    }

    impl IntoResponse for Rendered<&'static str> {
        #[inline]
        fn into_response(self) -> Response {
            ([(CONTENT_TYPE_HEADER, super::HTML_CONTENT_TYPE)], self.0).into_response()
        }
    }

    impl IntoResponse for Rendered<String> {
        #[inline]
        fn into_response(self) -> Response {
            ([(CONTENT_TYPE_HEADER, super::HTML_CONTENT_TYPE)], self.0).into_response()
        }
    }
}

#[cfg(feature = "ntex")]
mod ntex {
    #![allow(clippy::future_not_send)]

    use ntex::{
        http::Response,
        web::{ErrorRenderer, HttpRequest, Responder},
    };

    use crate::{Buffer, Lazy, alloc::string::String, prelude::*};

    impl<F: Fn(&mut Buffer), Err: ErrorRenderer> Responder<Err> for Lazy<F> {
        #[inline]
        async fn respond_to(self, req: &HttpRequest) -> Response {
            Responder::<Err>::respond_to(self.render(), req).await
        }
    }

    impl<Err: ErrorRenderer> Responder<Err> for Rendered<&'static str> {
        #[inline]
        async fn respond_to(self, _: &HttpRequest) -> Response {
            Response::Ok()
                .content_type(super::HTML_CONTENT_TYPE)
                .body(self.0)
        }
    }

    impl<Err: ErrorRenderer> Responder<Err> for Rendered<String> {
        #[inline]
        async fn respond_to(self, _: &HttpRequest) -> Response {
            Response::Ok()
                .content_type(super::HTML_CONTENT_TYPE)
                .body(self.0)
        }
    }
}

#[cfg(feature = "poem")]
mod poem {
    use core::marker::Send;

    use poem::{IntoResponse, Response, web::Html};

    use crate::{Buffer, Lazy, alloc::string::String, prelude::*};

    impl<F: Fn(&mut Buffer) + Send> IntoResponse for Lazy<F> {
        #[inline]
        fn into_response(self) -> Response {
            self.render().into_response()
        }
    }

    impl<T: Into<String> + Send> IntoResponse for Rendered<T> {
        #[inline]
        fn into_response(self) -> Response {
            Html(self.0).into_response()
        }
    }
}

#[cfg(feature = "rocket")]
mod rocket {
    use rocket::{
        Request,
        response::{Responder, Result, content::RawHtml},
    };

    use crate::{Buffer, Lazy, alloc::string::String, prelude::*};

    impl<'r, 'o: 'r, F: Fn(&mut Buffer) + Send> Responder<'r, 'o> for Lazy<F> {
        #[inline]
        fn respond_to(self, req: &'r Request<'_>) -> Result<'o> {
            self.render().respond_to(req)
        }
    }

    impl<'r, 'o: 'r> Responder<'r, 'o> for Rendered<&'o str> {
        #[inline]
        fn respond_to(self, req: &'r Request<'_>) -> Result<'o> {
            RawHtml(self.0).respond_to(req)
        }
    }

    impl<'r, 'o: 'r> Responder<'r, 'o> for Rendered<String> {
        #[inline]
        fn respond_to(self, req: &'r Request<'_>) -> Result<'o> {
            RawHtml(self.0).respond_to(req)
        }
    }
}

#[cfg(feature = "salvo")]
mod salvo {
    use salvo_core::{Response, Scribe, writing::Text};

    use crate::{Buffer, Lazy, RenderableExt, alloc::string::String, prelude::*};

    impl<F: Fn(&mut Buffer)> Scribe for Lazy<F> {
        #[inline]
        fn render(self, res: &mut Response) {
            RenderableExt::render(&self).render(res);
        }
    }

    impl Scribe for Rendered<&'static str> {
        #[inline]
        fn render(self, res: &mut Response) {
            Text::Html(self.0).render(res);
        }
    }

    impl Scribe for Rendered<String> {
        #[inline]
        fn render(self, res: &mut Response) {
            Text::Html(self.0).render(res);
        }
    }
}

#[cfg(feature = "tide")]
mod tide {

    use tide::{Response, http::mime};

    use crate::{Buffer, Lazy, alloc::string::String, prelude::*};

    impl<F: Fn(&mut Buffer)> From<Lazy<F>> for Response {
        #[inline]
        fn from(lazy: Lazy<F>) -> Self {
            lazy.render().into()
        }
    }

    impl<T: Into<String>> From<Rendered<T>> for Response {
        #[inline]
        fn from(Rendered(value): Rendered<T>) -> Self {
            let mut resp = Self::from(value.into());
            resp.set_content_type(mime::HTML);
            resp
        }
    }
}

#[cfg(feature = "warp")]
mod warp {
    use warp::reply::{self, Reply, Response};

    use crate::{Buffer, Lazy, alloc::string::String, prelude::*};

    impl<F: Fn(&mut Buffer) + Send> Reply for Lazy<F> {
        #[inline]
        fn into_response(self) -> Response {
            self.render().into_response()
        }
    }

    impl Reply for Rendered<&'static str> {
        #[inline]
        fn into_response(self) -> Response {
            reply::html(self.0).into_response()
        }
    }

    impl Reply for Rendered<String> {
        #[inline]
        fn into_response(self) -> Response {
            reply::html(self.0).into_response()
        }
    }
}
