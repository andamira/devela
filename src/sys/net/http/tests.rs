// devela::sys::net::http::tests

use super::*;

mod version {
    use super::*;

    #[test]
    fn properties() {
        assert_eq!(HttpVersion::Http10.label(), "HTTP/1.0");
        assert_eq!(HttpVersion::Http11.label(), "HTTP/1.1");
        assert_eq!(HttpVersion::Http2.label(), "HTTP/2");
        assert_eq!(HttpVersion::Http3.label(), "HTTP/3");
        assert_eq!(HttpVersion::Http10.parts(), (1, 0));
        assert_eq!(HttpVersion::Http11.parts(), (1, 1));
        assert_eq!(HttpVersion::Http2.parts(), (2, 0));
        assert_eq!(HttpVersion::Http3.parts(), (3, 0));
        assert!(HttpVersion::Http10.is_http1());
        assert!(HttpVersion::Http11.is_http1());
        assert!(!HttpVersion::Http2.is_http1());
        assert!(!HttpVersion::Http3.is_http1());
    }
    #[test]
    fn http1_tokens() {
        assert_eq!(HttpVersion::Http10.http1_token(), Some("HTTP/1.0"));
        assert_eq!(HttpVersion::Http11.http1_token(), Some("HTTP/1.1"));
        assert_eq!(HttpVersion::Http2.http1_token(), None);
        assert_eq!(HttpVersion::Http3.http1_token(), None);
        assert_eq!(HttpVersion::parse_http1_token(b"HTTP/1.0"), Some(HttpVersion::Http10));
        assert_eq!(HttpVersion::parse_http1_token(b"HTTP/1.1"), Some(HttpVersion::Http11));
        for invalid in [&b""[..], b"HTTP/1", b"HTTP/1.2", b"HTTP/2", b"http/1.1", b"HTTP/1.1\r\n"] {
            assert_eq!(HttpVersion::parse_http1_token(invalid), None);
        }
    }
}

mod method {
    use super::*;

    #[test]
    fn standard_methods() {
        let cases = [
            ("GET", HttpMethod::Get),
            ("HEAD", HttpMethod::Head),
            ("POST", HttpMethod::Post),
            ("PUT", HttpMethod::Put),
            ("DELETE", HttpMethod::Delete),
            ("CONNECT", HttpMethod::Connect),
            ("OPTIONS", HttpMethod::Options),
            ("TRACE", HttpMethod::Trace),
            ("PATCH", HttpMethod::Patch),
        ];
        for (token, expected) in cases {
            let method = HttpMethod::parse(token).unwrap();
            assert_eq!(method, expected);
            assert_eq!(method.as_str(), token);
        }
    }
    #[test]
    fn extension_and_case_sensitivity() {
        assert_eq!(HttpMethod::parse("PURGE"), Ok(HttpMethod::Other("PURGE")));
        assert_eq!(HttpMethod::parse("get"), Ok(HttpMethod::Other("get")));
    }
    #[test]
    fn invalid_tokens() {
        for invalid in ["", "BAD METHOD", "BAD/METHOD", "BAD\tMETHOD", "MÉTODO"] {
            assert_eq!(HttpMethod::parse(invalid), Err(HttpError::InvalidMethod));
        }
    }
    #[test]
    fn head_predicate() {
        assert!(HttpMethod::Head.is_head());
        assert!(!HttpMethod::Get.is_head());
        assert!(!HttpMethod::Other("HEADS").is_head());
    }
}

mod request_line {
    use super::*;

    #[test]
    fn parses_request_line() {
        let request = HttpRequestLine::parse(b"GET /index.html?lang=en HTTP/1.1").unwrap();
        assert_eq!(request.method(), HttpMethod::Get);
        assert_eq!(request.target(), "/index.html?lang=en");
        assert_eq!(request.origin_path(), Some("/index.html"));
        assert_eq!(request.version(), HttpVersion::Http11);
    }
    #[test]
    fn namespace_delegates() {
        let request = Http::parse_request_line(b"HEAD / HTTP/1.0").unwrap();
        assert_eq!(request.method(), HttpMethod::Head);
        assert_eq!(request.origin_path(), Some("/"));
        assert_eq!(request.version(), HttpVersion::Http10);
    }
    #[test]
    fn preserves_extension_method() {
        let request = HttpRequestLine::parse(b"PURGE /cache HTTP/1.1").unwrap();
        assert_eq!(request.method(), HttpMethod::Other("PURGE"));
        assert_eq!(request.origin_path(), Some("/cache"));
    }
    #[test]
    fn rejects_malformed_lines() {
        let cases: &[(&[u8], HttpError)] = &[
            (b"", HttpError::EmptyRequestLine),
            (b" GET / HTTP/1.1", HttpError::MissingMethod),
            (b"GET", HttpError::MissingTarget),
            (b"GET  HTTP/1.1", HttpError::MissingTarget),
            (b"GET /", HttpError::MissingVersion),
            (b"GET / ", HttpError::MissingVersion),
            (b"G@T / HTTP/1.1", HttpError::InvalidMethod),
            (b"GET /\tbad HTTP/1.1", HttpError::InvalidTarget),
            (b"GET / HTTP/2", HttpError::InvalidVersion),
            (b"GET / HTTP/1.1 extra", HttpError::TrailingData),
            (b"GET / HTTP/1.1\r\n", HttpError::InvalidVersion),
        ];
        for &(line, expected) in cases {
            assert_eq!(HttpRequestLine::parse(line), Err(expected), "{line:?}");
        }
    }
    #[test]
    fn rejects_invalid_utf8() {
        assert_eq!(HttpRequestLine::parse(b"G\xffT / HTTP/1.1"), Err(HttpError::InvalidMethod));
        assert_eq!(HttpRequestLine::parse(b"GET /\xff HTTP/1.1"), Err(HttpError::InvalidTarget));
    }
}

