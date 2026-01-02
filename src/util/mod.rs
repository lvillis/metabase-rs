use std::borrow::Cow;

use http::HeaderMap;
use serde::Serialize;
use serde_json::Value;
use url::Url;

use crate::error::Error;

pub(crate) const DEFAULT_BODY_SNIPPET_LIMIT: usize = 4 * 1024;

pub(crate) fn normalize_base_url(mut base_url: Url) -> Url {
    if !base_url.path().ends_with('/') {
        let normalized = format!("{}/", base_url.path().trim_end_matches('/'));
        base_url.set_path(&normalized);
    }
    base_url
}

pub(crate) fn build_url(base_url: &Url, path: &[&str]) -> Result<Url, Error> {
    let mut url = base_url.clone();

    {
        let mut segments = url
            .path_segments_mut()
            .map_err(|()| Error::build_url(url::ParseError::RelativeUrlWithoutBase))?;
        segments.pop_if_empty();
        for segment in path {
            segments.push(segment);
        }
    }

    Ok(url)
}

pub(crate) fn set_query<Q: Serialize + ?Sized>(
    url: &mut Url,
    query: Option<&Q>,
) -> Result<(), Error> {
    match query {
        None => {
            url.set_query(None);
            Ok(())
        }
        Some(query) => {
            let value = serde_json::to_value(query).map_err(Error::serialize_query)?;
            let pairs = query_pairs(value).map_err(Error::serialize_query)?;

            if pairs.is_empty() {
                url.set_query(None);
                return Ok(());
            }

            let mut serializer = url::form_urlencoded::Serializer::new(String::new());
            for (key, value) in pairs {
                serializer.append_pair(&key, &value);
            }
            let query = serializer.finish();
            url.set_query(Some(&query));
            Ok(())
        }
    }
}

#[derive(Debug)]
struct QueryPairsError(&'static str);

impl std::fmt::Display for QueryPairsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for QueryPairsError {}

fn query_pairs(value: Value) -> Result<Vec<(String, String)>, QueryPairsError> {
    match value {
        Value::Null => Ok(Vec::new()),
        Value::Object(map) => query_pairs_from_object(map),
        Value::Array(items) => query_pairs_from_pair_list(items),
        _ => Err(QueryPairsError(
            "query parameters must serialize to an object or a list of key/value pairs",
        )),
    }
}

fn query_pairs_from_object(
    map: serde_json::Map<String, Value>,
) -> Result<Vec<(String, String)>, QueryPairsError> {
    let mut pairs = Vec::new();
    for (key, value) in map {
        push_query_pairs(&mut pairs, key, value)?;
    }
    Ok(pairs)
}

fn query_pairs_from_pair_list(items: Vec<Value>) -> Result<Vec<(String, String)>, QueryPairsError> {
    let mut pairs = Vec::new();
    for item in items {
        let Value::Array(pair) = item else {
            return Err(QueryPairsError(
                "query pair list must be a list of [key, value] arrays",
            ));
        };
        if pair.len() != 2 {
            return Err(QueryPairsError(
                "query pair list entries must have length 2",
            ));
        }
        let key = scalar_to_string(&pair[0]).ok_or(QueryPairsError(
            "query pair keys must be strings, numbers, or booleans",
        ))?;
        let value = scalar_to_string(&pair[1]).ok_or(QueryPairsError(
            "query pair values must be strings, numbers, or booleans",
        ))?;
        pairs.push((key, value));
    }
    Ok(pairs)
}

fn push_query_pairs(
    pairs: &mut Vec<(String, String)>,
    key: String,
    value: Value,
) -> Result<(), QueryPairsError> {
    match value {
        Value::Null => Ok(()),
        Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            let value = scalar_to_string(&value).ok_or(QueryPairsError(
                "query values must be strings, numbers, or booleans",
            ))?;
            pairs.push((key, value));
            Ok(())
        }
        Value::Array(items) => {
            for item in items {
                if item.is_null() {
                    continue;
                }
                if !matches!(item, Value::Bool(_) | Value::Number(_) | Value::String(_)) {
                    return Err(QueryPairsError(
                        "query arrays must contain only strings, numbers, or booleans",
                    ));
                }
                let value = scalar_to_string(&item).ok_or(QueryPairsError(
                    "query arrays must contain only strings, numbers, or booleans",
                ))?;
                pairs.push((key.clone(), value));
            }
            Ok(())
        }
        Value::Object(_) => Err(QueryPairsError("nested query objects are not supported")),
    }
}

fn scalar_to_string(value: &Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::Bool(v) => Some(if *v {
            "true".to_owned()
        } else {
            "false".to_owned()
        }),
        Value::Number(v) => Some(v.to_string()),
        Value::String(v) => Some(v.clone()),
        Value::Array(_) | Value::Object(_) => None,
    }
}

