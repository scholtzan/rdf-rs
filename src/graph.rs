use uri::Uri;
use triple::TripleStore;
use namespace::*;

pub struct Graph {
  base_uri: Uri,
  triples: TripleStore,
  namespaces: NamespaceStore
}

impl Graph {
  pub fn is_empty(&self) -> bool {
    self.triples.is_empty()
  }
}