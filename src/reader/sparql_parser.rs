use error::{Error, ErrorType};
use node::Node;
use reader::lexer::rdf_lexer::RdfLexer;
use reader::lexer::sparql_lexer::SparqlLexer;
use reader::lexer::token::Token;
use sparql::pattern::{GroupPattern, NodePattern, Pattern, TriplePattern};
use sparql::query::{SparqlQuery, SparqlQueryType};
use specs::rdf_syntax_specs::RdfSyntaxDataTypes;
use specs::sparql_specs::SparqlKeyword;
use std::io::Cursor;
use std::io::Read;
use uri::Uri;
use Result;

/// SPARQL parser to generate a `SparqlQuery` from SPARQL syntax.
pub struct SparqlParser<R: Read> {
    lexer: SparqlLexer<R>,
}

impl SparqlParser<Cursor<Vec<u8>>> {
    /// Constructor of `SparqlParser` from input string.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf_rs::reader::sparql_parser::SparqlParser;
    ///
    /// let input = "SELECT ?test";
    ///
    /// let reader = SparqlParser::from_string(input.to_string());
    /// ```
    pub fn from_string<S>(input: S) -> SparqlParser<Cursor<Vec<u8>>>
    where
        S: Into<String>,
    {
        SparqlParser::from_reader(Cursor::new(input.into().into_bytes()))
    }
}

impl<R: Read> SparqlParser<R> {
    /// Constructor of `SparqlParser` from input reader.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf_rs::reader::sparql_parser::SparqlParser;
    ///
    /// let input = "SELECT ?test";
    ///
    /// let reader = SparqlParser::from_reader(input.as_bytes());
    /// ```
    pub fn from_reader(input: R) -> SparqlParser<R> {
        SparqlParser {
            lexer: SparqlLexer::new(input),
        }
    }

    /// Transforms a SPARQL query string into a `SparqlQuery` object.
    ///
    /// Returns an error if invalid SPARQL is provided.
    ///
    /// # Examples
    ///
    /// todo
    ///
    /// # Failures
    ///
    /// - Invalid input that does not conform with SPARQL standard.
    ///
    pub fn decode(&mut self) -> Result<SparqlQuery> {
        loop {
            match self.lexer.peek_next_token()? {
                Token::Comment(_) => {
                    let _ = self.lexer.get_next_token();
                    continue;
                }
                Token::Select => {
                    let _ = self.lexer.get_next_token();
                    return self.read_select_query();
                }
                _ => {
                    return Err(Error::new(
                        ErrorType::InvalidToken,
                        "Invalid token while parsing SPARQL syntax.",
                    ))
                }
            }
        }

        Err(Error::new(
            ErrorType::InvalidToken,
            "Unexpected end while parsing SPARQL syntax.",
        ))
    }

    /// Parses SELECT queries.
    ///
    /// # Failures
    ///
    /// - SELECT query does not conform to SPARQL standard.
    ///
    pub fn read_select_query(&mut self) -> Result<SparqlQuery> {
        let mut query_type = SparqlQueryType::Select;
        let mut variables: Vec<String> = Vec::new();

        // check if REDUCED or DISTINCT
        match self.lexer.peek_next_token()? {
            Token::Reduced => {
                let _ = self.lexer.get_next_token();
                query_type = SparqlQueryType::SelectReduced;
            }
            Token::Distinct => {
                let _ = self.lexer.get_next_token();
                query_type = SparqlQueryType::SelectDistinct;
            }
            _ => {}
        }

        // check if * or specific variables should be selected
        match self.lexer.get_next_token()? {
            Token::Asterisk => match query_type.clone() {
                SparqlQueryType::SelectDistinct => query_type = SparqlQueryType::SelectAllDistinct,
                SparqlQueryType::SelectReduced => query_type = SparqlQueryType::SelectAllReduced,
                _ => query_type = SparqlQueryType::SelectAll,
            },
            Token::SparqlVariable(var_name) => {
                // parse variables identifiers
                variables.push(var_name);

                loop {
                    match self.lexer.peek_next_token() {
                        Ok(Token::SparqlVariable(name)) => {
                            variables.push(name);
                            let _ = self.lexer.get_next_token();
                        }
                        Err(err) => match err.error_type() {
                            &ErrorType::EndOfInput(_) => break,
                            _ => {
                                return Err(Error::new(
                                    ErrorType::InvalidReaderInput,
                                    "Error while parsing SPARQL variables.",
                                ))
                            }
                        },
                        _ => break,
                    }
                }
            }
            _ => {
                return Err(Error::new(
                    ErrorType::InvalidToken,
                    "Unexpected end while parsing SPARQL SELECT syntax.",
                ))
            }
        }

        // instantiate the query
        let mut query = SparqlQuery::new(query_type);

        // parse WHERE clause
        match self.lexer.peek_next_token()? {
            Token::Where => {
                // WHERE keyword is optional but always followed by a group
                let _ = self.lexer.get_next_token();
            }
            _ => {}
        }

        // parse group
        match self.lexer.get_next_token()? {
            Token::GroupStart => {
                let group_pattern = self.parse_group(&mut query)?;
                query.add_pattern(Box::new(group_pattern));
            }
            _ => {
                return Err(Error::new(
                    ErrorType::InvalidToken,
                    "Unexpected token while parsing WHERE group",
                ))
            }
        }

        query.add_variables(variables);

        Ok(query)
    }

