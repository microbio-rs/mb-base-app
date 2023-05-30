// Copyright (c) 2023 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use std::future::Future;
use std::net::SocketAddr;

use axum::{routing::get, Json, Router};
use serde::Deserialize;
use tower_http::trace::TraceLayer;

#[derive(Debug, Deserialize)]
pub struct RestSettings {
    /// Rest host
    pub host: String,

    /// Rest port
    pub port: u16,
}

impl RestSettings {
    /// Return `SocketAddr`
    pub fn addr(&self) -> SocketAddr {
        let addr = format!("{}:{}", &self.host, self.port);
        addr.parse().expect("Unable to parse socket address")
    }
}

impl Default for RestSettings {
    fn default() -> Self {
        Self { host: "0.0.0.0".to_string(), port: 3000 }
    }
}

pub async fn run_server<F>(settings: &RestSettings, sig: F)
where
    F: Future<Output = ()>,
{
    let addr = settings.addr();
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .with_graceful_shutdown(sig)
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new()
        .route(
            "/health",
            get(|| async { Json(serde_json::json!({ "status": "ok" })) }),
        )
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use serde_json::{json, Value};
    use tower::ServiceExt;

    #[test]
    fn rest_setting_custom() {
        let setting = RestSettings { port: 3001, ..Default::default() };
        assert_eq!(&setting.host, "0.0.0.0");
        assert_eq!(setting.port, 3001);
        assert_eq!(setting.addr(), SocketAddr::from(([0, 0, 0, 0], 3001)));
    }

    #[test]
    fn rest_setting_default() {
        let setting = RestSettings::default();
        assert_eq!(&setting.host, "0.0.0.0");
        assert_eq!(setting.port, 3000);
        assert_eq!(setting.addr(), SocketAddr::from(([0, 0, 0, 0], 3000)));
    }

    #[tokio::test]
    async fn health() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/health")
                    .header(
                        http::header::CONTENT_TYPE,
                        mime::APPLICATION_JSON.as_ref(),
                    )
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "status": "ok" }));
    }

    #[tokio::test]
    async fn not_found() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/does-not-exist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(body.is_empty());
    }
}
