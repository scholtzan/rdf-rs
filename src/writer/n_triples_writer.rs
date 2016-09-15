use writer::formatter::rdf_formatter::*;
use writer::formatter::n_triples_formatter::NTriplesFormatter;
use writer::rdf_writer::RdfWriter;
use graph::Graph;
use node::Node;
use triple::*;
use error::*;
use Result;


pub struct NTriplesWriter {
  formatter: NTriplesFormatter
}

impl RdfWriter for NTriplesWriter {
  fn write_to_string(&self, graph: &Graph) -> Result<String> {
    let mut output_string = "".to_string();

    for triple in graph.triples_iter() {
      match self.triple_to_n_triples(&triple) {
        Ok(str) => output_string.push_str(&str),
        Err(error) => return Err(error),
      }
    }

    Ok(output_string)
  }
}


impl NTriplesWriter {
  pub fn new() -> NTriplesWriter {
    NTriplesWriter {
      formatter: NTriplesFormatter::new()
    }
  }

  pub fn triple_to_n_triples(&self, triple: &Triple) -> Result<String> {
    let mut output_string = "".to_string();

    match self.node_to_n_triples(triple.subject(), TripleSegment::Subject) {
      Ok(str) => output_string.push_str(&str),
      Err(error) => return Err(error),
    }

    output_string.push_str(" ");

    match self.node_to_n_triples(triple.predicate(), TripleSegment::Predicate) {
      Ok(str) => output_string.push_str(&str),
      Err(error) => return Err(error),
    }

    output_string.push_str(" ");

    match self.node_to_n_triples(triple.object(), TripleSegment::Object) {
      Ok(str) => output_string.push_str(&str),
      Err(error) => return Err(error),
    }

    output_string.push_str(" .");

    Ok(output_string)
  }


  pub fn node_to_n_triples(&self, node: &Node, segment: TripleSegment) -> Result<String> {
    match node {
      &Node::BlankNode { id } =>
        if segment == TripleSegment::Predicate {
          return Err(Error::InvalidWriterOutput)
        },
      _ => {},
    }

    let formatter = NTriplesFormatter::new();

    Ok(formatter.format_node(node))
  }
}
