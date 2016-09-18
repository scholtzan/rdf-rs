use writer::formatter::rdf_formatter::RdfFormatter;
use node::Node;
use uri::Uri;


/// Formatter for formatting nodes to N-Triple syntax.
pub struct NTriplesFormatter { }


impl NTriplesFormatter {
  /// Constructor of `NTriplesFormatter`.
  pub fn new() -> NTriplesFormatter {
    NTriplesFormatter { }
  }
}


impl RdfFormatter for NTriplesFormatter {
  /// Returns the corresponding N-Triple formatting for a node.
  ///
  /// Determines the node type, extracts its content and calls the
  /// right function for formatting this content.
  fn format_node(&self, node: &Node) -> String {
    match node {
      &Node::BlankNode { ref id } => self.format_blank(&id),
      &Node::LiteralNode { ref literal, prefix: _, ref data_type, ref language } =>
        self.format_literal(&literal, data_type, language),
      &Node::UriNode { ref uri } =>
        self.format_uri(uri),
    }
  }

  /// Formats a literal to the corresponding N-Triples syntax.
  ///
  /// Also considers the data type and language of the literal.
  ///
  fn format_literal(&self, literal: &String, data_type: &Option<Uri>, language: &Option<String>) -> String {
    let mut output_string = "\"".to_string();
    // todo: escaping?
    output_string.push_str(&literal);
    output_string.push_str("\"");

    match language {
      &Some(ref lang) => {
        output_string.push_str("@");
        output_string.push_str(lang);
      },
      &None => {},
    }

    match data_type {
      &Some(ref dt) => {
        output_string.push_str("^^");
        output_string.push_str(&self.format_uri(dt));
      },
      &None => {},
    }

    output_string
  }

  /// Formats the content of a blank node to the corresponding N-Triples syntax.
  fn format_blank(&self, id: &String) -> String {
    "_:".to_string() + &id.to_string()
  }

  /// Formats a URI to N-Triples syntax.
  fn format_uri(&self, uri: &Uri) -> String {
    let mut output_string = "<".to_string();
    output_string.push_str(uri.uri());
    output_string.push_str(">");

    output_string
  }
}
