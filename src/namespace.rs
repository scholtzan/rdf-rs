use std::collections::HashMap;
use uri::Uri;

/// Representation of a specific namespace.
pub struct Namespace {
  prefix: String,
  uri: Uri
}


// todo
impl Namespace {

  /// Returns the prefix of the namespace.
  pub fn prefix(&self) -> &String {
    &self.prefix
  }

  /// Returns the URI of the namespace.
  pub fn uri(&self) -> &Uri {
    &self.uri
  }
}


/// Storage for multiple namespaces.
pub struct NamespaceStore {
  /// The namespace prefix is associated with the namespace URI.
  namespaces: HashMap<String, Uri>
}

