use Result;
use std::io;
use std::str;
use std::io::prelude::*;
use error::Error;

// todo: store white spaces


/// Returns the next character of an input source.
///
/// # Example
/// ```
/// use rdf_rs::helper;
///
/// let mut input = "Hello World!".as_bytes();
/// assert_eq!(Some('H'), helper::get_next_char(&mut input).unwrap());
/// assert_eq!(Some('e'), helper::get_next_char(&mut input).unwrap());
/// ```
pub fn get_next_char<R: Read>(reader: &mut R) -> Result<Option<char>> {
  const MAX_BYTES: usize = 4;
  let mut buf = [0u8; MAX_BYTES];

  let mut bytes = reader.bytes();

  for pos in 0..MAX_BYTES {
    let byte = match bytes.next() {
      Some(Ok(b)) => b,
      None => return Ok(None),
      Some(Err(_)) => return Err(Error::InvalidReaderInput),
    };

    buf[pos] = byte;

    match str::from_utf8(&buf[..(pos + 1)]) {
      Ok(s) => return Ok(s.chars().next()),
      Err(_) if pos < MAX_BYTES - 1 => {},
      _ => return Err(Error::InvalidByteEncoding)
    }
  }

  Err(Error::InvalidReaderInput)
}


/// Returns the next character of an input source that is not a whitespace.
///
/// # Example
/// ```
/// use rdf_rs::helper;
///
/// let mut input = "H   ello World!".as_bytes();
/// assert_eq!(Some('H'), helper::get_next_char_discard_leading_spaces(&mut input).unwrap());
/// assert_eq!(Some('e'), helper::get_next_char_discard_leading_spaces(&mut input).unwrap());
/// ```
pub fn get_next_char_discard_leading_spaces<R: Read>(reader: &mut R) -> Result<Option<char>> {
  loop {
    match get_next_char(reader) {
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
/// use rdf_rs::helper;
///
/// let mut input = "Hello World!".as_bytes();
/// assert_eq!("Hello".to_string(), helper::get_until(&mut input, |c| c == ' ').unwrap());
/// assert_eq!("World".to_string(), helper::get_until(&mut input, |c| c == '!').unwrap());
/// ```
pub fn get_until<R: Read, F: Fn(char) -> bool>(reader: &mut R, delimiter: F) -> Result<String> {
  let mut buf = Vec::new();

  loop {
    match get_next_char(reader) {
      Ok(Some(c)) if delimiter(c) => return Ok(buf.into_iter().collect()),
      Ok(Some(c)) if !delimiter(c) => buf.push(c),
      Ok(_) => return Err(Error::EndOfInput(buf.into_iter().collect())),
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
/// use rdf_rs::helper;
///
/// let mut input = "Hello    World!".as_bytes();
/// assert_eq!("Hello".to_string(), helper::get_until_discard_leading_spaces(&mut input, |c| c == ' ').unwrap());
/// assert_eq!("World".to_string(), helper::get_until_discard_leading_spaces(&mut input, |c| c == '!').unwrap());
/// ```
pub fn get_until_discard_leading_spaces<R: Read, F: Fn(char) -> bool>(reader: &mut R, delimiter: F) -> Result<String> {
  match get_until(reader, delimiter) {
    Ok(str) => Ok(str.to_owned().trim().to_string()),
    Err(Error::EndOfInput(str)) => Err(Error::EndOfInput(str.to_owned().trim().to_string())),
    Err(err) => Err(err)
  }
}