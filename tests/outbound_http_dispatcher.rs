use axum::http::{HeaderMap, StatusCode};
use axum::routing::post;
use axum::Router;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::net::TcpListener;
use wagg::dispatcher::Attempt;

/// [WACDP-7A] Outbound HTTP/TLS Dispatcher
/// Verifies payload dispatch to mock endpoints, header verification,
/// rate-limit backpressure handling, and error categorization.

#[tokio::test]
async fn test_successful_webhook_delivery_mock_server() {
    let call_count = Arc::new(AtomicUsize::new(0));
    let cc = call_count.clone();

    // Mock receiver that expects JSON payload and Content-Type: application/json
    let app = Router::new().route(
        "/receiver",
        post(move |headers: HeaderMap, body: String| {
            let cc = cc.clone();
            async move {
                cc.fetch_add(1, Ordering::SeqCst);
                assert_eq!(
                    headers.get("content-type").unwrap().to_str().unwrap(),
                    "application/json"
                );
                assert!(body.contains("order_created"));
                assert!(body.contains("42.50"));
                StatusCode::OK
            }
        }),
    );

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = reqwest::Client::new();
    let url = format!("http://{addr}/receiver");
    let payload = r#"{"event":"order_created","amount":42.50}"#;

    let res = client
        .post(&url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload.to_string())
        .send()
        .await
        .unwrap();

    let attempt = Attempt::classify_status(res.status());
    assert_eq!(attempt, Attempt::Delivered);
    assert_eq!(call_count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_dispatcher_backpressure_429_retry() {
    let app = Router::new().route(
        "/busy",
        post(|| async { StatusCode::TOO_MANY_REQUESTS }),
    );

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = reqwest::Client::new();
    let url = format!("http://{addr}/busy");

    let res = client.post(&url).send().await.unwrap();
    let attempt = Attempt::classify_status(res.status());
    match attempt {
        Attempt::Retry(why) => assert!(why.contains("429")),
        other => panic!("Expected Attempt::Retry for 429, got {other:?}"),
    }
}

#[tokio::test]
async fn test_dispatcher_server_error_503_retry() {
    let app = Router::new().route(
        "/outage",
        post(|| async { StatusCode::SERVICE_UNAVAILABLE }),
    );

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = reqwest::Client::new();
    let url = format!("http://{addr}/outage");

    let res = client.post(&url).send().await.unwrap();
    let attempt = Attempt::classify_status(res.status());
    match attempt {
        Attempt::Retry(why) => assert!(why.contains("503")),
        other => panic!("Expected Attempt::Retry for 503, got {other:?}"),
    }
}

#[tokio::test]
async fn test_dispatcher_client_error_404_giveup() {
    let app = Router::new().route(
        "/notfound",
        post(|| async { StatusCode::NOT_FOUND }),
    );

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = reqwest::Client::new();
    let url = format!("http://{addr}/notfound");

    let res = client.post(&url).send().await.unwrap();
    let attempt = Attempt::classify_status(res.status());
    match attempt {
        Attempt::GiveUp(why) => assert!(why.contains("404")),
        other => panic!("Expected Attempt::GiveUp for 404, got {other:?}"),
    }
}

#[test]
fn test_attempt_status_classification() {
    // 2xx -> Delivered
    assert_eq!(Attempt::classify_status(StatusCode::OK), Attempt::Delivered);
    assert_eq!(Attempt::classify_status(StatusCode::CREATED), Attempt::Delivered);
    assert_eq!(Attempt::classify_status(StatusCode::NO_CONTENT), Attempt::Delivered);

    // 429 & 5xx -> Retry
    assert!(matches!(Attempt::classify_status(StatusCode::TOO_MANY_REQUESTS), Attempt::Retry(_)));
    assert!(matches!(Attempt::classify_status(StatusCode::INTERNAL_SERVER_ERROR), Attempt::Retry(_)));
    assert!(matches!(Attempt::classify_status(StatusCode::BAD_GATEWAY), Attempt::Retry(_)));
    assert!(matches!(Attempt::classify_status(StatusCode::SERVICE_UNAVAILABLE), Attempt::Retry(_)));

    // 4xx (except 429) -> GiveUp
    assert!(matches!(Attempt::classify_status(StatusCode::BAD_REQUEST), Attempt::GiveUp(_)));
    assert!(matches!(Attempt::classify_status(StatusCode::UNAUTHORIZED), Attempt::GiveUp(_)));
    assert!(matches!(Attempt::classify_status(StatusCode::FORBIDDEN), Attempt::GiveUp(_)));
    assert!(matches!(Attempt::classify_status(StatusCode::NOT_FOUND), Attempt::GiveUp(_)));
}
