use uri::Uri;

#[derive(Clone)]
pub enum LiteralNodeType {
  PlainLiteral,
  StringLiteral,
  DecimalLiteral,
  BooleanLiteral
}


#[derive(Clone)]
pub enum Node {
  UriNode { uri: Uri },
  LiteralNode { literal: String, prefix: String, nodeType: LiteralNodeType },
  BlankNode { id: i64 }
}






#[cfg(test)]
mod tests {
  use node::*;

  #[test]
  fn access_literal_node() {
    let node = Node::LiteralNode {
      literal: "abcd".to_string(),
      prefix: "resat".to_string(),
      nodeType: LiteralNodeType::PlainLiteral
    };

    match node {
      Node::LiteralNode { literal: lit, prefix: _, nodeType: _ } => assert_eq!(lit, "abcd".to_string()),
      _ => assert!(false)
    }
  }
}