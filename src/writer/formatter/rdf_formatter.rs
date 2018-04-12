use node::Node;
use uri::Uri;

/// Trait implemented by RDF formatters for formatting nodes.
pub trait RdfFormatter {
    /// Determines the node and its corresponding format.
    fn format_node(&self, node: &Node) -> String;

    /// Formats a literal.
    fn format_literal(
        &self,
        literal: &String,
        data_type: &Option<Uri>,
        language: &Option<String>,
    ) -> String;

    /// Formats the content of a blank node.
    fn format_blank(&self, id: &String) -> String;

    /// Formats a URI.
    fn format_uri(&self, uri: &Uri) -> String;
}
