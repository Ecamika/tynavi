use crate::selector::Selector;
use crate::traits::*;

impl<'a> AsSelector<'a, f32, ()> for f32 {
	fn as_selector(&'a self) -> Selector<'a, f32, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, f32, P> {}
