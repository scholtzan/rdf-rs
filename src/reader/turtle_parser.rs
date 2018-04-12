use Result;
use reader::rdf_parser::RdfParser;
use graph::Graph;
use error::{Error, ErrorType};
use triple::Triple;
use reader::lexer::turtle_lexer::TurtleLexer;
use reader::lexer::rdf_lexer::RdfLexer;
use node::Node;
use reader::lexer::token::Token;
use std::io::Read;
use uri::Uri;
use std::io::Cursor;
use namespace::Namespace;
use specs::rdf_syntax_specs::RdfSyntaxDataTypes;

/// RDF parser to generate an RDF graph from Turtle syntax.
pub struct TurtleParser<R: Read> {
    lexer: TurtleLexer<R>,
}

impl<R: Read> RdfParser for TurtleParser<R> {
    /// Generates an RDF graph from a string containing Turtle syntax.
    ///
    /// Returns an error in case invalid Turtle syntax is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::reader::turtle_parser::TurtleParser;
    /// use rdf::reader::rdf_parser::RdfParser;
    ///
    /// let input = "<http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://xmlns.com/foaf/0.1/maker> _:art .
    ///              _:art <http://xmlns.com/foaf/0.1/name> \"Art Barstow\" .";
    ///
    /// let mut reader = TurtleParser::from_string(input.to_string());
    ///
    /// match reader.decode() {
    ///   Ok(graph) => assert_eq!(graph.count(), 2),
    ///   Err(_) => assert!(false)
    /// }
    /// ```
    ///
    /// # Failures
    ///
    /// - Invalid input that does not conform with NTriples standard.
    /// - Invalid node type for triple segment.
    ///
    fn decode(&mut self) -> Result<Graph> {
        let mut graph = Graph::new(None);

        loop {
            match self.lexer.peek_next_token() {
                Ok(Token::Comment(_)) => {
                    let _ = self.lexer.get_next_token();
                    continue;
                }
                Ok(Token::EndOfInput) => return Ok(graph),
                Ok(Token::BaseDirective(_)) => {
                    let base_uri = self.read_base_directive()?;
                    graph.set_base_uri(&base_uri);
                }
                Ok(Token::PrefixDirective(_, _)) => {
                    let namespace = self.read_prefix_directive()?;
                    graph.add_namespace(&namespace);
                }
                Ok(Token::Uri(_))
                | Ok(Token::BlankNode(_))
                | Ok(Token::QName(_, _))
                | Ok(Token::CollectionStart)
                | Ok(Token::UnlabeledBlankNodeStart) => {
                    let triples = self.read_triples(&mut graph)?;
                    graph.add_triples(&triples);
                }
                Err(err) => match *err.error_type() {
                    ErrorType::EndOfInput(_) => return Ok(graph),
                    _ => {
                        return Err(Error::new(
                            ErrorType::InvalidReaderInput,
                            "Error while parsing Turtle syntax.",
                        ))
                    }
                },
                Ok(_) => {
                    return Err(Error::new(
                        ErrorType::InvalidToken,
                        "Invalid token while parsing Turtle syntax.",
                    ))
                }
            }
        }
    }
}

impl TurtleParser<Cursor<Vec<u8>>> {
    /// Constructor of `TurtleParser` from input string.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::reader::turtle_parser::TurtleParser;
    /// use rdf::reader::rdf_parser::RdfParser;
    ///
    /// let input = "<http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://xmlns.com/foaf/0.1/maker> _:art .
    ///              _:art <http://xmlns.com/foaf/0.1/name> \"Art Barstow\" .";
    ///
    /// let reader = TurtleParser::from_string(input.to_string());
    /// ```
    pub fn from_string<S>(input: S) -> TurtleParser<Cursor<Vec<u8>>>
    where
        S: Into<String>,
    {
        TurtleParser::from_reader(Cursor::new(input.into().into_bytes()))
    }
}

