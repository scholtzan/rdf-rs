use node::Node;
use uri::Uri;

pub trait RdfFormatter {
  fn format_node(&self, node: &Node) -> String;

  fn format_literal(&self, literal: &String, dataType: &Option<Uri>, language: &Option<String>) -> String;
  fn format_blank(&self, id: &String) -> String;
  fn format_uri(&self, uri: &Uri) -> String;
}