use writer::formatter::turtle_formatter::TurtleFormatter;
use writer::formatter::rdf_formatter::*;
use writer::rdf_writer::RdfWriter;
use graph::Graph;
use node::Node;
use triple::Triple;
use triple::TripleSegment;
use Result;
use std::iter::repeat;
use error::{Error, ErrorType};
use std::collections::HashMap;
use uri::Uri;

/// RDF writer to generate Turtle syntax.
pub struct TurtleWriter<'a> {
    formatter: TurtleFormatter<'a>,
}

// todo: decide if grouping should be done or ignored based on number of distinct subjects

impl<'a> RdfWriter for TurtleWriter<'a> {
    /// Generates the Turtle syntax for each triple stored in the provided graph.
    ///
    /// Returns an error if invalid Turtle syntax would be generated.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::writer::turtle_writer::TurtleWriter;
    /// use rdf::writer::rdf_writer::RdfWriter;
    /// use rdf::graph::Graph;
    /// use rdf::node::Node;
    /// use rdf::uri::Uri;
    /// use rdf::triple::Triple;
    ///
    /// let mut graph = Graph::new(None);
    ///
    /// let subject = graph.create_blank_node();
    /// let object = graph.create_blank_node();
    /// let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
    ///
    /// let trip = Triple::new(&subject, &predicate, &object);
    /// graph.add_triple(&trip);
    ///
    ///
    /// let writer = TurtleWriter::new(graph.namespaces());
    /// writer.write_to_string(&graph);
    /// ```
    ///
    /// # Failures
    ///
    /// - The node type is invalid for the triple segment.
    ///
    fn write_to_string(&self, graph: &Graph) -> Result<String> {
        let mut output_string = "".to_string();

        output_string.push_str(&self.write_base_uri(graph));
        output_string.push_str(&self.write_prefixes(graph));

        let mut triples_vec: Vec<Triple> = graph.triples_iter().cloned().collect();
        triples_vec.sort();

        // store subjects and predicates for grouping
        let mut previous_subject: Option<&Node> = None;
        let mut previous_predicate: Option<&Node> = None;

        // number of spaces required to indent the predicate and object
        let mut predicate_indentation = 0;
        let mut object_indentation = 0;

        for triple in &triples_vec {
            if previous_subject == Some(triple.subject()) {
                // continue group
                if previous_predicate == Some(triple.predicate()) {
                    // indent object
                    output_string.push_str(" ,\n");
                    output_string
                        .push_str(&repeat(" ").take(object_indentation).collect::<String>());
                } else {
                    output_string.push_str(" ;\n");

                    // write predicate
                    let turtle_predicate =
                        self.node_to_turtle(triple.predicate(), &TripleSegment::Predicate)?;
                    // indent predicate
                    output_string
                        .push_str(&repeat(" ").take(predicate_indentation).collect::<String>());
                    output_string.push_str(&turtle_predicate);

                    previous_predicate = Some(triple.predicate());

                    output_string.push_str(" ");

                    // recalculate object indentation
                    object_indentation = predicate_indentation + turtle_predicate.len() + 1;
                }
            } else {
                if previous_subject != None {
                    output_string.push_str(" .\n");
                }

                // start new group
                let turtle_subject =
                    self.node_to_turtle(triple.subject(), &TripleSegment::Subject)?;
                output_string.push_str(&turtle_subject);
                previous_subject = Some(triple.subject());

                output_string.push_str(" ");
                let turtle_predicate =
                    self.node_to_turtle(triple.predicate(), &TripleSegment::Predicate)?;
                output_string.push_str(&turtle_predicate);
                previous_predicate = Some(triple.predicate());
                output_string.push_str(" ");

                predicate_indentation = turtle_subject.len() + 1;
                object_indentation = predicate_indentation + turtle_predicate.len() + 1;
            }

            // write object
            let turtle_object = self.node_to_turtle(triple.object(), &TripleSegment::Object)?;
            output_string.push_str(&turtle_object);
        }

        if !graph.is_empty() {
            output_string.push_str(" .");
        }

        Ok(output_string)
    }
}

