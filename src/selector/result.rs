use crate::selector::Selector;
use crate::traits::{AsSelector, SelectorInstance, Snapshot};

impl<'a, T, E> AsSelector<'a, Result<T, E>, ()> for Result<T, E> {
	fn as_selector(&'a self) -> Selector<'a, Result<T, E>, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, T, E, P: SelectorInstance> Selector<'a, Result<T, E>, P> {
	pub fn ok(&self) -> Selector<'a, T, Self> {
		if let Some(cursor) = self.cursor
			&& let Ok(data) = cursor
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

	pub fn err(&self) -> Selector<'a, E, Self> {
		if let Some(cursor) = self.cursor
			&& let Err(data) = cursor
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
