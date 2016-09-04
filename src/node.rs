use uri::Uri;

pub enum Node {
  UriNode { uri: Uri },
  LiteralNode { literal: &'static str },
  BlankNode { id: i64 }
}

