# rdf-rs

> Note: This project is work in progress and currently not stable.

`rdf-rs` is a library for the [Resource Description Framework](https://www.w3.org/RDF/) (RDF) and [SPARQL](https://www.w3.org/TR/rdf-sparql-query/) implemented in Rust.

This project is a way for me to learn Rust and combine it with my interests in semantic web technologies.

## Usage

A comprehensive documentation of the API is available [**here**](https://scholtzan.github.io/rdf-rs/doc/rdf_rs/index.html).

### Installation

To use `rdf-rs`, add this to `Cargo.toml`:

```toml
[dependencies.rdf-rs]
git = "https://github.com/scholtzan/rdf-rs"
```


### Basic Examples


RDF triples can be stored and represented in a graph.

```rust
use rdf_rs::graph::Graph;
use rdf_rs::uri::Uri;
use rdf_rs::triple::Triple;

let mut graph = Graph::new(None);
let subject = graph.create_blank_node();
let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
let object = graph.create_blank_node();
let triple = Triple::new(&subject, &predicate, &object);

graph.add_triple(&triple);
```

RDF graphs can be serialized to a supported format.

```rust
use rdf_rs::writer::n_triples_writer::NTriplesWriter;
use rdf_rs::writer::rdf_writer::RdfWriter;
use rdf_rs::graph::Graph;
use rdf_rs::uri::Uri;
use rdf_rs::triple::Triple;

let writer = NTriplesWriter::new();

let mut graph = Graph::new(None);
let subject = graph.create_blank_node();
let predicate = graph.create_uri_node(&Uri::new("http://example.org/show/localName".to_string()));
let object = graph.create_blank_node();
let triple = Triple::new(&subject, &predicate, &object);

graph.add_triple(&triple);
assert_eq!(writer.write_to_string(&graph).unwrap(),
           "_:auto0 <http://example.org/show/localName> _:auto1 .\n".to_string());
```

RDF syntax can also be parsed and transformed into an RDF graph.

```rust
use rdf_rs::reader::turtle_parser::TurtleParser;
use rdf_rs::reader::rdf_parser::RdfParser;
use rdf_rs::uri::Uri;

let input = "@base <http://example.org/> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .

<http://www.w3.org/2001/sw/RDFCore/ntriples/> rdf:type foaf:Document ;
        <http://purl.org/dc/terms/title> \"N-Triples\"@en-US ;
        foaf:maker _:art .";

let mut reader = TurtleParser::from_string(input.to_string());
match reader.decode() {
  Ok(graph) => {
    assert_eq!(graph.count(), 3);
    assert_eq!(graph.namespaces().len(), 2);
    assert_eq!(graph.base_uri(), &Some(Uri::new("http://example.org/".to_string())))
  },
  Err(_) => assert!(false)
}
```

## Current State

Currently `rdf-rs` provides basic data structures for representing RDF graphs, triples and nodes.
The following formats can be parsed and serialized:

* Turtle
* N-Triples


## Future Work and Ideas

* Support querying with SPARQL
* Add support for more formats
* More comprehensive `Uri` data structure
