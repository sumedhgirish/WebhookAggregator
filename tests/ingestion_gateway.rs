use chrono::{DateTime, Utc};
use ed25519_dalek::{Signer, SigningKey};
use mysql_async::Opts;
use wagg::db::config::DatabaseConfig;
use wagg::db::query::{qhash, verify_signature, Query, QueryErr};
use wagg::listener::request::Request;

/// Ingestion Gateway & Security Suite
/// Tests input parsing, Ed25519 digital signature verification,
/// tamper detection, and database configuration.

#[test]
fn test_request_to_query_conversion_valid() {
    let dummy_sig = "a".repeat(128); // 64 bytes hex
    let dummy_nonce = "b".repeat(32); // 16 bytes hex

    let req = Request {
        client_name: "test_client".to_string(),
        nonce: dummy_nonce,
        id: dummy_sig,
        datetime: 1730000000,
        payload: r#"{"event":"test"}"#.to_string(),
    };

    let query_res = Query::try_from(req);
    assert!(query_res.is_ok(), "Valid request should convert to Query successfully");

    let query = query_res.unwrap();
    assert_eq!(query.client_name, "test_client");
    assert_eq!(query.event_id, [0xbb; 16]);
    assert_eq!(query.signature, [0xaa; 64]);
    assert_eq!(query.datetime.timestamp(), 1730000000);
}

#[test]
fn test_request_invalid_signature_hex_rejected() {
    let req = Request {
        client_name: "test_client".to_string(),
        nonce: "b".repeat(32),
        id: "not_a_valid_hex_string!".to_string(),
        datetime: 1730000000,
        payload: "{}".to_string(),
    };

    let res = Query::try_from(req);
    assert!(matches!(res, Err(QueryErr::BadRequest)));
}

#[test]
fn test_request_wrong_signature_length_rejected() {
    let req = Request {
        client_name: "test_client".to_string(),
        nonce: "b".repeat(32),
        id: "a".repeat(64), // only 32 bytes instead of 64 bytes
        datetime: 1730000000,
        payload: "{}".to_string(),
    };

    let res = Query::try_from(req);
    assert!(matches!(res, Err(QueryErr::BadRequest)));
}

#[test]
fn test_request_wrong_nonce_length_rejected() {
    let req = Request {
        client_name: "test_client".to_string(),
        nonce: "b".repeat(16), // only 8 bytes instead of 16 bytes
        id: "a".repeat(128),
        datetime: 1730000000,
        payload: "{}".to_string(),
    };

    let res = Query::try_from(req);
    assert!(matches!(res, Err(QueryErr::BadRequest)));
}

#[test]
fn test_ed25519_signature_verification_success() {
    // Generate deterministic 32-byte seed for test keypair
    let seed: [u8; 32] = [7u8; 32];
    let signing_key = SigningKey::from_bytes(&seed);
    let verifying_key = signing_key.verifying_key();
    let pub_key_bytes: [u8; 32] = verifying_key.to_bytes();

    let mut query = Query {
        signature: [0u8; 64],
        event_id: [1u8; 16],
        client_name: "acme_corp".to_string(),
        datetime: DateTime::<Utc>::from_timestamp(1730640000, 0).unwrap(),
        payload: r#"{"status":"active","ping":123}"#.to_string(),
    };

    // Digest and sign
    let digest = qhash(&query);
    let signature = signing_key.sign(&digest);
    query.signature = signature.to_bytes();

    // Verify signature succeeds
    let verify_res = verify_signature(&query, &pub_key_bytes);
    assert!(verify_res.is_ok(), "Valid Ed25519 signature must pass verification");
}

#[test]
fn test_ed25519_tampered_payload_rejected() {
    let seed: [u8; 32] = [42u8; 32];
    let signing_key = SigningKey::from_bytes(&seed);
    let pub_key_bytes: [u8; 32] = signing_key.verifying_key().to_bytes();

    let mut query = Query {
        signature: [0u8; 64],
        event_id: [2u8; 16],
        client_name: "secure_client".to_string(),
        datetime: DateTime::<Utc>::from_timestamp(1730640000, 0).unwrap(),
        payload: r#"{"amount":100}"#.to_string(),
    };

    let digest = qhash(&query);
    query.signature = signing_key.sign(&digest).to_bytes();

    // Tamper with payload
    query.payload = r#"{"amount":999999}"#.to_string();

    let verify_res = verify_signature(&query, &pub_key_bytes);
    assert!(
        matches!(verify_res, Err(QueryErr::UnauthorizedRequest)),
        "Tampered payload must be rejected as UnauthorizedRequest"
    );
}

#[test]
fn test_ed25519_tampered_timestamp_rejected() {
    let seed: [u8; 32] = [42u8; 32];
    let signing_key = SigningKey::from_bytes(&seed);
    let pub_key_bytes: [u8; 32] = signing_key.verifying_key().to_bytes();

    let mut query = Query {
        signature: [0u8; 64],
        event_id: [2u8; 16],
        client_name: "secure_client".to_string(),
        datetime: DateTime::<Utc>::from_timestamp(1730640000, 0).unwrap(),
        payload: r#"{"event":"login"}"#.to_string(),
    };

    let digest = qhash(&query);
    query.signature = signing_key.sign(&digest).to_bytes();

    // Tamper with timestamp
    query.datetime = DateTime::<Utc>::from_timestamp(1730640001, 0).unwrap();

    let verify_res = verify_signature(&query, &pub_key_bytes);
    assert!(matches!(verify_res, Err(QueryErr::UnauthorizedRequest)));
}

#[test]
fn test_database_config_defaults() {
    let cfg = DatabaseConfig::default();
    assert_eq!(cfg.host, "localhost");
    assert_eq!(cfg.port, 3306);
    assert_eq!(cfg.user, "root");
    assert_eq!(cfg.password, "password");
    assert_eq!(cfg.dbname, "webhookdb");

    // Test opts builder without database
    let opts_no_db: Opts = cfg.to_opts(false).into();
    assert_eq!(opts_no_db.ip_or_hostname(), "localhost");
    assert_eq!(opts_no_db.tcp_port(), 3306);
    assert_eq!(opts_no_db.db_name(), None);

    // Test opts builder with database
    let opts_with_db: Opts = cfg.to_opts(true).into();
    assert_eq!(opts_with_db.db_name(), Some("webhookdb"));
}
