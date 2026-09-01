//! Response adapter tests for supported web frameworks.
#![cfg(all(
    feature = "alloc",
    any(
        feature = "actix-web",
        feature = "axum",
        feature = "ntex",
        feature = "poem",
        feature = "rocket",
        feature = "salvo",
        feature = "warp"
    )
))]

use hypertext::{Buffer, Lazy, Rendered, prelude::*};
#[cfg(any(
    feature = "axum",
    feature = "ntex",
    feature = "poem",
    feature = "salvo",
    feature = "warp"
))]
use hypertext::{RenderedMathMl, RenderedSvg};

#[cfg(any(
    feature = "axum",
    feature = "ntex",
    feature = "poem",
    feature = "salvo",
    feature = "warp"
))]
const HTML: &str = "text/html; charset=utf-8";
#[cfg(any(
    feature = "axum",
    feature = "ntex",
    feature = "poem",
    feature = "salvo",
    feature = "warp"
))]
const SVG: &str = "image/svg+xml";
#[cfg(any(
    feature = "axum",
    feature = "ntex",
    feature = "poem",
    feature = "salvo",
    feature = "warp"
))]
const MATHML: &str = "application/mathml+xml";

#[cfg(any(
    feature = "axum",
    feature = "ntex",
    feature = "poem",
    feature = "salvo",
    feature = "warp"
))]
fn html() -> Rendered<String> {
    hypertext::maud::simple! { p { "adapter" } }.render()
}

#[cfg(any(
    feature = "axum",
    feature = "ntex",
    feature = "poem",
    feature = "salvo",
    feature = "warp"
))]
fn svg() -> RenderedSvg<String> {
    hypertext::svg::maud::simple! {
        svg {
            circle cx="1" cy="1" r="1";
        }
    }
    .render()
}

#[cfg(any(
    feature = "axum",
    feature = "ntex",
    feature = "poem",
    feature = "salvo",
    feature = "warp"
))]
fn mathml() -> RenderedMathMl<String> {
    hypertext::mathml::maud::simple! {
        math {
            mi { "x" }
        }
    }
    .render()
}

#[cfg(any(
    feature = "axum",
    feature = "ntex",
    feature = "poem",
    feature = "salvo",
    feature = "warp"
))]
fn lazy_html() -> Lazy<fn(&mut Buffer)> {
    Lazy::dangerously_create(|buffer| {
        buffer.push(hypertext::maud::simple! { p { "lazy" } });
    })
}

#[cfg(any(
    feature = "axum",
    feature = "ntex",
    feature = "poem",
    feature = "salvo",
    feature = "warp"
))]
macro_rules! assert_content_type {
    ($response:expr, $expected:expr) => {
        assert_eq!(
            $response
                .headers()
                .get("content-type")
                .and_then(|value| value.to_str().ok()),
            Some($expected)
        );
    };
}

#[cfg(feature = "actix-web")]
mod actix_web {
    use ::actix_web::Responder;

    use super::*;

    #[test]
    fn rendered_and_lazy_responders_are_implemented() {
        fn assert_responder<T: Responder>() {}

        assert_responder::<Rendered<&'static str>>();
        assert_responder::<Rendered<String>>();
        assert_responder::<Rendered<String, hypertext::context::Svg>>();
        assert_responder::<Rendered<String, hypertext::context::MathMl>>();
        assert_responder::<Lazy<fn(&mut Buffer)>>();
    }
}

#[cfg(feature = "axum")]
mod axum {
    use axum_core::response::IntoResponse;

    use super::*;

    #[test]
    fn rendered_and_lazy_responses() {
        fn assert_into_response<T: IntoResponse>() {}

        assert_into_response::<Rendered<&'static str>>();
        assert_into_response::<Rendered<String>>();
        assert_into_response::<Rendered<String, hypertext::context::Svg>>();
        assert_into_response::<Rendered<String, hypertext::context::MathMl>>();
        assert_into_response::<Lazy<fn(&mut Buffer)>>();

        let response = html().into_response();
        assert_eq!(response.status().as_u16(), 200);
        assert_content_type!(response, HTML);

        let response = svg().into_response();
        assert_eq!(response.status().as_u16(), 200);
        assert_content_type!(response, SVG);

        let response = mathml().into_response();
        assert_eq!(response.status().as_u16(), 200);
        assert_content_type!(response, MATHML);

        let response = lazy_html().into_response();
        assert_eq!(response.status().as_u16(), 200);
        assert_content_type!(response, HTML);
    }
}

#[cfg(feature = "ntex")]
mod ntex {
    use core::{
        future::Future,
        task::{Context, Poll, Waker},
    };

    use ::ntex::web::{Responder, test::TestRequest};

    use super::*;

