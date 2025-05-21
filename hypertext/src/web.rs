#[cfg(feature = "actix-web")]
mod actix_web {
    use actix_web::{HttpRequest, HttpResponse, Responder, web::Html};

    use crate::{Lazy, Renderable, Rendered, proc_macros::String};

    impl<F: Fn(&mut String)> Responder for Lazy<F> {
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
    use axum_core::{
        body::Body,
        response::{IntoResponse, Response},
    };
    use http::{HeaderName, HeaderValue, header};

    use crate::{Lazy, Renderable, Rendered, proc_macros::String};

    const HEADER: (HeaderName, HeaderValue) = (
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );

    impl<F: Fn(&mut String)> IntoResponse for Lazy<F> {
        #[inline]
        fn into_response(self) -> Response {
            self.render().into_response()
        }
    }

    impl<T: Into<Body>> IntoResponse for Rendered<T> {
        #[inline]
        fn into_response(self) -> Response {
            ([HEADER], self.0.into()).into_response()
        }
    }
}

#[cfg(feature = "poem")]
mod poem {
    use core::marker::Send;

    use poem::{IntoResponse, Response, web::Html};

    use crate::{Lazy, Renderable, Rendered, proc_macros::String};

    impl<F: Fn(&mut String) + Send> IntoResponse for Lazy<F> {
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
        response::{self, Responder, content::RawHtml},
    };

    use crate::{Lazy, Renderable, Rendered, proc_macros::String};

    impl<'r, 'o: 'r, F: Fn(&mut String) + Send> Responder<'r, 'o> for Lazy<F> {
        #[inline]
        fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
            self.render().respond_to(req)
        }
    }

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

    use crate::{Lazy, Renderable, Rendered, proc_macros::String};

    impl<F: Fn(&mut String)> Scribe for Lazy<F> {
        #[inline]
        fn render(self, res: &mut Response) {
            Renderable::render(&self).render(res);
        }
    }

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

    use crate::{Lazy, Renderable, Rendered, proc_macros::String};

    impl<F: Fn(&mut String)> From<Lazy<F>> for Response {
        #[inline]
        fn from(lazy: Lazy<F>) -> Self {
            lazy.render().into()
        }
    }

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

    use crate::{Lazy, Renderable, Rendered, proc_macros::String};

    impl<F: Fn(&mut String) + Send> Reply for Lazy<F> {
        #[inline]
        fn into_response(self) -> Response {
            self.render().into_response()
        }
    }

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

#[cfg(feature = "http")]
mod http {
    use http::Uri;

    use crate::{AttributeRenderable, DisplayExt, Renderable, proc_macros::String};

    impl Renderable for Uri {
        #[inline]
        fn render_to(&self, output: &mut String) {
            self.renderable().render_to(output);
        }
    }

    impl AttributeRenderable for Uri {
        #[inline]
        fn render_attribute_to(&self, output: &mut String) {
            self.renderable().render_attribute_to(output);
        }
    }
}
