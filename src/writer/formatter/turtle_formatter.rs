use node::Node;
use uri::Uri;
use writer::formatter::rdf_formatter::RdfFormatter;
use specs::turtle_specs::TurtleSpecs;
use std::collections::HashMap;


pub struct TurtleFormatter<'a> {
  namespaces: &'a HashMap<String, Uri>
}

impl<'a> TurtleFormatter<'a> {
  pub fn new(namespaces: &'a HashMap<String, Uri>) -> TurtleFormatter<'a> {
    TurtleFormatter {
      namespaces: namespaces
    }
  }
}

// todo: escaping of specific characters

impl<'a> RdfFormatter for TurtleFormatter<'a> {
  /// Returns the corresponding Turtle formatting for a node.
  ///
  /// Determines the node type, extracts its content and calls the
  /// right function for formatting this content.
  fn format_node(&self, node: &Node) -> String {
    match node {
      &Node::BlankNode { ref id } => self.format_blank(&id),
      &Node::LiteralNode { ref literal, ref data_type, ref language } =>
        self.format_literal(&literal, data_type, language),
      &Node::UriNode { ref uri } =>
        self.format_uri(uri),
    }
  }

  /// todo: multiline, escaping
  /// todo: error handling if wrong type
  /// Formats a literal to the corresponding Turtle syntax.
  ///
  /// Also considers the data type and language of the literal.
  ///
  fn format_literal(&self, literal: &String, data_type: &Option<Uri>, language: &Option<String>) -> String {
    let mut output_string = "".to_string();

    if TurtleSpecs::is_plain_literal(literal, data_type) && *language == None {
      // some number or boolean
      output_string.push_str(&literal);
    } else {
      output_string.push_str(&literal);
      output_string.push_str("\"");
    }

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

  /// Formats the content of a blank node to the corresponding Turtle syntax.
  fn format_blank(&self, id: &String) -> String {
    "_:".to_string() + &id.to_string()
  }

  /// Formats a URI to Turtle syntax.
  fn format_uri(&self, uri: &Uri) -> String {
    let mut output_string = "".to_string();

    for (prefix, namespace_uri) in self.namespaces.iter() {
      if uri.uri().starts_with(namespace_uri.uri()) {
        output_string.push_str(prefix);
        output_string.push_str(":");
        output_string.push_str(&uri.uri().to_owned().replace(namespace_uri.uri(), ""));

        return output_string;
      }
    }

    output_string.push_str("<");
    output_string.push_str(uri.uri());
    output_string.push_str(">");

    output_string
  }
}