use node::{Node};
use std::slice::Iter;
use std::vec::IntoIter;


/// Triple segment.
#[derive(PartialEq)]
pub enum TripleSegment {
  Subject,
  Predicate,
  Object
}


/// Triple representation.
#[derive(Clone)]
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




/// Storage for triples.
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
