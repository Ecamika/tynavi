use crate::error::{SelectorNotMatched, SelectorResult};
use crate::traits::*;

pub mod number;
pub mod option;
pub mod ptr;
pub mod result;
pub mod string;
pub mod traits;

pub struct Selector<'a, Current, Parent: SelectorInstance> {
	/// 游标
	pub cursor: Option<&'a Current>,
	/// 父节点
	pub parent: Parent,
}

impl<'a, Current, Parent: SelectorInstance> Copy for Selector<'a, Current, Parent> {}

impl<'a, Current, Parent: SelectorInstance> Clone for Selector<'a, Current, Parent> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<'a, C, P: SelectorInstance> Snapshot for Selector<'a, C, P> {
	fn snapshot(&self) -> Self {
		*self
	}
}

impl<'a, C, P: SelectorInstance> Unmatch for Selector<'a, C, P> {
	fn unmatch(&self) -> Self {
		self.same_parent(None)
	}

	fn cond_unmatch(&self, condition: bool) -> Self {
		if condition {
			self.unmatch()
		} else {
			self.snapshot()
		}
	}
}

impl<'b, A, B: SelectorInstance> Selector<'b, A, B> {
	pub fn with<'a, C, P: SelectorInstance>(cursor: Option<&'a C>, parent: P) -> Selector<'a, C, P> {
		Selector { cursor, parent }
	}

	pub fn new<'a, C>(current: &'a C) -> Selector<'a, C, ()> {
		Self::with(Some(current), ())
	}

	pub fn same_parent<C>(&self, cursor: Option<&'b C>) -> Selector<'b, C, B> {
		Self::with(cursor, self.parent.snapshot())
	}
}

impl<'a, C, P: SelectorInstance> Selector<'a, C, P> {
	pub fn route_to<R>(
		&self,
		extractor: impl FnOnce(&'a C, &Self) -> Option<&'a R>,
	) -> Selector<'a, R, Self> {
		Self::with(
			self.cursor.and_then(|cursor| extractor(cursor, self)),
			self.snapshot(),
		)
	}

	pub fn backtrack(&self) -> P {
		self.parent.snapshot()
	}

	pub fn up(&self) -> P {
		if self.cursor.is_some() {
			self.parent.snapshot()
		} else {
			self.parent.snapshot().unmatch()
		}
	}
}

