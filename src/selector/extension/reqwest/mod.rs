use crate::selector::Selector;
use crate::traits::{AsSelector, SelectorInstance, Snapshot};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Body, Method, Request, Response, StatusCode, Url, Version};

// ─── Request ──────────────────────────────────────────────────────────────────

impl<'a> AsSelector<'a, Request, ()> for Request {
	fn as_selector(&'a self) -> Selector<'a, Request, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, Request, P> {
	pub fn method(&self) -> Selector<'a, Method, Self> {
		self.route_to(|cursor, _| Some(cursor.method()))
	}

	pub fn url(&self) -> Selector<'a, Url, Self> {
		self.route_to(|cursor, _| Some(cursor.url()))
	}

	pub fn headers(&self) -> Selector<'a, HeaderMap<HeaderValue>, Self> {
		self.route_to(|cursor, _| Some(cursor.headers()))
	}

	pub fn body(&self) -> Selector<'a, Body, Self> {
		self.route_to(|cursor, _| cursor.body())
	}

	pub fn version_eq(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() == v)
	}

	pub fn version_not_eq(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() != v)
	}

	pub fn cond_version_eq(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_eq(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_version_not_eq(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_not_eq(v)
		} else {
			self.snapshot()
		}
	}

	pub fn version_gt(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() > v)
	}

	pub fn version_not_gt(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() <= v)
	}

	pub fn cond_version_gt(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_gt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_version_not_gt(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_not_gt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn version_lt(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() < v)
	}

	pub fn version_not_lt(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() >= v)
	}

	pub fn cond_version_lt(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_lt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_version_not_lt(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_not_lt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn version_ge(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() >= v)
	}

	pub fn version_not_ge(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() < v)
	}

	pub fn cond_version_ge(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_ge(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_version_not_ge(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_not_ge(v)
		} else {
			self.snapshot()
		}
	}

	pub fn version_le(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() <= v)
	}

	pub fn version_not_le(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() > v)
	}

	pub fn cond_version_le(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_le(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_version_not_le(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_not_le(v)
		} else {
			self.snapshot()
		}
	}
}

// ─── Response ─────────────────────────────────────────────────────────────────

impl<'a> AsSelector<'a, Response, ()> for Response {
	fn as_selector(&'a self) -> Selector<'a, Response, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, Response, P> {
	pub fn headers(&self) -> Selector<'a, HeaderMap<HeaderValue>, Self> {
		self.route_to(|cursor, _| Some(cursor.headers()))
	}

	pub fn url(&self) -> Selector<'a, Url, Self> {
		self.route_to(|cursor, _| Some(cursor.url()))
	}

	pub fn is_informational(&self) -> Self {
		self.filter(|cursor, _| cursor.status().is_informational())
	}

	pub fn not_is_informational(&self) -> Self {
		self.filter(|cursor, _| !cursor.status().is_informational())
	}

