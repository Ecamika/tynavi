use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

use crate::selector::Selector;
use crate::traits::{AsSelector, SelectorInstance, Snapshot};

impl<'a, K, V> AsSelector<'a, HashMap<K, V>, ()> for HashMap<K, V> {
	fn as_selector(&'a self) -> Selector<'a, HashMap<K, V>, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, K, V, P: SelectorInstance> Selector<'a, HashMap<K, V>, P> {
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

	pub fn keyof<Q>(&self, key: &Q) -> Selector<'a, V, Self>
	where
		K: Borrow<Q> + Eq + Hash,
		Q: Eq + Hash + ?Sized,
	{
		self.route_to(|cursor, _| cursor.get(key))
	}

	pub fn find_key(&self, mut f: impl FnMut(&'a K, &'a V, &Self) -> bool) -> Selector<'a, K, Self> {
		self.route_to(|cursor, sel| {
			cursor
				.iter()
				.find_map(|(key, value)| if f(key, value, sel) { Some(key) } else { None })
		})
	}

	pub fn find(&self, mut f: impl FnMut(&'a K, &'a V, &Self) -> bool) -> Selector<'a, V, Self> {
		self.route_to(|cursor, sel| {
			cursor.iter().find_map(|(key, value)| {
				if f(key, value, sel) {
					Some(value)
				} else {
					None
				}
			})
		})
	}

	pub fn any(&self, mut f: impl FnMut(&'a K, &'a V, &Self) -> bool) -> Self {
		self.filter(|cursor, sel| cursor.iter().any(|(key, value)| f(key, value, sel)))
	}

	pub fn not_any(&self, mut f: impl FnMut(&'a K, &'a V, &Self) -> bool) -> Self {
		self.filter(|cursor, sel| !cursor.iter().any(|(key, value)| f(key, value, sel)))
	}

	pub fn all(&self, mut f: impl FnMut(&'a K, &'a V, &Self) -> bool) -> Self {
		self.filter(|cursor, sel| cursor.iter().all(|(key, value)| f(key, value, sel)))
	}

	pub fn not_all(&self, mut f: impl FnMut(&'a K, &'a V, &Self) -> bool) -> Self {
		self.filter(|cursor, sel| !cursor.iter().all(|(key, value)| f(key, value, sel)))
	}

	pub fn none(&self, mut f: impl FnMut(&'a K, &'a V, &Self) -> bool) -> Self {
		self.filter(|cursor, sel| !cursor.iter().any(|(key, value)| f(key, value, sel)))
	}

	pub fn contains_key<Q>(&self, key: &Q) -> Self
	where
		K: Borrow<Q> + Eq + Hash,
		Q: Eq + Hash + ?Sized,
	{
		self.filter(|cursor, _| cursor.contains_key(key))
	}

	pub fn not_contains_key<Q>(&self, key: &Q) -> Self
	where
		K: Borrow<Q> + Eq + Hash,
		Q: Eq + Hash + ?Sized,
	{
		self.filter(|cursor, _| !cursor.contains_key(key))
	}

	pub fn contains_value(&self, value: &V) -> Self
	where
		V: PartialEq,
	{
		self.filter(|cursor, _| cursor.values().any(|data| data == value))
	}

	pub fn not_contains_value(&self, value: &V) -> Self
	where
		V: PartialEq,
	{
		self.filter(|cursor, _| cursor.values().all(|data| data != value))
	}

	pub fn cond_any(&self, condition: bool, f: impl FnMut(&'a K, &'a V, &Self) -> bool) -> Self {
		if condition {
			self.any(f)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_any(&self, condition: bool, f: impl FnMut(&'a K, &'a V, &Self) -> bool) -> Self {
		if condition {
			self.not_any(f)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_all(&self, condition: bool, f: impl FnMut(&'a K, &'a V, &Self) -> bool) -> Self {
		if condition {
			self.all(f)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_all(&self, condition: bool, f: impl FnMut(&'a K, &'a V, &Self) -> bool) -> Self {
		if condition {
			self.not_all(f)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_none(&self, condition: bool, f: impl FnMut(&'a K, &'a V, &Self) -> bool) -> Self {
		if condition {
			self.none(f)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_contains_key<Q>(&self, condition: bool, key: &Q) -> Self
	where
		K: Borrow<Q> + Eq + Hash,
		Q: Eq + Hash + ?Sized,
	{
		if condition {
			self.contains_key(key)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_contains_key<Q>(&self, condition: bool, key: &Q) -> Self
	where
		K: Borrow<Q> + Eq + Hash,
		Q: Eq + Hash + ?Sized,
	{
		if condition {
			self.not_contains_key(key)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_contains_value(&self, condition: bool, value: &V) -> Self
	where
		V: PartialEq,
	{
		if condition {
			self.contains_value(value)
		} else {
			self.snapshot()
		}
	}

	pub fn cond_not_contains_value(&self, condition: bool, value: &V) -> Self
	where
		V: PartialEq,
	{
		if condition {
			self.not_contains_value(value)
		} else {
			self.snapshot()
		}
	}

	pub async fn any_async(&self, mut f: impl AsyncFnMut(&'a K, &'a V, &Self) -> bool) -> Self {
		self
			.filter_async(async |cursor, sel| {
				for (key, value) in cursor.iter() {
					if f(key, value, sel).await {
						return true;
					}
				}
				false
			})
			.await
	}

	pub async fn not_any_async(&self, mut f: impl AsyncFnMut(&'a K, &'a V, &Self) -> bool) -> Self {
		self
			.filter_async(async |cursor, sel| {
				for (key, value) in cursor.iter() {
					if f(key, value, sel).await {
						return false;
					}
				}
				true
			})
			.await
	}

	pub async fn all_async(&self, mut f: impl AsyncFnMut(&'a K, &'a V, &Self) -> bool) -> Self {
		self
			.filter_async(async |cursor, sel| {
				for (key, value) in cursor.iter() {
					if !f(key, value, sel).await {
						return false;
					}
				}
				true
			})
			.await
	}

	pub async fn not_all_async(&self, mut f: impl AsyncFnMut(&'a K, &'a V, &Self) -> bool) -> Self {
		self
			.filter_async(async |cursor, sel| {
				for (key, value) in cursor.iter() {
					if !f(key, value, sel).await {
						return true;
					}
				}
				false
			})
			.await
	}

	pub async fn none_async(&self, mut f: impl AsyncFnMut(&'a K, &'a V, &Self) -> bool) -> Self {
		self
			.filter_async(async |cursor, sel| {
				for (key, value) in cursor.iter() {
					if f(key, value, sel).await {
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
		f: impl AsyncFnMut(&'a K, &'a V, &Self) -> bool,
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
		f: impl AsyncFnMut(&'a K, &'a V, &Self) -> bool,
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
		f: impl AsyncFnMut(&'a K, &'a V, &Self) -> bool,
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
		f: impl AsyncFnMut(&'a K, &'a V, &Self) -> bool,
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
		f: impl AsyncFnMut(&'a K, &'a V, &Self) -> bool,
	) -> Self {
		if condition {
			self.none_async(f).await
		} else {
			self.snapshot()
		}
	}
}
