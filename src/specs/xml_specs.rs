use crate::uri::Uri;

/// XML schema data types.
pub enum XmlDataTypes {
    String,
    Decimal,
    Double,
    Boolean,
    Date,
    Long,
    UnsignedLong,
    Int,
    Integer,
}

impl XmlDataTypes {
    /// Returns a specific data type as URI.
    pub fn to_uri(&self) -> Uri {
        Uri::new(self.to_string())
    }

    /// Returns a specific data type as string.
    pub fn to_string(&self) -> String {
        let schema_name = "http://www.w3.org/2001/XMLSchema#".to_string();

        match *self {
            XmlDataTypes::Boolean => schema_name + "boolean",
            XmlDataTypes::Integer => schema_name + "integer",
            XmlDataTypes::Decimal => schema_name + "decimal",
            XmlDataTypes::Double => schema_name + "double",
            XmlDataTypes::Date => schema_name + "date",
            XmlDataTypes::Long => schema_name + "long",
            XmlDataTypes::UnsignedLong => schema_name + "unsignedLong",
            XmlDataTypes::Int => schema_name + "int",
            XmlDataTypes::String => schema_name + "string",
        }
    }
}
