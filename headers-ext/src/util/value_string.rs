use std::{fmt, str::{self, FromStr}};

use bytes::Bytes;
use http::header::HeaderValue;

/// A value that is both a valid `HeaderValue` and `String`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct HeaderValueString {
    /// Care must be taken to only set this value when it is also
    /// a valid `String`, since `as_str` will convert to a `&str`
    /// in an unchecked manner.
    value: HeaderValue,
}

impl HeaderValueString {
    pub(crate) fn from_val(val: &HeaderValue) -> Option<Self> {
        if val.to_str().is_ok() {
            Some(HeaderValueString {
                value: val.clone(),
            })
        } else {
            None
        }
    }

    pub(crate) fn from_string(src: String) -> Option<Self> {
        // A valid `str` (the argument)...
        let bytes = Bytes::from(src);
        HeaderValue::from_shared(bytes)
            .ok()
            .map(|value| HeaderValueString {
                value,
            })
    }

    pub(crate) fn from_static(src: &'static str) -> HeaderValueString {
        // A valid `str` (the argument)...
        HeaderValueString {
            value: HeaderValue::from_static(src),
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        // HeaderValueString is only created from HeaderValues
        // that have validated they are also UTF-8 strings.
        unsafe {
            str::from_utf8_unchecked(self.value.as_bytes())
        }
    }
}

impl fmt::Debug for HeaderValueString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.value, f)
    }
}

impl ::headers_core::decode::TryFromValues for HeaderValueString {
    fn try_from_values(values: &mut ::Values) -> Option<Self> {
        HeaderValueString::from_val(values.next()?)
    }
}

impl<'a> From<&'a HeaderValueString> for HeaderValue {
    fn from(src: &'a HeaderValueString) -> HeaderValue {
        src.value.clone()
    }
}

#[derive(Debug)]
pub(crate) struct FromStrError(());

impl FromStr for HeaderValueString {
    type Err = FromStrError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        // A valid `str` (the argument)...
        src
            .parse()
            .map(|value| HeaderValueString {
                value,
            })
            .map_err(|_| FromStrError(()))
    }
}
