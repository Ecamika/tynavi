use crate::{
	selector::Selector,
	traits::{AsSelector, SelectorInstance, Snapshot, Unmatch},
};

impl<'a> AsSelector<'a, &'a str, ()> for &'a str {
	fn as_selector(&'a self) -> Selector<'a, &'a str, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, &str, P> {
	pub fn starts_with(&self, pat: &str) -> Self {
		if let Some(cursor) = self.cursor
			&& !cursor.starts_with(pat)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_starts_with(&self, pat: &str) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor.starts_with(pat)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
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
		if let Some(cursor) = self.cursor
			&& !cursor.ends_with(pat)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_ends_with(&self, pat: &str) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor.ends_with(pat)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
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
		if let Some(cursor) = self.cursor
			&& !cursor.contains(pat)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_contains(&self, pat: &str) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor.contains(pat)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
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

	pub fn eq(&self, v: &str) -> Self {
		if let Some(cursor) = self.cursor
			&& *cursor != v
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_eq(&self, v: &str) -> Self {
		if let Some(cursor) = self.cursor
			&& *cursor == v
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_eq(&self, condition: bool, v: &str) -> Self {
		if condition {
			self.eq(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_eq(&self, condition: bool, v: &str) -> Self {
		if condition {
			self.not_eq(v)
		} else {
			self.snapshot()
		}
	}

	pub fn empty(&self) -> Self {
		if let Some(cursor) = self.cursor
			&& !cursor.is_empty()
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_empty(&self) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor.is_empty()
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
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

	pub fn contains_char(&self, char: char) -> Self {
		if let Some(cursor) = self.cursor
			&& !cursor.contains(char)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_contains_char(&self, char: char) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor.contains(char)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_contains_char(&self, condition: bool, char: char) -> Self {
		if condition {
			self.contains_char(char)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_contains_char(&self, condition: bool, char: char) -> Self {
		if condition {
			self.not_contains_char(char)
		} else {
			self.snapshot()
		}
	}
}

impl<'a> AsSelector<'a, String, ()> for String {
	fn as_selector(&'a self) -> Selector<'a, String, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, String, P> {
	pub fn starts_with(&self, pat: &str) -> Self {
		if let Some(cursor) = self.cursor
			&& !cursor.starts_with(pat)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_starts_with(&self, pat: &str) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor.starts_with(pat)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
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
		if let Some(cursor) = self.cursor
			&& !cursor.ends_with(pat)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_ends_with(&self, pat: &str) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor.ends_with(pat)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
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
		if let Some(cursor) = self.cursor
			&& !cursor.contains(pat)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_contains(&self, pat: &str) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor.contains(pat)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
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

	pub fn eq(&self, v: &str) -> Self {
		if let Some(cursor) = self.cursor
			&& *cursor != v
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_eq(&self, v: &str) -> Self {
		if let Some(cursor) = self.cursor
			&& *cursor == v
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_eq(&self, condition: bool, v: &str) -> Self {
		if condition {
			self.eq(v)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_eq(&self, condition: bool, v: &str) -> Self {
		if condition {
			self.not_eq(v)
		} else {
			self.snapshot()
		}
	}

	pub fn empty(&self) -> Self {
		if let Some(cursor) = self.cursor
			&& !cursor.is_empty()
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_empty(&self) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor.is_empty()
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
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

	pub fn contains_char(&self, char: char) -> Self {
		if let Some(cursor) = self.cursor
			&& !cursor.contains(char)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn not_contains_char(&self, char: char) -> Self {
		if let Some(cursor) = self.cursor
			&& cursor.contains(char)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_contains_char(&self, condition: bool, char: char) -> Self {
		if condition {
			self.contains_char(char)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_contains_char(&self, condition: bool, char: char) -> Self {
		if condition {
			self.not_contains_char(char)
		} else {
			self.snapshot()
		}
	}
}
