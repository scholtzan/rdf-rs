use crate::uri::Uri;

/// Node representation.
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum Node {
    /// Node for representing a URI.
    UriNode { uri: Uri },

    /// Node for representing literals.
    LiteralNode {
        literal: String,
        data_type: Option<Uri>,
        language: Option<String>,
    },

    /// Node for representing blanks.
    BlankNode { id: String },
}

#[cfg(test)]
mod tests {
    use crate::node::*;

    #[test]
    fn access_literal_node() {
        let node = Node::LiteralNode {
            literal: "abcd".to_string(),
            data_type: None,
            language: None,
        };

        match node {
            Node::LiteralNode {
                literal: lit,
                data_type: _,
                language: _,
            } => assert_eq!(lit, "abcd".to_string()),
            _ => assert!(false),
        }
    }
}
