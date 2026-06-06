use crate::selector::Selector;
use crate::traits::*;

impl<'a> AsSelector<'a, f64, ()> for f64 {
	fn as_selector(&'a self) -> Selector<'a, f64, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, f64, P> {}
