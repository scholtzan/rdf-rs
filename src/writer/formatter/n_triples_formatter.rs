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
      &Node::BlankNode { ref id } => self.format_blank(&id),
      &Node::LiteralNode { ref literal, ref prefix, ref data_type, ref language } =>
        self.format_literal(&literal, data_type, language),
      &Node::UriNode { ref uri } =>
        self.format_uri(uri),
    }
  }


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

  fn format_blank(&self, id: &String) -> String {
    "_:".to_string() + &id.to_string()
  }

  fn format_uri(&self, uri: &Uri) -> String {
    let mut output_string = "<".to_string();
    output_string.push_str(uri.uri());
    output_string.push_str(">");

    output_string
  }
}
