use std::io::Read;
use error::{Error, ErrorType};
use Result;
use std::str;


/// todo
pub struct InputReader<R: Read> {
  input: R,
  peeked_char: Option<char>
}

impl<R: Read> InputReader<R> {
  /// Constructor for `InputReader`.
  ///
  /// # Examples
  ///
  /// todo
  ///
  pub fn new(input: R) -> InputReader<R> {
    InputReader {
      input: input,
      peeked_char: None
    }
  }

  /// todo
  pub fn peek_next_char(&mut self) -> Result<Option<char>> {
    match self.peeked_char {
      Some(peeked_char) => Ok(Some(peeked_char)),
      None =>
        match self.get_next_char() {
          Ok(Some(next_char)) => {
            self.peeked_char = Some(next_char);
            Ok(Some(next_char))
          },
          Ok(None) => Ok(None),
          Err(err) => Err(err)
        }
    }
  }

  /// todo
  pub fn peek_next_char_discard_leading_spaces(&mut self) -> Result<Option<char>> {
    match self.get_next_char_discard_leading_spaces() {
      Ok(Some(next_char)) => {
        self.peeked_char = Some(next_char);
        Ok(Some(next_char))
      },
      Ok(None) => Ok(None),
      Err(err) => Err(err)
    }
  }

  /// Returns the next character of an input source.
  ///
  /// # Example
  /// ```
  /// use rdf_rs::reader::input_reader::InputReader;
  ///
  /// let mut input = "Hello World!".as_bytes();
  /// let mut input_reader = InputReader::new(input);
  ///
  /// assert_eq!(Some('H'), input_reader.get_next_char().unwrap());
  /// assert_eq!(Some('e'), input_reader.get_next_char().unwrap());
  /// ```
  pub fn get_next_char(&mut self) -> Result<Option<char>> {
    match self.peeked_char {
      Some(c) => {
        self.peeked_char = None;
        return Ok(Some(c))
      },
      None => {}
    }

    const MAX_BYTES: usize = 4;
    let mut buf = [0u8; MAX_BYTES];

    let input = &mut self.input;
    let mut bytes = input.bytes();

    for pos in 0..MAX_BYTES {
      let byte = match bytes.next() {
        Some(Ok(b)) => b,
        None => return Ok(None),
        Some(Err(_)) => return Err(Error::new(ErrorType::InvalidReaderInput,
                                              "Invalid input character.")),
      };

      buf[pos] = byte;

      match str::from_utf8(&buf[..(pos + 1)]) {
        Ok(s) => return Ok(s.chars().next()),
        Err(_) if pos < MAX_BYTES - 1 => {},
        _ => return Err(Error::new(ErrorType::InvalidByteEncoding,
                                   "Invalid byte encoding of input."))
      }
    }

    Err(Error::new(ErrorType::InvalidReaderInput,
                   "Unexpected error while reading input."))
  }


  /// Returns the next character of an input source that is not a whitespace.
  ///
  /// # Example
  /// ```
  /// use rdf_rs::reader::input_reader::InputReader;
  ///
  /// let mut input = "H   ello World!".as_bytes();
  /// let mut input_reader = InputReader::new(input);
  ///
  /// assert_eq!(Some('H'), input_reader.get_next_char_discard_leading_spaces().unwrap());
  /// assert_eq!(Some('e'), input_reader.get_next_char_discard_leading_spaces().unwrap());
  /// ```
  pub fn get_next_char_discard_leading_spaces(&mut self) -> Result<Option<char>> {
    loop {
      match self.get_next_char() {
        Ok(Some(' ')) => { },
        Ok(Some('\n')) => { },
        Ok(Some('\t')) => { },
        Ok(Some('\r')) => { },
        c => return c
      }
    }
  }


  /// Returns all characters of a input source until a certain delimiter occurs.
  ///
  /// The delimiter itself is skipped.
  ///
  /// # Example
  /// ```
  /// use rdf_rs::reader::input_reader::InputReader;
  ///
  /// let mut input = "Hello World!".as_bytes();
  /// let mut input_reader = InputReader::new(input);
  ///
  /// assert_eq!("Hello".to_string(), input_reader.get_until(|c| c == ' ').unwrap());
  /// assert_eq!(" World".to_string(), input_reader.get_until(|c| c == '!').unwrap());
  /// ```
  pub fn get_until<F: Fn(char) -> bool>(&mut self, delimiter: F) -> Result<String> {
    let mut buf = Vec::new();

    loop {
      match self.get_next_char() {
        Ok(Some(c)) if delimiter(c) => {
          self.peeked_char = Some(c);
          return Ok(buf.into_iter().collect())
        },
        Ok(Some(c)) if !delimiter(c) => buf.push(c),
        Ok(_) => return Err(Error::new(ErrorType::EndOfInput(buf.into_iter().collect()),
                            "End of input.")),
        Err(err) => return Err(err)
      }
    }
  }


  /// Returns all characters of a input source until a certain delimiter occurs and removes leading whitespaces.
  ///
  /// The delimiter itself is skipped.
  ///
  /// # Example
  /// ```
  /// use rdf_rs::reader::input_reader::InputReader;
  ///
  /// let mut input = "Hello    World!".as_bytes();
  /// let mut input_reader = InputReader::new(input);
  ///
  /// assert_eq!("Hello".to_string(), input_reader.get_until_discard_leading_spaces(|c| c == ' ').unwrap());
  /// assert_eq!("World".to_string(), input_reader.get_until_discard_leading_spaces(|c| c == '!').unwrap());
  /// ```
  pub fn get_until_discard_leading_spaces<F: Fn(char) -> bool>(&mut self, delimiter: F) -> Result<String> {
    match self.get_until(delimiter) {
      Ok(str) => Ok(str.to_owned().trim().to_string()),
      Err(err) => {
        match err.error_type() {
          &ErrorType::EndOfInput(ref str) => Err(Error::new(ErrorType::EndOfInput(str.to_owned().trim().to_string()),
                                                        "End of input")),
          _ => Err(Error::new(ErrorType::InvalidReaderInput, "Error while reading input."))
        }
      }
    }
  }
}
