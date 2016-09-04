use node::{Node};

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


pub struct TripleStore {
  triples: Vec<Triple>
}

impl TripleStore {
  pub fn count(&self) -> usize {
    self.triples.len()
  }

  pub fn is_empty(&self) -> bool {
    self.count() == 0
  }

  pub fn add_triple(&mut self, triple: Triple) {
    self.triples.push(triple);
  }
}



#[cfg(test)]
mod tests {
  use node::*;
  use triple::Triple;

  #[test]
  fn creating_tripe_works() {
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
