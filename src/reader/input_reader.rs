use std::io::Read;
use error::{Error, ErrorType};
use Result;
use std::str;
use std::ops::Index;

pub struct InputReaderHelper {

}

// todo
impl InputReaderHelper {
  pub fn whitespace(c: char) -> bool {
    c == '\n' || c == '\r' || c == ' '
  }

  pub fn line_break(c: char) -> bool {
    c == '\n' || c == '\r'
  }

  pub fn node_delimiter(c: char) -> bool {
    c == '\n' || c == '\r' || c == ' ' || c == '.'
  }

  pub fn digit(c: char) -> bool {
    c >= '0' && c <= '9'
  }
}

type InputChar = Option<char>;


// todo
#[derive(Debug, Clone)]
pub struct InputChars {
  input_chars: Vec<InputChar>
}

impl ToString for InputChars {
  fn to_string(&self) -> String {
    let s: String = self.input_chars.clone().into_iter().flat_map(|c| c).collect();
    s
  }
}


impl Index<usize> for InputChars {
  type Output = InputChar;

  fn index(&self, i: usize) -> &InputChar {
    &self.input_chars[i]
  }
}


impl InputChars {
  pub fn new(chars: Vec<InputChar>) -> InputChars {
    InputChars {
      input_chars: chars
    }
  }

  pub fn to_vec(&self) -> Vec<InputChar> {
    self.input_chars.clone()
  }

  pub fn len(&self) -> usize {
    self.input_chars.len()
  }

  pub fn push(&mut self, c: InputChar) {
    self.input_chars.push(c);
  }

  pub fn insert(&mut self, i: usize, c: InputChar) {
    self.input_chars.insert(i, c);
  }

  pub fn remove(&mut self, i: usize) -> InputChar {
    self.input_chars.remove(i)
  }

  pub fn append(&mut self, other: &mut InputChars) {
    self.input_chars.append(&mut other.to_vec());
  }
}


// todo
// todo: specific return type that can be mapped to string and vec
pub struct InputReader<R: Read> {
  input: R,
  peeked_chars: InputChars
}

impl<R: Read> InputReader<R> {
  /// Constructor for `InputReader`.
  ///
  /// # Exampless
  ///
  /// todo
  ///
  pub fn new(input: R) -> InputReader<R> {
    InputReader {
      input: input,
      peeked_chars: InputChars::new(Vec::new())
    }
  }

  // todo
  pub fn peek_next_k_chars(&mut self, k: usize) -> Result<InputChars> {
    if self.peeked_chars.len() >= k {
      Ok(InputChars::new(self.peeked_chars.to_vec()[0..k].to_vec()))
    } else {
      let next_k_chars = try!(self.get_next_k_chars(k));
      self.peeked_chars = next_k_chars.clone();
      Ok(next_k_chars)
    }
  }

  // todo
  pub fn peek_next_char(&mut self) -> Result<InputChar> {
    let peeked_char = try!(self.peek_next_k_chars(1));
    Ok(peeked_char.to_vec()[0])
  }

  // todo
  pub fn peek_next_char_discard_leading_spaces(&mut self) -> Result<InputChar> {
    match self.get_next_char_discard_leading_spaces() {
      Ok(Some(next_char)) => {
        if self.peeked_chars.len() <= 0 {
          self.peeked_chars.push(Some(next_char));
        }

        Ok(Some(next_char))
      },
      Ok(None) => Ok(None),
      Err(err) => Err(err)
    }
  }

