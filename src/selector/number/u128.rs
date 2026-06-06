use crate::selector::Selector;
use crate::traits::*;

impl<'a> AsSelector<'a, u128, ()> for u128 {
	fn as_selector(&'a self) -> Selector<'a, u128, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, u128, P> {}