impl<R: Read> TurtleParser<R> {
    /// Constructor of `TurtleParser` from input reader.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::reader::turtle_parser::TurtleParser;
    /// use rdf::reader::rdf_parser::RdfParser;
    ///
    /// let input = "<http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://xmlns.com/foaf/0.1/maker> _:art .
    ///              _:art <http://xmlns.com/foaf/0.1/name> \"Art Barstow\" .";
    ///
    /// let reader = TurtleParser::from_reader(input.as_bytes());
    /// ```
    pub fn from_reader(input: R) -> TurtleParser<R> {
        TurtleParser {
            lexer: TurtleLexer::new(input),
        }
    }

    /// Parses prefix directives and returns the created namespace.
    fn read_base_directive(&mut self) -> Result<Uri> {
        match self.lexer.get_next_token()? {
            Token::BaseDirective(uri) => match self.lexer.get_next_token()? {
                Token::TripleDelimiter => Ok(Uri::new(uri)),
                _ => Err(Error::new(
                    ErrorType::InvalidReaderInput,
                    "Turtle base directive does not end with '.'",
                )),
            },
            _ => Err(Error::new(
                ErrorType::InvalidReaderInput,
                "Invalid input for Turtle base directive.",
            )),
        }
    }

    /// Parses prefix directives and returns the created namespace.
    fn read_prefix_directive(&mut self) -> Result<Namespace> {
        match self.lexer.get_next_token()? {
            Token::PrefixDirective(prefix, uri) => match self.lexer.get_next_token()? {
                Token::TripleDelimiter => Ok(Namespace::new(prefix, Uri::new(uri))),
                _ => Err(Error::new(
                    ErrorType::InvalidReaderInput,
                    "Turtle prefix directive does not end with '.'",
                )),
            },
            _ => Err(Error::new(
                ErrorType::InvalidReaderInput,
                "Invalid input for Turtle prefix.",
            )),
        }
    }

    /// Creates a triple from the parsed tokens.
    fn read_triples(&mut self, graph: &mut Graph) -> Result<Vec<Triple>> {
        let subject = self.read_subject(graph)?;

        self.read_predicate_object_list(&subject, graph)
    }

    /// Get the next token and check if it is a valid subject and create a new subject node.
    fn read_subject(&mut self, graph: &mut Graph) -> Result<Node> {
        match self.lexer.get_next_token()? {
            Token::BlankNode(id) => Ok(Node::BlankNode { id }),
            Token::QName(prefix, path) => {
                let mut uri = graph.get_namespace_uri_by_prefix(prefix)?.to_owned();
                uri.append_resource_path(&path.replace(":", "/")); // adjust the QName path to URI path
                Ok(Node::UriNode { uri })
            }
            Token::Uri(uri) => Ok(Node::UriNode { uri: Uri::new(uri) }),
            Token::CollectionStart => self.read_collection(graph),
            Token::UnlabeledBlankNodeStart => self.read_unlabeled_blank_node(graph),
            _ => Err(Error::new(
                ErrorType::InvalidToken,
                "Invalid token for Turtle subject.",
            )),
        }
    }

    /// Reads a list or a single pair of predicate and object nodes.
    fn read_predicate_object_list(
        &mut self,
        subject: &Node,
        graph: &mut Graph,
    ) -> Result<Vec<Triple>> {
        let mut triples: Vec<Triple> = Vec::new();

        let (predicate, object) = self.read_predicate_with_object(graph)?;
        triples.push(Triple::new(subject, &predicate, &object));

        loop {
            match self.lexer.get_next_token()? {
                Token::TripleDelimiter => break,
                Token::UnlabeledBlankNodeEnd => break,
                Token::PredicateListDelimiter => {
                    let (predicate, object) = self.read_predicate_with_object(graph)?;
                    triples.push(Triple::new(subject, &predicate, &object));
                }
                Token::ObjectListDelimiter => {
                    let object = self.read_object(graph)?;
                    triples.push(Triple::new(subject, &predicate, &object));
                }
                _ => {
                    return Err(Error::new(
                        ErrorType::InvalidToken,
                        "Invalid token while reading Turtle triples.",
                    ))
                }
            }
        }

        Ok(triples)
    }

