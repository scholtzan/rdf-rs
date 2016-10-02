use writer::formatter::rdf_formatter::*;
use writer::formatter::n_triples_formatter::NTriplesFormatter;
use writer::rdf_writer::RdfWriter;
use graph::Graph;
use node::Node;
use triple::*;
use error::*;
use Result;


/// RDF writer to generate N-Triples syntax.
pub struct NTriplesWriter {
  formatter: NTriplesFormatter
}

impl RdfWriter for NTriplesWriter {
  /// Generates the N-Triples syntax for each triple stored in the provided graph.
  ///
  /// Returns an error if invalid N-Triple syntax would be generated.
  ///
  fn write_to_string(&self, graph: &Graph) -> Result<String> {
    let mut output_string = "".to_string();

    for triple in graph.triples_iter() {
      // convert each triple of the graph to N-Triple syntax
      match self.triple_to_n_triples(&triple) {
        Ok(str) => {
          output_string.push_str(&str);
          output_string.push_str("\n");
        },
        Err(error) => return Err(error),
      }
    }

    Ok(output_string)
  }
}


impl NTriplesWriter {
  /// Constructor of `NTriplesWriter`.
  pub fn new() -> NTriplesWriter {
    NTriplesWriter {
      formatter: NTriplesFormatter::new()
    }
  }

  /// Generates the corresponding N-Triples syntax of the provided triple.
  pub fn triple_to_n_triples(&self, triple: &Triple) -> Result<String> {
    let mut output_string = "".to_string();

    // convert subject
    match self.node_to_n_triples(triple.subject(), TripleSegment::Subject) {
      Ok(str) => output_string.push_str(&str),
      Err(error) => return Err(error),
    }

    output_string.push_str(" ");

    // convert predicate
    match self.node_to_n_triples(triple.predicate(), TripleSegment::Predicate) {
      Ok(str) => output_string.push_str(&str),
      Err(error) => return Err(error),
    }

    output_string.push_str(" ");

    // convert object
    match self.node_to_n_triples(triple.object(), TripleSegment::Object) {
      Ok(str) => output_string.push_str(&str),
      Err(error) => return Err(error),
    }

    output_string.push_str(" .");

    Ok(output_string)
  }

  /// Converts a single node to its corresponding N-Triples representation.
  ///
  /// Checks if the node type is valid considering the triple segment.
  ///
  pub fn node_to_n_triples(&self, node: &Node, segment: TripleSegment) -> Result<String> {
    match node {
      &Node::BlankNode { id: _ } =>
        // blank nodes are not allowed as predicates
        if segment == TripleSegment::Predicate {
          return Err(Error::InvalidWriterOutput)
        },
      &Node::LiteralNode { literal: _, data_type: ref dt, language: ref lang } => {
        // literal nodes are only allowed as objects
        if segment != TripleSegment::Object {
          return Err(Error::InvalidWriterOutput)
        }

        // either language or data type could be defined, but not both
        if *lang != None && *dt != None {
          return Err(Error::InvalidWriterOutput)
        }
      },
      _ => {},
    }

    // use the formatter to get the corresponding N-Triple syntax
    Ok(self.formatter.format_node(node))
  }
}


#[cfg(test)]
mod tests {
  use node::Node;
  use triple::*;
  use uri::Uri;
  use writer::n_triples_writer::NTriplesWriter;

  #[test]
  fn test_n_triples_writer() {
    let subject = Node::BlankNode { id: "blank".to_string() };
    let object = Node::LiteralNode { literal: "literal".to_string(), data_type: None, language: Some("en".to_string()) };
    let predicate = Node::UriNode { uri: Uri::new("http://example.org/show/localName".to_string()) };

    let trip = Triple::new(subject, predicate, object);

    let result = "_:blank <http://example.org/show/localName> \"literal\"@en .".to_string();

    let writer = NTriplesWriter::new();
    match writer.triple_to_n_triples(&trip) {
      Ok(str) => assert_eq!(result, str),
      Err(_) => assert!(false)
    }
  }
}