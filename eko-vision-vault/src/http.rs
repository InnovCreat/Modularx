use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct Request {
    pub method: String,
    pub path: String,
    pub query: HashMap<String, String>,
    #[allow(dead_code)]
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn parse(stream: &mut TcpStream) -> Option<Self> {
        stream.set_read_timeout(Some(Duration::from_secs(5))).ok();

        let mut buf: Vec<u8> = Vec::with_capacity(4096);
        let mut tmp = [0u8; 4096];

        // Read until end-of-headers marker
        loop {
            match stream.read(&mut tmp) {
                Ok(0) => break,
                Ok(n) => {
                    buf.extend_from_slice(&tmp[..n]);
                    if buf.windows(4).any(|w| w == b"\r\n\r\n") {
                        break;
                    }
                    if buf.len() > 65536 {
                        break;
                    }
                }
                Err(_) => break,
            }
        }

        let header_end = buf.windows(4).position(|w| w == b"\r\n\r\n")?;
        let header_str = std::str::from_utf8(&buf[..header_end]).ok()?;

        let mut lines = header_str.split("\r\n");
        let request_line = lines.next()?;
        let mut parts = request_line.split_whitespace();
        let method = parts.next()?.to_string();
        let full_path = parts.next()?.to_string();

        let (path, query_str) = match full_path.split_once('?') {
            Some((p, q)) => (p.to_string(), q.to_string()),
            None => (full_path, String::new()),
        };

        let mut query = HashMap::new();
        for pair in query_str.split('&').filter(|s| !s.is_empty()) {
            if let Some((k, v)) = pair.split_once('=') {
                query.insert(url_decode(k), url_decode(v));
            }
        }

        let mut headers = HashMap::new();
        for line in lines {
            if let Some((k, v)) = line.split_once(':') {
                headers.insert(k.trim().to_lowercase(), v.trim().to_string());
            }
        }

        let content_length: usize = headers
            .get("content-length")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);

        let body_start = header_end + 4;
        let mut body_buf: Vec<u8> = buf[body_start..].to_vec();

        while body_buf.len() < content_length {
            match stream.read(&mut tmp) {
                Ok(0) => break,
                Ok(n) => body_buf.extend_from_slice(&tmp[..n]),
                Err(_) => break,
            }
        }

        let body = String::from_utf8_lossy(
            &body_buf[..content_length.min(body_buf.len())],
        )
        .to_string();

        Some(Request { method, path, query, headers, body })
    }
}

// ── Response ──────────────────────────────────────────────────────────────────

pub struct Response {
    status: u16,
    content_type: &'static str,
    extra_headers: Vec<(&'static str, String)>,
    body: Vec<u8>,
}

impl Response {
    pub fn html(body: String) -> Self {
        Self {
            status: 200,
            content_type: "text/html; charset=utf-8",
            extra_headers: vec![],
            body: body.into_bytes(),
        }
    }

    pub fn json(body: String) -> Self {
        Self {
            status: 200,
            content_type: "application/json",
            extra_headers: vec![("Access-Control-Allow-Origin", "*".to_string())],
            body: body.into_bytes(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            status: 404,
            content_type: "text/plain",
            extra_headers: vec![],
            body: b"404 Not Found".to_vec(),
        }
    }

    pub fn bad_request(msg: &str) -> Self {
        Self {
            status: 400,
            content_type: "text/plain",
            extra_headers: vec![],
            body: msg.as_bytes().to_vec(),
        }
    }

    pub fn redirect(location: &str) -> Self {
        Self {
            status: 302,
            content_type: "text/plain",
            extra_headers: vec![("Location", location.to_string())],
            body: vec![],
        }
    }

    pub fn send(self, stream: &mut TcpStream) {
        let status_text = match self.status {
            200 => "OK",
            302 => "Found",
            400 => "Bad Request",
            404 => "Not Found",
            _ => "Internal Server Error",
        };

        let mut header = format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n",
            self.status,
            status_text,
            self.content_type,
            self.body.len(),
        );

        for (k, v) in &self.extra_headers {
            header.push_str(&format!("{}: {}\r\n", k, v));
        }
        header.push_str("\r\n");

        let _ = stream.write_all(header.as_bytes());
        let _ = stream.write_all(&self.body);
    }
}

// ── URL decode ────────────────────────────────────────────────────────────────

fn url_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(s.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(hex) = std::str::from_utf8(&bytes[i + 1..i + 3]) {
                if let Ok(byte) = u8::from_str_radix(hex, 16) {
                    out.push(byte);
                    i += 3;
                    continue;
                }
            }
        } else if bytes[i] == b'+' {
            out.push(b' ');
            i += 1;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).to_string()
}
