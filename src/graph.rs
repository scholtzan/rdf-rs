use uri::Uri;
use triple::*;
use namespace::*;
use node::*;
use std::slice::Iter;


/// Representation of an RDF graph.
#[derive(Debug)]
pub struct Graph {
  /// Base URI of the RDF graph.
  base_uri: Option<Uri>,

  /// All triples of the RDF graph.
  triples: TripleStore,

  /// All namespaces associated to the RDF graph.
  namespaces: NamespaceStore,

  /// Next unique ID that can be used for a new blank node.
  next_id: u64
}


impl Graph {
  /// Constructor for the RDF graph.
  pub fn new() -> Graph {
    Graph {
      base_uri: None,
      triples: TripleStore::new(),
      namespaces: NamespaceStore::new(),
      next_id: 0
    }
  }

  /// Returns `true` if the graph does not contain any triples.
  pub fn is_empty(&self) -> bool {
    self.triples.is_empty()
  }

  /// Returns a literal node of the specified namespace.
  pub fn create_literal_node(&self, literal: String, namespace: Option<&Namespace>) -> Node {
    Node::LiteralNode {
      literal: literal,
      prefix: match namespace {
        Some(ns) => Some(ns.prefix().clone()),
        None => None
      },
      data_type: None,
      language: None
    }
  }

  /// Returns a literal node with a specified data type.
  pub fn create_literal_node_with_data_type(&self, literal: String, data_type: &Uri, namespace: Option<&Namespace>) -> Node {
    Node::LiteralNode {
      literal: literal,
      prefix: match namespace {
        Some(ns) => Some(ns.prefix().clone()),
        None => None
      },
      data_type: Some(data_type.clone()),
      language: None
    }
  }

  /// Returns a literal node with a specified language.
  pub fn create_literal_node_with_language(&self, literal: String, language: String, namespace: Option<&Namespace>) -> Node {
    Node::LiteralNode {
      literal: literal,
      prefix: match namespace {
        Some(ns) => Some(ns.prefix().clone()),
        None => None
      },
      data_type: None,
      language: Some(language)
    }
  }

  /// Returns the next unique ID that can be used for a blank node.
  fn get_next_id(&self) -> u64 {
    self.next_id
  }

  /// Creates a blank node with a unique ID.
  pub fn create_blank_node(&mut self) -> Node {
    let id = self.get_next_id();

    self.next_id = id + 1;

    Node::BlankNode {
      id: "auto".to_string() + &id.to_string()
    }
  }

  /// Creates a blank node with the specified ID.
  pub fn create_blank_node_with_id(&self, id: String) -> Node {
    Node::BlankNode {
      id: id
    }
  }

  /// Creates a new URI node.
  pub fn create_uri_node(&self, uri: &Uri) -> Node {
    Node::UriNode {
      uri: uri.clone()
    }
  }

  /// Adds a triple to the graph.
  pub fn add_triple(&mut self, triple: Triple) {
    self.triples.add_triple(triple);
  }

  /// Deletes the triple from the graph.
  pub fn remove_triple(&mut self, triple: Triple) {
    self.triples.remove_triple(triple);
  }

  /// Returns an iterator over the triples of the graph.
  pub fn triples_iter(&self) -> Iter<Triple> {
    self.triples.iter()
  }
}