use uri::Uri;

pub enum Node {
  UriNode { uri: Uri },
  LiteralNode { literal: &'static str },
  BlankNode { id: i64 }
}


#[cfg(test)]
mod tests {
  use node::*;

  #[test]
  fn access_literal_node() {
    let node = Node::LiteralNode { literal: "abcd" };

    match node {
      Node::LiteralNode { literal: lit } => assert_eq!(lit, "abcd"),
      _ => assert!(false)
    }
  }
}