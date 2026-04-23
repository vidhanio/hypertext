use crate::context::{Html, MathMl, NodeKind, Svg};

#[allow(dead_code)]
const HTML_CONTENT_TYPE: &str = "text/html; charset=utf-8";
#[allow(dead_code)]
const SVG_CONTENT_TYPE: &str = "image/svg+xml";
#[allow(dead_code)]
const MATHML_CONTENT_TYPE: &str = "application/mathml+xml";

#[allow(dead_code)]
trait ResponseMarkup: NodeKind {
    const CONTENT_TYPE: &'static str;
}

impl ResponseMarkup for Html {
    const CONTENT_TYPE: &'static str = HTML_CONTENT_TYPE;
}

impl ResponseMarkup for Svg {
    const CONTENT_TYPE: &'static str = SVG_CONTENT_TYPE;
}

impl ResponseMarkup for MathMl {
    const CONTENT_TYPE: &'static str = MATHML_CONTENT_TYPE;
}

#[cfg(feature = "actix-web")]
mod actix_web {
    use actix_web::{HttpRequest, HttpResponse, Responder};

    use crate::{Buffer, Lazy, Rendered, alloc::string::String, context::Node, prelude::*};

    impl<F: Fn(&mut Buffer<Node<K>>), K: super::ResponseMarkup> Responder for Lazy<F, Node<K>> {
        type Body = <Rendered<String, K> as Responder>::Body;

        #[inline]
        fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
            self.render().respond_to(req)
        }
    }

    impl<T: Into<String>, K: super::ResponseMarkup> Responder for Rendered<T, K> {
        type Body = actix_web::body::BoxBody;

        #[inline]
        fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
            HttpResponse::Ok()
                .content_type(K::CONTENT_TYPE)
                .body(self.into_inner().into())
        }
    }
}

#[cfg(feature = "axum")]
mod axum {
    use axum_core::response::{IntoResponse, Response};

    use crate::{Buffer, Lazy, Rendered, alloc::string::String, context::Node, prelude::*};

    const CONTENT_TYPE_HEADER: &str = "content-type";

    impl<F: Fn(&mut Buffer<Node<K>>), K: super::ResponseMarkup> IntoResponse for Lazy<F, Node<K>> {
        #[inline]
        fn into_response(self) -> Response {
            self.render().into_response()
        }
    }

    impl<K: super::ResponseMarkup> IntoResponse for Rendered<&'static str, K> {
        #[inline]
        fn into_response(self) -> Response {
            ([(CONTENT_TYPE_HEADER, K::CONTENT_TYPE)], self.into_inner()).into_response()
        }
    }

    impl<K: super::ResponseMarkup> IntoResponse for Rendered<String, K> {
        #[inline]
        fn into_response(self) -> Response {
            ([(CONTENT_TYPE_HEADER, K::CONTENT_TYPE)], self.into_inner()).into_response()
        }
    }
}

#[cfg(feature = "ntex")]
mod ntex {
    #![expect(clippy::future_not_send)]

    use ntex::{
        http::Response,
        web::{ErrorRenderer, HttpRequest, Responder},
    };

    use crate::{Buffer, Lazy, Rendered, alloc::string::String, context::Node, prelude::*};

    impl<F: Fn(&mut Buffer<Node<K>>), Err: ErrorRenderer, K: super::ResponseMarkup> Responder<Err>
        for Lazy<F, Node<K>>
    {
        #[inline]
        async fn respond_to(self, req: &HttpRequest) -> Response {
            Responder::<Err>::respond_to(self.render(), req).await
        }
    }

    impl<Err: ErrorRenderer, K: super::ResponseMarkup> Responder<Err> for Rendered<&'static str, K> {
        #[inline]
        async fn respond_to(self, _: &HttpRequest) -> Response {
            Response::Ok()
                .content_type(K::CONTENT_TYPE)
                .body(self.into_inner())
        }
    }

    impl<Err: ErrorRenderer, K: super::ResponseMarkup> Responder<Err> for Rendered<String, K> {
        #[inline]
        async fn respond_to(self, _: &HttpRequest) -> Response {
            Response::Ok()
                .content_type(K::CONTENT_TYPE)
                .body(self.into_inner())
        }
    }
}

#[cfg(feature = "poem")]
mod poem {
    use core::marker::Send;

    use poem::{IntoResponse, Response};

    use crate::{Buffer, Lazy, Rendered, alloc::string::String, context::Node, prelude::*};

    impl<F: Fn(&mut Buffer<Node<K>>) + Send, K: super::ResponseMarkup + Send> IntoResponse
        for Lazy<F, Node<K>>
    {
        #[inline]
        fn into_response(self) -> Response {
            self.render().into_response()
        }
    }

