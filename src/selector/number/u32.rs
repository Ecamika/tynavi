use crate::selector::Selector;
use crate::traits::*;

impl<'a> AsSelector<'a, u32, ()> for u32 {
	fn as_selector(&'a self) -> Selector<'a, u32, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, u32, P> {}
