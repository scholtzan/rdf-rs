use graph::Graph;
use Result;

/// Trait implemented by RDF writers to generate a specific syntax.
pub trait RdfWriter {
  /// Generates RDF syntax from a provided RDF graph and writes it to a string.
  /// Returns an error if invalid RDF would be generated.
  fn write_to_string(&self, graph: &Graph) -> Result<String>;
}
