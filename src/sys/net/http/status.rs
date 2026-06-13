// devela/src/sys/net/http/status.rs
//
//! Defines [`HttpStatusClass`], [`HttpStatus`].
//

#[doc = crate::_tags!(network protocol)]
/// The response class of an HTTP status code.
#[doc = crate::_doc_meta!{location("sys/net/http")}]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HttpStatusClass {
    /// `1xx`: processing continues.
    Informational = 1,
    /// `2xx`: successful result.
    Success = 2,
    /// `3xx`: redirection or related action.
    Redirection = 3,
    /// `4xx`: client error.
    ClientError = 4,
    /// `5xx`: server error.
    ServerError = 5,
}

#[doc = crate::_tags!(network protocol)]
/// HTTP response status code.
#[doc = crate::_doc_meta!{location("sys/net/http")}]
///
/// Valid HTTP status codes are in the inclusive range `100..=599`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HttpStatus(u16);

/// # Names
impl HttpStatus {
    /* informational */

    /// `100 Continue`.
    pub const CONTINUE: Self = Self(100);
    /// `101 Switching Protocols`.
    pub const SWITCHING_PROTOCOLS: Self = Self(101);
    /// `102 Processing`.
    pub const PROCESSING: Self = Self(102);
    /// `103 Early Hints`.
    pub const EARLY_HINTS: Self = Self(103);

    /* success */

    /// `200 OK`.
    pub const OK: Self = Self(200);
    /// `201 Created`.
    pub const CREATED: Self = Self(201);
    /// `202 Accepted`.
    pub const ACCEPTED: Self = Self(202);
    /// `203 Non-Authoritative Information`.
    pub const NON_AUTHORITATIVE_INFORMATION: Self = Self(203);
    /// `204 No Content`.
    pub const NO_CONTENT: Self = Self(204);
    /// `205 Reset Content`.
    pub const RESET_CONTENT: Self = Self(205);
    /// `206 Partial Content`.
    pub const PARTIAL_CONTENT: Self = Self(206);
    /// `207 Multi-Status`.
    pub const MULTI_STATUS: Self = Self(207);
    /// `208 Already Reported`.
    pub const ALREADY_REPORTED: Self = Self(208);
    /// `226 IM Used`.
    pub const IM_USED: Self = Self(226);

    /* redirection */

    /// `300 Multiple Choices`.
    pub const MULTIPLE_CHOICES: Self = Self(300);
    /// `301 Moved Permanently`.
    pub const MOVED_PERMANENTLY: Self = Self(301);
    /// `302 Found`.
    pub const FOUND: Self = Self(302);
    /// `303 See Other`.
    pub const SEE_OTHER: Self = Self(303);
    /// `304 Not Modified`.
    pub const NOT_MODIFIED: Self = Self(304);
    /// `305 Use Proxy`.
    pub const USE_PROXY: Self = Self(305);
    /// `307 Temporary Redirect`.
    pub const TEMPORARY_REDIRECT: Self = Self(307);
    /// `308 Permanent Redirect`.
    pub const PERMANENT_REDIRECT: Self = Self(308);

    /* client error */

