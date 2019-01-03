use crate::ast::*;
use failure::Fail;
use nom::*;
use std::option::NoneError;

fn is_dec_digit(c: char) -> bool {
    c.is_ascii_digit()
}

named!(decimal<&str, &str>,
       take_while1!(is_dec_digit)
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Sign {
    Positive,
    Negative,
}

fn str_to_int(s: &str, radix: u32) -> u32 {
    let mut n = 0;
    for digit in s.chars() {
        n *= radix;
        n += digit.to_digit(radix).unwrap();
    }
    n
}

/// Possibly the laziest `String -> Float` algorithm there is; not
/// particularly efficient or accurate.
fn make_number(
    sign: Option<Sign>,
    whole_part: &str,
    fractional_part: Option<&str>,
    exponent: Option<&str>,
) -> Value {
    let mut n = f64::from(str_to_int(whole_part, 10));

    if let Some(fractional_part) = fractional_part {
        let numerator = f64::from(str_to_int(fractional_part, 10));
        let denominator = (10.0f64).powi(fractional_part.len() as _);
        n += numerator / denominator;
    }

    if let Some(exponent) = exponent {
        let exponent = str_to_int(exponent, 10);
        n *= (10.0f64).powi(exponent as _);
    }

    if let Some(Sign::Negative) = sign {
        n *= -1.0;
    }

    Value::Number(n)
}

fn is_whitespace(c: char) -> bool {
    const WHITESPACE: &str = " \t\n";

    WHITESPACE.contains(c)
}

named!(whitespace<&str, &str>,
       take_while!(is_whitespace)
);

macro_rules! token {
    ($i:expr, $($args:tt)*) => {
        match tuple!($i, complete!($($args)*) , opt!(whitespace)) {
            Err(e) => Err(e),
            Ok((remaining, (out, _whitespace))) => Ok((remaining, out)),
        }
    }
}

named!(sign<&str, Sign>,
       alt!(
           tag!("+") => { |_| Sign::Positive } |
           tag!("-") => { |_| Sign::Negative }
       )
);

named!(number<&str, Value>,
       do_parse!(
           sign: opt!(sign)
               >> whole_part: decimal
               >> fractional_part: opt!(complete!(preceded!(tag!("."), decimal)))
               >> exponent: opt!(complete!(preceded!(one_of!("eE"), decimal)))
               >> (make_number(sign, whole_part, fractional_part, exponent))
       )
);

named!(word_str<&str, &str>,
       take_until_either1!(".:, \t\n)]}\"'")
);

named!(word<&str, Value>,
       map!(word_str, Value::make_word)
);

macro_rules! complement_part {
    ($i:expr, $pfx:expr) => {
        match tuple!($i, tag!($pfx), char!(':'), value) {
            Ok((rest, (_, _, val))) => Ok((rest, val)),
            Err(e) => Err(e),
        }
    };
}

named!(complement<&str, Value>,
       do_parse!(
           prefix: word_str
               >> char!(':')
               >> head: value
               >> comp: many0!(complete!(complement_part!(prefix)))
               >> (Value::make_complement(head, comp))
       )
);

named!(seq_delimiter<&str, char>,
       token!(one_of!(",;"))
);

named!(sequence<&str, Value>,
       do_parse!(
           delim: seq_delimiter
               >> values: separated_nonempty_list!(token!(char!(delim)),
                                                   value)
               >> (Value::make_sequence(values))
       )
);

pub struct ConlangReader<R> {
    // buffer: Option<String>,
    reader: R,
}

#[derive(Debug, Fail)]
pub enum ReaderError {
    #[fail(display = "too much input: {}", line)]
    TooMuchInput { line: String },
    #[fail(display = "end of file")]
    Eof,
}

impl From<NoneError> for ReaderError {
    fn from(_: NoneError) -> Self {
        ReaderError::Eof
    }
}

impl<R: Iterator<Item = String>> ConlangReader<R> {
    pub fn new(reader: R) -> Self {
        ConlangReader {
            // buffer: None,
            reader,
        }
    }
    pub fn parse_next(&mut self) -> Result<Value, ReaderError> {
        let buf = self.reader.next()?;

        let (remaining, val) = value_line(&buf[..]).unwrap();
        if remaining.is_empty() || buf.chars().all(char::is_whitespace) {
            Ok(val)
        } else {
            Err(ReaderError::TooMuchInput {
                line: remaining.to_owned(),
            })
        }
    }
}

impl<R: Iterator<Item = String>> Iterator for ConlangReader<R> {
    type Item = Value;
    fn next(&mut self) -> Option<Value> {
        self.parse_next().ok()
    }
}

named!(value_line<&str, Value>,
       terminated!(value, opt!(complete!(char!('.'))))
);

named!(value<&str, Value>,
       alt!(
           complement
               | sequence
               | number
               | word
       )
);

#[cfg(test)]
mod test {
    use super::*;

    /// Your input string must end with a space, and should consume
    /// all but that final space.
    // This is so that it is terminated by a whitespace character,
    // rather than an end-of-input, which will cause different
    // behavior, and behavior which is not worth testing because it
    // doesn't appear outside of tests.
    macro_rules! assert_parser_eq {
        ($parse:expr, $res:expr) => {
            assert_eq!($parse, Ok((&" "[..], $res)))
        };
    }
    #[test]
    fn parse_number() {
        assert_parser_eq!(number("123 "), Value::Number(123.0));
        assert_parser_eq!(number("123.4 "), Value::Number(123.4));
        assert_parser_eq!(number("123e4 "), Value::Number(123.0e4));
    }
    #[test]
    fn parse_word() {
        assert_parser_eq!(word("asdf "), Value::make_word("asdf"));
        assert_parser_eq!(word("foo_bar_baz "), Value::make_word("foo_bar_baz"));
    }
}
