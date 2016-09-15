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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
