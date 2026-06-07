use crate::selector::Selector;
use crate::traits::{AsSelector, SelectorInstance, Snapshot};
use serde_json::{Map, Number, Value};

pub mod map;

impl<'a> AsSelector<'a, Value, ()> for Value {
	fn as_selector(&'a self) -> Selector<'a, Value, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, Value, P> {
	pub fn is_null(&self) -> Self {
		self.filter(|cursor, _| matches!(cursor, Value::Null))
	}

	pub fn not_is_null(&self) -> Self {
		self.filter(|cursor, _| !matches!(cursor, Value::Null))
	}

	pub fn cond_is_null(&self, condition: bool) -> Self {
		if condition {
			self.is_null()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_null(&self, condition: bool) -> Self {
		if condition {
			self.not_is_null()
		} else {
			self.snapshot()
		}
	}

	pub fn is_bool(&self) -> Self {
		self.filter(|cursor, _| matches!(cursor, Value::Bool(_)))
	}

	pub fn not_is_bool(&self) -> Self {
		self.filter(|cursor, _| !matches!(cursor, Value::Bool(_)))
	}

	pub fn cond_is_bool(&self, condition: bool) -> Self {
		if condition {
			self.is_bool()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_bool(&self, condition: bool) -> Self {
		if condition {
			self.not_is_bool()
		} else {
			self.snapshot()
		}
	}

	pub fn is_number(&self) -> Self {
		self.filter(|cursor, _| matches!(cursor, Value::Number(_)))
	}

	pub fn not_is_number(&self) -> Self {
		self.filter(|cursor, _| !matches!(cursor, Value::Number(_)))
	}

	pub fn cond_is_number(&self, condition: bool) -> Self {
		if condition {
			self.is_number()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_number(&self, condition: bool) -> Self {
		if condition {
			self.not_is_number()
		} else {
			self.snapshot()
		}
	}

	pub fn is_string(&self) -> Self {
		self.filter(|cursor, _| matches!(cursor, Value::String(_)))
	}

	pub fn not_is_string(&self) -> Self {
		self.filter(|cursor, _| !matches!(cursor, Value::String(_)))
	}

	pub fn cond_is_string(&self, condition: bool) -> Self {
		if condition {
			self.is_string()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_string(&self, condition: bool) -> Self {
		if condition {
			self.not_is_string()
		} else {
			self.snapshot()
		}
	}

	pub fn is_array(&self) -> Self {
		self.filter(|cursor, _| matches!(cursor, Value::Array(_)))
	}

	pub fn not_is_array(&self) -> Self {
		self.filter(|cursor, _| !matches!(cursor, Value::Array(_)))
	}

	pub fn cond_is_array(&self, condition: bool) -> Self {
		if condition {
			self.is_array()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_array(&self, condition: bool) -> Self {
		if condition {
			self.not_is_array()
		} else {
			self.snapshot()
		}
	}

	pub fn is_object(&self) -> Self {
		self.filter(|cursor, _| matches!(cursor, Value::Object(_)))
	}

	pub fn not_is_object(&self) -> Self {
		self.filter(|cursor, _| !matches!(cursor, Value::Object(_)))
	}

	pub fn cond_is_object(&self, condition: bool) -> Self {
		if condition {
			self.is_object()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_is_object(&self, condition: bool) -> Self {
		if condition {
			self.not_is_object()
		} else {
			self.snapshot()
		}
	}

	pub fn as_null(&self) -> Self {
		self.filter(|cursor, _| matches!(cursor, Value::Null))
	}

	pub fn as_bool(&self) -> Selector<'a, bool, Self> {
		self.route_to(|cursor, _| match cursor {
			Value::Bool(v) => Some(v),
			_ => None,
		})
	}

	pub fn as_number(&self) -> Selector<'a, Number, Self> {
		self.route_to(|cursor, _| match cursor {
			Value::Number(v) => Some(v),
			_ => None,
		})
	}

	pub fn as_str(&self) -> Selector<'a, String, Self> {
		self.route_to(|cursor, _| match cursor {
			Value::String(v) => Some(v),
			_ => None,
		})
	}

	pub fn as_object(&self) -> Selector<'a, Map<String, Value>, Self> {
		self.route_to(|cursor, _| match cursor {
			Value::Object(v) => Some(v),
			_ => None,
		})
	}

	pub fn keyof(&self, key: &str) -> Selector<'a, Value, Self> {
		self.route_to(|cursor, _| match cursor {
			Value::Object(map) => map.get(key),
			_ => None,
		})
	}

	pub fn indexof(&self, index: usize) -> Selector<'a, Value, Self> {
		self.route_to(|cursor, _| match cursor {
			Value::Array(arr) => arr.get(index),
			_ => None,
		})
	}

	pub fn first(&self) -> Selector<'a, Value, Self> {
		self.route_to(|cursor, _| match cursor {
			Value::Array(arr) => arr.first(),
			_ => None,
		})
	}

	pub fn last(&self) -> Selector<'a, Value, Self> {
		self.route_to(|cursor, _| match cursor {
			Value::Array(arr) => arr.last(),
			_ => None,
		})
	}

	pub fn empty(&self) -> Self {
		self.filter(|cursor, _| match cursor {
			Value::Null => true,
			Value::String(s) => s.is_empty(),
			Value::Array(arr) => arr.is_empty(),
			Value::Object(map) => map.is_empty(),
			_ => false,
		})
	}

	pub fn not_empty(&self) -> Self {
		self.filter(|cursor, _| match cursor {
			Value::Null => false,
			Value::String(s) => !s.is_empty(),
			Value::Array(arr) => !arr.is_empty(),
			Value::Object(map) => !map.is_empty(),
			_ => true,
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
}
