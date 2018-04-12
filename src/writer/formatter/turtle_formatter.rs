use node::Node;
use uri::Uri;
use writer::formatter::rdf_formatter::RdfFormatter;
use specs::turtle_specs::TurtleSpecs;
use std::collections::HashMap;
use specs::rdf_syntax_specs::RdfSyntaxSpecs;

/// Formatter for formatting nodes to Turtle syntax.
/// This formatter is used by `TurtleWriter`.
pub struct TurtleFormatter<'a> {
    namespaces: &'a HashMap<String, Uri>,
}

impl<'a> TurtleFormatter<'a> {
    /// Constructor of `TurtleFormatter`.
    pub fn new(namespaces: &'a HashMap<String, Uri>) -> TurtleFormatter<'a> {
        TurtleFormatter { namespaces }
    }
}

impl<'a> RdfFormatter for TurtleFormatter<'a> {
    /// Returns the corresponding Turtle formatting for a node.
    ///
    /// Determines the node type, extracts its content and calls the
    /// right function for formatting this content.
    fn format_node(&self, node: &Node) -> String {
        match *node {
            Node::BlankNode { ref id } => self.format_blank(id),
            Node::LiteralNode {
                ref literal,
                ref data_type,
                ref language,
            } => self.format_literal(literal, data_type, language),
            Node::UriNode { ref uri } => self.format_uri(uri),
        }
    }

    /// Formats a literal to the corresponding Turtle syntax.
    ///
    /// Also considers the data type and language of the literal.
    fn format_literal(
        &self,
        literal: &str,
        data_type: &Option<Uri>,
        language: &Option<String>,
    ) -> String {
        let mut output_string = "".to_string();

        if TurtleSpecs::is_plain_literal(literal, data_type) && *language == None {
            // some number or boolean
            output_string.push_str(literal);
        } else {
            output_string.push_str("\"");
            output_string.push_str(&RdfSyntaxSpecs::escape_literal(literal));
            output_string.push_str("\"");
        }

        if let Some(ref lang) = *language {
            output_string.push_str("@");
            output_string.push_str(lang);
        }

        if let Some(ref dt) = *data_type {
            output_string.push_str("^^");
            output_string.push_str(&self.format_uri(dt));
        }

        output_string
    }

    /// Formats the content of a blank node to the corresponding Turtle syntax.
    fn format_blank(&self, id: &str) -> String {
        "_:".to_string() + id
    }

    /// Formats a URI to Turtle syntax.
    fn format_uri(&self, uri: &Uri) -> String {
        let mut output_string = "".to_string();

        // write QName if namespace for URI exists
        for (prefix, namespace_uri) in self.namespaces.iter() {
            if uri.to_string().starts_with(namespace_uri.to_string()) {
                output_string.push_str(prefix);
                output_string.push_str(":");

                let path = uri.to_string()
                    .to_owned()
                    .replace(namespace_uri.to_string(), "")
                    .replace("/", ":");
                output_string.push_str(&path);

                return output_string;
            }
        }

        output_string.push_str("<");
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
    use writer::formatter::turtle_formatter::TurtleFormatter;
    use specs::xml_specs::XmlDataTypes;
    use std::collections::HashMap;

    #[test]
    fn test_turtle_blank_node_formatting() {
        let hashmap = HashMap::new();
        let formatter = TurtleFormatter::new(&hashmap);
        let node = Node::BlankNode {
            id: "auto0".to_string(),
        };

        assert_eq!(formatter.format_node(&node), "_:auto0".to_string());
    }

    #[test]
    fn test_turtle_uri_node_formatting() {
        let hashmap = HashMap::new();
        let formatter = TurtleFormatter::new(&hashmap);
        let node = Node::UriNode {
            uri: Uri::new("http://example.org/show/localName".to_string()),
        };

        assert_eq!(
            formatter.format_node(&node),
            "<http://example.org/show/localName>".to_string()
        );
    }

    #[test]
    fn test_turtle_qname_node_formatting() {
        let mut hashmap = HashMap::new();
        hashmap.insert(
            "example".to_string(),
            Uri::new("http://example.org/".to_string()),
        );

        let formatter = TurtleFormatter::new(&hashmap);
        let node = Node::UriNode {
            uri: Uri::new("http://example.org/show/localName".to_string()),
        };

        assert_eq!(
            formatter.format_node(&node),
            "example:show:localName".to_string()
        );
    }

    #[test]
    fn test_turtle_plain_literal_node_formatting() {
        let hashmap = HashMap::new();
        let formatter = TurtleFormatter::new(&hashmap);
        let node = Node::LiteralNode {
            literal: "literal".to_string(),
            data_type: None,
            language: None,
        };

        assert_eq!(formatter.format_node(&node), "\"literal\"".to_string());
    }

    #[test]
    fn test_turtle_escaped_literal_node_formatting() {
        let hashmap = HashMap::new();
        let formatter = TurtleFormatter::new(&hashmap);
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
    fn test_turtle_literal_node_with_datatype_formatting() {
        let hashmap = HashMap::new();
        let formatter = TurtleFormatter::new(&hashmap);
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
    fn test_turtle_literal_node_with_language_formatting() {
        let hashmap = HashMap::new();
        let formatter = TurtleFormatter::new(&hashmap);
        let node = Node::LiteralNode {
            literal: "literal".to_string(),
            data_type: None,
            language: Some("en".to_string()),
        };

        assert_eq!(formatter.format_node(&node), "\"literal\"@en".to_string());
    }

    #[test]
    fn test_turtle_boolean_literal_node_formatting() {
        let hashmap = HashMap::new();
        let formatter = TurtleFormatter::new(&hashmap);
        let node = Node::LiteralNode {
            literal: "true".to_string(),
            data_type: Some(XmlDataTypes::Boolean.to_uri()),
            language: None,
        };

        assert_eq!(
            formatter.format_node(&node),
            "true^^<http://www.w3.org/2001/XMLSchema#boolean>".to_string()
        );
    }

    #[test]
    fn test_turtle_integer_literal_node_formatting() {
        let hashmap = HashMap::new();
        let formatter = TurtleFormatter::new(&hashmap);
        let node = Node::LiteralNode {
            literal: "123".to_string(),
            data_type: Some(XmlDataTypes::Integer.to_uri()),
            language: None,
        };

        assert_eq!(
            formatter.format_node(&node),
            "123^^<http://www.w3.org/2001/XMLSchema#integer>".to_string()
        );
    }

    #[test]
    fn test_turtle_decimal_literal_node_formatting() {
        let hashmap = HashMap::new();
        let formatter = TurtleFormatter::new(&hashmap);
        let node = Node::LiteralNode {
            literal: "123.123".to_string(),
            data_type: Some(XmlDataTypes::Decimal.to_uri()),
            language: None,
        };

        assert_eq!(
            formatter.format_node(&node),
            "123.123^^<http://www.w3.org/2001/XMLSchema#decimal>".to_string()
        );
    }
}
