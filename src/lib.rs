//! # rdf-rs
//!
//! A crate for the Resource Description Framework (RDF) and SPARQL.
//!
//! todo
//!
//! ## Usage
//!
//! todo


use std::result;


pub mod uri;
pub mod namespace;
pub mod node;
pub mod triple;
pub mod graph;
pub mod error;

pub type Result<T> = result::Result<T, error::Error>;

pub mod writer {
  pub mod formatter {
    pub mod rdf_formatter;
    pub mod turtle_formatter;
    pub mod n_triples_formatter;
  }

  pub mod rdf_writer;
  pub mod turtle_writer;
  pub mod n_triples_writer;
}

pub mod reader {
  pub mod lexer {
    pub mod token;
    pub mod rdf_lexer;
    pub mod n_triples_lexer;
    pub mod turtle_lexer;
  }

  pub mod rdf_parser;
  pub mod n_triples_parser;
  pub mod turtle_parser;
  pub mod input_reader;
}

pub mod specs {
  pub mod xml_specs;
  pub mod turtle_specs;
  pub mod rdf_syntax_specs;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
