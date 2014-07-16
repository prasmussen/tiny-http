use std::ascii::{AsciiCast, StrAsciiExt};
use std::fmt::{Formatter, FormatError, Show};

/// Status code of a request or response.
#[deriving(Eq, PartialEq, Clone, Show, Ord, PartialOrd)]
#[stable]
pub struct StatusCode(pub uint);

impl StatusCode {
    #[stable]
    /// Returns the status code as a number.
    pub fn as_uint(&self) -> uint {
        match *self { StatusCode(n) => n }
    }

    #[stable]
    /// Returns the default reason phrase for this status code.
    /// For example the status code 404 corresponds to "Not Found".
    pub fn get_default_reason_phrase(&self) -> &'static str {
        match self.as_uint() {
            100 => "Continue",
            101 => "Switching Protocols",
            102 => "Processing",
            118 => "Connection timed out",
            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            203 => "Non-Authoritative Information",
            204 => "No Content",
            205 => "Reset Content",
            206 => "Partial Content",
            207 => "Multi-Status",
            210 => "Content Different",
            300 => "Multiple Choices",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            304 => "Not Modified",
            305 => "Use Proxy",
            307 => "Temporary Redirect",
            400 => "Bad Request",
            401 => "Unauthorized",
            402 => "Payment Required",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            406 => "Not Acceptable",
            407 => "Proxy Authentication Required",
            408 => "Request Time-out",
            409 => "Conflict",
            410 => "Gone",
            411 => "Length Required",
            412 => "Precondition Failed",
            413 => "Request Entity Too Large",
            414 => "Reques-URI Too Large",
            415 => "Unsupported Media Type",
            416 => "Request range not satisfiable",
            417 => "Expectation Failed",
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Time-out",
            505 => "HTTP Version not supported",
            _ => "Unknown"
        }
    }
}

impl Equiv<uint> for StatusCode {
    fn equiv(&self, other: &uint) -> bool {
        self.as_uint() == *other
    }
}

/// Represents a HTTP header.
/// 
/// The easiest way to create a `Header` object is to call `from_str`.
/// 
/// ```
/// let header: Header = from_str("Content-Type: text/plain").unwrap();
/// ```
#[deriving(Clone)]
#[unstable]
pub struct Header {
    pub field: HeaderField,
    pub value: String,
}

impl ::std::from_str::FromStr for Header {
    fn from_str(input: &str) -> Option<Header> {
        let elems = input.splitn(':', 2).map(|e| e.to_string()).collect::<Vec<String>>();

        if elems.len() <= 1 {
            return None;
        }

        let field = match from_str(elems.get(0).as_slice().trim()) {
            None => return None,
            Some(f) => f
        };

        Some(Header {
            field: field,
            value: elems.get(1).as_slice().trim().to_string()
        })
    }
}

impl Show for Header {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
        (format!("{}: {}", self.field, self.value)).fmt(formatter)
    }
}

/// Field of a header (eg. `Content-Type`, `Content-Length`, etc.)
/// 
/// Comparaison between two `HeaderField`s ignores case.
#[unstable]
#[deriving(Clone)]
pub struct HeaderField(Vec<Ascii>);

impl HeaderField {
    fn as_str<'a>(&'a self) -> &'a [Ascii] {
        match self { &HeaderField(ref s) => s.as_slice() }
    }
}

impl ::std::from_str::FromStr for HeaderField {
    fn from_str(s: &str) -> Option<HeaderField> {
        s.trim().to_ascii_opt().map(|s| HeaderField(Vec::from_slice(s)))
    }
}

impl IntoStr for HeaderField {
    fn into_string(self) -> String {
        match self { HeaderField(s) => s.into_string() }
    }
}

impl Show for HeaderField {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
        let method = self.as_str();
        method.as_str_ascii().fmt(formatter)
    }
}

impl PartialEq for HeaderField {
    fn eq(&self, other: &HeaderField) -> bool {
        self.as_str().eq_ignore_case(other.as_str())
    }
}

impl Eq for HeaderField {}

impl<S: Str> Equiv<S> for HeaderField {
    fn equiv(&self, other: &S) -> bool {
        other.as_slice().eq_ignore_ascii_case(self.as_str().as_str_ascii())
    }
}


/// HTTP method (eg. `GET`, `POST`, etc.)
/// 
/// The user chooses the method he wants.
/// 
/// Comparaison between two `Method`s ignores case.
#[unstable]
#[deriving(Clone)]
pub struct Method(Vec<Ascii>);

impl Method {
    fn as_str<'a>(&'a self) -> &'a [Ascii] {
        match self { &Method(ref s) => s.as_slice() }
    }
}

impl ::std::from_str::FromStr for Method {
    fn from_str(s: &str) -> Option<Method> {
        s.to_ascii_opt().map(|s| Method(Vec::from_slice(s)))
    }
}

impl IntoStr for Method {
    fn into_string(self) -> String {
        match self { Method(s) => s.into_string() }
    }
}

impl Show for Method {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
        let method = self.as_str();
        method.as_str_ascii().fmt(formatter)
    }
}

impl PartialEq for Method {
    fn eq(&self, other: &Method) -> bool {
        self.as_str().eq_ignore_case(other.as_str())
    }
}

impl Eq for Method {}

impl<S: Str> Equiv<S> for Method {
    fn equiv(&self, other: &S) -> bool {
        other.as_slice().eq_ignore_ascii_case(self.as_str().as_str_ascii())
    }
}

/// HTTP version (usually 1.0 or 1.1).
#[unstable]
#[deriving(Clone, PartialEq, Eq, Ord)]
pub struct HTTPVersion(pub uint, pub uint);

impl Show for HTTPVersion {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
        let (major, minor) = match self { &HTTPVersion(m, n) => (m, n) };
        (format!("{}.{}", major, minor)).fmt(formatter)
    }
}

impl PartialOrd for HTTPVersion {
    fn partial_cmp(&self, other: &HTTPVersion) -> Option<Ordering> {
        let (my_major, my_minor) = match self { &HTTPVersion(m, n) => (m, n) };
        let (other_major, other_minor) = match other { &HTTPVersion(m, n) => (m, n) };

        if my_major != other_major {
            return my_major.partial_cmp(&other_major)
        }

        my_minor.partial_cmp(&other_minor)
    }
}


#[cfg(test)]
mod test {
    use super::Header;

    #[test]
    fn test_parse_header() {
        let header: Header = from_str("Content-Type: text/html").unwrap();

        assert!(header.field.equiv(&"content-type"));
        assert!(header.value.as_slice() == "text/html");

        assert!(from_str::<Header>("hello world").is_none());
    }
}
