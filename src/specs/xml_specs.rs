use uri::Uri;

// todo
pub enum XmlDataTypes {
  String,
  Decimal,
  Boolean,
  Date,
  Long,
  Int,
  Integer,
}

impl XmlDataTypes {
  pub fn to_uri(&self) -> Uri {
    let schema_name = "http://www.w3.org/2001/XMLSchema#".to_string();

    // todo
    match *self {
      XmlDataTypes::Boolean => Uri::new(schema_name + "boolean"),
      XmlDataTypes::Integer => Uri::new(schema_name + "integer"),
      XmlDataTypes::Decimal => Uri::new(schema_name + "decimal"),
      _ => Uri::new("todo".to_string())
    }
  }

  pub fn to_string(&self) -> String {
    let schema_name = "http://www.w3.org/2001/XMLSchema#".to_string();

    // todo
    match *self {
      XmlDataTypes::Boolean => schema_name + "boolean",
      XmlDataTypes::Integer => schema_name + "integer",
      XmlDataTypes::Decimal => schema_name + "decimal",
      _ => "todo".to_string()
    }
  }
}

pub struct XmlSpecs { }

impl XmlSpecs {
  // todo

}