    impl<T: Into<String> + Send, K: super::ResponseMarkup + Send> IntoResponse for Rendered<T, K> {
        #[inline]
        fn into_response(self) -> Response {
            Response::builder()
                .content_type(K::CONTENT_TYPE)
                .body(self.into_inner().into())
        }
    }
}

#[cfg(feature = "rocket")]
mod rocket {
    use rocket::{
        Request,
        response::{Responder, Result},
    };

    use crate::{Buffer, Lazy, Rendered, alloc::string::String, context::Node, prelude::*};

    fn content_type<K: super::ResponseMarkup>() -> rocket::http::ContentType {
        match K::CONTENT_TYPE {
            super::HTML_CONTENT_TYPE => rocket::http::ContentType::HTML,
            super::SVG_CONTENT_TYPE => rocket::http::ContentType::new("image", "svg+xml"),
            super::MATHML_CONTENT_TYPE => {
                rocket::http::ContentType::new("application", "mathml+xml")
            }
            _ => rocket::http::ContentType::Binary,
        }
    }

    impl<'r, 'o: 'r, F: Fn(&mut Buffer<Node<K>>) + Send, K: super::ResponseMarkup> Responder<'r, 'o>
        for Lazy<F, Node<K>>
    {
        #[inline]
        fn respond_to(self, req: &'r Request<'_>) -> Result<'o> {
            self.render().respond_to(req)
        }
    }

    impl<'r, 'o: 'r, K: super::ResponseMarkup> Responder<'r, 'o> for Rendered<&'o str, K> {
        #[inline]
        fn respond_to(self, req: &'r Request<'_>) -> Result<'o> {
            (content_type::<K>(), self.into_inner()).respond_to(req)
        }
    }

    impl<'r, K: super::ResponseMarkup> Responder<'r, 'static> for Rendered<String, K> {
        #[inline]
        fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
            (content_type::<K>(), self.into_inner()).respond_to(req)
        }
    }
}

#[cfg(feature = "salvo")]
mod salvo {
    use salvo_core::{Response, Scribe, http::HeaderValue};

    use crate::{Buffer, Lazy, RenderableExt, Rendered, alloc::string::String, context::Node};

    impl<F: Fn(&mut Buffer<Node<K>>), K: super::ResponseMarkup> Scribe for Lazy<F, Node<K>> {
        #[inline]
        fn render(self, res: &mut Response) {
            RenderableExt::render(&self).render(res);
        }
    }

    impl<K: super::ResponseMarkup> Scribe for Rendered<&'static str, K> {
        #[inline]
        fn render(self, res: &mut Response) {
            if let Ok(content_type) = HeaderValue::from_str(K::CONTENT_TYPE) {
                res.headers_mut().insert("content-type", content_type);
            }
            res.body(self.into_inner());
        }
    }

    impl<K: super::ResponseMarkup> Scribe for Rendered<String, K> {
        #[inline]
        fn render(self, res: &mut Response) {
            if let Ok(content_type) = HeaderValue::from_str(K::CONTENT_TYPE) {
                res.headers_mut().insert("content-type", content_type);
            }
            res.body(self.into_inner());
        }
    }
}

#[cfg(feature = "tide")]
mod tide {
    use tide::Response;

    use crate::{Buffer, Lazy, Rendered, alloc::string::String, context::Node, prelude::*};

    impl<F: Fn(&mut Buffer<Node<K>>), K: super::ResponseMarkup> From<Lazy<F, Node<K>>> for Response {
        #[inline]
        fn from(lazy: Lazy<F, Node<K>>) -> Self {
            lazy.render().into()
        }
    }

    impl<T: Into<String>, K: super::ResponseMarkup> From<Rendered<T, K>> for Response {
        #[inline]
        fn from(rendered: Rendered<T, K>) -> Self {
            let mut resp = Self::from(rendered.into_inner().into());
            resp.set_content_type(K::CONTENT_TYPE);
            resp
        }
    }
}

#[cfg(feature = "warp")]
mod warp {
    use warp::reply::{self, Reply, Response};

    use crate::{Buffer, Lazy, Rendered, alloc::string::String, context::Node, prelude::*};

    impl<F: Fn(&mut Buffer<Node<K>>) + Send, K: super::ResponseMarkup + Send> Reply
        for Lazy<F, Node<K>>
    {
        #[inline]
        fn into_response(self) -> Response {
            self.render().into_response()
        }
    }

    impl<K: super::ResponseMarkup + Send> Reply for Rendered<&'static str, K> {
        #[inline]
        fn into_response(self) -> Response {
            reply::with_header(self.into_inner(), "content-type", K::CONTENT_TYPE).into_response()
        }
    }

    impl<K: super::ResponseMarkup + Send> Reply for Rendered<String, K> {
        #[inline]
        fn into_response(self) -> Response {
            reply::with_header(self.into_inner(), "content-type", K::CONTENT_TYPE).into_response()
        }
    }
}
