use graph::Graph;
use Result;

pub trait RdfWriter {
  fn write_to_string(&self, graph: &Graph) -> Result<String>;
}