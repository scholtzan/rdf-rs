use writer::formatter::turtle_formatter::TurtleFormatter;
use writer::rdf_writer::RdfWriter;
use graph::Graph;
use node::Node;
use triple::Triple;
use Result;

// todo
pub struct TurtleWriter {
  formatter: TurtleFormatter
}

impl RdfWriter for TurtleWriter {
  fn write_to_string(&self, graph: &Graph) -> Result<String> {
    Ok("test".to_string())
  }
}

impl TurtleWriter {
  fn new() -> TurtleWriter {
    TurtleWriter {
      formatter: TurtleFormatter::new()
    }
  }

  fn triple_to_turtle(&self, triple: &Triple) -> Result<String> {
    Ok("test".to_string())
  }

  fn node_to_turtle(&self, node: &Node) -> Result<String> {
    Ok("test".to_string())
  }
}
