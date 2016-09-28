use graph::Graph;
use Result;


/// Trait implemented by RDF readers to generate a RDF graph from RDF syntax.
pub trait RdfReader {
  /// Generates an RDF graph from a provided RDF syntax.
  /// Returns an error if invalid RDF input is provided.
  fn decode(&mut self) -> Result<Graph>;
}