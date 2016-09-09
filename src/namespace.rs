use std::collections::HashMap;
use uri::Uri;

pub struct Namespace {
  prefix: String,
  uri: Uri
}

impl Namespace {
  pub fn prefix(&self) -> &String {
    &self.prefix
  }


  pub fn uri(&self) -> &Uri {
    &self.uri
  }
}


pub struct NamespaceStore {
  namespaces: HashMap<String, Uri>
}

