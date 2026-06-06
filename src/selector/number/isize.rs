use crate::selector::Selector;
use crate::traits::*;

impl<'a> AsSelector<'a, isize, ()> for isize {
	fn as_selector(&'a self) -> Selector<'a, isize, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, isize, P> {}
