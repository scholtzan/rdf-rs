use uri::Uri;
use specs::xml_specs::XmlDataTypes;

pub struct TurtleSpecs { }

impl TurtleSpecs {
  /// Checks if the provided literal is a plain literal that corresponds to the provided data type.
  ///
  /// # Example
  ///
  /// ```
  /// use rdf_rs::specs::turtle_specs::TurtleSpecs;
  /// use rdf_rs::specs::xml_specs::XmlDataTypes;
  ///
  /// assert!(TurtleSpecs::is_plain_literal(&"3.0".to_string(), &Some(XmlDataTypes::Decimal.to_uri())));
  /// assert!(TurtleSpecs::is_plain_literal(&"true".to_string(), &Some(XmlDataTypes::Boolean.to_uri())));
  /// assert!(TurtleSpecs::is_plain_literal(&"3e10".to_string(), &Some(XmlDataTypes::Decimal.to_uri())));
  /// assert_eq!(TurtleSpecs::is_plain_literal(&"a".to_string(), &Some(XmlDataTypes::Decimal.to_uri())), false);
  /// ```
  pub fn is_plain_literal(literal: &String, data_type: &Option<Uri>) -> bool {
    if TurtleSpecs::is_double_literal(literal) && *data_type == Some(XmlDataTypes::Decimal.to_uri()) {
      return true;
    }

    if TurtleSpecs::is_boolean_literal(literal) && *data_type == Some(XmlDataTypes::Boolean.to_uri()) {
      return true;
    }

    if TurtleSpecs::is_integer_literal(literal) && *data_type == Some(XmlDataTypes::Integer.to_uri()) {
      return true;
    }

    false
  }

  /// Checks if the provided literal is decimal.
  ///
  /// # Example
  ///
  /// ```
  /// use rdf_rs::specs::turtle_specs::TurtleSpecs;
  ///
  /// assert!(TurtleSpecs::is_double_literal(&"3.0".to_string()));
  /// assert!(TurtleSpecs::is_double_literal(&"3e10".to_string()));
  /// assert_eq!(TurtleSpecs::is_double_literal(&"a".to_string()), false);
  /// ```
  pub fn is_double_literal(literal: &String) -> bool {
    match literal.parse::<f64>() {
      Ok(_) => true,
      Err(_) => false
    }
  }

  /// Checks if the provided literal is an integer.
  ///
  /// # Example
  ///
  /// ```
  /// use rdf_rs::specs::turtle_specs::TurtleSpecs;
  ///
  /// assert!(TurtleSpecs::is_integer_literal(&"3".to_string()));
  /// assert_eq!(TurtleSpecs::is_integer_literal(&"3.0".to_string()), false);
  /// ```
  pub fn is_integer_literal(literal: &String) -> bool {
    match literal.parse::<i64>() {
      Ok(_) => true,
      Err(_) => false
    }
  }


  /// Checks if the provided literal is a boolean.
  ///
  /// # Example
  ///
  /// ```
  /// use rdf_rs::specs::turtle_specs::TurtleSpecs;
  ///
  /// assert!(TurtleSpecs::is_boolean_literal(&"true".to_string()));
  /// assert!(TurtleSpecs::is_boolean_literal(&"false".to_string()));
  /// assert_eq!(TurtleSpecs::is_boolean_literal(&"1".to_string()), false);
  /// ```
  pub fn is_boolean_literal(literal: &String) -> bool {
    match literal.parse::<bool>() {
      Ok(_) => true,
      Err(_) => false
    }
  }
}