use crate::selector::Selector;

pub trait Snapshot {
	fn snapshot(&self) -> Self;
}

pub trait AsSelector<'a, C, P: SelectorInstance> {
	fn as_selector(&'a self) -> Selector<'a, C, P>;
}

impl Snapshot for () {
	fn snapshot(&self) -> Self {
		()
	}
}

pub trait Unmatch {
	fn unmatch(&self) -> Self;
	fn cond_unmatch(&self, condition: bool) -> Self;
}

impl Unmatch for () {
	fn unmatch(&self) -> Self {
		()
	}

	fn cond_unmatch(&self, _: bool) -> Self {
		()
	}
}

pub trait SelectorInstance: Snapshot + Unmatch + Copy {}

impl<T: Snapshot + Unmatch + Copy> SelectorInstance for T {}
