use crate::selector::Selector;
use crate::traits::{SelectorInstance, Snapshot, Unmatch};

impl<'a, C: PartialEq, P: SelectorInstance> Selector<'a, C, P> {
	pub fn eq(&self, v: &C) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor != v
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_eq(&self, v: &C) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor == v
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_eq(&self, condition: bool, v: &C) -> Self {
		if condition {
			self.eq(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_eq(&self, condition: bool, v: &C) -> Self {
		if condition {
			self.not_eq(v)
		} else {
			self.snapshot()
		}
	}
}
