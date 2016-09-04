use uri::Uri;

pub trait Node { }

pub struct UriNode {
  uri: Uri
}

impl Node for UriNode {

}

impl UriNode {
  pub fn new(uri: Uri) -> UriNode {
    UriNode {
      uri: uri
    }
  }

  pub fn uri(&self) -> &Uri {
    &self.uri
  }
}

