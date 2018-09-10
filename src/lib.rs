//! # rdf-rs
//!
//! A crate for the Resource Description Framework (RDF) and SPARQL.
//!
//! ---
//!
//! This crate enables to parse certain RDF formats to an RDF graph, as well as writing
//! an RDF graph as RDF syntax.
//!
//! Currently supported RDF formats are:
//!
//! * Turtle
//! * N-Triples
//!
//!
//! ## Usage
//!
//! Each module contains a comprehensive documentation with usage examples.
//! In the following, only the basic functionality is introduced.
//!
//! RDF triples can be stored and represented in a graph.
//!
//! ```
//! use rdf::graph::Graph;
//! use rdf::uri::Uri;
//! use rdf::triple::Triple;
//!
//! let mut graph = Graph::new(None);
//!
//! let subject = graph.create_blank_node();
//! let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
//! let object = graph.create_blank_node();
//! let triple = Triple::new(&subject, &predicate, &object);
//!
//! graph.add_triple(&triple);
//! ```
//!
//! RDF graphs can be serialized to a supported format.
//!
//! ```
//! use rdf::writer::n_triples_writer::NTriplesWriter;
//! use rdf::writer::rdf_writer::RdfWriter;
//! use rdf::graph::Graph;
//! use rdf::uri::Uri;
//! use rdf::triple::Triple;
//!
//! let writer = NTriplesWriter::new();
//!
//! let mut graph = Graph::new(None);
//! let subject = graph.create_blank_node();
//! let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
//! let object = graph.create_blank_node();
//! let triple = Triple::new(&subject, &predicate, &object);
//!
//! graph.add_triple(&triple);
//!
//! assert_eq!(writer.write_to_string(&graph).unwrap(),
//!            "_:auto0 <http://example.org/show/localName> _:auto1 .\n".to_string());
//! ```
//!
//! RDF syntax can also be parsed and transformed into an RDF graph.
//!
//! ```
//! use rdf::reader::turtle_parser::TurtleParser;
//! use rdf::reader::rdf_parser::RdfParser;
//! use rdf::uri::Uri;
//!
//! let input = "@base <http://example.org/> .
//! @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
//! @prefix foaf: <http://xmlns.com/foaf/0.1/> .
//!
//! <http://www.w3.org/2001/sw/RDFCore/ntriples/> rdf:type foaf:Document ;
//!     <http://purl.org/dc/terms/title> \"N-Triples\"@en-US ;
//!     foaf:maker _:art .";
//!
//! let mut reader = TurtleParser::from_string(input.to_string());
//!
//! match reader.decode() {
//!   Ok(graph) => {
//!     assert_eq!(graph.count(), 3);
//!     assert_eq!(graph.namespaces().len(), 2);
//!     assert_eq!(graph.base_uri(), &Some(Uri::new("http://example.org/".to_string())))
//!   },
//!   Err(_) => assert!(false)
//! }
//! ```

use std::result;

pub mod error;
pub mod graph;
pub mod namespace;
pub mod node;
pub mod triple;
pub mod uri;

pub type Result<T> = result::Result<T, error::Error>;

pub mod writer {
    pub mod formatter {
        pub mod n_triples_formatter;
        pub mod rdf_formatter;
        pub mod turtle_formatter;
    }

    pub mod n_triples_writer;
    pub mod rdf_writer;
    pub mod turtle_writer;
}

pub mod reader {
    pub mod lexer {
        pub mod n_triples_lexer;
        pub mod rdf_lexer;
        pub mod token;
        pub mod turtle_lexer;
    }

    pub mod input_reader;
    pub mod n_triples_parser;
    pub mod rdf_parser;
    pub mod turtle_parser;
}

pub mod specs {
    pub mod rdf_syntax_specs;
    pub mod turtle_specs;
    pub mod xml_specs;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
