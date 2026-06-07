use crate::selector::Selector;
use crate::traits::{AsSelector, SelectorInstance, Snapshot};
use serde_json::{Map, Value};

impl<'a> AsSelector<'a, Map<String, Value>, ()> for Map<String, Value> {
	fn as_selector(&'a self) -> Selector<'a, Map<String, Value>, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, Map<String, Value>, P> {
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

	pub fn keyof(&self, key: &str) -> Selector<'a, Value, Self> {
		self.route_to(|cursor, _| cursor.get(key))
	}

	pub fn contains_key(&self, key: &str) -> Self {
		self.filter(|cursor, _| cursor.contains_key(key))
	}

	pub fn not_contains_key(&self, key: &str) -> Self {
		self.filter(|cursor, _| !cursor.contains_key(key))
	}

	pub fn cond_contains_key(&self, condition: bool, key: &str) -> Self {
		if condition {
			self.contains_key(key)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_contains_key(&self, condition: bool, key: &str) -> Self {
		if condition {
			self.not_contains_key(key)
		} else {
			self.snapshot()
		}
	}
}
