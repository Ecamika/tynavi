use crate::selector::Selector;
use crate::traits::*;

impl<'a> AsSelector<'a, i16, ()> for i16 {
	fn as_selector(&'a self) -> Selector<'a, i16, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, i16, P> {}
