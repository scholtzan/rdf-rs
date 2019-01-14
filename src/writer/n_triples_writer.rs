use crate::error::*;
use crate::graph::Graph;
use crate::node::Node;
use crate::triple::*;
use crate::writer::formatter::n_triples_formatter::NTriplesFormatter;
use crate::writer::formatter::rdf_formatter::*;
use crate::writer::rdf_writer::RdfWriter;
use crate::Result;

/// RDF writer to generate N-Triples syntax.
#[derive(Default)]
pub struct NTriplesWriter {
    formatter: NTriplesFormatter,
}

impl RdfWriter for NTriplesWriter {
    /// Generates the N-Triples syntax for each triple stored in the provided graph.
    ///
    /// Returns an error if invalid N-Triple syntax would be generated.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::writer::n_triples_writer::NTriplesWriter;
    /// use rdf::writer::rdf_writer::RdfWriter;
    /// use rdf::graph::Graph;
    ///
    /// let writer = NTriplesWriter::new();
    /// let graph = Graph::new(None);
    ///
    /// assert_eq!(writer.write_to_string(&graph).unwrap(), "".to_string());
    /// ```
    ///
    /// # Failures
    ///
    /// - Invalid triples are to be written to the output that do not conform the NTriples syntax standard.
    ///
    fn write_to_string(&self, graph: &Graph) -> Result<String> {
        let mut output_string = "".to_string();

        for triple in graph.triples_iter() {
            // convert each triple of the graph to N-Triple syntax
            match self.triple_to_n_triples(triple) {
                Ok(str) => {
                    output_string.push_str(&str);
                    output_string.push_str("\n");
                }
                Err(error) => return Err(error),
            }
        }

        Ok(output_string)
    }
}

impl NTriplesWriter {
    /// Constructor of `NTriplesWriter`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::writer::n_triples_writer::NTriplesWriter;
    /// use rdf::writer::rdf_writer::RdfWriter;
    ///
    /// let writer = NTriplesWriter::new();
    /// ```
    pub fn new() -> NTriplesWriter {
        NTriplesWriter {
            formatter: NTriplesFormatter::new(),
        }
    }

    /// Generates the corresponding N-Triples syntax of the provided triple.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::writer::n_triples_writer::NTriplesWriter;
    /// use rdf::writer::rdf_writer::RdfWriter;
    /// use rdf::node::Node;
    /// use rdf::triple::Triple;
    /// use rdf::uri::Uri;
    ///
    /// let writer = NTriplesWriter::new();
    ///
    /// let subject = Node::BlankNode { id: "blank".to_string() };
    /// let object = Node::LiteralNode { literal: "literal".to_string(), data_type: None, language: Some("en".to_string()) };
    /// let predicate = Node::UriNode { uri: Uri::new("http://example.org/show/localName".to_string()) };
    /// let triple = Triple::new(&subject, &predicate, &object);
    ///
    /// assert_eq!(writer.triple_to_n_triples(&triple).unwrap(),
    ///            "_:blank <http://example.org/show/localName> \"literal\"@en .".to_string());
    /// ```
    ///
    /// # Failures
    ///
    /// - Invalid node type for a certain position.
    ///
    pub fn triple_to_n_triples(&self, triple: &Triple) -> Result<String> {
        let mut output_string = "".to_string();

        // convert subject
        match self.node_to_n_triples(triple.subject(), &TripleSegment::Subject) {
            Ok(str) => output_string.push_str(&str),
            Err(error) => return Err(error),
        }

        output_string.push_str(" ");

        // convert predicate
        match self.node_to_n_triples(triple.predicate(), &TripleSegment::Predicate) {
            Ok(str) => output_string.push_str(&str),
            Err(error) => return Err(error),
        }

        output_string.push_str(" ");

        // convert object
        match self.node_to_n_triples(triple.object(), &TripleSegment::Object) {
            Ok(str) => output_string.push_str(&str),
            Err(error) => return Err(error),
        }

        output_string.push_str(" .");

        Ok(output_string)
    }

    /// Converts a single node to its corresponding N-Triples representation.
    ///
    /// Checks if the node type is valid considering the triple segment.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::writer::n_triples_writer::NTriplesWriter;
    /// use rdf::writer::rdf_writer::RdfWriter;
    /// use rdf::node::Node;
    /// use rdf::triple::TripleSegment;
    ///
    /// let writer = NTriplesWriter::new();
    ///
    /// let node = Node::BlankNode { id: "blank".to_string() };
    ///
    /// assert_eq!(writer.node_to_n_triples(&node, &TripleSegment::Subject).unwrap(),
    ///            "_:blank".to_string());
    /// ```
    ///
    /// # Failures
    ///
    /// - Node type for triple segment does not conform with NTriples syntax standard.
    ///
    pub fn node_to_n_triples(&self, node: &Node, segment: &TripleSegment) -> Result<String> {
        match *node {
            Node::BlankNode { .. } =>
            // blank nodes are not allowed as predicates
            {
                if *segment == TripleSegment::Predicate {
                    return Err(Error::new(
                        ErrorType::InvalidWriterOutput,
                        "Blank nodes are not allowed as predicates.",
                    ));
                }
            }
            Node::LiteralNode {
                data_type: ref dt,
                language: ref lang,
                ..
            } => {
                // literal nodes are only allowed as objects
                if *segment != TripleSegment::Object {
                    return Err(Error::new(
                        ErrorType::InvalidWriterOutput,
                        "Literals are not allowed as subjects or predicates.",
                    ));
                }

                // either language or data type could be defined, but not both
                if *lang != None && *dt != None {
                    return Err(Error::new(
                        ErrorType::InvalidWriterOutput,
                        "Language and data type defined for a literal.",
                    ));
                }
            }
            _ => {}
        }

        // use the formatter to get the corresponding N-Triple syntax
        Ok(self.formatter.format_node(node))
    }
}
