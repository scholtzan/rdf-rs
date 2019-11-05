use crate::uri::Uri;

/// RDF Schema data types and vocabulary.
pub enum RdfSyntaxDataTypes {
    A,
    ListFirst,
    ListRest,
    ListNil,
}

impl RdfSyntaxDataTypes {
    /// Returns a specific data type as URI.
    pub fn to_uri(&self) -> Uri {
        Uri::new(self.to_string())
    }

    /// Returns a specific data type as string.
    pub fn to_string(&self) -> String {
        let schema_name = "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string();

        match *self {
            RdfSyntaxDataTypes::A => schema_name + "type",
            RdfSyntaxDataTypes::ListFirst => schema_name + "first",
            RdfSyntaxDataTypes::ListRest => schema_name + "rest",
            RdfSyntaxDataTypes::ListNil => schema_name + "nil",
        }
    }
}
