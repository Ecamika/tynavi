use std::rc::Rc;
use std::sync::Arc;

use crate::selector::Selector;
use crate::traits::{AsSelector, SelectorInstance};

impl<'a, T> AsSelector<'a, Box<T>, ()> for Box<T> {
	fn as_selector(&'a self) -> Selector<'a, Box<T>, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, T, P: SelectorInstance> Selector<'a, Box<T>, P> {
	pub fn as_ref(&self) -> Selector<'a, T, Self> {
		self.route_to(|cursor, _| Some(cursor.as_ref()))
	}
}

impl<'a, T> AsSelector<'a, Rc<T>, ()> for Rc<T> {
	fn as_selector(&'a self) -> Selector<'a, Rc<T>, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, T, P: SelectorInstance> Selector<'a, Rc<T>, P> {
	pub fn as_ref(&self) -> Selector<'a, T, Self> {
		self.route_to(|cursor, _| Some(cursor.as_ref()))
	}
}

impl<'a, T> AsSelector<'a, Arc<T>, ()> for Arc<T> {
	fn as_selector(&'a self) -> Selector<'a, Arc<T>, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, T, P: SelectorInstance> Selector<'a, Arc<T>, P> {
	pub fn as_ref(&self) -> Selector<'a, T, Self> {
		self.route_to(|cursor, _| Some(cursor.as_ref()))
	}
}
