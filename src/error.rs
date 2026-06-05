use std::{error::Error, fmt::{Debug, Display}};

pub struct SelectorNotMatched;

impl Debug for SelectorNotMatched {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "SelectorNotMatched(selector not matched)")
  }
}

impl Display for SelectorNotMatched {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "selector not matched")
  }
}

impl Error for SelectorNotMatched {}

pub type SelectorResult<T> = Result<T, SelectorNotMatched>;
