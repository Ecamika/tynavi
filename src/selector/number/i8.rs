use crate::selector::Selector;
use crate::traits::*;

impl<'a> AsSelector<'a, i8, ()> for i8 {
	fn as_selector(&'a self) -> Selector<'a, i8, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, i8, P> {}
