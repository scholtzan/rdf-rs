use uri::Uri;
use triple::*;
use namespace::*;
use node::*;
use std::slice::Iter;
use std::collections::HashMap;


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

  /// Returns the number of triples that are stored in the graph.
  pub fn count(&self) -> usize {
    self.triples.count()
  }

  /// Returns the base URI of the graph.
  ///
  /// # Example
  ///
  /// todo
  ///
  pub fn base_uri(&self) -> &Option<Uri> {
    &self.base_uri
  }

  /// Returns a hashmap of namespaces and prefixes.
  ///
  /// # Example
  ///
  /// todo
  ///
  pub fn namespaces(&self) -> &HashMap<String, Uri> {
    self.namespaces.namespaces()
  }

  /// Returns a literal node of the specified namespace.
  ///
  /// # Example
  /// ```
  /// use rdf_rs::graph::Graph;
  /// use rdf_rs::node::Node;
  ///
  /// let graph = Graph::new();
  /// let literal_node = graph.create_literal_node("literal".to_string());
  ///
  /// assert_eq!(literal_node, Node::LiteralNode {
  ///   literal: "literal".to_string(),
  ///   data_type: None,
  ///   language: None
  /// });
  /// ```
  pub fn create_literal_node(&self, literal: String) -> Node {
    Node::LiteralNode {
      literal: literal,
      data_type: None,
      language: None
    }
  }

  /// Returns a literal node with a specified data type.
  ///
  /// # Example
  /// ```
  /// use rdf_rs::graph::Graph;
  /// use rdf_rs::node::Node;
  /// use rdf_rs::uri::Uri;
  ///
  /// let graph = Graph::new();
  /// let literal_node = graph.create_literal_node_with_data_type("literal".to_string(), &Uri::new("http://example.org/show/localName".to_string()));
  ///
  /// assert_eq!(literal_node, Node::LiteralNode {
  ///   literal: "literal".to_string(),
  ///   data_type: Some(Uri::new("http://example.org/show/localName".to_string())),
  ///   language: None
  /// });
  /// ```
  pub fn create_literal_node_with_data_type(&self, literal: String, data_type: &Uri) -> Node {
    Node::LiteralNode {
      literal: literal,
      data_type: Some(data_type.clone()),
      language: None
    }
  }

  /// Returns a literal node with a specified language.
  ///
  /// # Example
  /// ```
  /// use rdf_rs::graph::Graph;
  /// use rdf_rs::node::Node;
  ///
  /// let graph = Graph::new();
  /// let literal_node = graph.create_literal_node_with_language("literal".to_string(), "en".to_string());
  ///
  /// assert_eq!(literal_node, Node::LiteralNode {
  ///   literal: "literal".to_string(),
  ///   data_type: None,
  ///   language: Some("en".to_string())
  /// });
  /// ```
  pub fn create_literal_node_with_language(&self, literal: String, language: String) -> Node {
    Node::LiteralNode {
      literal: literal,
      data_type: None,
      language: Some(language)
    }
  }

  /// Returns the next unique ID that can be used for a blank node.
  fn get_next_id(&self) -> u64 {
    self.next_id
  }

  /// Creates a blank node with a unique ID.
  ///
  /// # Example
  /// ```
  /// use rdf_rs::graph::Graph;
  /// use rdf_rs::node::Node;
  ///
  /// let mut graph = Graph::new();
  /// let blank_node = graph.create_blank_node();
  ///
  /// assert_eq!(blank_node, Node::BlankNode {
  ///   id: "auto0".to_string()
  /// });
  /// ```
  pub fn create_blank_node(&mut self) -> Node {
    let id = self.get_next_id();

    self.next_id = id + 1;

    Node::BlankNode {
      id: "auto".to_string() + &id.to_string()
    }
  }

  /// Creates a blank node with the specified ID.
  ///
  /// # Example
  /// ```
  /// use rdf_rs::graph::Graph;
  /// use rdf_rs::node::Node;
  ///
  /// let graph = Graph::new();
  /// let blank_node = graph.create_blank_node_with_id("foobar".to_string());
  ///
  /// assert_eq!(blank_node, Node::BlankNode {
  ///   id: "foobar".to_string()
  /// });
  /// ```
  pub fn create_blank_node_with_id(&self, id: String) -> Node {
    Node::BlankNode {
      id: id
    }
  }

  /// Creates a new URI node.
  ///
  /// # Example
  /// ```
  /// use rdf_rs::graph::Graph;
  /// use rdf_rs::node::Node;
  /// use rdf_rs::uri::Uri;
  ///
  /// let graph = Graph::new();
  /// let uri_node = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
  ///
  /// assert_eq!(uri_node, Node::UriNode {
  ///   uri: Uri::new("http://example.org/show/localName".to_string())
  /// });
  /// ```
  pub fn create_uri_node(&self, uri: &Uri) -> Node {
    Node::UriNode {
      uri: uri.clone()
    }
  }

  /// Adds a triple to the graph.
  ///
  /// # Example
  /// ```
  /// use rdf_rs::graph::Graph;
  /// use rdf_rs::uri::Uri;
  /// use rdf_rs::triple::Triple;
  ///
  /// let mut graph = Graph::new();
  ///
  /// let subject = graph.create_blank_node();
  /// let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
  /// let object = graph.create_blank_node();
  /// let triple = Triple::new(subject, predicate, object);
  ///
  /// graph.add_triple(&triple);
  ///
  /// assert_eq!(graph.count(), 1);
  /// ```
  pub fn add_triple(&mut self, triple: &Triple) {
    self.triples.add_triple(triple);
  }

  /// Deletes the triple from the graph.
  ///
  /// # Example
  /// ```
  /// use rdf_rs::graph::Graph;
  /// use rdf_rs::uri::Uri;
  /// use rdf_rs::triple::Triple;
  ///
  /// let mut graph = Graph::new();
  ///
  /// let subject = graph.create_blank_node();
  /// let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
  /// let object = graph.create_blank_node();
  /// let triple = Triple::new(subject, predicate, object);
  ///
  /// graph.add_triple(&triple);
  /// graph.remove_triple(&triple);
  ///
  /// assert_eq!(graph.count(), 0);
  /// ```
  pub fn remove_triple(&mut self, triple: &Triple) {
    self.triples.remove_triple(triple);
  }

  /// Returns all triples from the store that have the specified subject node.
  pub fn get_triples_with_subject(&self, node: &Node) -> Vec<&Triple> {
    self.triples.get_triples_with_subject(node)
  }

  /// Returns all triples from the store that have the specified predicate node.
  pub fn get_triples_with_predicate(&self, node: &Node) -> Vec<&Triple> {
    self.triples.get_triples_with_predicate(node)
  }

  /// Returns all triples from the store that have the specified object node.
  pub fn get_triples_with_object(&self, node: &Node) -> Vec<&Triple> {
    self.triples.get_triples_with_object(node)
  }

  /// Returns all triples from the triple store where the subject and object nodes match the provided nodes.
  pub fn get_triples_with_subject_and_object(&self, subject_node: &Node, object_node: &Node) -> Vec<&Triple> {
    self.triples.get_triples_with_subject_and_object(subject_node, object_node)
  }

  /// Returns all triples from the triple store where the subject and predicate nodes match the provided nodes.
  pub fn get_triples_with_subject_and_predicate(&self, subject_node: &Node, predicate_node: &Node) -> Vec<&Triple> {
    self.triples.get_triples_with_subject_and_predicate(subject_node, predicate_node)
  }

  /// Returns all triples from the triple store where the predicate and object nodes match the provided nodes.
  pub fn get_triples_with_predicate_and_object(&self, predicate_node: &Node, object_node: &Node) -> Vec<&Triple> {
    self.triples.get_triples_with_predicate_and_object(predicate_node, object_node)
  }

  /// Returns an iterator over the triples of the graph.
  pub fn triples_iter(&self) -> Iter<Triple> {
    self.triples.iter()
  }
}


#[cfg(test)]
mod tests {
  use graph::Graph;
  use node::*;
  use namespace::Namespace;

  #[test]
  fn empty_graph() {
    let graph = Graph::new();
    assert_eq!(graph.is_empty(), true);
  }

  #[test]
  fn create_literal_node() {
    let graph = Graph::new();
    let literal_node = graph.create_literal_node("literal".to_string());

    assert_eq!(literal_node, Node::LiteralNode {
      literal: "literal".to_string(),
      data_type: None,
      language: None
    });
  }

  #[test]
  fn create_multiple_blank_nodes() {
    let mut graph = Graph::new();

    let blank_node_0 = graph.create_blank_node();
    let blank_node_1 = graph.create_blank_node();

    assert_eq!(blank_node_0, Node::BlankNode {
      id: "auto0".to_string()
    });

    assert_eq!(blank_node_1, Node::BlankNode {
      id: "auto1".to_string()
    });
  }
}