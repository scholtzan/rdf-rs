use writer::formatter::rdf_formatter::RdfFormatter;
use node::Node;
use uri::Uri;

pub struct NTriplesFormatter { }

impl NTriplesFormatter {
  pub fn new() -> NTriplesFormatter {
    NTriplesFormatter { }
  }
}



impl RdfFormatter for NTriplesFormatter {
  fn format_node(&self, node: &Node) -> String {
    match node {
      &Node::BlankNode { id } => self.format_blank(id),
      _ => "".to_string()
    }
  }


  fn format_literal(&self, literal: &String) -> String {
    "".to_string()
  }

  fn format_blank(&self, id: i64) -> String {
    "".to_string()
  }

  fn format_uri(&self, uri: &Uri) -> String {
    "".to_string()
  }

  fn format_variable(&self, var: &String) -> String {
    "".to_string()
  }
}
