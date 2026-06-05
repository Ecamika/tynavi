use crate::traits::*;
use crate::selector::Selector;

impl<'a> AsSelector<'a, usize, ()> for usize {
  fn as_selector(&'a self) -> Selector<'a, usize, ()> {
    Selector {
      cursor: Some(self),
      parent: ()
    }
  }
}

impl<'a, P: SelectorInstance> Selector<'a, usize, P> {
  pub fn eq(&self, v: &usize) -> Self {
    if let Some(cursor) = self.cursor && cursor != v {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn not_eq(&self, v: &usize) -> Self {
    if let Some(cursor) = self.cursor && cursor == v {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn cond_eq(&self, condition: bool, v: &usize) -> Self {
    if condition {
      self.eq(v)
    } else {
      self.snapshot()
    }
  }

  pub fn cond_not_eq(&self, condition: bool, v: &usize) -> Self {
    if condition {
      self.not_eq(v)
    } else {
      self.snapshot()
    }
  }

  pub fn gt(&self, v: &usize) -> Self {
    if let Some(cursor) = self.cursor && cursor <= v {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn not_gt(&self, v: &usize) -> Self {
    if let Some(cursor) = self.cursor && cursor > v {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn cond_gt(&self, condition: bool, v: &usize) -> Self {
    if condition {
      self.gt(v)
    } else {
      self.snapshot()
    }
  }

  pub fn cond_not_gt(&self, condition: bool, v: &usize) -> Self {
    if condition {
      self.not_gt(v)
    } else {
      self.snapshot()
    }
  }

  pub fn lt(&self, v: &usize) -> Self {
    if let Some(cursor) = self.cursor && cursor >= v {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn not_lt(&self, v: &usize) -> Self {
    if let Some(cursor) = self.cursor && cursor < v {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn cond_lt(&self, condition: bool, v: &usize) -> Self {
    if condition {
      self.lt(v)
    } else {
      self.snapshot()
    }
  }

  pub fn cond_not_lt(&self, condition: bool, v: &usize) -> Self {
    if condition {
      self.not_lt(v)
    } else {
      self.snapshot()
    }
  }

  pub fn ge(&self, v: &usize) -> Self {
    if let Some(cursor) = self.cursor && cursor < v {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn not_ge(&self, v: &usize) -> Self {
    if let Some(cursor) = self.cursor && cursor >= v {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn cond_ge(&self, condition: bool, v: &usize) -> Self {
    if condition {
      self.ge(v)
    } else {
      self.snapshot()
    }
  }

  pub fn cond_not_ge(&self, condition: bool, v: &usize) -> Self {
    if condition {
      self.not_ge(v)
    } else {
      self.snapshot()
    }
  }

  pub fn le(&self, v: &usize) -> Self {
    if let Some(cursor) = self.cursor && cursor > v {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn not_le(&self, v: &usize) -> Self {
    if let Some(cursor) = self.cursor && cursor <= v {
      self.unmatch()
    } else {
      self.snapshot()
    }
  }

  pub fn cond_le(&self, condition: bool, v: &usize) -> Self {
    if condition {
      self.le(v)
    } else {
      self.snapshot()
    }
  }

  pub fn cond_not_le(&self, condition: bool, v: &usize) -> Self {
    if condition {
      self.not_le(v)
    } else {
      self.snapshot()
    }
  }
}
