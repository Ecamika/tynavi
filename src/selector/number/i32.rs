use crate::selector::Selector;
use crate::traits::*;

impl<'a> AsSelector<'a, i32, ()> for i32 {
	fn as_selector(&'a self) -> Selector<'a, i32, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, i32, P> {}
