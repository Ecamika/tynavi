use tungstenite::Bytes;
use tungstenite::protocol::Message;
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::frame::{CloseFrame, Utf8Bytes};

use crate::selector::Selector;
use crate::traits::{AsSelector, SelectorInstance, Snapshot};

// ─── Message ────────────────────────────────────────────────────────────────

impl<'a> AsSelector<'a, Message, ()> for Message {
	fn as_selector(&'a self) -> Selector<'a, Message, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, Message, P> {
	pub fn is_text(&self) -> Self {
		self.filter(|cursor, _| cursor.is_text())
	}

	pub fn not_is_text(&self) -> Self {
		self.filter(|cursor, _| !cursor.is_text())
	}

	pub fn cond_is_text(&self, condition: bool) -> Self {
		if condition {
			self.is_text()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_text(&self, condition: bool) -> Self {
		if condition {
			self.not_is_text()
		} else {
			self.snapshot()
		}
	}

	pub fn is_binary(&self) -> Self {
		self.filter(|cursor, _| cursor.is_binary())
	}

	pub fn not_is_binary(&self) -> Self {
		self.filter(|cursor, _| !cursor.is_binary())
	}

	pub fn cond_is_binary(&self, condition: bool) -> Self {
		if condition {
			self.is_binary()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_binary(&self, condition: bool) -> Self {
		if condition {
			self.not_is_binary()
		} else {
			self.snapshot()
		}
	}

	pub fn is_ping(&self) -> Self {
		self.filter(|cursor, _| cursor.is_ping())
	}

	pub fn not_is_ping(&self) -> Self {
		self.filter(|cursor, _| !cursor.is_ping())
	}

	pub fn cond_is_ping(&self, condition: bool) -> Self {
		if condition {
			self.is_ping()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_ping(&self, condition: bool) -> Self {
		if condition {
			self.not_is_ping()
		} else {
			self.snapshot()
		}
	}

	pub fn is_pong(&self) -> Self {
		self.filter(|cursor, _| cursor.is_pong())
	}

	pub fn not_is_pong(&self) -> Self {
		self.filter(|cursor, _| !cursor.is_pong())
	}

	pub fn cond_is_pong(&self, condition: bool) -> Self {
		if condition {
			self.is_pong()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_pong(&self, condition: bool) -> Self {
		if condition {
			self.not_is_pong()
		} else {
			self.snapshot()
		}
	}

	pub fn is_close(&self) -> Self {
		self.filter(|cursor, _| cursor.is_close())
	}

	pub fn not_is_close(&self) -> Self {
		self.filter(|cursor, _| !cursor.is_close())
	}

	pub fn cond_is_close(&self, condition: bool) -> Self {
		if condition {
			self.is_close()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_close(&self, condition: bool) -> Self {
		if condition {
			self.not_is_close()
		} else {
			self.snapshot()
		}
	}

	pub fn is_frame(&self) -> Self {
		self.filter(|cursor, _| matches!(cursor, Message::Frame(_)))
	}

	pub fn not_is_frame(&self) -> Self {
		self.filter(|cursor, _| !matches!(cursor, Message::Frame(_)))
	}

	pub fn cond_is_frame(&self, condition: bool) -> Self {
		if condition {
			self.is_frame()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_frame(&self, condition: bool) -> Self {
		if condition {
			self.not_is_frame()
		} else {
			self.snapshot()
		}
	}

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

	pub fn as_text(&self) -> Selector<'a, Utf8Bytes, Self> {
		self.route_to(|cursor, _| match cursor {
			Message::Text(v) => Some(v),
			_ => None,
		})
	}

	pub fn as_binary(&self) -> Selector<'a, Bytes, Self> {
		self.route_to(|cursor, _| match cursor {
			Message::Binary(v) => Some(v),
			_ => None,
		})
	}

	pub fn as_ping(&self) -> Selector<'a, Bytes, Self> {
		self.route_to(|cursor, _| match cursor {
			Message::Ping(v) => Some(v),
			_ => None,
		})
	}

	pub fn as_pong(&self) -> Selector<'a, Bytes, Self> {
		self.route_to(|cursor, _| match cursor {
			Message::Pong(v) => Some(v),
			_ => None,
		})
	}

	pub fn as_close(&self) -> Selector<'a, Option<CloseFrame>, Self> {
		self.route_to(|cursor, _| match cursor {
			Message::Close(v) => Some(v),
			_ => None,
		})
	}
}

// ─── Utf8Bytes ──────────────────────────────────────────────────────────────

impl<'a> AsSelector<'a, Utf8Bytes, ()> for Utf8Bytes {
	fn as_selector(&'a self) -> Selector<'a, Utf8Bytes, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, Utf8Bytes, P> {
	pub fn starts_with(&self, pat: &str) -> Self {
		self.filter(|cursor, _| cursor.starts_with(pat))
	}

	pub fn not_starts_with(&self, pat: &str) -> Self {
		self.filter(|cursor, _| !cursor.starts_with(pat))
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
		self.filter(|cursor, _| cursor.ends_with(pat))
	}

	pub fn not_ends_with(&self, pat: &str) -> Self {
		self.filter(|cursor, _| !cursor.ends_with(pat))
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
		self.filter(|cursor, _| cursor.contains(pat))
	}

	pub fn not_contains(&self, pat: &str) -> Self {
		self.filter(|cursor, _| !cursor.contains(pat))
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

	pub fn contains_char(&self, ch: char) -> Self {
		self.filter(|cursor, _| cursor.contains(ch))
	}

	pub fn not_contains_char(&self, ch: char) -> Self {
		self.filter(|cursor, _| !cursor.contains(ch))
	}

	pub fn cond_contains_char(&self, condition: bool, ch: char) -> Self {
		if condition {
			self.contains_char(ch)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_contains_char(&self, condition: bool, ch: char) -> Self {
		if condition {
			self.not_contains_char(ch)
		} else {
			self.snapshot()
		}
	}
}

// ─── CloseFrame ─────────────────────────────────────────────────────────────

impl<'a> AsSelector<'a, CloseFrame, ()> for CloseFrame {
	fn as_selector(&'a self) -> Selector<'a, CloseFrame, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, CloseFrame, P> {
	pub fn code(&self) -> Selector<'a, CloseCode, Self> {
		self.route_to(|cursor, _| Some(&cursor.code))
	}

	pub fn reason(&self) -> Selector<'a, Utf8Bytes, Self> {
		self.route_to(|cursor, _| Some(&cursor.reason))
	}
}

// ─── CloseCode ──────────────────────────────────────────────────────────────

impl<'a> AsSelector<'a, CloseCode, ()> for CloseCode {
	fn as_selector(&'a self) -> Selector<'a, CloseCode, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, CloseCode, P> {
	pub fn is_allowed(&self) -> Self {
		self.filter(|cursor, _| cursor.is_allowed())
	}

	pub fn not_is_allowed(&self) -> Self {
		self.filter(|cursor, _| !cursor.is_allowed())
	}

	pub fn cond_is_allowed(&self, condition: bool) -> Self {
		if condition {
			self.is_allowed()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_allowed(&self, condition: bool) -> Self {
		if condition {
			self.not_is_allowed()
		} else {
			self.snapshot()
		}
	}
}

// ─── Bytes ──────────────────────────────────────────────────────────────────

impl<'a> AsSelector<'a, Bytes, ()> for Bytes {
	fn as_selector(&'a self) -> Selector<'a, Bytes, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, Bytes, P> {
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
}
