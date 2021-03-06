use http::header::HeaderValue;

/// `Referer` header, defined in
/// [RFC7231](http://tools.ietf.org/html/rfc7231#section-5.5.2)
///
/// The `Referer` [sic] header field allows the user agent to specify a
/// URI reference for the resource from which the target URI was obtained
/// (i.e., the "referrer", though the field name is misspelled).  A user
/// agent MUST NOT include the fragment and userinfo components of the
/// URI reference, if any, when generating the Referer field value.
///
/// ## ABNF
///
/// ```text
/// Referer = absolute-URI / partial-URI
/// ```
///
/// ## Example values
///
/// * `http://www.example.org/hypertext/Overview.html`
///
/// # Examples
///
/// ```
/// # extern crate headers_ext as headers;
/// use headers::Referer;
///
/// let r = Referer::from_static("/People.html#tim");
/// ```
#[derive(Debug, Clone, PartialEq, Header)]
pub struct Referer(HeaderValue);

impl Referer {
    /// Create a `Referer` with a static string.
    ///
    /// # Panic
    ///
    /// Panics if the string is not a legal header value.
    pub fn from_static(s: &'static str) -> Referer {
        Referer(HeaderValue::from_static(s))
    }
}
