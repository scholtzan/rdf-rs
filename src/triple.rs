use node::Node;
use std::slice::Iter;
use std::vec::IntoIter;
use std::cmp::PartialEq;

/// Triple segment.
#[derive(PartialEq, Debug)]
pub enum TripleSegment {
    Subject,
    Predicate,
    Object,
}

/// Triple representation.
#[derive(PartialOrd, Ord, Clone, Debug)]
pub struct Triple {
    subject: Node,
    predicate: Node,
    object: Node,
}

impl Triple {
    /// Constructor for Triple struct.
    ///
    /// Requires subject, predicate and object nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::triple::Triple;
    /// use rdf::node::Node;
    /// use rdf::uri::Uri;
    ///
    /// let subject = Node::BlankNode { id: "a".to_string() };
    /// let predicate = Node::UriNode { uri: Uri::new("http://example.org/show/localName".to_string()) } ;
    /// let object = Node::BlankNode { id: "b".to_string() };
    ///
    /// Triple::new(&subject, &predicate, &object);
    /// ```
    pub fn new(subject: &Node, predicate: &Node, object: &Node) -> Triple {
        Triple {
            subject: subject.clone(),
            predicate: predicate.clone(),
            object: object.clone(),
        }
    }

    /// Returns a reference to the subject node of the triple.
    pub fn subject(&self) -> &Node {
        &self.subject
    }

    /// Returns a reference to the predicate node of the triple.
    pub fn predicate(&self) -> &Node {
        &self.predicate
    }

    /// Returns a reference to the object node of the triple.
    pub fn object(&self) -> &Node {
        &self.object
    }
}

impl PartialEq for Triple {
    fn eq(&self, other: &Triple) -> bool {
        self.subject() == other.subject() && self.predicate() == other.predicate()
            && self.object() == other.object()
    }
}

impl Eq for Triple {}

/// Storage for triples.
#[derive(Debug, Default)]
pub struct TripleStore {
    triples: Vec<Triple>,
}

impl TripleStore {
    /// Constructs a new triple store.
    pub fn new() -> TripleStore {
        TripleStore {
            triples: Vec::new(),
        }
    }

    /// Returns the number of triples that are stored.
    pub fn count(&self) -> usize {
        self.triples.len()
    }

    /// Checks if the triple store is empty.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Adds a new triple to the store.
    pub fn add_triple(&mut self, triple: &Triple) {
        self.triples.push(triple.clone());
    }

    /// Deletes the triple from the store.
    pub fn remove_triple(&mut self, triple: &Triple) {
        self.triples.retain(|t| t != triple);
    }

    /// Returns all triples where the subject node matches the provided node.
    pub fn get_triples_with_subject(&self, node: &Node) -> Vec<&Triple> {
        self.triples
            .iter()
            .filter(|t| t.subject() == node)
            .collect::<Vec<_>>()
    }

    /// Returns all triples where the predicate node matches the provided node.
    pub fn get_triples_with_predicate(&self, node: &Node) -> Vec<&Triple> {
        self.triples
            .iter()
            .filter(|t| t.predicate() == node)
            .collect::<Vec<_>>()
    }

    /// Returns all triples where the object node matches the provided node.
    pub fn get_triples_with_object(&self, node: &Node) -> Vec<&Triple> {
        self.triples
            .iter()
            .filter(|t| t.object() == node)
            .collect::<Vec<_>>()
    }

    /// Returns all triples where the subject and object nodes match the provided nodes.
    pub fn get_triples_with_subject_and_object(
        &self,
        subject_node: &Node,
        object_node: &Node,
    ) -> Vec<&Triple> {
        self.triples
            .iter()
            .filter(|t| t.object() == object_node && t.subject() == subject_node)
            .collect::<Vec<_>>()
    }

    /// Returns all triples where the subject and predicate nodes match the provided nodes.
    pub fn get_triples_with_subject_and_predicate(
        &self,
        subject_node: &Node,
        predicate_node: &Node,
    ) -> Vec<&Triple> {
        self.triples
            .iter()
            .filter(|t| t.predicate() == predicate_node && t.subject() == subject_node)
            .collect::<Vec<_>>()
    }

    /// Returns all triples where the predicate and object nodes match the provided nodes.
    pub fn get_triples_with_predicate_and_object(
        &self,
        predicate_node: &Node,
        object_node: &Node,
    ) -> Vec<&Triple> {
        self.triples
            .iter()
            .filter(|t| t.predicate() == predicate_node && t.object() == object_node)
            .collect::<Vec<_>>()
    }

    /// Returns all blank nodes of the store.
    pub fn get_blank_nodes(&self) -> Vec<&Node> {
        let mut blank_nodes = Vec::new();

        for triple in &self.triples {
            match *triple {
                Triple {
                    subject: Node::BlankNode { .. },
                    object: Node::BlankNode { .. },
                    ..
                } => {
                    blank_nodes.push(triple.subject());
                    blank_nodes.push(triple.object());
                }
                Triple {
                    subject: Node::BlankNode { .. },
                    ..
                } => blank_nodes.push(triple.subject()),
                Triple {
                    object: Node::BlankNode { .. },
                    ..
                } => blank_nodes.push(triple.object()),
                _ => {}
            }
        }

        blank_nodes
    }

    /// Returns the stored triples as vector.
    pub fn into_vec(self) -> Vec<Triple> {
        self.triples
    }

    /// Returns an iterator over the stored triples.
    pub fn iter(&self) -> Iter<Triple> {
        self.triples.iter()
    }
}

impl IntoIterator for TripleStore {
    type Item = Triple;
    type IntoIter = IntoIter<Triple>;

    fn into_iter(self) -> Self::IntoIter {
        self.triples.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use node::*;
    use triple::*;

    #[test]
    fn empty_triple_store() {
        let store = TripleStore::new();

        assert!(store.is_empty());
    }

    #[test]
    fn count_triples_in_triple_store() {
        let mut store = TripleStore::new();

        let subject = Node::LiteralNode {
            literal: "abcd".to_string(),
            data_type: None,
            language: None,
        };

        let predicate = Node::LiteralNode {
            literal: "d".to_string(),
            data_type: None,
            language: None,
        };

        let object = Node::LiteralNode {
            literal: "s".to_string(),
            data_type: None,
            language: None,
        };

        let trip = Triple::new(&subject, &predicate, &object);

        store.add_triple(&trip);

        assert_eq!(store.count(), 1);
    }
}
