// todo: implement

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Uri {
  uri: String
}

// todo
impl Uri {
  pub fn new(uri: String) -> Uri {
    Uri {
      uri: uri
    }
  }

  /// Returns the string representation of the URI.
  pub fn to_string(&self) -> &String {
    &self.uri
  }
}