pub(crate) fn body_snippet(bytes: &[u8], limit: usize) -> String {
    let len = bytes.len().min(limit);
    match String::from_utf8_lossy(&bytes[..len]) {
        Cow::Borrowed(s) => s.to_string(),
        Cow::Owned(s) => s,
    }
}

pub(crate) fn capture_body_snippet(bytes: &[u8], limit: usize, redact: bool) -> String {
    let snippet = body_snippet(bytes, limit);
    if !redact {
        return snippet;
    }

    if !looks_sensitive(&snippet) {
        return snippet;
    }

    redact_body_snippet(&snippet)
}

pub(crate) fn extract_request_id(headers: &HeaderMap) -> Option<String> {
    const CANDIDATES: [&str; 3] = ["x-request-id", "x-amzn-requestid", "x-correlation-id"];

    for name in CANDIDATES {
        let Some(value) = headers.get(name) else {
            continue;
        };
        let Ok(value) = value.to_str() else {
            continue;
        };

        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_owned());
        }
    }

    None
}

fn looks_sensitive(snippet: &str) -> bool {
    let lower = snippet.to_ascii_lowercase();
    const MARKERS: [&str; 8] = [
        "token",
        "password",
        "secret",
        "api_key",
        "api-key",
        "authorization",
        "cookie",
        "session",
    ];

    MARKERS.iter().any(|m| lower.contains(m))
}

fn redact_body_snippet(snippet: &str) -> String {
    match serde_json::from_str::<Value>(snippet) {
        Ok(mut value) => {
            redact_json_value(&mut value);
            serde_json::to_string(&value).unwrap_or_else(|_| "<redacted>".to_owned())
        }
        Err(_) => "<redacted>".to_owned(),
    }
}

fn redact_json_value(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for (key, value) in map.iter_mut() {
                if is_sensitive_key(key) {
                    *value = Value::String("<redacted>".to_owned());
                    continue;
                }
                redact_json_value(value);
            }
        }
        Value::Array(items) => {
            for item in items {
                redact_json_value(item);
            }
        }
        _ => {}
    }
}

fn is_sensitive_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    const KEYS: [&str; 6] = [
        "password", "token", "secret", "api_key", "api-key", "session",
    ];
    KEYS.iter().any(|k| key.contains(k))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;

    #[test]
    fn normalize_base_url_adds_trailing_slash() -> Result<(), Box<dyn StdError + Send + Sync>> {
        let base_url = Url::parse("https://example.com/metabase")?;
        let normalized = normalize_base_url(base_url);
        assert_eq!(normalized.as_str(), "https://example.com/metabase/");
        Ok(())
    }

    #[test]
    fn build_url_encodes_path_segments() -> Result<(), Box<dyn StdError + Send + Sync>> {
        let base_url = Url::parse("https://example.com/api/")?;
        let url = build_url(&base_url, &["a/b", "c"])?;
        assert_eq!(url.as_str(), "https://example.com/api/a%2Fb/c");
        Ok(())
    }

    #[test]
    fn set_query_serializes_parameters() -> Result<(), Box<dyn StdError + Send + Sync>> {
        #[derive(serde::Serialize)]
        struct Params<'a> {
            q: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            tags: Vec<&'a str>,
        }

        let mut url = Url::parse("https://example.com/api/")?;
        let params = Params {
            q: "hello world",
            limit: None,
            tags: vec!["a", "b"],
        };
        set_query(&mut url, Some(&params))?;

        let pairs: Vec<(String, String)> = url
            .query_pairs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        assert!(pairs.contains(&("q".to_owned(), "hello world".to_owned())));
        assert!(pairs.contains(&("tags".to_owned(), "a".to_owned())));
        assert!(pairs.contains(&("tags".to_owned(), "b".to_owned())));
        assert!(!pairs.iter().any(|(k, _)| k == "limit"));
        Ok(())
    }

    #[test]
    fn capture_body_snippet_redacts_sensitive_json() {
        let bytes = br#"{"token":"abcd","nested":{"password":"pw"}}"#;
        let snippet = capture_body_snippet(bytes, 1024, true);
        assert!(snippet.contains("<redacted>"));
        assert!(!snippet.contains("abcd"));
        assert!(!snippet.contains("pw"));
    }

    #[test]
    fn capture_body_snippet_leaves_nonsensitive_content() {
        let bytes = br#"{"message":"boom"}"#;
        let snippet = capture_body_snippet(bytes, 1024, true);
        assert_eq!(snippet, r#"{"message":"boom"}"#);
    }

    #[test]
    fn capture_body_snippet_redacts_sensitive_text() {
        let bytes = b"token=abcd";
        let snippet = capture_body_snippet(bytes, 1024, true);
        assert_eq!(snippet, "<redacted>");
    }
}
