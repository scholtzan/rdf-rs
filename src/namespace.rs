use std::collections::HashMap;
use uri::Uri;

/// Representation of a specific namespace.
#[derive(Debug)]
pub struct Namespace {
  prefix: String,
  uri: Uri
}


impl Namespace {
  /// `Namespace` constructor.
  pub fn new(prefix: String, uri: Uri) -> Namespace {
    Namespace {
      prefix: prefix,
      uri: uri
    }
  }

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
#[derive(PartialEq, Debug)]
pub struct NamespaceStore {
  /// The namespace prefix is associated with the namespace URI.
  namespaces: HashMap<String, Uri>
}


impl NamespaceStore {
  /// Constructor for `NamespaceStore`.
  pub fn new() -> NamespaceStore {
    NamespaceStore {
      namespaces: HashMap::new()
    }
  }

  /// Returns the stored namespaces with prefixes.
  pub fn namespaces(&self) -> &HashMap<String, Uri> {
    &self.namespaces()
  }
}