    /// `400 Bad Request`.
    pub const BAD_REQUEST: Self = Self(400);
    /// `401 Unauthorized`.
    pub const UNAUTHORIZED: Self = Self(401);
    /// `402 Payment Required`.
    pub const PAYMENT_REQUIRED: Self = Self(402);
    /// `403 Forbidden`.
    pub const FORBIDDEN: Self = Self(403);
    /// `404 Not Found`.
    pub const NOT_FOUND: Self = Self(404);
    /// `405 Method Not Allowed`.
    pub const METHOD_NOT_ALLOWED: Self = Self(405);
    /// `406 Not Acceptable`.
    pub const NOT_ACCEPTABLE: Self = Self(406);
    /// `407 Proxy Authentication Required`.
    pub const PROXY_AUTHENTICATION_REQUIRED: Self = Self(407);
    /// `408 Request Timeout`.
    pub const REQUEST_TIMEOUT: Self = Self(408);
    /// `409 Conflict`.
    pub const CONFLICT: Self = Self(409);
    /// `410 Gone`.
    pub const GONE: Self = Self(410);
    /// `411 Length Required`.
    pub const LENGTH_REQUIRED: Self = Self(411);
    /// `412 Precondition Failed`.
    pub const PRECONDITION_FAILED: Self = Self(412);
    /// `413 Content Too Large`.
    pub const CONTENT_TOO_LARGE: Self = Self(413);
    /// `414 URI Too Long`.
    pub const URI_TOO_LONG: Self = Self(414);
    /// `415 Unsupported Media Type`.
    pub const UNSUPPORTED_MEDIA_TYPE: Self = Self(415);
    /// `416 Range Not Satisfiable`.
    pub const RANGE_NOT_SATISFIABLE: Self = Self(416);
    /// `417 Expectation Failed`.
    pub const EXPECTATION_FAILED: Self = Self(417);
    /// `421 Misdirected Request`.
    pub const MISDIRECTED_REQUEST: Self = Self(421);
    /// `422 Unprocessable Content`.
    pub const UNPROCESSABLE_CONTENT: Self = Self(422);
    /// `423 Locked`.
    pub const LOCKED: Self = Self(423);
    /// `424 Failed Dependency`.
    pub const FAILED_DEPENDENCY: Self = Self(424);
    /// `425 Too Early`.
    pub const TOO_EARLY: Self = Self(425);
    /// `426 Upgrade Required`.
    pub const UPGRADE_REQUIRED: Self = Self(426);
    /// `428 Precondition Required`.
    pub const PRECONDITION_REQUIRED: Self = Self(428);
    /// `429 Too Many Requests`.
    pub const TOO_MANY_REQUESTS: Self = Self(429);
    /// `431 Request Header Fields Too Large`.
    pub const REQUEST_HEADER_FIELDS_TOO_LARGE: Self = Self(431);
    /// `451 Unavailable For Legal Reasons`.
    pub const UNAVAILABLE_FOR_LEGAL_REASONS: Self = Self(451);

    /* server error */

    /// `500 Internal Server Error`.
    pub const INTERNAL_SERVER_ERROR: Self = Self(500);
    /// `501 Not Implemented`.
    pub const NOT_IMPLEMENTED: Self = Self(501);
    /// `502 Bad Gateway`.
    pub const BAD_GATEWAY: Self = Self(502);
    /// `503 Service Unavailable`.
    pub const SERVICE_UNAVAILABLE: Self = Self(503);
    /// `504 Gateway Timeout`.
    pub const GATEWAY_TIMEOUT: Self = Self(504);
    /// `505 HTTP Version Not Supported`.
    pub const HTTP_VERSION_NOT_SUPPORTED: Self = Self(505);
    /// `506 Variant Also Negotiates`.
    pub const VARIANT_ALSO_NEGOTIATES: Self = Self(506);
    /// `507 Insufficient Storage`.
    pub const INSUFFICIENT_STORAGE: Self = Self(507);
    /// `508 Loop Detected`.
    pub const LOOP_DETECTED: Self = Self(508);
    /// `511 Network Authentication Required`.
    pub const NETWORK_AUTHENTICATION_REQUIRED: Self = Self(511);
}

/// # Fundamental methods
impl HttpStatus {
    /// Creates a valid HTTP status code.
    #[must_use]
    pub const fn from_code(code: u16) -> Option<Self> {
        if code >= 100 && code <= 599 { Some(Self(code)) } else { None }
    }

    /// Returns the three-digit status code.
    #[must_use]
    pub const fn code(self) -> u16 {
        self.0
    }

    /// Returns the status-code class.
    #[must_use]
    pub const fn class(self) -> HttpStatusClass {
        match self.0 / 100 {
            1 => HttpStatusClass::Informational,
            2 => HttpStatusClass::Success,
            3 => HttpStatusClass::Redirection,
            4 => HttpStatusClass::ClientError,
            5 => HttpStatusClass::ServerError,
            _ => unreachable!(),
        }
    }

