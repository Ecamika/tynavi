use crate::selector::Selector;
use crate::traits::*;

impl<'a> AsSelector<'a, u8, ()> for u8 {
	fn as_selector(&'a self) -> Selector<'a, u8, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, u8, P> {}
