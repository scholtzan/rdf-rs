use node::{Node};


/// Triple representation.
#[derive(Clone)]
pub struct Triple {
  subject: Node,
  predicate: Node,
  object: Node
}


impl Triple {
  pub fn new(subject: Node, predicate: Node, object: Node) -> Triple {
    Triple {
      subject: subject.clone(),
      predicate: predicate.clone(),
      object: object.clone()
    }
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

    let subject = Node::LiteralNode { literal: "abcd".to_string(), prefix: "saf".to_string() };
    let predicate = Node::LiteralNode { literal: "d".to_string(), prefix: "asdf".to_string() };
    let object = Node::LiteralNode { literal: "s".to_string(), prefix: "asdf".to_string() };

    let trip = Triple::new(subject, predicate, object);

    store.add_triple(trip);

    assert_eq!(store.count(), 1);
  }
}