    /// Parse and return the detected patterns.
    fn parse_group(&mut self, query: &mut SparqlQuery) -> Result<GroupPattern> {
        let mut group_pattern = GroupPattern::new();

        loop {
            // try parse triple
            match self.lexer.peek_next_token()? {
                Token::SparqlVariable(_)
                | Token::BlankNode(_)
                | Token::QName(_, _)
                | Token::Uri(_) => {
                    let patterns = self.read_triples_pattern(query)?;

                    for pattern in patterns {
                        group_pattern.add_pattern(Box::new(pattern));
                    }
                }
                Token::Optional => {
                    let _ = self.lexer.get_next_token(); // consume OPTIONAL
                    let _ = self.lexer.get_next_token(); // after OPTIONAL always follows the start of a new group
                    let mut optional_group = self.parse_group(query)?;
                    optional_group.set_is_optional();
                    group_pattern.add_pattern(Box::new(optional_group));
                }
                Token::GroupStart => {
                    let _ = self.lexer.get_next_token(); // consume '{'
                    let nested_group = self.parse_group(query)?;
                    group_pattern.add_pattern(Box::new(nested_group));
                }
                Token::Filter => {} // todo
                Token::GroupEnd => {
                    let _ = self.lexer.get_next_token(); // consume "."
                    break; // stop looking for next element within loop
                }
                _ => {} // todo: UNION
            }
        }

        Ok(group_pattern)
    }

    /// Creates a triple pattern from the parsed tokens.
    fn read_triples_pattern(&mut self, query: &mut SparqlQuery) -> Result<Vec<TriplePattern>> {
        let subject = self.read_subject_pattern(query)?;

        self.read_predicate_object_list_pattern(&subject, query)
    }

    /// Get the next token and check if it is a valid subject pattern.
    fn read_subject_pattern(&mut self, query: &mut SparqlQuery) -> Result<NodePattern> {
        match self.lexer.get_next_token()? {
            Token::BlankNode(id) => Ok(NodePattern::FixedNode(Node::BlankNode { id: id })),
            Token::QName(prefix, path) => {
                let mut uri = query.get_namespace_uri_by_prefix(prefix)?.to_owned();
                uri.append_resource_path(&path.replace(":", "/")); // adjust the QName path to URI path
                Ok(NodePattern::FixedNode(Node::UriNode { uri: uri }))
            }
            Token::Uri(uri) => Ok(NodePattern::FixedNode(Node::UriNode { uri: Uri::new(uri) })),
            Token::SparqlVariable(variable_name) => Ok(NodePattern::VariableNode(variable_name)),
            _ => Err(Error::new(
                ErrorType::InvalidToken,
                "Invalid token for SPARQL subject pattern.",
            )),
        }
    }

    /// Reads a list or a single pair of predicate and object patterns.
    fn read_predicate_object_list_pattern(
        &mut self,
        subject: &NodePattern,
        query: &mut SparqlQuery,
    ) -> Result<Vec<TriplePattern>> {
        let mut triples: Vec<TriplePattern> = Vec::new();

        let (predicate, object) = self.read_predicate_with_object_pattern(query)?;
        triples.push(TriplePattern::new(subject, &predicate, &object));

        loop {
            match self.lexer.peek_next_token()? {
                Token::TripleDelimiter => {
                    let _ = self.lexer.get_next_token();
                    break;
                }
                Token::GroupEnd => break,
                Token::PredicateListDelimiter => {
                    let _ = self.lexer.get_next_token();
                    let (predicate, object) = self.read_predicate_with_object_pattern(query)?;
                    triples.push(TriplePattern::new(subject, &predicate, &object));
                }
                Token::ObjectListDelimiter => {
                    let _ = self.lexer.get_next_token();
                    let object = self.read_object_pattern(query)?;
                    triples.push(TriplePattern::new(subject, &predicate, &object));
                }
                _ => {
                    return Err(Error::new(
                        ErrorType::InvalidToken,
                        "Invalid token while reading SPARQL triples patterns",
                    ))
                }
            }
        }

        Ok(triples)
    }

