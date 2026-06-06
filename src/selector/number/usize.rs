use crate::selector::Selector;
use crate::traits::*;

impl<'a> AsSelector<'a, usize, ()> for usize {
	fn as_selector(&'a self) -> Selector<'a, usize, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, usize, P> {}
