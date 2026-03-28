use std::io::{Error as IoError, ErrorKind};
use std::sync::Arc;

use lolzteam::client::{MultipartFile, RequestBody};
use lolzteam::forum::types::ForumUsersAvatarUploadParams;
use lolzteam::{ApiClient, Error, LolzteamClient};
use serde::Deserialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
struct MockResponse {
    status_line: &'static str,
    headers: Vec<(String, String)>,
    body: String,
}

impl MockResponse {
    fn json(status_line: &'static str, body: impl Into<String>) -> Self {
        Self {
            status_line,
            headers: vec![("Content-Type".into(), "application/json".into())],
            body: body.into(),
        }
    }

    fn with_headers(
        status_line: &'static str,
        body: impl Into<String>,
        headers: Vec<(String, String)>,
    ) -> Self {
        Self {
            status_line,
            headers,
            body: body.into(),
        }
    }
}

async fn spawn_server(
    responses: Vec<MockResponse>,
) -> (String, Arc<Mutex<Vec<Vec<u8>>>>, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let captured = Arc::new(Mutex::new(Vec::new()));
    let captured_clone = Arc::clone(&captured);

    let handle = tokio::spawn(async move {
        for response in responses {
            let (mut socket, _) = listener.accept().await.unwrap();
            let request = read_http_request(&mut socket).await.unwrap();
            captured_clone.lock().await.push(request);

            let body = response.body.into_bytes();
            let mut raw = format!("HTTP/1.1 {}\r\n", response.status_line).into_bytes();
            let mut has_content_length = false;
            for (name, value) in response.headers {
                if name.eq_ignore_ascii_case("Content-Length") {
                    has_content_length = true;
                }
                raw.extend_from_slice(format!("{}: {}\r\n", name, value).as_bytes());
            }
            if !has_content_length {
                raw.extend_from_slice(format!("Content-Length: {}\r\n", body.len()).as_bytes());
            }
            raw.extend_from_slice(b"Connection: close\r\n\r\n");
            raw.extend_from_slice(&body);
            socket.write_all(&raw).await.unwrap();
            socket.shutdown().await.unwrap();
        }
    });

    (format!("http://{}", addr), captured, handle)
}

async fn read_http_request(stream: &mut TcpStream) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let header_end = loop {
        let mut chunk = [0_u8; 4096];
        let read = stream.read(&mut chunk).await?;
        if read == 0 {
            return Err(IoError::new(
                ErrorKind::UnexpectedEof,
                "connection closed before request headers",
            ));
        }
        buf.extend_from_slice(&chunk[..read]);
        if let Some(pos) = find_subsequence(&buf, b"\r\n\r\n") {
            break pos + 4;
        }
    };

    let content_length = String::from_utf8_lossy(&buf[..header_end])
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            if name.eq_ignore_ascii_case("Content-Length") {
                value.trim().parse::<usize>().ok()
            } else {
                None
            }
        })
        .unwrap_or(0);

    let already_read = buf.len().saturating_sub(header_end);
    if already_read < content_length {
        let mut remaining = vec![0_u8; content_length - already_read];
        stream.read_exact(&mut remaining).await?;
        buf.extend_from_slice(&remaining);
    }

    Ok(buf)
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

#[derive(Debug, Deserialize)]
struct SimpleResponse {
    ok: bool,
}

#[tokio::test]
async fn retries_429_with_retry_after_then_succeeds() {
    let (base_url, captured, handle) = spawn_server(vec![
        MockResponse::with_headers(
            "429 Too Many Requests",
            r#"{"error":{"message":"slow down"}}"#,
            vec![("Retry-After".into(), "0".into())],
        ),
        MockResponse::json("200 OK", r#"{"ok":true}"#),
    ])
    .await;

    let client = ApiClient::builder(base_url.as_str(), "")
        .max_retries(1)
        .build()
        .unwrap();

    let response: SimpleResponse = client
        .request("get", "/ping", None::<&()>, None::<RequestBody>)
        .await
        .unwrap();

    assert!(response.ok);
    handle.await.unwrap();

    let requests = captured.lock().await;
    assert_eq!(requests.len(), 2);
    assert!(String::from_utf8_lossy(&requests[0]).starts_with("GET /ping HTTP/1.1"));
    assert!(String::from_utf8_lossy(&requests[1]).starts_with("GET /ping HTTP/1.1"));
}

#[tokio::test]
async fn retries_504_then_succeeds() {
    let (base_url, captured, handle) = spawn_server(vec![
        MockResponse::json("504 Gateway Timeout", r#"{"error":"upstream timeout"}"#),
        MockResponse::json("200 OK", r#"{"ok":true}"#),
    ])
    .await;

    let client = ApiClient::builder(base_url.as_str(), "")
        .max_retries(1)
        .build()
        .unwrap();

    let response: SimpleResponse = client
        .request("get", "/unstable", None::<&()>, None::<RequestBody>)
        .await
        .unwrap();

    assert!(response.ok);
    handle.await.unwrap();

    let requests = captured.lock().await;
    assert_eq!(requests.len(), 2);
}

#[tokio::test]
async fn exhausted_503_returns_api_503_not_rate_limited() {
    let (base_url, captured, handle) = spawn_server(vec![
        MockResponse::json("503 Service Unavailable", r#"{"error":"busy"}"#),
        MockResponse::json("503 Service Unavailable", r#"{"error":"still busy"}"#),
    ])
    .await;

    let client = ApiClient::builder(base_url.as_str(), "")
        .max_retries(1)
        .build()
        .unwrap();

    let error = client
        .request::<(), SimpleResponse>("get", "/always-503", None::<&()>, None::<RequestBody>)
        .await
        .unwrap_err();

    match error {
        Error::Api { status, body } => {
            assert_eq!(status, 503);
            assert!(body.contains("still busy"));
        }
        other => panic!("expected Error::Api(503), got {other:?}"),
    }

    handle.await.unwrap();
    assert_eq!(captured.lock().await.len(), 2);
}

#[tokio::test]
async fn generated_avatar_upload_uses_retry_safe_multipart() {
    let (base_url, captured, handle) = spawn_server(vec![
        MockResponse::json("503 Service Unavailable", r#"{"error":"retry me"}"#),
        MockResponse::json(
            "200 OK",
            r#"{"message":"uploaded","status":"ok","system_info":{}}"#,
        ),
    ])
    .await;

    let client = LolzteamClient::builder("")
        .forum_base_url(base_url.as_str())
        .market_base_url(base_url.as_str())
        .max_retries(1)
        .build()
        .unwrap();

    let response = client
        .forum()
        .users_avatar_upload(
            7,
            ForumUsersAvatarUploadParams {
                avatar: MultipartFile::new(b"avatar-bytes".to_vec()).with_filename("avatar.png"),
                crop: Some(128),
                x: Some(10),
                y: Some(20),
            },
        )
        .await
        .unwrap();

    assert_eq!(response.status, "ok");
    handle.await.unwrap();

    let requests = captured.lock().await;
    assert_eq!(requests.len(), 2);

    for raw in requests.iter() {
        let request = String::from_utf8_lossy(raw);
        assert!(request.starts_with("POST /users/7/avatar HTTP/1.1"));
        assert!(request.contains("multipart/form-data; boundary="));
        assert!(request.contains("name=\"avatar\""));
        assert!(request.contains("filename=\"avatar.png\""));
        assert!(request.contains("avatar-bytes"));
        assert!(request.contains("name=\"crop\""));
        assert!(request.contains("name=\"x\""));
        assert!(request.contains("name=\"y\""));
    }
}
