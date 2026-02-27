//! Redis key helpers
//!
//! Central location for building and validating Redis keys.  Every key in the
//! platform should be constructed through this module so that the naming
//! convention is consistent, easy to audit, and easy to change (e.g. if we add
//! an environment prefix later).  The codebase previously repeated
//! `format!("{}:session:{}", prefix, id)` everywhere; this replaces that
//! repetition with a tiny typed builder.

use std::fmt;

/// Strongly‑typed Redis key.  Internally it's just a string but having a newtype
/// allows us to implement convenience methods, conversions and keep callers
/// honest about the fact they're dealing with a Redis key rather than an
/// arbitrary string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RedisKey(String);

impl RedisKey {
    /// Build a namespaced key from the given segments.  Segments are joined with
    /// a colon (`:`) character.  Empty segments are ignored.
    pub fn from_parts(parts: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        let joined = parts
            .into_iter()
            .map(|p| p.as_ref())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(":");
        RedisKey(joined)
    }

    /// Convenience constructor when a prefix is used across many keys.
    pub fn with_prefix(prefix: impl AsRef<str>, parts: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        let mut all: Vec<String> = vec![prefix.as_ref().to_string()];
        all.extend(parts.into_iter().map(|p| p.as_ref().to_string()));
        RedisKey::from_parts(all)
    }

    /// Cache key under provided prefix
    pub fn cache(prefix: impl AsRef<str>, key: impl AsRef<str>) -> Self {
        Self::with_prefix(prefix, ["cache", key.as_ref()])
    }

    /// Session key (single session)
    pub fn session(prefix: impl AsRef<str>, session_id: impl AsRef<str>) -> Self {
        Self::with_prefix(prefix, ["session", session_id.as_ref()])
    }

    /// Key that holds set of sessions for a user
    pub fn user_sessions(prefix: impl AsRef<str>, user_id: impl AsRef<str>) -> Self {
        Self::with_prefix(prefix, ["user_sessions", user_id.as_ref()])
    }

    /// Rate limit key for a generic identifier
    pub fn rate_limit(prefix: impl AsRef<str>, key: impl AsRef<str>) -> Self {
        Self::with_prefix(prefix, ["rate_limit", key.as_ref()])
    }

    /// Distributed lock key
    pub fn lock(prefix: impl AsRef<str>, resource: impl AsRef<str>) -> Self {
        Self::with_prefix(prefix, ["lock", resource.as_ref()])
    }

    /// Return the inner string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RedisKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<RedisKey> for String {
    fn from(key: RedisKey) -> String {
        key.0
    }
}

impl AsRef<str> for RedisKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&str> for RedisKey {
    fn from(s: &str) -> RedisKey {
        RedisKey(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_basic_key() {
        let k = RedisKey::from_parts(["app", "cache", "foo"]);
        assert_eq!(k.as_str(), "app:cache:foo");
    }

    #[test]
    fn prefix_helpers() {
        let k = RedisKey::cache("app", "token");
        assert_eq!(k.as_str(), "app:cache:token");

        let s = RedisKey::session("app", "sess123");
        assert_eq!(s.as_str(), "app:session:sess123");

        let us = RedisKey::user_sessions("app", "user1");
        assert_eq!(us.as_str(), "app:user_sessions:user1");

        let r = RedisKey::rate_limit("app", "ip");
        assert_eq!(r.as_str(), "app:rate_limit:ip");

        let l = RedisKey::lock("app", "resource");
        assert_eq!(l.as_str(), "app:lock:resource");
    }

    #[test]
    fn ignore_empty_segments() {
        let k = RedisKey::from_parts(["", "foo", "", "bar"]);
        assert_eq!(k.as_str(), "foo:bar");
    }
}
