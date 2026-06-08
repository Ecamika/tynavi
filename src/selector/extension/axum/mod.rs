use crate::selector::Selector;
use crate::traits::{AsSelector, SelectorInstance};
use axum::extract::{Json, Path, Query, State};
use std::ops::Deref;

// ─── Path ─────────────────────────────────────────────────────────────────────

impl<'a, T> AsSelector<'a, Path<T>, ()> for Path<T> {
	fn as_selector(&'a self) -> Selector<'a, Path<T>, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, T, P: SelectorInstance> Selector<'a, Path<T>, P> {
	pub fn inner(&self) -> Selector<'a, T, Self> {
		self.route_to(|cursor, _| Some(cursor.deref()))
	}
}

// ─── Query ────────────────────────────────────────────────────────────────────

impl<'a, T> AsSelector<'a, Query<T>, ()> for Query<T> {
	fn as_selector(&'a self) -> Selector<'a, Query<T>, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, T, P: SelectorInstance> Selector<'a, Query<T>, P> {
	pub fn inner(&self) -> Selector<'a, T, Self> {
		self.route_to(|cursor, _| Some(cursor.deref()))
	}
}

// ─── Json ─────────────────────────────────────────────────────────────────────

impl<'a, T> AsSelector<'a, Json<T>, ()> for Json<T> {
	fn as_selector(&'a self) -> Selector<'a, Json<T>, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, T, P: SelectorInstance> Selector<'a, Json<T>, P> {
	pub fn inner(&self) -> Selector<'a, T, Self> {
		self.route_to(|cursor, _| Some(cursor.deref()))
	}
}

// ─── State ────────────────────────────────────────────────────────────────────

impl<'a, T> AsSelector<'a, State<T>, ()> for State<T> {
	fn as_selector(&'a self) -> Selector<'a, State<T>, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, T, P: SelectorInstance> Selector<'a, State<T>, P> {
	pub fn inner(&self) -> Selector<'a, T, Self> {
		self.route_to(|cursor, _| Some(cursor.deref()))
	}
}
