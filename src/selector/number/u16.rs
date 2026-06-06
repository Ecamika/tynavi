use crate::selector::Selector;
use crate::traits::*;

impl<'a> AsSelector<'a, u16, ()> for u16 {
	fn as_selector(&'a self) -> Selector<'a, u16, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, u16, P> {}
