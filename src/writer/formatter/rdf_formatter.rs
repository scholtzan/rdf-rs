use node::Node;
use uri::Uri;

pub trait RdfFormatter {
  fn format_node(&self, node: &Node) -> String;

  fn format_literal(&self, literal: &String) -> String;
  fn format_blank(&self, id: i64) -> String;
  fn format_uri(&self, uri: &Uri) -> String;
  fn format_variable(&self, var: &String) -> String;
}