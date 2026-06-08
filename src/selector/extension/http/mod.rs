use crate::selector::Selector;
use crate::traits::{AsSelector, SelectorInstance, Snapshot};
use http::header::{HeaderMap, HeaderValue};
use http::{Method, Request, Response, StatusCode, Uri, Version};

// ─── Request ──────────────────────────────────────────────────────────────────

impl<'a, B> AsSelector<'a, Request<B>, ()> for Request<B> {
	fn as_selector(&'a self) -> Selector<'a, Request<B>, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, B, P: SelectorInstance> Selector<'a, Request<B>, P> {
	pub fn method(&self) -> Selector<'a, Method, Self> {
		self.route_to(|cursor, _| Some(cursor.method()))
	}

	pub fn uri(&self) -> Selector<'a, Uri, Self> {
		self.route_to(|cursor, _| Some(cursor.uri()))
	}

	pub fn headers(&self) -> Selector<'a, HeaderMap<HeaderValue>, Self> {
		self.route_to(|cursor, _| Some(cursor.headers()))
	}

	pub fn body(&self) -> Selector<'a, B, Self> {
		self.route_to(|cursor, _| Some(cursor.body()))
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

impl<'a, B> AsSelector<'a, Response<B>, ()> for Response<B> {
	fn as_selector(&'a self) -> Selector<'a, Response<B>, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, B, P: SelectorInstance> Selector<'a, Response<B>, P> {
	pub fn headers(&self) -> Selector<'a, HeaderMap<HeaderValue>, Self> {
		self.route_to(|cursor, _| Some(cursor.headers()))
	}

	pub fn body(&self) -> Selector<'a, B, Self> {
		self.route_to(|cursor, _| Some(cursor.body()))
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
}

// ─── Method ───────────────────────────────────────────────────────────────────

