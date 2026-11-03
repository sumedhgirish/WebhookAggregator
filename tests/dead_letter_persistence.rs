use axum::http::StatusCode;
use axum::response::IntoResponse;
use reqwest::StatusCode as ReqStatusCode;
use wagg::db::query::QueryErr;
use wagg::dispatcher::{calculate_retry_delay, Attempt, BASE_DELAY, MAX_ATTEMPTS, MAX_DELAY};

/// [WACDP-11] Dead-Letter Queue Persistence & Error Handling
/// Tests poison pill isolation, terminal failure classification,
/// retry limit exhaustion, and HTTP status mapping.

#[test]
fn test_poison_pill_client_error_isolation() {
    // 4xx errors should never be retried; they must transition directly to GiveUp / DLQ
    let client_errors = [
        ReqStatusCode::BAD_REQUEST,
        ReqStatusCode::UNAUTHORIZED,
        ReqStatusCode::FORBIDDEN,
        ReqStatusCode::NOT_FOUND,
        ReqStatusCode::UNPROCESSABLE_ENTITY,
    ];

    for status in client_errors {
        let attempt = Attempt::classify_status(status);
        match attempt {
            Attempt::GiveUp(reason) => {
                assert!(
                    reason.contains(&status.as_u16().to_string()),
                    "GiveUp reason must include status code {status}"
                );
            }
            _ => panic!("Expected status {status} to yield Attempt::GiveUp, got {attempt:?}"),
        }
    }
}

#[test]
fn test_transient_error_retry_classification() {
    // 5xx and 429 errors must trigger retry
    let retryable_errors = [
        ReqStatusCode::TOO_MANY_REQUESTS,
        ReqStatusCode::INTERNAL_SERVER_ERROR,
        ReqStatusCode::BAD_GATEWAY,
        ReqStatusCode::SERVICE_UNAVAILABLE,
        ReqStatusCode::GATEWAY_TIMEOUT,
    ];

    for status in retryable_errors {
        let attempt = Attempt::classify_status(status);
        match attempt {
            Attempt::Retry(reason) => {
                assert!(
                    reason.contains(&status.as_u16().to_string()),
                    "Retry reason must include status code {status}"
                );
            }
            _ => panic!("Expected status {status} to yield Attempt::Retry, got {attempt:?}"),
        }
    }
}

#[test]
fn test_retry_exhaustion_boundary() {
    // Assert maximum attempt constant is strictly bounded
    assert_eq!(MAX_ATTEMPTS, 5, "System must attempt at most 5 total tries before DLQ");

    // Simulate attempts 1 to 5
    for attempt in 1..=MAX_ATTEMPTS {
        let delay = calculate_retry_delay(attempt);
        assert!(delay >= BASE_DELAY, "Delay must not be less than BASE_DELAY");
        assert!(delay <= MAX_DELAY, "Delay must not exceed MAX_DELAY");
    }
}

#[test]
fn test_query_err_http_status_mapping() {
    // QueryErr::BadRequest -> 400
    let resp = QueryErr::BadRequest.into_response();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    // QueryErr::UnknownClient -> 401
    let resp = QueryErr::UnknownClient("client_alpha".into()).into_response();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // QueryErr::UnauthorizedRequest -> 401
    let resp = QueryErr::UnauthorizedRequest.into_response();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // QueryErr::UnknownService -> 400
    let resp = QueryErr::UnknownService("route_omega".into()).into_response();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    // QueryErr::ServiceUnavailable -> 503
    let resp = QueryErr::ServiceUnavailable("service_down".into()).into_response();
    assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
}