    /// Get the next token and check if it is a valid predicate and create a new predicate node patterns.
    fn read_predicate_with_object_pattern(
        &mut self,
        query: &mut SparqlQuery,
    ) -> Result<(NodePattern, NodePattern)> {
        // read the predicate
        let predicate = match self.lexer.get_next_token()? {
            Token::Uri(uri) => NodePattern::FixedNode(Node::UriNode { uri: Uri::new(uri) }),
            Token::KeywordA => NodePattern::FixedNode(Node::UriNode {
                uri: RdfSyntaxDataTypes::A.to_uri(),
            }),
            Token::QName(prefix, path) => {
                let mut uri = query.get_namespace_uri_by_prefix(prefix)?.to_owned();
                uri.append_resource_path(&path.replace(":", "/")); // adjust the QName path to URI path
                NodePattern::FixedNode(Node::UriNode { uri: uri })
            }
            Token::BlankNode(id) => NodePattern::FixedNode(Node::BlankNode { id: id }),
            Token::SparqlVariable(variable_name) => NodePattern::VariableNode(variable_name),
            _ => {
                return Err(Error::new(
                    ErrorType::InvalidToken,
                    "Invalid token for SPARQL triple pattern predicate.",
                ))
            }
        };

        // read the object
        let object = self.read_object_pattern(query)?;

        Ok((predicate, object))
    }

    /// Get the next token and check if it is a valid object and create a new object node pattern.
    fn read_object_pattern(&mut self, query: &mut SparqlQuery) -> Result<NodePattern> {
        match self.lexer.get_next_token()? {
            Token::BlankNode(id) => Ok(NodePattern::FixedNode(Node::BlankNode { id: id })),
            Token::Uri(uri) => Ok(NodePattern::FixedNode(Node::UriNode { uri: Uri::new(uri) })),
            Token::QName(prefix, path) => {
                let mut uri = query.get_namespace_uri_by_prefix(prefix)?.to_owned();
                uri.append_resource_path(&path.replace(":", "/")); // adjust the QName path to URI path
                Ok(NodePattern::FixedNode(Node::UriNode { uri: uri }))
            }
            Token::SparqlVariable(variable_name) => Ok(NodePattern::VariableNode(variable_name)),
            Token::LiteralWithLanguageSpecification(literal, lang) => {
                Ok(NodePattern::FixedNode(Node::LiteralNode {
                    literal: literal,
                    data_type: None,
                    language: Some(lang),
                }))
            }
            Token::LiteralWithUrlDatatype(literal, datatype) => {
                Ok(NodePattern::FixedNode(Node::LiteralNode {
                    literal: literal,
                    data_type: Some(Uri::new(datatype)),
                    language: None,
                }))
            }
            Token::Literal(literal) => Ok(NodePattern::FixedNode(Node::LiteralNode {
                literal: literal,
                data_type: None,
                language: None,
            })),
            _ => Err(Error::new(
                ErrorType::InvalidToken,
                "Invalid token for SPARQL object pattern.",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use reader::sparql_parser::SparqlParser;
    use sparql::query::*;
    use uri::Uri;

    #[test]
    fn sparql_query_type_from_string() {
        let input = "SELECT ?a ?b ?c WHERE { ?v ?p 123 }";
        let mut reader = SparqlParser::from_string(input.to_string());

        match reader.decode() {
            Ok(sparql_query) => match sparql_query.get_query_type() {
                &SparqlQueryType::Select => assert!(true),
                _ => assert!(false),
            },
            Err(e) => {
                println!("Err {}", e.to_string());
                assert!(false)
            }
        }
    }

    #[test]
    fn sparql_variables_from_string() {
        let input = "SELECT ?a ?b ?c WHERE { ?v ?p 123 }";
        let mut reader = SparqlParser::from_string(input.to_string());

        match reader.decode() {
            Ok(sparql_query) => {
                let query_variables = sparql_query.get_query_variables();

                assert_eq!(query_variables[0], "a".to_string());
                assert_eq!(query_variables[1], "b".to_string());
                assert_eq!(query_variables[2], "c".to_string());

                // todo
                //        let expected_triple
            }
            Err(e) => {
                println!("Err {}", e.to_string());
                assert!(false)
            }
        }
    }

    // todo: tests
}
