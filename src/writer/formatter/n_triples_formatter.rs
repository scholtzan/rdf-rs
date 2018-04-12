use writer::formatter::rdf_formatter::RdfFormatter;
use node::Node;
use uri::Uri;
use specs::rdf_syntax_specs::RdfSyntaxSpecs;

/// Formatter for formatting nodes to N-Triple syntax.
/// This formatter is used by `NTriplesWriter`.
pub struct NTriplesFormatter {}

impl NTriplesFormatter {
    /// Constructor of `NTriplesFormatter`.
    pub fn new() -> NTriplesFormatter {
        NTriplesFormatter {}
    }
}

impl RdfFormatter for NTriplesFormatter {
    /// Returns the corresponding N-Triple formatting for a node.
    ///
    /// Determines the node type, extracts its content and calls the
    /// right function for formatting this content.
    fn format_node(&self, node: &Node) -> String {
        match node {
            &Node::BlankNode { ref id } => self.format_blank(&id),
            &Node::LiteralNode {
                ref literal,
                ref data_type,
                ref language,
            } => self.format_literal(&literal, data_type, language),
            &Node::UriNode { ref uri } => self.format_uri(uri),
        }
    }

    /// Formats a literal to the corresponding N-Triples syntax.
    ///
    /// Also considers the data type and language of the literal.
    ///
    fn format_literal(
        &self,
        literal: &String,
        data_type: &Option<Uri>,
        language: &Option<String>,
    ) -> String {
        let mut output_string = "\"".to_string();
        output_string.push_str(&RdfSyntaxSpecs::escape_literal(&literal));
        output_string.push_str("\"");

        match language {
            &Some(ref lang) => {
                output_string.push_str("@");
                output_string.push_str(lang);
            }
            &None => {}
        }

        match data_type {
            &Some(ref dt) => {
                output_string.push_str("^^");
                output_string.push_str(&self.format_uri(dt));
            }
            &None => {}
        }

        output_string
    }

    /// Formats the content of a blank node to the corresponding N-Triples syntax.
    fn format_blank(&self, id: &String) -> String {
        "_:".to_string() + &id.to_string()
    }

    /// Formats a URI to N-Triples syntax.
    fn format_uri(&self, uri: &Uri) -> String {
        let mut output_string = "<".to_string();
        output_string.push_str(uri.to_string());
        output_string.push_str(">");

        output_string
    }
}

#[cfg(test)]
mod tests {
    use node::*;
    use writer::formatter::rdf_formatter::RdfFormatter;
    use uri::Uri;
    use writer::formatter::n_triples_formatter::NTriplesFormatter;

    #[test]
    fn test_n_triples_blank_node_formatting() {
        let formatter = NTriplesFormatter::new();
        let node = Node::BlankNode {
            id: "auto0".to_string(),
        };

        assert_eq!(formatter.format_node(&node), "_:auto0".to_string());
    }

    #[test]
    fn test_n_triples_uri_node_formatting() {
        let formatter = NTriplesFormatter::new();
        let node = Node::UriNode {
            uri: Uri::new("http://example.org/show/localName".to_string()),
        };

        assert_eq!(
            formatter.format_node(&node),
            "<http://example.org/show/localName>".to_string()
        );
    }

    #[test]
    fn test_n_triples_plain_literal_node_formatting() {
        let formatter = NTriplesFormatter::new();
        let node = Node::LiteralNode {
            literal: "literal".to_string(),
            data_type: None,
            language: None,
        };

        assert_eq!(formatter.format_node(&node), "\"literal\"".to_string());
    }

    #[test]
    fn test_n_triples_literal_node_with_datatype_formatting() {
        let formatter = NTriplesFormatter::new();
        let node = Node::LiteralNode {
            literal: "literal".to_string(),
            data_type: Some(Uri::new("http://example.org/show/localName".to_string())),
            language: None,
        };

        assert_eq!(
            formatter.format_node(&node),
            "\"literal\"^^<http://example.org/show/localName>".to_string()
        );
    }

    #[test]
    fn test_n_triples_escaped_literal_node_formatting() {
        let formatter = NTriplesFormatter::new();
        let node = Node::LiteralNode {
            literal: "literal ' \" ".to_string(),
            data_type: None,
            language: None,
        };

        assert_eq!(
            formatter.format_node(&node),
            "\"literal \' \" \"".to_string()
        );
    }

    #[test]
    fn test_n_triples_literal_node_with_language_formatting() {
        let formatter = NTriplesFormatter::new();
        let node = Node::LiteralNode {
            literal: "literal".to_string(),
            data_type: None,
            language: Some("en".to_string()),
        };

        assert_eq!(formatter.format_node(&node), "\"literal\"@en".to_string());
    }
}