impl<'a> TurtleWriter<'a> {
    /// Constructor of `TurtleWriter`.
    pub fn new(namespaces: &'a HashMap<String, Uri>) -> TurtleWriter<'a> {
        TurtleWriter {
            formatter: TurtleFormatter::new(namespaces),
        }
    }

    /// Returns the formatted base URI as string.
    fn write_base_uri(&self, graph: &Graph) -> String {
        let mut output_string = "".to_string();

        if let Some(ref base) = *graph.base_uri() {
            output_string.push_str("@base ");
            output_string.push_str(&self.formatter.format_uri(base));
            output_string.push_str(" .\n");
        }

        output_string
    }

    /// Returns all prefixes as formatted string.
    fn write_prefixes(&self, graph: &Graph) -> String {
        let mut output_string = "".to_string();

        // write prefixes
        for (prefix, namespace_uri) in graph.namespaces() {
            output_string.push_str("@prefix ");
            output_string.push_str(prefix);
            output_string.push_str(": <");
            output_string.push_str(namespace_uri.to_string());
            output_string.push_str("> .\n");
        }

        output_string
    }

    /// Converts a single node to its corresponding Turtle representation.
    ///
    /// Checks if the node type is valid considering the triple segment.
    ///
    /// # Failures
    ///
    /// - The node type is invalid for the triple segment.
    ///
    fn node_to_turtle(&self, node: &Node, segment: &TripleSegment) -> Result<String> {
        match *node {
      Node::BlankNode { .. } =>
      // blank nodes are not allowed as predicates
        if *segment == TripleSegment::Predicate {
          return Err(Error::new(ErrorType::InvalidWriterOutput,
                                "Blank nodes are not allowed as predicates in Turtle."))
        },
      Node::LiteralNode { data_type: ref dt, language: ref lang, .. } => {
        // literal nodes are only allowed as objects
        if *segment != TripleSegment::Object {
          return Err(Error::new(ErrorType::InvalidWriterOutput,
                                "Literals are not allowed as subjects or predicates in Turtle."))
        }

        // either language or data type could be defined, but not both
        if *lang != None && *dt != None {
          return Err(Error::new(ErrorType::InvalidWriterOutput,
                                "Literal has data type and language."))
        }
      },
      _ => {},
    }

        // use the formatter to get the corresponding N-Triple syntax
        Ok(self.formatter.format_node(node))
    }
}

#[cfg(test)]
mod tests {
    use triple::*;
    use uri::Uri;
    use graph::Graph;
    use writer::rdf_writer::RdfWriter;
    use writer::turtle_writer::TurtleWriter;
    use namespace::Namespace;

    #[test]
    fn test_turtle_writer() {
        let mut graph = Graph::new(None);

        let subject = graph.create_blank_node();
        let object = graph.create_blank_node();
        let predicate =
            graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));

        let trip = Triple::new(&subject, &predicate, &object);
        graph.add_triple(&trip);

        let result = "_:auto0 <http://example.org/show/localName> _:auto1 .".to_string();

        let writer = TurtleWriter::new(graph.namespaces());
        match writer.write_to_string(&graph) {
            Ok(str) => assert_eq!(result, str),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_turtle_writer_predicate_grouping() {
        let mut graph = Graph::new(None);

        let subject1 = graph.create_blank_node();
        let object1 = graph.create_blank_node();
        let predicate1 =
            graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));

        let subject2 = graph.create_blank_node();
        let object2 = graph.create_blank_node();
        let predicate2 = graph.create_uri_node(&Uri::new("http://example.org/test".to_string()));

        graph.add_triple(&Triple::new(&subject1, &predicate1, &object1));
        graph.add_triple(&Triple::new(&subject2, &predicate1, &object1));
        graph.add_triple(&Triple::new(&subject1, &predicate2, &object2));
        graph.add_triple(&Triple::new(&subject2, &predicate2, &object2));

        let result = "_:auto0 <http://example.org/show/localName> _:auto1 ;
        <http://example.org/test> _:auto3 .
_:auto2 <http://example.org/show/localName> _:auto1 ;
        <http://example.org/test> _:auto3 ."
            .to_string();

        let writer = TurtleWriter::new(graph.namespaces());
        match writer.write_to_string(&graph) {
            Ok(str) => assert_eq!(result, str),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_turtle_writer_object_grouping() {
        let mut graph = Graph::new(None);

        let subject1 = graph.create_blank_node();
        let object1 = graph.create_blank_node();
        let predicate1 =
            graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));

        let subject2 = graph.create_blank_node();
        let object2 = graph.create_blank_node();

        graph.add_triple(&Triple::new(&subject2, &predicate1, &object2));
        graph.add_triple(&Triple::new(&subject1, &predicate1, &object1));
        graph.add_triple(&Triple::new(&subject1, &predicate1, &object2));
        graph.add_triple(&Triple::new(&subject2, &predicate1, &object1));

        let result = "_:auto0 <http://example.org/show/localName> _:auto1 ,
                                            _:auto3 .
_:auto2 <http://example.org/show/localName> _:auto1 ,
                                            _:auto3 ."
            .to_string();

        let writer = TurtleWriter::new(graph.namespaces());
        match writer.write_to_string(&graph) {
            Ok(str) => assert_eq!(result, str),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_turtle_writer_base_uri() {
        let graph = Graph::new(Some(&Uri::new("http://example.org/".to_string())));

        let result = "@base <http://example.org/> .\n".to_string();

        let writer = TurtleWriter::new(graph.namespaces());
        match writer.write_to_string(&graph) {
            Ok(str) => assert_eq!(result, str),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_turtle_writer_prefixes() {
        let mut graph = Graph::new(None);

        graph.add_namespace(&Namespace::new(
            "example".to_string(),
            Uri::new("http://example.org/".to_string()),
        ));

        let result = "@prefix example: <http://example.org/> .\n".to_string();

        let writer = TurtleWriter::new(graph.namespaces());
        match writer.write_to_string(&graph) {
            Ok(str) => assert_eq!(result, str),
            Err(_) => assert!(false),
        }
    }
}
