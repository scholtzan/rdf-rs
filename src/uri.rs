// todo: implement

#[derive(Clone)]
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

  pub fn uri(&self) -> &String {
    &self.uri
  }
}