  /// Returns the next character of an input source.
  ///
  /// # Examples
  /// ```
  /// use rdf_rs::reader::input_reader::InputReader;
  ///
  /// let mut input = "Hello World!".as_bytes();
  /// let mut input_reader = InputReader::new(input);
  ///
  /// assert_eq!(Some('H'), input_reader.get_next_char().unwrap());
  /// assert_eq!(Some('e'), input_reader.get_next_char().unwrap());
  /// ```
  pub fn get_next_char(&mut self) -> Result<InputChar> {
    if self.peeked_chars.len() > 0 {
      return Ok(self.peeked_chars.remove(0));
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


  // todo
  pub fn get_next_k_chars(&mut self, k: usize) -> Result<InputChars> {
    let mut next_k_chars = Vec::new();

    for _ in 0..k {
      let next_char = try!(self.get_next_char());
      next_k_chars.push(next_char);
    }

    Ok(InputChars::new(next_k_chars))
  }


  /// Returns the next character of an input source that is not a whitespace.
  ///
  /// # Examples
  /// ```
  /// use rdf_rs::reader::input_reader::InputReader;
  ///
  /// let mut input = "H   ello World!".as_bytes();
  /// let mut input_reader = InputReader::new(input);
  ///
  /// assert_eq!(Some('H'), input_reader.get_next_char_discard_leading_spaces().unwrap());
  /// assert_eq!(Some('e'), input_reader.get_next_char_discard_leading_spaces().unwrap());
  /// ```
  pub fn get_next_char_discard_leading_spaces(&mut self) -> Result<InputChar> {
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

  // todo
  pub fn peek_until<F: Fn(char) -> bool>(&mut self, delimiter: F) -> Result<InputChars> {
    let mut chars = self.get_until(delimiter)?;
    let result = chars.clone();
    chars.append(&mut self.peeked_chars);
    self.peeked_chars = chars;
    Ok(result)
  }

  // todo
  pub fn peek_until_discard_leading_spaces<F: Fn(char) -> bool>(&mut self, delimiter: F) -> Result<InputChars> {
    let mut chars = self.get_until_discard_leading_spaces(delimiter)?;
    let result = chars.clone();
    chars.append(&mut self.peeked_chars);
    self.peeked_chars = chars;
    Ok(result)
  }

  /// Returns all characters of a input source until a certain delimiter occurs.
  ///
  /// The delimiter itself is skipped.
  ///
  /// # Examples
  /// ```
  /// use rdf_rs::reader::input_reader::InputReader;
  ///
  /// let mut input = "Hello World!".as_bytes();
  /// let mut input_reader = InputReader::new(input);
  ///
  /// assert_eq!("Hello".to_string(), input_reader.get_until(|c| c == ' ').unwrap().to_string());
  /// assert_eq!(" World".to_string(), input_reader.get_until(|c| c == '!').unwrap().to_string());
  /// ```
  pub fn get_until<F: Fn(char) -> bool>(&mut self, delimiter: F) -> Result<InputChars> {
    let mut buf = Vec::new();

    loop {
      match self.get_next_char()? {
        Some(c) if delimiter(c) => {
          self.peeked_chars.insert(0, Some(c));

          return Ok(InputChars::new(buf.into_iter().collect()))
        },
        Some(c) if !delimiter(c) => buf.push(Some(c)),
        _ => return Err(Error::new(ErrorType::EndOfInput(InputChars::new(buf.into_iter().collect())),
                            "End of input."))
      }
    }
  }


  /// Returns all characters of a input source until a certain delimiter occurs and removes leading whitespaces.
  ///
  /// The delimiter itself is skipped.
  ///
  /// # Examples
  /// ```
  /// use rdf_rs::reader::input_reader::InputReader;
  ///
  /// let mut input = "Hello    World!".as_bytes();
  /// let mut input_reader = InputReader::new(input);
  ///
  /// assert_eq!("Hello".to_string(), input_reader.get_until_discard_leading_spaces(|c| c == ' ').unwrap().to_string());
  /// assert_eq!("World".to_string(), input_reader.get_until_discard_leading_spaces(|c| c == '!').unwrap().to_string());
  /// ```
  pub fn get_until_discard_leading_spaces<F: Fn(char) -> bool>(&mut self, delimiter: F) -> Result<InputChars> {
    let whitespaces = InputReaderHelper::whitespace;

    // consume leading whitespaces
    while whitespaces(self.peek_next_char()?.unwrap_or('x')) {
      let _ = self.get_next_char();
    }

    self.get_until(delimiter)
  }
}
