use crate::selector::Selector;
use crate::traits::*;

impl<'a> AsSelector<'a, u64, ()> for u64 {
	fn as_selector(&'a self) -> Selector<'a, u64, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, u64, P> {}
