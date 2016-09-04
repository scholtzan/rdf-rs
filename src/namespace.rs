use std::collections::HashMap;
use uri::Uri;

pub struct Namespace {
  prefix: String,
  uri: Uri
}


pub struct NamespaceStore {
  namespaces: HashMap<String, Uri>
}

