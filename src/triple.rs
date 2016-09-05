use node::{Node};


/// Triple representation.
pub struct Triple {
  subject: Node,
  predicate: Node,
  object: Node
}


impl Triple {
  pub fn new(subject: Node, predicate: Node, object: Node) -> Triple {
    Triple {
      subject: subject,
      predicate: predicate,
      object: object
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
    self.triples.push(triple);
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

    let subject = Node::LiteralNode { literal: "abcd" };
    let predicate = Node::LiteralNode { literal: "d" };
    let object = Node::LiteralNode { literal: "s" };

    let trip = Triple::new(subject, predicate, object);
    store.add_triple(trip);

    assert_eq!(store.count(), 1);
  }


  #[test]
  fn creating_triple_works() {
    let subject = Node::LiteralNode { literal: "abcd" };
    let predicate = Node::LiteralNode { literal: "d" };
    let object = Node::LiteralNode { literal: "s" };

    let trip = Triple::new(subject, predicate, object);

    let s = Node::BlankNode { id: 12 };
    let p = Node::LiteralNode { literal: "s" };
    let o = Node::BlankNode { id: 3 };

    let t = Triple::new(s, p, o);

    let mut st: Vec<Triple> = Vec::new();
    st.push(trip);
    st.push(t);

  }
}