    /// Get the next token and check if it is a valid predicate and create a new predicate node.
    fn read_predicate_with_object(&mut self, graph: &mut Graph) -> Result<(Node, Node)> {
        // read the predicate
        let predicate = match self.lexer.get_next_token()? {
            Token::Uri(uri) => Node::UriNode { uri: Uri::new(uri) },
            Token::KeywordA => Node::UriNode {
                uri: RdfSyntaxDataTypes::A.to_uri(),
            },
            Token::QName(prefix, path) => {
                let mut uri = graph.get_namespace_uri_by_prefix(prefix)?.to_owned();
                uri.append_resource_path(&path.replace(":", "/")); // adjust the QName path to URI path
                Node::UriNode { uri }
            }
            Token::BlankNode(id) => Node::BlankNode { id },
            _ => {
                return Err(Error::new(
                    ErrorType::InvalidToken,
                    "Invalid token for Turtle predicate.",
                ))
            }
        };

        // read the object
        let object = self.read_object(graph)?;

        Ok((predicate, object))
    }

    /// Get the next token and check if it is a valid object and create a new object node.
    fn read_object(&mut self, graph: &mut Graph) -> Result<Node> {
        match self.lexer.get_next_token()? {
            Token::BlankNode(id) => Ok(Node::BlankNode { id }),
            Token::Uri(uri) => Ok(Node::UriNode { uri: Uri::new(uri) }),
            Token::QName(prefix, path) => {
                let mut uri = graph.get_namespace_uri_by_prefix(prefix)?.to_owned();
                uri.append_resource_path(&path.replace(":", "/")); // adjust the QName path to URI path
                Ok(Node::UriNode { uri })
            }
            Token::LiteralWithLanguageSpecification(literal, lang) => Ok(Node::LiteralNode {
                literal,
                data_type: None,
                language: Some(lang),
            }),
            Token::LiteralWithUrlDatatype(literal, datatype) => Ok(Node::LiteralNode {
                literal,
                data_type: Some(Uri::new(datatype)),
                language: None,
            }),
            Token::Literal(literal) => Ok(Node::LiteralNode {
                literal,
                data_type: None,
                language: None,
            }),
            Token::CollectionStart => self.read_collection(graph),
            Token::UnlabeledBlankNodeStart => self.read_unlabeled_blank_node(graph),
            _ => Err(Error::new(
                ErrorType::InvalidToken,
                "Invalid token for Turtle object.",
            )),
        }
    }

    /// Reads a unlabeled blank node.
    ///
    /// Returns the subject node and add all other nested nodes to the graph.
    fn read_unlabeled_blank_node(&mut self, graph: &mut Graph) -> Result<Node> {
        let subject = graph.create_blank_node();

        if self.lexer.peek_next_token()? == Token::UnlabeledBlankNodeEnd {
            let _ = self.lexer.get_next_token()?; // consume the token indicating the node end ']'
        } else {
            let triples = self.read_predicate_object_list(&subject, graph)?;
            graph.add_triples(&triples);
        }

        Ok(subject)
    }

