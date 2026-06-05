use crate::{
	selector::Selector,
	traits::{AsSelector, SelectorInstance, Snapshot, Unmatch},
};

impl<'a> AsSelector<'a, &'a str, ()> for &'a str {
	fn as_selector(&'a self) -> Selector<'a, &'a str, ()> {
		Selector {
			cursor: Some(self),
			parent: (),
		}
	}
}

impl<'a, P: SelectorInstance> Selector<'a, &str, P> {
  pub fn starts_with(&self, pat: &str) -> Self {
    if let Some(cursor) = self.cursor && !cursor.starts_with(pat) {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn cond_starts_with(&self, condition: bool, pat: &str) -> Self {
    if condition {
      self.starts_with(pat)
    } else {
      self.snapshot()
    }
  }

  pub fn ends_with(&self, pat: &str) -> Self {
    if let Some(cursor) = self.cursor && !cursor.ends_with(pat) {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn cond_ends_with(&self, condition: bool, pat: &str) -> Self {
    if condition {
      self.ends_with(pat)
    } else {
      self.snapshot()
    }
  }

  pub fn contains(&self, pat: &str) -> Self {
    if let Some(cursor) = self.cursor && !cursor.contains(pat) {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn cond_contains(&self, condition: bool, pat: &str) -> Self {
    if condition {
      self.contains(pat)
    } else {
      self.snapshot()
    }
  }
}

impl<'a> AsSelector<'a, String, ()> for String {
  fn as_selector(&'a self) -> Selector<'a, String, ()> {
    Selector { cursor: Some(self), parent: () }
  }
}

impl<'a, P: SelectorInstance> Selector<'a, String, P> {
  pub fn starts_with(&self, pat: &str) -> Self {
    if let Some(cursor) = self.cursor && !cursor.starts_with(pat) {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn cond_starts_with(&self, condition: bool, pat: &str) -> Self {
    if condition {
      self.starts_with(pat)
    } else {
      self.snapshot()
    }
  }

  pub fn ends_with(&self, pat: &str) -> Self {
    if let Some(cursor) = self.cursor && !cursor.ends_with(pat) {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn cond_ends_with(&self, condition: bool, pat: &str) -> Self {
    if condition {
      self.ends_with(pat)
    } else {
      self.snapshot()
    }
  }

  pub fn contains(&self, pat: &str) -> Self {
    if let Some(cursor) = self.cursor && !cursor.contains(pat) {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn cond_contains(&self, condition: bool, pat: &str) -> Self {
    if condition {
      self.contains(pat)
    } else {
      self.snapshot()
    }
  }
}
