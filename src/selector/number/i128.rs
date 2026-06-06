use crate::selector::Selector;
use crate::traits::*;

impl<'a> AsSelector<'a, i128, ()> for i128 {
	fn as_selector(&'a self) -> Selector<'a, i128, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, i128, P> {}