impl<'a, C, P: SelectorInstance> Selector<'a, C, P> {
	pub fn replace<T>(&self, v: &'a T) -> Selector<'a, T, P> {
		self.same_parent(Some(v))
	}

	pub fn map<R>(&self, f: impl FnOnce(&'a C, &Self) -> &'a R) -> Selector<'a, R, P> {
		self.same_parent(self.cursor.map(|cursor| f(cursor, self)))
	}

	pub fn require_matched(&self) -> SelectorResult<Self> {
		if self.cursor.is_some() {
			Ok(self.snapshot())
		} else {
			Err(SelectorNotMatched)
		}
	}

	pub fn filter(&self, f: impl FnOnce(&'a C, &Self) -> bool) -> Self {
		if let Some(cursor) = self.cursor
			&& !f(cursor, self)
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub fn cond_filter(&self, condition: bool, f: impl FnOnce(&'a C, &Self) -> bool) -> Self {
		if condition {
			self.filter(f)
		} else {
			self.snapshot()
		}
	}

	pub async fn filter_async(&self, f: impl AsyncFnOnce(&'a C, &Self) -> bool) -> Self {
		if let Some(cursor) = self.cursor
			&& !f(cursor, self).await
		{
			self.unmatch()
		} else {
			self.snapshot()
		}
	}

	pub async fn cond_filter_async(
		&self,
		condition: bool,
		f: impl AsyncFnOnce(&'a C, &Self) -> bool,
	) -> Self {
		if condition {
			self.filter_async(f).await
		} else {
			self.snapshot()
		}
	}
}

impl<'a, C, P: SelectorInstance> Selector<'a, C, P> {
	pub fn select(&self) -> Option<&'a C> {
		self.cursor
	}

	pub fn parent(&self) -> P {
		self.parent.snapshot()
	}

	pub fn is_matched(&self) -> bool {
		self.cursor.is_some()
	}

	pub fn extract<R>(&self, f: impl FnOnce(&'a C, &Self) -> R) -> Option<R> {
		self.cursor.map(|cursor| f(cursor, self))
	}

	pub fn cond_extract<R>(&self, condition: bool, f: impl FnOnce(&'a C, &Self) -> R) -> Option<R> {
		if condition { self.extract(f) } else { None }
	}

	pub async fn extract_async<R>(&self, f: impl AsyncFnOnce(&'a C, &Self) -> R) -> Option<R> {
		if let Some(cursor) = self.cursor {
			Some(f(cursor, self).await)
		} else {
			None
		}
	}

	pub async fn cond_extract_async<R>(
		&self,
		condition: bool,
		f: impl AsyncFnOnce(&'a C, &Self) -> R,
	) -> Option<R> {
		if condition {
			self.extract_async(f).await
		} else {
			None
		}
	}

	pub fn inspect(&self, f: impl FnOnce(Option<&'a C>, &Self)) -> Self {
		f(self.cursor, self);
		self.snapshot()
	}

	pub fn cond_inspect(&self, condition: bool, f: impl FnOnce(Option<&'a C>, &Self)) -> Self {
		if condition {
			self.inspect(f)
		} else {
			self.snapshot()
		}
	}

	pub async fn inspect_async(&self, f: impl AsyncFnOnce(Option<&'a C>, &Self)) -> Self {
		f(self.cursor, self).await;
		self.snapshot()
	}

	pub async fn cond_inspect_async(
		&self,
		condition: bool,
		f: impl AsyncFnOnce(Option<&'a C>, &Self),
	) -> Self {
		if condition {
			self.inspect_async(f).await
		} else {
			self.snapshot()
		}
	}

	pub fn inspect_cursor(&self, f: impl FnOnce(&'a C, &Self)) -> Self {
		if let Some(cursor) = self.cursor {
			f(cursor, self)
		}
		self.snapshot()
	}

	pub fn cond_inspect_cursor(&self, condition: bool, f: impl FnOnce(&'a C, &Self)) -> Self {
		if condition {
			self.inspect_cursor(f)
		} else {
			self.snapshot()
		}
	}

	pub async fn inspect_cursor_async(&self, f: impl AsyncFnOnce(&'a C, &Self)) -> Self {
		if let Some(cursor) = self.cursor {
			f(cursor, self).await
		}
		self.snapshot()
	}

	pub async fn cond_inspect_cursor_async(
		&self,
		condition: bool,
		f: impl AsyncFnOnce(&'a C, &Self),
	) -> Self {
		if condition {
			self.inspect_cursor_async(f).await
		} else {
			self.snapshot()
		}
	}
}

impl<'a, C, P: SelectorInstance> Selector<'a, C, P> {
	pub fn or_a_parent_a<T: SelectorInstance>(&self, selb: Selector<'a, C, T>) -> Selector<'a, C, P> {
		let a_cursor = self.cursor;
		let b_cursor = selb.cursor;
		let cursor = a_cursor.or(b_cursor);
		Selector {
			cursor,
			parent: self.parent,
		}
	}

	pub fn or_a_parent_b<T: SelectorInstance>(&self, selb: Selector<'a, C, T>) -> Selector<'a, C, T> {
		let a_cursor = self.cursor;
		let b_cursor = selb.cursor;
		let cursor = a_cursor.or(b_cursor);
		Selector {
			cursor,
			parent: selb.parent,
		}
	}

	pub fn or_b_parent_a<T: SelectorInstance>(&self, selb: Selector<'a, C, T>) -> Selector<'a, C, P> {
		let a_cursor = self.cursor;
		let b_cursor = selb.cursor;
		let cursor = b_cursor.or(a_cursor);
		Selector {
			cursor,
			parent: self.parent,
		}
	}

	pub fn or_b_parent_b<T: SelectorInstance>(&self, selb: Selector<'a, C, T>) -> Selector<'a, C, T> {
		let a_cursor = self.cursor;
		let b_cursor = selb.cursor;
		let cursor = b_cursor.or(a_cursor);
		Selector {
			cursor,
			parent: selb.parent,
		}
	}
}