    /// Reads a collection and returns the collection start as node.
    ///
    /// The remaining elements are implicitly added to the graph.
    fn read_collection(&mut self, graph: &mut Graph) -> Result<Node> {
        // check if the list is empty and return list:nil
        if self.lexer.peek_next_token()? == Token::CollectionEnd {
            let _ = self.lexer.get_next_token()?; // consume the token indicating the collection end ')'

            return Ok(Node::UriNode {
                uri: RdfSyntaxDataTypes::ListNil.to_uri(),
            });
        }

        // for non-empty list generate blank node
        let subject = graph.create_blank_node();

        let mut next_subject = subject.to_owned();

        loop {
            let rest = graph.create_blank_node();
            let object = self.read_object(graph)?;

            graph.add_triple(&Triple::new(
                &next_subject,
                &Node::UriNode {
                    uri: RdfSyntaxDataTypes::ListFirst.to_uri(),
                },
                &object,
            ));

            // check if the rest of the list is nil
            if self.lexer.peek_next_token()? == Token::CollectionEnd {
                let _ = self.lexer.get_next_token()?; // consume the token indicating the collection end ')'

                // create list:nil node
                graph.add_triple(&Triple::new(
                    &next_subject,
                    &Node::UriNode {
                        uri: RdfSyntaxDataTypes::ListRest.to_uri(),
                    },
                    &Node::UriNode {
                        uri: RdfSyntaxDataTypes::ListNil.to_uri(),
                    },
                ));
                break; // stop further list evaluation
            } else {
                // create node referring to the non-empty rest of the list
                graph.add_triple(&Triple::new(
                    &next_subject,
                    &Node::UriNode {
                        uri: RdfSyntaxDataTypes::ListRest.to_uri(),
                    },
                    &rest,
                ));
            }

            next_subject = rest;
        }

        Ok(subject)
    }
}

#[cfg(test)]
mod tests {
    use reader::turtle_parser::TurtleParser;
    use reader::rdf_parser::RdfParser;
    use uri::Uri;

    #[test]
    fn test_read_n_triples_as_turtle_from_string() {
        let input = "<http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://xmlns.com/foaf/0.1/Document> .
                 <http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://purl.org/dc/terms/title> \"N-Triples\"@en-US .
                 <http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://xmlns.com/foaf/0.1/maker> _:art .
                 _:art <http://xmlns.com/foaf/0.1/name> \"Art Barstow\" .";

        let mut reader = TurtleParser::from_string(input.to_string());

        match reader.decode() {
            Ok(graph) => assert_eq!(graph.count(), 4),
            Err(e) => {
                println!("Err {}", e.to_string());
                assert!(false)
            }
        }
    }

    #[test]
    fn test_read_uncompressed_turtle_from_string() {
        let input = "@base <http://example.org/> .
                 @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
                 @prefix foaf: <http://xmlns.com/foaf/0.1/> .

                 <http://www.w3.org/2001/sw/RDFCore/ntriples/> rdf:type foaf:Document .
                 <http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://purl.org/dc/terms/title> \"N-Triples\"@en-US .
                 <http://www.w3.org/2001/sw/RDFCore/ntriples/> foaf:maker _:art .
                 _:art foaf:name \"Art Barstow\" .";

        let mut reader = TurtleParser::from_string(input.to_string());

        match reader.decode() {
            Ok(graph) => {
                assert_eq!(graph.count(), 4);
                assert_eq!(graph.namespaces().len(), 2);
                assert_eq!(
                    graph.base_uri(),
                    &Some(Uri::new("http://example.org/".to_string()))
                )
            }
            Err(e) => {
                println!("Err {}", e.to_string());
                assert!(false)
            }
        }
    }

