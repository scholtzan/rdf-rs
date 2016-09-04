use node::Node;

pub struct Triple<'a> {
  subject: &'a Node,
  predicate: &'a Node,
  object: &'a Node
}


pub struct TripleStore {
  triples: Vec<f64>
}

impl TripleStore {
  pub fn count(&self) -> usize {
    self.triples.len()
  }


  pub fn is_empty(&self) -> bool {
    self.count() == 0
  }
}