    /// Returns whether this is a `1xx` informational status.
    #[must_use]
    pub const fn is_informational(self) -> bool {
        self.0 >= 100 && self.0 < 200
    }
    /// Returns whether this is a `2xx` successful status.
    #[must_use]
    pub const fn is_success(self) -> bool {
        self.0 >= 200 && self.0 < 300
    }
    /// Returns whether this is a `3xx` redirection status.
    #[must_use]
    pub const fn is_redirection(self) -> bool {
        self.0 >= 300 && self.0 < 400
    }
    /// Returns whether this is a `4xx` client-error status.
    #[must_use]
    pub const fn is_client_error(self) -> bool {
        self.0 >= 400 && self.0 < 500
    }
    /// Returns whether this is a `5xx` server-error status.
    #[must_use]
    pub const fn is_server_error(self) -> bool {
        self.0 >= 500 && self.0 < 600
    }
    /// Returns whether this is a final, non-informational status.
    #[must_use]
    pub const fn is_final(self) -> bool {
        self.0 >= 200
    }
    /// Returns whether this is a client or server error.
    #[must_use]
    pub const fn is_error(self) -> bool {
        self.0 >= 400
    }
}

/// # Reason phrases
impl HttpStatus {
    /// Returns the registered reason phrase, when known.
    #[must_use]
    pub const fn reason(self) -> Option<&'static str> {
        match self.0 {
            /* informational */
            100 => Some("Continue"),
            101 => Some("Switching Protocols"),
            102 => Some("Processing"),
            103 => Some("Early Hints"),
            /* sucess */
            200 => Some("OK"),
            201 => Some("Created"),
            202 => Some("Accepted"),
            203 => Some("Non-Authoritative Information"),
            204 => Some("No Content"),
            205 => Some("Reset Content"),
            206 => Some("Partial Content"),
            207 => Some("Multi-Status"),
            208 => Some("Already Reported"),
            226 => Some("IM Used"),
            /* redirection */
            300 => Some("Multiple Choices"),
            301 => Some("Moved Permanently"),
            302 => Some("Found"),
            303 => Some("See Other"),
            304 => Some("Not Modified"),
            305 => Some("Use Proxy"),
            307 => Some("Temporary Redirect"),
            308 => Some("Permanent Redirect"),
            /* client error */
            400 => Some("Bad Request"),
            401 => Some("Unauthorized"),
            402 => Some("Payment Required"),
            403 => Some("Forbidden"),
            404 => Some("Not Found"),
            405 => Some("Method Not Allowed"),
            406 => Some("Not Acceptable"),
            407 => Some("Proxy Authentication Required"),
            408 => Some("Request Timeout"),
            409 => Some("Conflict"),
            410 => Some("Gone"),
            411 => Some("Length Required"),
            412 => Some("Precondition Failed"),
            413 => Some("Content Too Large"),
            414 => Some("URI Too Long"),
            415 => Some("Unsupported Media Type"),
            416 => Some("Range Not Satisfiable"),
            417 => Some("Expectation Failed"),
            421 => Some("Misdirected Request"),
            422 => Some("Unprocessable Content"),
            423 => Some("Locked"),
            424 => Some("Failed Dependency"),
            425 => Some("Too Early"),
            426 => Some("Upgrade Required"),
            428 => Some("Precondition Required"),
            429 => Some("Too Many Requests"),
            431 => Some("Request Header Fields Too Large"),
            451 => Some("Unavailable For Legal Reasons"),
            /* server error */
            500 => Some("Internal Server Error"),
            501 => Some("Not Implemented"),
            502 => Some("Bad Gateway"),
            503 => Some("Service Unavailable"),
            504 => Some("Gateway Timeout"),
            505 => Some("HTTP Version Not Supported"),
            506 => Some("Variant Also Negotiates"),
            507 => Some("Insufficient Storage"),
            508 => Some("Loop Detected"),
            511 => Some("Network Authentication Required"),
            _ => None,
        }
    }
}
