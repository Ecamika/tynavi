use crate::selector::Selector;
use crate::traits::{AsSelector, SelectorInstance, Snapshot};

impl<'a, T> AsSelector<'a, Option<T>, ()> for Option<T> {
	fn as_selector(&'a self) -> Selector<'a, Option<T>, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, T, P: SelectorInstance> Selector<'a, Option<T>, P> {
	pub fn flatten(&self) -> Selector<'a, T, Self> {
		if let Some(cursor) = self.cursor
			&& let Some(data) = cursor
		{
			Selector {
				cursor: Some(data),
				parent: self.snapshot(),
			}
		} else {
			Selector {
				cursor: None,
				parent: self.snapshot(),
			}
		}
	}
}