    #[test]
    fn test_read_compressed_turtle_from_string() {
        let input = "@base <http://example.org/> .
                 @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
                 @prefix foaf: <http://xmlns.com/foaf/0.1/> .

                 <http://www.w3.org/2001/sw/RDFCore/ntriples/> rdf:type foaf:Document ;
                                                               <http://purl.org/dc/terms/title> \"N-Triples\"@en-US ;
                                                               foaf:maker _:art .

                 _:art foaf:name \"Art Barstow\" ,
                                 \"Art Барстоу\" ,
                                 \"아트 바스트\" .";

        let mut reader = TurtleParser::from_string(input.to_string());

        match reader.decode() {
            Ok(graph) => {
                assert_eq!(graph.count(), 6);
                assert_eq!(graph.namespaces().len(), 2);
                assert_eq!(
                    graph.base_uri(),
                    &Some(Uri::new("http://example.org/".to_string()))
                )
            }
            Err(e) => {
                println!("Err {}", e.to_string());
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parsing_turtle_base_uri() {
        let input = "@base <http://example/> .";
        let mut reader = TurtleParser::from_string(input.to_string());

        match reader.decode() {
            Ok(graph) => assert_eq!(
                graph.base_uri(),
                &Some(Uri::new("http://example/".to_string()))
            ),
            Err(e) => {
                println!("Err {}", e.to_string());
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parsing_turtle_sparql_base_uri() {
        let input = "BASE <http://example/> .";
        let mut reader = TurtleParser::from_string(input.to_string());

        match reader.decode() {
            Ok(graph) => assert_eq!(
                graph.base_uri(),
                &Some(Uri::new("http://example/".to_string()))
            ),
            Err(e) => {
                println!("Err {}", e.to_string());
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parsing_turtle_prefix() {
        let input = "@prefix p: <http://p.example/> .";
        let mut reader = TurtleParser::from_string(input.to_string());

        match reader.decode() {
            Ok(graph) => assert_eq!(graph.namespaces().len(), 1),
            Err(e) => {
                println!("Err {}", e.to_string());
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parsing_turtle_sparql_prefix() {
        let input = "PREFIX p: <http://p.example/> .";
        let mut reader = TurtleParser::from_string(input.to_string());

        match reader.decode() {
            Ok(graph) => assert_eq!(graph.namespaces().len(), 1),
            Err(e) => {
                println!("Err {}", e.to_string());
                assert!(false)
            }
        }
    }

    #[test]
    fn test_read_turtle_with_empty_prefix_from_string() {
        let input = "@prefix : <http://example/> .
                 :subject :predicate :object .";

        let mut reader = TurtleParser::from_string(input.to_string());

        match reader.decode() {
            Ok(graph) => assert_eq!(graph.count(), 1),
            Err(e) => {
                println!("Err {}", e.to_string());
                assert!(false)
            }
        }
    }

    #[test]
    fn read_collection_from_string() {
        let input = "_:a _:b ( _:c _:g ) .";

        let mut reader = TurtleParser::from_string(input.to_string());

        match reader.decode() {
            Ok(graph) => assert_eq!(graph.count(), 5),
            Err(e) => {
                println!("Err {}", e.to_string());
                assert!(false)
            }
        }
    }

    #[test]
    fn read_empty_collection_from_string() {
        let input = "() _:b (  ) .";

        let mut reader = TurtleParser::from_string(input.to_string());

        match reader.decode() {
            Ok(graph) => assert_eq!(graph.count(), 1),
            Err(e) => {
                println!("Err {}", e.to_string());
                assert!(false)
            }
        }
    }

    #[test]
    fn read_nested_collections_from_string() {
        let input = "( _:a (_:b _:c ) ) _:b ( _:b ( ( ( ) ) ) ) .";

        let mut reader = TurtleParser::from_string(input.to_string());

        match reader.decode() {
            Ok(graph) => assert_eq!(graph.count(), 17),
            Err(e) => {
                println!("Err {}", e.to_string());
                assert!(false)
            }
        }
    }

    #[test]
    fn read_empty_unlabeled_node_from_string() {
        let input = "[ ] _:b [ ] .";

        let mut reader = TurtleParser::from_string(input.to_string());

        match reader.decode() {
            Ok(graph) => assert_eq!(graph.count(), 1),
            Err(e) => {
                println!("Err {}", e.to_string());
                assert!(false)
            }
        }
    }

    #[test]
    fn read_unlabeled_nodes_from_string() {
        let input = "[ _:a _:g ] _:b [ _:c [
      _:s _:d ,
          [ _:asd _:asdf ] ;
      _:g _:h
    ] ] .";

        let mut reader = TurtleParser::from_string(input.to_string());

        match reader.decode() {
            Ok(graph) => assert_eq!(graph.count(), 7),
            Err(e) => {
                println!("Err {}", e.to_string());
                assert!(false)
            }
        }
    }
}
