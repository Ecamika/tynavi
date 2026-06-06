use crate::selector::Selector;
use crate::traits::AsSelector;

impl<'a, T> AsSelector<'a, &'a [T], ()> for &[T] {
	fn as_selector(&'a self) -> Selector<'a, &'a [T], ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}
