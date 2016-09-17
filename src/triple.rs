use node::{Node};
use std::slice::Iter;
use std::vec::IntoIter;
use std::cmp::PartialEq;
use std::fmt;

/// Triple segment.
#[derive(PartialEq, Debug)]
pub enum TripleSegment {
  Subject,
  Predicate,
  Object
}


/// Triple representation.
#[derive(Clone, Debug)]
pub struct Triple {
  subject: Node,
  predicate: Node,
  object: Node
}


impl Triple {
  /// Constructor for Triple struct.
  ///
  /// Requires subject, predicate and object nodes.
  ///
  /// # Examples
  ///
  /// todo
  ///
  pub fn new(subject: Node, predicate: Node, object: Node) -> Triple {
    Triple {
      subject: subject.clone(),
      predicate: predicate.clone(),
      object: object.clone()
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
    self.subject() == other.subject() &&
    self.predicate() == other.predicate() &&
    self.object() == other.object()
  }
}

impl Eq for Triple { }




/// Storage for triples.
#[derive(Debug)]
pub struct TripleStore {
  triples: Vec<Triple>
}


impl TripleStore {
  /// Constructs a new triple store.
  pub fn new() -> TripleStore {
    TripleStore {
      triples: Vec::new()
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
  pub fn add_triple(&mut self, triple: Triple) {
    self.triples.push(triple.clone());
  }

  /// Deletes the triple from the store.
  pub fn remove_triple(&mut self, triple: Triple) {
    self.triples.retain(|ref t| {
      **t != triple
    });
  }

  /// Returns all triples where the subject node matches the provided node.
  ///
  /// # Example
  ///
  /// todo
  ///
  pub fn get_triples_with_subject(&self, node: &Node) -> Vec<&Triple> {
    self.triples.iter().filter(|t| t.subject() == node).collect::<Vec<_>>()
  }

  /// Returns all triples where the predicate node matches the provided node.
  ///
  /// # Example
  ///
  /// todo
  ///
  pub fn get_triples_with_predicate(&self, node: &Node) -> Vec<&Triple> {
    self.triples.iter().filter(|t| t.predicate() == node).collect::<Vec<_>>()
  }

  /// Returns all triples where the object node matches the provided node.
  ///
  /// # Example
  ///
  /// todo
  ///
  pub fn get_triples_with_object(&self, node: &Node) -> Vec<&Triple> {
    self.triples.iter().filter(|t| t.object() == node).collect::<Vec<_>>()
  }

  /// Returns all blank nodes of the store.
  pub fn get_blank_nodes(&self) -> Vec<&Node> {
    let mut blank_subject_nodes = self.triples.iter().filter_map(|t| {
      match t {
        &Triple { subject: Node::BlankNode {id : _}, predicate: _, object: _ } =>
          Some(t.subject()),
        _ => None // does not contain a blank node
      }
    }).collect::<Vec<&Node>>();

    let mut blank_object_nodes = self.triples.iter().filter_map(|t| {
      match t {
        &Triple { subject: _, predicate: _, object: Node::BlankNode {id : _} } =>
          Some(t.object()),
        _ => None // does not contain a blank node
      }
    });

    blank_subject_nodes.extend(blank_object_nodes);

    blank_subject_nodes
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
    self.into_iter()
  }
}



#[cfg(test)]
mod tests {
  use node::*;
  use triple::*;
  use std::collections::LinkedList;

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
      prefix: None,
      data_type: None,
      language: None
    };

    let predicate = Node::LiteralNode {
      literal: "d".to_string(),
      prefix: None,
      data_type: None,
      language: None
    };

    let object = Node::LiteralNode {
      literal: "s".to_string(),
      prefix: None,
      data_type: None,
      language: None
    };

    let trip = Triple::new(subject, predicate, object);

    store.add_triple(trip);

    assert_eq!(store.count(), 1);
  }
}