	pub fn cond_is_informational(&self, condition: bool) -> Self {
		if condition {
			self.is_informational()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_informational(&self, condition: bool) -> Self {
		if condition {
			self.not_is_informational()
		} else {
			self.snapshot()
		}
	}

	pub fn is_success(&self) -> Self {
		self.filter(|cursor, _| cursor.status().is_success())
	}

	pub fn not_is_success(&self) -> Self {
		self.filter(|cursor, _| !cursor.status().is_success())
	}

	pub fn cond_is_success(&self, condition: bool) -> Self {
		if condition {
			self.is_success()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_success(&self, condition: bool) -> Self {
		if condition {
			self.not_is_success()
		} else {
			self.snapshot()
		}
	}

	pub fn is_redirection(&self) -> Self {
		self.filter(|cursor, _| cursor.status().is_redirection())
	}

	pub fn not_is_redirection(&self) -> Self {
		self.filter(|cursor, _| !cursor.status().is_redirection())
	}

	pub fn cond_is_redirection(&self, condition: bool) -> Self {
		if condition {
			self.is_redirection()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_redirection(&self, condition: bool) -> Self {
		if condition {
			self.not_is_redirection()
		} else {
			self.snapshot()
		}
	}

	pub fn is_client_error(&self) -> Self {
		self.filter(|cursor, _| cursor.status().is_client_error())
	}

	pub fn not_is_client_error(&self) -> Self {
		self.filter(|cursor, _| !cursor.status().is_client_error())
	}

	pub fn cond_is_client_error(&self, condition: bool) -> Self {
		if condition {
			self.is_client_error()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_client_error(&self, condition: bool) -> Self {
		if condition {
			self.not_is_client_error()
		} else {
			self.snapshot()
		}
	}

	pub fn is_server_error(&self) -> Self {
		self.filter(|cursor, _| cursor.status().is_server_error())
	}

	pub fn not_is_server_error(&self) -> Self {
		self.filter(|cursor, _| !cursor.status().is_server_error())
	}

	pub fn cond_is_server_error(&self, condition: bool) -> Self {
		if condition {
			self.is_server_error()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_server_error(&self, condition: bool) -> Self {
		if condition {
			self.not_is_server_error()
		} else {
			self.snapshot()
		}
	}

	pub fn status_eq(&self, code: StatusCode) -> Self {
		self.filter(|cursor, _| cursor.status() == code)
	}

	pub fn status_not_eq(&self, code: StatusCode) -> Self {
		self.filter(|cursor, _| cursor.status() != code)
	}

	pub fn cond_status_eq(&self, condition: bool, code: StatusCode) -> Self {
		if condition {
			self.status_eq(code)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_status_not_eq(&self, condition: bool, code: StatusCode) -> Self {
		if condition {
			self.status_not_eq(code)
		} else {
			self.snapshot()
		}
	}

	pub fn status_gt(&self, code: StatusCode) -> Self {
		self.filter(|cursor, _| cursor.status() > code)
	}

	pub fn status_not_gt(&self, code: StatusCode) -> Self {
		self.filter(|cursor, _| cursor.status() <= code)
	}

	pub fn cond_status_gt(&self, condition: bool, code: StatusCode) -> Self {
		if condition {
			self.status_gt(code)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_status_not_gt(&self, condition: bool, code: StatusCode) -> Self {
		if condition {
			self.status_not_gt(code)
		} else {
			self.snapshot()
		}
	}

	pub fn status_lt(&self, code: StatusCode) -> Self {
		self.filter(|cursor, _| cursor.status() < code)
	}

	pub fn status_not_lt(&self, code: StatusCode) -> Self {
		self.filter(|cursor, _| cursor.status() >= code)
	}

	pub fn cond_status_lt(&self, condition: bool, code: StatusCode) -> Self {
		if condition {
			self.status_lt(code)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_status_not_lt(&self, condition: bool, code: StatusCode) -> Self {
		if condition {
			self.status_not_lt(code)
		} else {
			self.snapshot()
		}
	}

	pub fn status_ge(&self, code: StatusCode) -> Self {
		self.filter(|cursor, _| cursor.status() >= code)
	}

	pub fn status_not_ge(&self, code: StatusCode) -> Self {
		self.filter(|cursor, _| cursor.status() < code)
	}

	pub fn cond_status_ge(&self, condition: bool, code: StatusCode) -> Self {
		if condition {
			self.status_ge(code)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_status_not_ge(&self, condition: bool, code: StatusCode) -> Self {
		if condition {
			self.status_not_ge(code)
		} else {
			self.snapshot()
		}
	}

	pub fn status_le(&self, code: StatusCode) -> Self {
		self.filter(|cursor, _| cursor.status() <= code)
	}

	pub fn status_not_le(&self, code: StatusCode) -> Self {
		self.filter(|cursor, _| cursor.status() > code)
	}

	pub fn cond_status_le(&self, condition: bool, code: StatusCode) -> Self {
		if condition {
			self.status_le(code)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_status_not_le(&self, condition: bool, code: StatusCode) -> Self {
		if condition {
			self.status_not_le(code)
		} else {
			self.snapshot()
		}
	}

	pub fn version_eq(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() == v)
	}

	pub fn version_not_eq(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() != v)
	}

	pub fn cond_version_eq(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_eq(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_version_not_eq(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_not_eq(v)
		} else {
			self.snapshot()
		}
	}

	pub fn version_gt(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() > v)
	}

	pub fn version_not_gt(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() <= v)
	}

	pub fn cond_version_gt(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_gt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_version_not_gt(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_not_gt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn version_lt(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() < v)
	}

	pub fn version_not_lt(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() >= v)
	}

	pub fn cond_version_lt(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_lt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_version_not_lt(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_not_lt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn version_ge(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() >= v)
	}

	pub fn version_not_ge(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() < v)
	}

	pub fn cond_version_ge(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_ge(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_version_not_ge(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_not_ge(v)
		} else {
			self.snapshot()
		}
	}

	pub fn version_le(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() <= v)
	}

	pub fn version_not_le(&self, v: Version) -> Self {
		self.filter(|cursor, _| cursor.version() > v)
	}

	pub fn cond_version_le(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_le(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_version_not_le(&self, condition: bool, v: Version) -> Self {
		if condition {
			self.version_not_le(v)
		} else {
			self.snapshot()
		}
	}

	pub fn content_length_eq(&self, v: u64) -> Self {
		self.filter(|cursor, _| cursor.content_length() == Some(v))
	}

	pub fn content_length_not_eq(&self, v: u64) -> Self {
		self.filter(|cursor, _| cursor.content_length() != Some(v))
	}

	pub fn cond_content_length_eq(&self, condition: bool, v: u64) -> Self {
		if condition {
			self.content_length_eq(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_content_length_not_eq(&self, condition: bool, v: u64) -> Self {
		if condition {
			self.content_length_not_eq(v)
		} else {
			self.snapshot()
		}
	}

	pub fn content_length_gt(&self, v: u64) -> Self {
		self.filter(|cursor, _| cursor.content_length().map(|len| len > v).unwrap_or(false))
	}

	pub fn content_length_not_gt(&self, v: u64) -> Self {
		self.filter(|cursor, _| !cursor.content_length().map(|len| len > v).unwrap_or(false))
	}

	pub fn cond_content_length_gt(&self, condition: bool, v: u64) -> Self {
		if condition {
			self.content_length_gt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_content_length_not_gt(&self, condition: bool, v: u64) -> Self {
		if condition {
			self.content_length_not_gt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn content_length_lt(&self, v: u64) -> Self {
		self.filter(|cursor, _| cursor.content_length().map(|len| len < v).unwrap_or(false))
	}

	pub fn content_length_not_lt(&self, v: u64) -> Self {
		self.filter(|cursor, _| !cursor.content_length().map(|len| len < v).unwrap_or(false))
	}

	pub fn cond_content_length_lt(&self, condition: bool, v: u64) -> Self {
		if condition {
			self.content_length_lt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_content_length_not_lt(&self, condition: bool, v: u64) -> Self {
		if condition {
			self.content_length_not_lt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn content_length_ge(&self, v: u64) -> Self {
		self.filter(|cursor, _| cursor.content_length().map(|len| len >= v).unwrap_or(false))
	}

	pub fn content_length_not_ge(&self, v: u64) -> Self {
		self.filter(|cursor, _| !cursor.content_length().map(|len| len >= v).unwrap_or(false))
	}

	pub fn cond_content_length_ge(&self, condition: bool, v: u64) -> Self {
		if condition {
			self.content_length_ge(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_content_length_not_ge(&self, condition: bool, v: u64) -> Self {
		if condition {
			self.content_length_not_ge(v)
		} else {
			self.snapshot()
		}
	}

	pub fn content_length_le(&self, v: u64) -> Self {
		self.filter(|cursor, _| cursor.content_length().map(|len| len <= v).unwrap_or(false))
	}

	pub fn content_length_not_le(&self, v: u64) -> Self {
		self.filter(|cursor, _| !cursor.content_length().map(|len| len <= v).unwrap_or(false))
	}

	pub fn cond_content_length_le(&self, condition: bool, v: u64) -> Self {
		if condition {
			self.content_length_le(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_content_length_not_le(&self, condition: bool, v: u64) -> Self {
		if condition {
			self.content_length_not_le(v)
		} else {
			self.snapshot()
		}
	}
}

// ─── Url ──────────────────────────────────────────────────────────────────────

impl<'a> AsSelector<'a, Url, ()> for Url {
	fn as_selector(&'a self) -> Selector<'a, Url, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, Url, P> {
	pub fn starts_with(&self, pat: &str) -> Self {
		self.filter(|cursor, _| cursor.as_str().starts_with(pat))
	}

	pub fn not_starts_with(&self, pat: &str) -> Self {
		self.filter(|cursor, _| !cursor.as_str().starts_with(pat))
	}

	pub fn cond_starts_with(&self, condition: bool, pat: &str) -> Self {
		if condition {
			self.starts_with(pat)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_starts_with(&self, condition: bool, pat: &str) -> Self {
		if condition {
			self.not_starts_with(pat)
		} else {
			self.snapshot()
		}
	}

	pub fn ends_with(&self, pat: &str) -> Self {
		self.filter(|cursor, _| cursor.as_str().ends_with(pat))
	}

	pub fn not_ends_with(&self, pat: &str) -> Self {
		self.filter(|cursor, _| !cursor.as_str().ends_with(pat))
	}

	pub fn cond_ends_with(&self, condition: bool, pat: &str) -> Self {
		if condition {
			self.ends_with(pat)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_ends_with(&self, condition: bool, pat: &str) -> Self {
		if condition {
			self.not_ends_with(pat)
		} else {
			self.snapshot()
		}
	}

	pub fn contains(&self, pat: &str) -> Self {
		self.filter(|cursor, _| cursor.as_str().contains(pat))
	}

	pub fn not_contains(&self, pat: &str) -> Self {
		self.filter(|cursor, _| !cursor.as_str().contains(pat))
	}

	pub fn cond_contains(&self, condition: bool, pat: &str) -> Self {
		if condition {
			self.contains(pat)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_contains(&self, condition: bool, pat: &str) -> Self {
		if condition {
			self.not_contains(pat)
		} else {
			self.snapshot()
		}
	}

	pub fn empty(&self) -> Self {
		self.filter(|cursor, _| cursor.as_str().is_empty())
	}

	pub fn not_empty(&self) -> Self {
		self.filter(|cursor, _| !cursor.as_str().is_empty())
	}

	pub fn cond_empty(&self, condition: bool) -> Self {
		if condition {
			self.empty()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_empty(&self, condition: bool) -> Self {
		if condition {
			self.not_empty()
		} else {
			self.snapshot()
		}
	}

	pub fn path(&self) -> Selector<'a, str, Self> {
		self.route_to(|cursor, _| Some(cursor.path()))
	}

	pub fn query(&self) -> Selector<'a, str, Self> {
		self.route_to(|cursor, _| cursor.query())
	}

	pub fn host(&self) -> Selector<'a, str, Self> {
		self.route_to(|cursor, _| cursor.host_str())
	}

	pub fn scheme(&self) -> Selector<'a, str, Self> {
		self.route_to(|cursor, _| Some(cursor.scheme()))
	}

	pub fn port_eq(&self, v: u16) -> Self {
		self.filter(|cursor, _| cursor.port() == Some(v))
	}

	pub fn port_not_eq(&self, v: u16) -> Self {
		self.filter(|cursor, _| cursor.port() != Some(v))
	}

	pub fn cond_port_eq(&self, condition: bool, v: u16) -> Self {
		if condition {
			self.port_eq(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_port_not_eq(&self, condition: bool, v: u16) -> Self {
		if condition {
			self.port_not_eq(v)
		} else {
			self.snapshot()
		}
	}

	pub fn port_gt(&self, v: u16) -> Self {
		self.filter(|cursor, _| cursor.port().map(|p| p > v).unwrap_or(false))
	}

	pub fn port_not_gt(&self, v: u16) -> Self {
		self.filter(|cursor, _| !cursor.port().map(|p| p > v).unwrap_or(false))
	}

	pub fn cond_port_gt(&self, condition: bool, v: u16) -> Self {
		if condition {
			self.port_gt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_port_not_gt(&self, condition: bool, v: u16) -> Self {
		if condition {
			self.port_not_gt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn port_lt(&self, v: u16) -> Self {
		self.filter(|cursor, _| cursor.port().map(|p| p < v).unwrap_or(false))
	}

	pub fn port_not_lt(&self, v: u16) -> Self {
		self.filter(|cursor, _| !cursor.port().map(|p| p < v).unwrap_or(false))
	}

	pub fn cond_port_lt(&self, condition: bool, v: u16) -> Self {
		if condition {
			self.port_lt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_port_not_lt(&self, condition: bool, v: u16) -> Self {
		if condition {
			self.port_not_lt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn port_ge(&self, v: u16) -> Self {
		self.filter(|cursor, _| cursor.port().map(|p| p >= v).unwrap_or(false))
	}

	pub fn port_not_ge(&self, v: u16) -> Self {
		self.filter(|cursor, _| !cursor.port().map(|p| p >= v).unwrap_or(false))
	}

	pub fn cond_port_ge(&self, condition: bool, v: u16) -> Self {
		if condition {
			self.port_ge(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_port_not_ge(&self, condition: bool, v: u16) -> Self {
		if condition {
			self.port_not_ge(v)
		} else {
			self.snapshot()
		}
	}

	pub fn port_le(&self, v: u16) -> Self {
		self.filter(|cursor, _| cursor.port().map(|p| p <= v).unwrap_or(false))
	}

	pub fn port_not_le(&self, v: u16) -> Self {
		self.filter(|cursor, _| !cursor.port().map(|p| p <= v).unwrap_or(false))
	}

	pub fn cond_port_le(&self, condition: bool, v: u16) -> Self {
		if condition {
			self.port_le(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_port_not_le(&self, condition: bool, v: u16) -> Self {
		if condition {
			self.port_not_le(v)
		} else {
			self.snapshot()
		}
	}
}

// ─── Body ─────────────────────────────────────────────────────────────────────

impl<'a> AsSelector<'a, Body, ()> for Body {
	fn as_selector(&'a self) -> Selector<'a, Body, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, Body, P> {
	pub fn empty(&self) -> Self {
		self.filter(|cursor, _| {
			cursor
				.as_bytes()
				.map(|bytes| bytes.is_empty())
				.unwrap_or(false)
		})
	}

	pub fn not_empty(&self) -> Self {
		self.filter(|cursor, _| {
			!cursor
				.as_bytes()
				.map(|bytes| bytes.is_empty())
				.unwrap_or(false)
		})
	}

	pub fn cond_empty(&self, condition: bool) -> Self {
		if condition {
			self.empty()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_empty(&self, condition: bool) -> Self {
		if condition {
			self.not_empty()
		} else {
			self.snapshot()
		}
	}

	pub fn starts_with(&self, pat: &[u8]) -> Self {
		self.filter(|cursor, _| {
			cursor
				.as_bytes()
				.map(|bytes| bytes.starts_with(pat))
				.unwrap_or(false)
		})
	}

	pub fn not_starts_with(&self, pat: &[u8]) -> Self {
		self.filter(|cursor, _| {
			!cursor
				.as_bytes()
				.map(|bytes| bytes.starts_with(pat))
				.unwrap_or(false)
		})
	}

	pub fn cond_starts_with(&self, condition: bool, pat: &[u8]) -> Self {
		if condition {
			self.starts_with(pat)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_starts_with(&self, condition: bool, pat: &[u8]) -> Self {
		if condition {
			self.not_starts_with(pat)
		} else {
			self.snapshot()
		}
	}

	pub fn ends_with(&self, pat: &[u8]) -> Self {
		self.filter(|cursor, _| {
			cursor
				.as_bytes()
				.map(|bytes| bytes.ends_with(pat))
				.unwrap_or(false)
		})
	}

	pub fn not_ends_with(&self, pat: &[u8]) -> Self {
		self.filter(|cursor, _| {
			!cursor
				.as_bytes()
				.map(|bytes| bytes.ends_with(pat))
				.unwrap_or(false)
		})
	}

	pub fn cond_ends_with(&self, condition: bool, pat: &[u8]) -> Self {
		if condition {
			self.ends_with(pat)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_ends_with(&self, condition: bool, pat: &[u8]) -> Self {
		if condition {
			self.not_ends_with(pat)
		} else {
			self.snapshot()
		}
	}

	pub fn contains(&self, pat: &[u8]) -> Self {
		self.filter(|cursor, _| {
			cursor
				.as_bytes()
				.map(|bytes| pat.is_empty() || bytes.windows(pat.len()).any(|window| window == pat))
				.unwrap_or(false)
		})
	}

	pub fn not_contains(&self, pat: &[u8]) -> Self {
		self.filter(|cursor, _| {
			!cursor
				.as_bytes()
				.map(|bytes| pat.is_empty() || bytes.windows(pat.len()).any(|window| window == pat))
				.unwrap_or(false)
		})
	}

	pub fn cond_contains(&self, condition: bool, pat: &[u8]) -> Self {
		if condition {
			self.contains(pat)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_contains(&self, condition: bool, pat: &[u8]) -> Self {
		if condition {
			self.not_contains(pat)
		} else {
			self.snapshot()
		}
	}
}
