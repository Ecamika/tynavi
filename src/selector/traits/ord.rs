use crate::selector::Selector;
use crate::traits::{SelectorInstance, Snapshot, Unmatch};

impl<'a, C: PartialOrd, P: SelectorInstance> Selector<'a, C, P> {
	pub fn gt(&self, v: &C) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor <= v
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_gt(&self, v: &C) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor > v
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_gt(&self, condition: bool, v: &C) -> Self {
		if condition {
			self.gt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_gt(&self, condition: bool, v: &C) -> Self {
		if condition {
			self.not_gt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn lt(&self, v: &C) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor >= v
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_lt(&self, v: &C) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor < v
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_lt(&self, condition: bool, v: &C) -> Self {
		if condition {
			self.lt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_lt(&self, condition: bool, v: &C) -> Self {
		if condition {
			self.not_lt(v)
		} else {
			self.snapshot()
		}
	}

	pub fn ge(&self, v: &C) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor < v
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_ge(&self, v: &C) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor >= v
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_ge(&self, condition: bool, v: &C) -> Self {
		if condition {
			self.ge(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_ge(&self, condition: bool, v: &C) -> Self {
		if condition {
			self.not_ge(v)
		} else {
			self.snapshot()
		}
	}

	pub fn le(&self, v: &C) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor > v
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_le(&self, v: &C) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor <= v
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_le(&self, condition: bool, v: &C) -> Self {
		if condition {
			self.le(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_le(&self, condition: bool, v: &C) -> Self {
		if condition {
			self.not_le(v)
		} else {
			self.snapshot()
		}
	}

	pub fn in_open_range(&self, min: &C, max: &C) -> Self {
		self.gt(min).lt(max)
	}

	pub fn not_in_open_range(&self, min: &C, max: &C) -> Self {
		let min_sel = self.snapshot().le(min);
		let max_sel = self.snapshot().ge(max);
		min_sel.or_a_parent_a(max_sel)
	}

	pub fn cond_in_open_range(&self, condition: bool, min: &C, max: &C) -> Self {
		if condition {
			self.in_open_range(min, max)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_in_open_range(&self, condition: bool, min: &C, max: &C) -> Self {
		if condition {
			self.not_in_open_range(min, max)
		} else {
			self.snapshot()
		}
	}

	pub fn in_closed_range(&self, min: &C, max: &C) -> Self {
		self.ge(min).le(max)
	}

	pub fn not_in_closed_range(&self, min: &C, max: &C) -> Self {
		let min_sel = self.snapshot().lt(min);
		let max_sel = self.snapshot().gt(max);
		min_sel.or_a_parent_a(max_sel)
	}

	pub fn cond_in_closed_range(&self, condition: bool, min: &C, max: &C) -> Self {
		if condition {
			self.in_closed_range(min, max)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_in_closed_range(&self, condition: bool, min: &C, max: &C) -> Self {
		if condition {
			self.not_in_closed_range(min, max)
		} else {
			self.snapshot()
		}
	}

	pub fn in_left_open_range(&self, min: &C, max: &C) -> Self {
		self.gt(min).le(max)
	}

	pub fn not_in_left_open_range(&self, min: &C, max: &C) -> Self {
		let min_sel = self.snapshot().le(min);
		let max_sel = self.snapshot().gt(max);
		min_sel.or_a_parent_a(max_sel)
	}

	pub fn cond_in_left_open_range(&self, condition: bool, min: &C, max: &C) -> Self {
		if condition {
			self.in_left_open_range(min, max)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_in_left_open_range(&self, condition: bool, min: &C, max: &C) -> Self {
		if condition {
			self.not_in_left_open_range(min, max)
		} else {
			self.snapshot()
		}
	}

	pub fn in_right_open_range(&self, min: &C, max: &C) -> Self {
		self.ge(min).lt(max)
	}

	pub fn not_in_right_open_range(&self, min: &C, max: &C) -> Self {
		let min_sel = self.snapshot().lt(min);
		let max_sel = self.snapshot().ge(max);
		min_sel.or_a_parent_a(max_sel)
	}

	pub fn cond_in_right_open_range(&self, condition: bool, min: &C, max: &C) -> Self {
		if condition {
			self.in_right_open_range(min, max)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_in_right_open_range(&self, condition: bool, min: &C, max: &C) -> Self {
		if condition {
			self.not_in_right_open_range(min, max)
		} else {
			self.snapshot()
		}
	}
}