impl<'a> AsSelector<'a, Method, ()> for Method {
	fn as_selector(&'a self) -> Selector<'a, Method, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, Method, P> {}

// ─── StatusCode ───────────────────────────────────────────────────────────────

impl<'a> AsSelector<'a, StatusCode, ()> for StatusCode {
	fn as_selector(&'a self) -> Selector<'a, StatusCode, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, StatusCode, P> {
	pub fn is_informational(&self) -> Self {
		self.filter(|cursor, _| cursor.is_informational())
	}

	pub fn not_is_informational(&self) -> Self {
		self.filter(|cursor, _| !cursor.is_informational())
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
		self.filter(|cursor, _| cursor.is_success())
	}

	pub fn not_is_success(&self) -> Self {
		self.filter(|cursor, _| !cursor.is_success())
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
		self.filter(|cursor, _| cursor.is_redirection())
	}

	pub fn not_is_redirection(&self) -> Self {
		self.filter(|cursor, _| !cursor.is_redirection())
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
		self.filter(|cursor, _| cursor.is_client_error())
	}

	pub fn not_is_client_error(&self) -> Self {
		self.filter(|cursor, _| !cursor.is_client_error())
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
		self.filter(|cursor, _| cursor.is_server_error())
	}

	pub fn not_is_server_error(&self) -> Self {
		self.filter(|cursor, _| !cursor.is_server_error())
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
}

// ─── Version ──────────────────────────────────────────────────────────────────

impl<'a> AsSelector<'a, Version, ()> for Version {
	fn as_selector(&'a self) -> Selector<'a, Version, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, Version, P> {}

// ─── Uri ──────────────────────────────────────────────────────────────────────

impl<'a> AsSelector<'a, Uri, ()> for Uri {
	fn as_selector(&'a self) -> Selector<'a, Uri, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, Uri, P> {
	pub fn starts_with(&self, pat: &str) -> Self {
		self.filter(|cursor, _| cursor.to_string().starts_with(pat))
	}

	pub fn not_starts_with(&self, pat: &str) -> Self {
		self.filter(|cursor, _| !cursor.to_string().starts_with(pat))
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
		self.filter(|cursor, _| cursor.to_string().ends_with(pat))
	}

	pub fn not_ends_with(&self, pat: &str) -> Self {
		self.filter(|cursor, _| !cursor.to_string().ends_with(pat))
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
		self.filter(|cursor, _| cursor.to_string().contains(pat))
	}

	pub fn not_contains(&self, pat: &str) -> Self {
		self.filter(|cursor, _| !cursor.to_string().contains(pat))
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
		self.filter(|cursor, _| cursor.to_string().is_empty())
	}

	pub fn not_empty(&self) -> Self {
		self.filter(|cursor, _| !cursor.to_string().is_empty())
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
		self.route_to(|cursor, _| cursor.host())
	}

	pub fn scheme(&self) -> Selector<'a, str, Self> {
		self.route_to(|cursor, _| cursor.scheme_str())
	}

	pub fn port_eq(&self, v: u16) -> Self {
		self.filter(|cursor, _| cursor.port_u16() == Some(v))
	}

	pub fn port_not_eq(&self, v: u16) -> Self {
		self.filter(|cursor, _| cursor.port_u16() != Some(v))
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
		self.filter(|cursor, _| cursor.port_u16().map(|p| p > v).unwrap_or(false))
	}

	pub fn port_not_gt(&self, v: u16) -> Self {
		self.filter(|cursor, _| !cursor.port_u16().map(|p| p > v).unwrap_or(false))
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
		self.filter(|cursor, _| cursor.port_u16().map(|p| p < v).unwrap_or(false))
	}

	pub fn port_not_lt(&self, v: u16) -> Self {
		self.filter(|cursor, _| !cursor.port_u16().map(|p| p < v).unwrap_or(false))
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
		self.filter(|cursor, _| cursor.port_u16().map(|p| p >= v).unwrap_or(false))
	}

	pub fn port_not_ge(&self, v: u16) -> Self {
		self.filter(|cursor, _| !cursor.port_u16().map(|p| p >= v).unwrap_or(false))
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
		self.filter(|cursor, _| cursor.port_u16().map(|p| p <= v).unwrap_or(false))
	}

	pub fn port_not_le(&self, v: u16) -> Self {
		self.filter(|cursor, _| !cursor.port_u16().map(|p| p <= v).unwrap_or(false))
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

// ─── HeaderMap ────────────────────────────────────────────────────────────────

impl<'a, V> AsSelector<'a, HeaderMap<V>, ()> for HeaderMap<V> {
	fn as_selector(&'a self) -> Selector<'a, HeaderMap<V>, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, V, P: SelectorInstance> Selector<'a, HeaderMap<V>, P> {
	pub fn empty(&self) -> Self {
		self.filter(|cursor, _| cursor.is_empty())
	}

	pub fn not_empty(&self) -> Self {
		self.filter(|cursor, _| !cursor.is_empty())
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

	pub fn contains_key(&self, name: &str) -> Self {
		self.filter(|cursor, _| cursor.contains_key(name))
	}

	pub fn not_contains_key(&self, name: &str) -> Self {
		self.filter(|cursor, _| !cursor.contains_key(name))
	}

	pub fn cond_contains_key(&self, condition: bool, name: &str) -> Self {
		if condition {
			self.contains_key(name)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_contains_key(&self, condition: bool, name: &str) -> Self {
		if condition {
			self.not_contains_key(name)
		} else {
			self.snapshot()
		}
	}

	pub fn keyof(&self, name: &str) -> Selector<'a, V, Self> {
		self.route_to(|cursor, _| cursor.get(name))
	}
}

// ─── HeaderValue ──────────────────────────────────────────────────────────────

impl<'a> AsSelector<'a, HeaderValue, ()> for HeaderValue {
	fn as_selector(&'a self) -> Selector<'a, HeaderValue, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, HeaderValue, P> {
	pub fn starts_with(&self, pat: &str) -> Self {
		self.filter(|cursor, _| cursor.to_str().map(|s| s.starts_with(pat)).unwrap_or(false))
	}

	pub fn not_starts_with(&self, pat: &str) -> Self {
		self.filter(|cursor, _| !cursor.to_str().map(|s| s.starts_with(pat)).unwrap_or(false))
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
		self.filter(|cursor, _| cursor.to_str().map(|s| s.ends_with(pat)).unwrap_or(false))
	}

	pub fn not_ends_with(&self, pat: &str) -> Self {
		self.filter(|cursor, _| !cursor.to_str().map(|s| s.ends_with(pat)).unwrap_or(false))
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
		self.filter(|cursor, _| cursor.to_str().map(|s| s.contains(pat)).unwrap_or(false))
	}

	pub fn not_contains(&self, pat: &str) -> Self {
		self.filter(|cursor, _| !cursor.to_str().map(|s| s.contains(pat)).unwrap_or(false))
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
		self.filter(|cursor, _| cursor.as_bytes().is_empty())
	}

	pub fn not_empty(&self) -> Self {
		self.filter(|cursor, _| !cursor.as_bytes().is_empty())
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
}
