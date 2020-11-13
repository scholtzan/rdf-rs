use crate::namespace::*;
use crate::node::*;
use std::collections::HashMap;
use std::slice::Iter;
use crate::triple::*;
use crate::uri::Uri;
use crate::Result;
use crate::specs::xml_specs::XmlDataTypes;

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
    next_id: u64,
}

impl Graph {
    /// Constructor for the RDF graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    ///
    /// let graph = Graph::new(None);
    /// ```
    pub fn new(base_uri: Option<&Uri>) -> Graph {
        let cloned_uri = match base_uri {
            None => None,
            Some(u) => Some(u.clone()),
        };

        Graph {
            base_uri: cloned_uri,
            triples: TripleStore::new(),
            namespaces: NamespaceStore::new(),
            next_id: 0,
        }
    }

    /// Returns `true` if the graph does not contain any triples.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    ///
    /// let graph = Graph::new(None);
    ///
    /// assert_eq!(graph.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.triples.is_empty()
    }

    /// Returns the number of triples that are stored in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    ///
    /// let graph = Graph::new(None);
    ///
    /// assert_eq!(graph.count(), 0);
    /// ```
    pub fn count(&self) -> usize {
        self.triples.count()
    }

    /// Returns the base URI of the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::uri::Uri;
    /// use rdf::graph::Graph;
    ///
    /// let base_uri = Uri::new("http://example.org/".to_string());
    /// let graph = Graph::new(Some(&base_uri));
    ///
    /// assert_eq!(graph.base_uri(), &Some(base_uri));
    /// ```
    pub fn base_uri(&self) -> &Option<Uri> {
        &self.base_uri
    }

    /// Sets the base URI of the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::uri::Uri;
    /// use rdf::graph::Graph;
    ///
    /// let base_uri = Uri::new("http://base.example.org/".to_string());
    /// let mut graph = Graph::new(None);
    ///
    /// graph.set_base_uri(&base_uri);
    ///
    /// assert_eq!(graph.base_uri(), &Some(base_uri));
    /// ```
    pub fn set_base_uri(&mut self, uri: &Uri) {
        self.base_uri = Some(uri.clone());
    }

    /// Returns a hash map of namespaces and prefixes.
    pub fn namespaces(&self) -> &HashMap<String, Uri> {
        self.namespaces.namespaces()
    }

    /// Adds a new namespace with a specific prefix to the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::uri::Uri;
    /// use rdf::graph::Graph;
    /// use rdf::namespace::Namespace;
    ///
    /// let mut graph = Graph::new(None);
    /// graph.add_namespace(&Namespace::new("example".to_string(),
    ///                                     Uri::new("http://example.org/".to_string())));
    ///
    /// assert_eq!(graph.namespaces().len(), 1);
    /// ```
    pub fn add_namespace(&mut self, ns: &Namespace) {
        self.namespaces.add(ns);
    }

    /// Returns the URI of a namespace with the provided prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::uri::Uri;
    /// use rdf::graph::Graph;
    /// use rdf::namespace::Namespace;
    ///
    /// let mut graph = Graph::new(None);
    /// let uri = Uri::new("http://example.org/".to_string());
    /// graph.add_namespace(&Namespace::new("example".to_string(), uri.to_owned()));
    ///
    /// assert_eq!(graph.get_namespace_uri_by_prefix("example").unwrap(), &uri);
    /// ```
    ///
    /// # Failures
    ///
    /// - No namespace with the provided prefix exists
    ///
    pub fn get_namespace_uri_by_prefix(&self, prefix: &str) -> Result<&Uri> {
        self.namespaces.get_uri_by_prefix(prefix)
    }

    /// Returns a literal node of the specified namespace.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::node::Node;
    ///
    /// let graph = Graph::new(None);
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
            literal,
            data_type: None,
            language: None,
        }
    }

    pub fn create_integer_node(&self, literal: i32) -> Node {
        Node::LiteralNode {
            literal: literal.to_string(),
            data_type: Some(XmlDataTypes::Integer.to_uri()),
            language: None,
        }
    }

    /// Returns a literal node with a specified data type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::node::Node;
    /// use rdf::uri::Uri;
    ///
    /// let graph = Graph::new(None);
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
            literal,
            data_type: Some(data_type.clone()),
            language: None,
        }
    }

    /// Returns a literal node with a specified language.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::node::Node;
    ///
    /// let graph = Graph::new(None);
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
            literal,
            data_type: None,
            language: Some(language),
        }
    }

    /// Returns the next unique ID that can be used for a blank node.
    fn get_next_id(&self) -> u64 {
        self.next_id
    }

    /// Creates a blank node with a unique ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::node::Node;
    ///
    /// let mut graph = Graph::new(None);
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
            id: "auto".to_string() + &id.to_string(),
        }
    }

    /// Creates a blank node with the specified ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::node::Node;
    ///
    /// let graph = Graph::new(None);
    /// let blank_node = graph.create_blank_node_with_id("foobar".to_string());
    ///
    /// assert_eq!(blank_node, Node::BlankNode {
    ///   id: "foobar".to_string()
    /// });
    /// ```
    pub fn create_blank_node_with_id(&self, id: String) -> Node {
        Node::BlankNode { id }
    }

    /// Creates a new URI node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::node::Node;
    /// use rdf::uri::Uri;
    ///
    /// let graph = Graph::new(None);
    /// let uri_node = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
    ///
    /// assert_eq!(uri_node, Node::UriNode {
    ///   uri: Uri::new("http://example.org/show/localName".to_string())
    /// });
    /// ```
    pub fn create_uri_node(&self, uri: &Uri) -> Node {
        Node::UriNode { uri: uri.clone() }
    }

    /// Creates a new URI node from a string slice.
    /// 
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::node::Node;
    /// use rdf::uri::Uri;
    ///
    /// let graph = Graph::new(None);
    /// let uri_node = graph.create_uri_node_str("http://example.org/show/localName");
    ///
    /// assert_eq!(uri_node, Node::UriNode {
    ///   uri: Uri::new("http://example.org/show/localName".to_string())
    /// });
    /// 
    /// let graph = Graph::new(Some(&Uri::new("http://www.w3.org/2006/vcard/ns".to_string())));
    /// let uri_node = graph.create_uri_node_str("#fn");
    /// assert_eq!(uri_node, Node::UriNode {
    ///   uri: Uri::new("http://www.w3.org/2006/vcard/ns#fn".to_string())
    /// });
    /// ```
    pub fn create_uri_node_str(&self, uri: &str) -> Node {
        let uri = match (uri.starts_with("#"),self.base_uri()){
            (true, Some(base))=> {
                let mut s = base.to_string().clone();
                s.push_str(uri);
                Uri::new(s)
            },
            (_,_) =>  Uri::new(uri.to_string())
        };

        Node::UriNode { uri: uri }
    }

    /// Adds a triple to the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::uri::Uri;
    /// use rdf::triple::Triple;
    ///
    /// let mut graph = Graph::new(None);
    ///
    /// let subject = graph.create_blank_node();
    /// let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
    /// let object = graph.create_blank_node();
    /// let triple = Triple::new(&subject, &predicate, &object);
    ///
    /// graph.add_triple(&triple);
    ///
    /// assert_eq!(graph.count(), 1);
    /// ```
    pub fn add_triple(&mut self, triple: &Triple) {
        self.triples.add_triple(triple);
    }

    /// Adds a vector of triples.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::uri::Uri;
    /// use rdf::triple::Triple;
    ///
    /// let mut graph = Graph::new(None);
    ///
    /// let subject = graph.create_blank_node();
    /// let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
    /// let object = graph.create_blank_node();
    ///
    /// let triple1 = Triple::new(&subject, &predicate, &object);
    /// let triple2 = Triple::new(&subject, &predicate, &object);
    ///
    /// graph.add_triples(&vec![triple1, triple2]);
    ///
    /// assert_eq!(graph.count(), 2);
    /// ```
    pub fn add_triples(&mut self, triples: &[Triple]) {
        for triple in triples {
            self.add_triple(triple);
        }
    }

    /// Deletes the triple from the graph.
    ///
    /// # Examples
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::uri::Uri;
    /// use rdf::triple::Triple;
    ///
    /// let mut graph = Graph::new(None);
    ///
    /// let subject = graph.create_blank_node();
    /// let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
    /// let object = graph.create_blank_node();
    /// let triple = Triple::new(&subject, &predicate, &object);
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
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::uri::Uri;
    /// use rdf::triple::Triple;
    ///
    /// let mut graph = Graph::new(None);
    ///
    /// let subject1 = graph.create_blank_node();
    /// let subject2 = graph.create_blank_node();
    /// let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
    /// let object = graph.create_blank_node();
    ///
    /// let triple1 = Triple::new(&subject1, &predicate, &object);
    /// let triple2 = Triple::new(&subject2, &predicate, &object);
    ///
    /// graph.add_triples(&vec![triple1.to_owned(), triple2]);
    ///
    /// assert_eq!(graph.get_triples_with_subject(&subject1), vec![&triple1]);
    /// ```
    pub fn get_triples_with_subject(&self, node: &Node) -> Vec<&Triple> {
        self.triples.get_triples_with_subject(node)
    }

    /// Returns all triples from the store that have the specified predicate node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::uri::Uri;
    /// use rdf::triple::Triple;
    ///
    /// let mut graph = Graph::new(None);
    ///
    /// let subject1 = graph.create_blank_node();
    /// let subject2 = graph.create_blank_node();
    /// let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
    /// let object = graph.create_blank_node();
    ///
    /// let triple1 = Triple::new(&subject1, &predicate, &object);
    /// let triple2 = Triple::new(&subject2, &predicate, &object);
    ///
    /// graph.add_triples(&vec![triple1.to_owned(), triple2.to_owned()]);
    ///
    /// assert_eq!(graph.get_triples_with_predicate(&predicate), vec![&triple1, &triple2]);
    /// ```
    pub fn get_triples_with_predicate(&self, node: &Node) -> Vec<&Triple> {
        self.triples.get_triples_with_predicate(node)
    }

    /// Returns all triples from the store that have the specified object node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::uri::Uri;
    /// use rdf::triple::Triple;
    ///
    /// let mut graph = Graph::new(None);
    ///
    /// let subject1 = graph.create_blank_node();
    /// let subject2 = graph.create_blank_node();
    /// let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
    /// let object = graph.create_blank_node();
    ///
    /// let triple1 = Triple::new(&subject1, &predicate, &object);
    /// let triple2 = Triple::new(&subject2, &predicate, &object);
    ///
    /// graph.add_triples(&vec![triple1.to_owned(), triple2.to_owned()]);
    ///
    /// assert_eq!(graph.get_triples_with_object(&object), vec![&triple1, &triple2]);
    /// ```
    pub fn get_triples_with_object(&self, node: &Node) -> Vec<&Triple> {
        self.triples.get_triples_with_object(node)
    }

    /// Returns all triples from the triple store where the subject and object nodes match the provided nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::uri::Uri;
    /// use rdf::triple::Triple;
    ///
    /// let mut graph = Graph::new(None);
    ///
    /// let subject1 = graph.create_blank_node();
    /// let subject2 = graph.create_blank_node();
    /// let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
    /// let object = graph.create_blank_node();
    ///
    /// let triple1 = Triple::new(&subject1, &predicate, &object);
    /// let triple2 = Triple::new(&subject2, &predicate, &object);
    ///
    /// graph.add_triples(&vec![triple1.to_owned(), triple2]);
    ///
    /// assert_eq!(graph.get_triples_with_subject_and_object(&subject1, &object), vec![&triple1]);
    /// ```
    pub fn get_triples_with_subject_and_object(
        &self,
        subject_node: &Node,
        object_node: &Node,
    ) -> Vec<&Triple> {
        self.triples
            .get_triples_with_subject_and_object(subject_node, object_node)
    }

    /// Returns all triples from the triple store where the subject and predicate nodes match the provided nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::uri::Uri;
    /// use rdf::triple::Triple;
    ///
    /// let mut graph = Graph::new(None);
    ///
    /// let subject1 = graph.create_blank_node();
    /// let subject2 = graph.create_blank_node();
    /// let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
    /// let object = graph.create_blank_node();
    ///
    /// let triple1 = Triple::new(&subject1, &predicate, &object);
    /// let triple2 = Triple::new(&subject2, &predicate, &object);
    ///
    /// graph.add_triples(&vec![triple1.to_owned(), triple2]);
    ///
    /// assert_eq!(graph.get_triples_with_subject_and_predicate(&subject1, &predicate), vec![&triple1]);
    /// ```
    pub fn get_triples_with_subject_and_predicate(
        &self,
        subject_node: &Node,
        predicate_node: &Node,
    ) -> Vec<&Triple> {
        self.triples
            .get_triples_with_subject_and_predicate(subject_node, predicate_node)
    }

    /// Returns all triples from the triple store where the predicate and object nodes match the provided nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::graph::Graph;
    /// use rdf::uri::Uri;
    /// use rdf::triple::Triple;
    ///
    /// let mut graph = Graph::new(None);
    ///
    /// let subject1 = graph.create_blank_node();
    /// let subject2 = graph.create_blank_node();
    /// let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
    /// let object = graph.create_blank_node();
    ///
    /// let triple1 = Triple::new(&subject1, &predicate, &object);
    /// let triple2 = Triple::new(&subject2, &predicate, &object);
    ///
    /// graph.add_triples(&vec![triple1.to_owned(), triple2.to_owned()]);
    ///
    /// assert_eq!(graph.get_triples_with_predicate_and_object(&predicate, &object), vec![&triple1, &triple2]);
    /// ```
    pub fn get_triples_with_predicate_and_object(
        &self,
        predicate_node: &Node,
        object_node: &Node,
    ) -> Vec<&Triple> {
        self.triples
            .get_triples_with_predicate_and_object(predicate_node, object_node)
    }

    /// Returns an iterator over the triples of the graph.
    pub fn triples_iter(&self) -> Iter<Triple> {
        self.triples.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::Graph;
    use crate::node::*;

    #[test]
    fn empty_graph() {
        let graph = Graph::new(None);
        assert_eq!(graph.is_empty(), true);
    }

    #[test]
    fn create_literal_node() {
        let graph = Graph::new(None);
        let literal_node = graph.create_literal_node("literal".to_string());

        assert_eq!(
            literal_node,
            Node::LiteralNode {
                literal: "literal".to_string(),
                data_type: None,
                language: None,
            }
        );
    }

    #[test]
    fn create_multiple_blank_nodes() {
        let mut graph = Graph::new(None);

        let blank_node_0 = graph.create_blank_node();
        let blank_node_1 = graph.create_blank_node();

        assert_eq!(
            blank_node_0,
            Node::BlankNode {
                id: "auto0".to_string(),
            }
        );

        assert_eq!(
            blank_node_1,
            Node::BlankNode {
                id: "auto1".to_string(),
            }
        );
    }
}
