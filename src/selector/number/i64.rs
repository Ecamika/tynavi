use crate::selector::Selector;
use crate::traits::*;

impl<'a> AsSelector<'a, i64, ()> for i64 {
	fn as_selector(&'a self) -> Selector<'a, i64, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, i64, P> {}