mod status {
    use super::*;

    #[test]
    fn validates_code_range() {
        assert_eq!(HttpStatus::from_code(99), None);
        assert_eq!(HttpStatus::from_code(600), None);
        assert_eq!(HttpStatus::from_code(100).unwrap().code(), 100);
        assert_eq!(HttpStatus::from_code(599).unwrap().code(), 599);
    }
    #[test]
    fn classifies_statuses() {
        let cases = [
            (HttpStatus::CONTINUE, HttpStatusClass::Informational),
            (HttpStatus::OK, HttpStatusClass::Success),
            (HttpStatus::PERMANENT_REDIRECT, HttpStatusClass::Redirection),
            (HttpStatus::NOT_FOUND, HttpStatusClass::ClientError),
            (HttpStatus::INTERNAL_SERVER_ERROR, HttpStatusClass::ServerError),
        ];
        for (status, class) in cases {
            assert_eq!(status.class(), class);
        }
        assert!(HttpStatus::CONTINUE.is_informational());
        assert!(HttpStatus::OK.is_success());
        assert!(HttpStatus::FOUND.is_redirection());
        assert!(HttpStatus::BAD_REQUEST.is_client_error());
        assert!(HttpStatus::BAD_GATEWAY.is_server_error());
        assert!(!HttpStatus::CONTINUE.is_final());
        assert!(HttpStatus::OK.is_final());
        assert!(!HttpStatus::FOUND.is_error());
        assert!(HttpStatus::NOT_FOUND.is_error());
    }
    #[test]
    fn reason_phrases() {
        assert_eq!(HttpStatus::OK.reason(), Some("OK"));
        assert_eq!(HttpStatus::NOT_FOUND.reason(), Some("Not Found"));
        assert_eq!(HttpStatus::INTERNAL_SERVER_ERROR.reason(), Some("Internal Server Error"));
        assert_eq!(Http::reason_phrase(HttpStatus::OK), Some("OK"));
        let extension = HttpStatus::from_code(471).unwrap();
        assert_eq!(extension.reason(), None);
        assert!(extension.is_client_error());
    }
}

mod response_head {
    use super::*;

    fn assert_encoded(head: HttpResponseHead<'_>, expected: &[u8]) {
        let needed = head.http1_encoded_len().unwrap();
        assert_eq!(needed, expected.len());
        let mut dst = [0u8; 256];
        let written = head.write_http1_into(&mut dst).unwrap();
        assert_eq!(written, expected.len());
        assert_eq!(&dst[..written], expected);
    }
    #[test]
    fn constructors_and_accessors() {
        let ok = HttpResponseHead::ok();
        assert_eq!(ok.version(), HttpVersion::Http11);
        assert_eq!(ok.status(), HttpStatus::OK);
        assert_eq!(ok.content_type(), None);
        assert_eq!(ok.content_length(), None);
        let not_found = HttpResponseHead::not_found();
        assert_eq!(not_found.status(), HttpStatus::NOT_FOUND);
    }
    #[test]
    fn writes_minimal_response() {
        assert_encoded(HttpResponseHead::ok(), b"HTTP/1.1 200 OK\r\n\r\n");
    }
    #[test]
    fn writes_headers() {
        assert_encoded(
            HttpResponseHead::ok()
                .with_content_type("text/html; charset=utf-8")
                .with_content_length(42),
            b"HTTP/1.1 200 OK\r\n\
              Content-Type: text/html; charset=utf-8\r\n\
              Content-Length: 42\r\n\
              \r\n",
        );
    }
    #[test]
    fn writes_http10() {
        assert_encoded(
            HttpResponseHead::not_found()
                .with_version(HttpVersion::Http10)
                .with_content_type("text/plain")
                .with_content_length(9),
            b"HTTP/1.0 404 Not Found\r\n\
              Content-Type: text/plain\r\n\
              Content-Length: 9\r\n\
              \r\n",
        );
    }
    #[test]
    fn writes_unknown_status_without_reason() {
        let status = HttpStatus::from_code(471).unwrap();
        assert_encoded(HttpResponseHead::new(status), b"HTTP/1.1 471 \r\n\r\n");
    }
    #[test]
    fn rejects_non_http1_version() {
        let head = HttpResponseHead::ok().with_version(HttpVersion::Http2);
        assert_eq!(head.http1_encoded_len(), Err(HttpError::InvalidVersion));
        let mut dst = [0u8; 64];
        assert_eq!(head.write_http1_into(&mut dst), Err(HttpError::InvalidVersion));
    }
    #[test]
    fn rejects_header_injection() {
        let head = HttpResponseHead::ok().with_content_type("text/plain\r\nInjected: true");
        assert_eq!(head.http1_encoded_len(), Err(HttpError::InvalidHeaderValue));
    }
    #[test]
    fn reports_needed_space_without_writing() {
        let head = HttpResponseHead::ok().with_content_type("text/plain").with_content_length(1234);
        let needed = head.http1_encoded_len().unwrap();
        let mut dst = [0xA5; 8];
        let original = dst;
        assert_eq!(head.write_http1_into(&mut dst), Err(HttpError::NotEnoughSpace(needed)));
        assert_eq!(dst, original);
    }
}

mod error {
    use super::*;

    #[test]
    fn needed_space() {
        assert_eq!(HttpError::NotEnoughSpace(73).needed_space(), Some(73));
        assert_eq!(HttpError::InvalidVersion.needed_space(), None);
    }
}