    fn poll_ready<F: Future>(future: F) -> F::Output {
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        let mut future = core::pin::pin!(future);

        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => output,
            Poll::Pending => panic!("ntex response adapter future was not ready"),
        }
    }

    #[test]
    fn rendered_and_lazy_responses() {
        fn assert_responder<T: Responder>() {}

        assert_responder::<Rendered<&'static str>>();
        assert_responder::<Rendered<String>>();
        assert_responder::<Rendered<String, hypertext::context::Svg>>();
        assert_responder::<Rendered<String, hypertext::context::MathMl>>();
        assert_responder::<Lazy<fn(&mut Buffer)>>();

        let request = TestRequest::default().to_http_request();
        let response = poll_ready(<Rendered<String> as Responder>::respond_to(
            html(),
            &request,
        ));
        assert_eq!(response.status().as_u16(), 200);
        assert_content_type!(response, HTML);

        let response = poll_ready(
            <Rendered<String, hypertext::context::Svg> as Responder>::respond_to(svg(), &request),
        );
        assert_eq!(response.status().as_u16(), 200);
        assert_content_type!(response, SVG);

        let response = poll_ready(
            <Rendered<String, hypertext::context::MathMl> as Responder>::respond_to(
                mathml(),
                &request,
            ),
        );
        assert_eq!(response.status().as_u16(), 200);
        assert_content_type!(response, MATHML);

        let response = poll_ready(<Lazy<fn(&mut Buffer)> as Responder>::respond_to(
            lazy_html(),
            &request,
        ));
        assert_eq!(response.status().as_u16(), 200);
        assert_content_type!(response, HTML);
    }
}

#[cfg(feature = "poem")]
mod poem {
    use ::poem::IntoResponse;

    use super::*;

    #[test]
    fn rendered_and_lazy_responses() {
        fn assert_into_response<T: IntoResponse>() {}

        assert_into_response::<Rendered<&'static str>>();
        assert_into_response::<Rendered<String>>();
        assert_into_response::<Rendered<String, hypertext::context::Svg>>();
        assert_into_response::<Rendered<String, hypertext::context::MathMl>>();
        assert_into_response::<Lazy<fn(&mut Buffer)>>();

        let response = html().into_response();
        assert!(response.is_ok());
        assert_content_type!(response, HTML);

        let response = svg().into_response();
        assert!(response.is_ok());
        assert_content_type!(response, SVG);

        let response = mathml().into_response();
        assert!(response.is_ok());
        assert_content_type!(response, MATHML);

        let response = lazy_html().into_response();
        assert!(response.is_ok());
        assert_content_type!(response, HTML);
    }
}

#[cfg(feature = "rocket")]
mod rocket {
    use ::rocket::response::Responder;

    use super::*;

    #[test]
    fn rendered_and_lazy_responders_are_implemented() {
        fn assert_responder<T>()
        where
            for<'r> T: Responder<'r, 'static>,
        {
        }

        assert_responder::<Rendered<&'static str>>();
        assert_responder::<Rendered<String>>();
        assert_responder::<Rendered<String, hypertext::context::Svg>>();
        assert_responder::<Rendered<String, hypertext::context::MathMl>>();
        assert_responder::<Lazy<fn(&mut Buffer)>>();
    }
}

#[cfg(feature = "salvo")]
mod salvo {
    use salvo_core::Scribe;

    use super::*;

    #[test]
    fn rendered_and_lazy_responses() {
        fn assert_scribe<T: Scribe>() {}

        assert_scribe::<Rendered<&'static str>>();
        assert_scribe::<Rendered<String>>();
        assert_scribe::<Rendered<String, hypertext::context::Svg>>();
        assert_scribe::<Rendered<String, hypertext::context::MathMl>>();
        assert_scribe::<Lazy<fn(&mut Buffer)>>();

        let mut response = salvo_core::Response::default();
        Scribe::render(html(), &mut response);
        assert_content_type!(response, HTML);

        let mut response = salvo_core::Response::default();
        Scribe::render(svg(), &mut response);
        assert_content_type!(response, SVG);

        let mut response = salvo_core::Response::default();
        Scribe::render(mathml(), &mut response);
        assert_content_type!(response, MATHML);

        let mut response = salvo_core::Response::default();
        Scribe::render(lazy_html(), &mut response);
        assert_content_type!(response, HTML);
    }
}

#[cfg(feature = "warp")]
mod warp {
    use ::warp::reply::Reply;

    use super::*;

    #[test]
    fn rendered_and_lazy_responses() {
        fn assert_reply<T: Reply>() {}

        assert_reply::<Rendered<&'static str>>();
        assert_reply::<Rendered<String>>();
        assert_reply::<Rendered<String, hypertext::context::Svg>>();
        assert_reply::<Rendered<String, hypertext::context::MathMl>>();
        assert_reply::<Lazy<fn(&mut Buffer)>>();

        let response = html().into_response();
        assert_eq!(response.status().as_u16(), 200);
        assert_content_type!(response, HTML);

        let response = svg().into_response();
        assert_eq!(response.status().as_u16(), 200);
        assert_content_type!(response, SVG);

        let response = mathml().into_response();
        assert_eq!(response.status().as_u16(), 200);
        assert_content_type!(response, MATHML);

        let response = lazy_html().into_response();
        assert_eq!(response.status().as_u16(), 200);
        assert_content_type!(response, HTML);
    }
}
