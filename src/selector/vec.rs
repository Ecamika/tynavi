use crate::selector::Selector;
use crate::traits::{AsSelector, SelectorInstance, Snapshot};

impl<'a, T> AsSelector<'a, &'a [T], ()> for &[T] {
	fn as_selector(&'a self) -> Selector<'a, &'a [T], ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, T, P: SelectorInstance> Selector<'a, &'a [T], P> {
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

	pub fn first(&self) -> Selector<'a, T, Self> {
		self.route_to(|cursor, _| cursor.first())
	}

	pub fn last(&self) -> Selector<'a, T, Self> {
		self.route_to(|cursor, _| cursor.last())
	}

	pub fn indexof(&self, index: usize) -> Selector<'a, T, Self> {
		self.route_to(|cursor, _| cursor.get(index))
	}

	pub fn find(&self, mut f: impl FnMut(&'a T, &Self) -> bool) -> Selector<'a, T, Self> {
		self.route_to(|cursor, sel| cursor.iter().find(|data| f(data, sel)))
	}

	pub fn any(&self, mut f: impl FnMut(&'a T, &Self) -> bool) -> Self {
		self.filter(|cursor, sel| cursor.iter().any(|data| f(data, sel)))
	}

	pub fn not_any(&self, mut f: impl FnMut(&'a T, &Self) -> bool) -> Self {
		self.filter(|cursor, sel| !cursor.iter().any(|data| f(data, sel)))
	}

	pub fn all(&self, mut f: impl FnMut(&'a T, &Self) -> bool) -> Self {
		self.filter(|cursor, sel| cursor.iter().all(|data| f(data, sel)))
	}

	pub fn not_all(&self, mut f: impl FnMut(&'a T, &Self) -> bool) -> Self {
		self.filter(|cursor, sel| !cursor.iter().all(|data| f(data, sel)))
	}

	pub fn none(&self, mut f: impl FnMut(&'a T, &Self) -> bool) -> Self {
		self.filter(|cursor, sel| !cursor.iter().any(|data| f(data, sel)))
	}

	pub fn contains(&self, value: &T) -> Self
	where
		T: PartialEq,
	{
		self.filter(|cursor, _| cursor.contains(value))
	}

	pub fn not_contains(&self, value: &T) -> Self
	where
		T: PartialEq,
	{
		self.filter(|cursor, _| !cursor.contains(value))
	}

	pub fn cond_any(&self, condition: bool, f: impl FnMut(&'a T, &Self) -> bool) -> Self {
		if condition {
			self.any(f)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_any(&self, condition: bool, f: impl FnMut(&'a T, &Self) -> bool) -> Self {
		if condition {
			self.not_any(f)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_all(&self, condition: bool, f: impl FnMut(&'a T, &Self) -> bool) -> Self {
		if condition {
			self.all(f)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_all(&self, condition: bool, f: impl FnMut(&'a T, &Self) -> bool) -> Self {
		if condition {
			self.not_all(f)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_none(&self, condition: bool, f: impl FnMut(&'a T, &Self) -> bool) -> Self {
		if condition {
			self.none(f)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_contains(&self, condition: bool, value: &T) -> Self
	where
		T: PartialEq,
	{
		if condition {
			self.contains(value)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_contains(&self, condition: bool, value: &T) -> Self
	where
		T: PartialEq,
	{
		if condition {
			self.not_contains(value)
		} else {
			self.snapshot()
		}
	}

	pub async fn any_async(&self, mut f: impl AsyncFnMut(&'a T, &Self) -> bool) -> Self {
		self
			.filter_async(async |cursor, sel| {
				for element in cursor.iter() {
					if f(element, sel).await {
						return true;
					}
				}
				false
			})
			.await
	}

	pub async fn not_any_async(&self, mut f: impl AsyncFnMut(&'a T, &Self) -> bool) -> Self {
		self
			.filter_async(async |cursor, sel| {
				for element in cursor.iter() {
					if f(element, sel).await {
						return false;
					}
				}
				true
			})
			.await
	}

	pub async fn all_async(&self, mut f: impl AsyncFnMut(&'a T, &Self) -> bool) -> Self {
		self
			.filter_async(async |cursor, sel| {
				for element in cursor.iter() {
					if !f(element, sel).await {
						return false;
					}
				}
				true
			})
			.await
	}

	pub async fn not_all_async(&self, mut f: impl AsyncFnMut(&'a T, &Self) -> bool) -> Self {
		self
			.filter_async(async |cursor, sel| {
				for element in cursor.iter() {
					if !f(element, sel).await {
						return true;
					}
				}
				false
			})
			.await
	}

	pub async fn none_async(&self, mut f: impl AsyncFnMut(&'a T, &Self) -> bool) -> Self {
		self
			.filter_async(async |cursor, sel| {
				for element in cursor.iter() {
					if f(element, sel).await {
						return false;
					}
				}
				true
			})
			.await
	}

	pub async fn cond_any_async(
		&self,
		condition: bool,
		f: impl AsyncFnMut(&'a T, &Self) -> bool,
	) -> Self {
		if condition {
			self.any_async(f).await
		} else {
			self.snapshot()
		}
	}

	pub async fn cond_not_any_async(
		&self,
		condition: bool,
		f: impl AsyncFnMut(&'a T, &Self) -> bool,
	) -> Self {
		if condition {
			self.not_any_async(f).await
		} else {
			self.snapshot()
		}
	}

	pub async fn cond_all_async(
		&self,
		condition: bool,
		f: impl AsyncFnMut(&'a T, &Self) -> bool,
	) -> Self {
		if condition {
			self.all_async(f).await
		} else {
			self.snapshot()
		}
	}

	pub async fn cond_not_all_async(
		&self,
		condition: bool,
		f: impl AsyncFnMut(&'a T, &Self) -> bool,
	) -> Self {
		if condition {
			self.not_all_async(f).await
		} else {
			self.snapshot()
		}
	}

	pub async fn cond_none_async(
		&self,
		condition: bool,
		f: impl AsyncFnMut(&'a T, &Self) -> bool,
	) -> Self {
		if condition {
			self.none_async(f).await
		} else {
			self.snapshot()
		}
	}
}
