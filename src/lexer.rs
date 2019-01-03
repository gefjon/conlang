use failure::Fail;
use nom::*;

fn lex<'a, I: Iterator<Item = &'a str>>(iter: I) -> LexemeIterator<I> {
    LexemeIterator { inner: iter }
}

pub struct LexemeIterator<I> {
    inner: I,
}

#[derive(Debug, Fail)]
pub enum LexerError {
    #[fail(display = "{}", _0)]
    ErrorText(String),
}

impl<'a, I> Iterator for LexemeIterator<I>
where
    I: Iterator<Item = &'a str>,
{
    type Item = Result<Lexeme<'a>, LexerError>;
    fn next(&mut self) -> Option<Result<Lexeme<'a>, LexerError>> {
        unimplemented!()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum Lexeme<'a> {
    Word(&'a str),
    /// `:`
    Decline,
    /// `-`
    Compound,
    /// `,`
    Comma,
    /// `;`
    Semicolon,
    /// `.`
    Stop,
    /// `?`
    Inquisitive,
    /// `!`
    Imperative,
    /// `&`
    Ampersand,
}

/// These chars may not be word constituients. In addition, no
/// character for which `char::is_whitespace` returns `true` may be a
/// word constituient.
const RESERVED_CHARS: &str = "`\"'\\-,:;?!&()[]{}<>=%$@^";

macro_rules! lexeme {
    ($i:expr, $($args:tt)*) => {
        terminated!($i, $($args)*, opt!(complete!(whitespace)))
    }
}

macro_rules! special_character {
    ($i:expr, $c:expr, $($and_then:tt)*) => {
        map!($i, lexeme!(char!($c)), |_| $($and_then)*)
    }
}

macro_rules! take_if {
    ($i:expr, $fname:path) => {
        map_opt!($i, none_of!(""), |c| if $fname(c) { Some(c) } else { None })
    };
}

named!(whitespace<&str, &str>,
       take_while!(char::is_whitespace)
);

named!(word_starter<&str, char>,
       take_if!(is_word_starter)
);

named!(word<&str, Lexeme>,
       lexeme!(do_parse!(
           peek!(word_starter)
               >> word: take_while!(is_word_constituient)
               >> (Lexeme::Word(word))
       ))
);

named!(lexeme<&str, Lexeme>,
       alt!(word
            | special_character!(':', Lexeme::Decline)
            | special_character!('-', Lexeme::Compound)
            | special_character!(',', Lexeme::Comma)
            | special_character!(';', Lexeme::Semicolon)
            | special_character!('.', Lexeme::Stop)
            | special_character!('?', Lexeme::Inquisitive)
            | special_character!('&', Lexeme::Ampersand)
       )
);

fn is_word_constituient(c: char) -> bool {
    !(c.is_whitespace() || RESERVED_CHARS.contains(c))
}

fn is_word_starter(c: char) -> bool {
    c == '_' || c.is_alphanumeric()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn specials_are_not_word_constituients() {
        assert!(!is_word_constituient(':'));
        assert!(!is_word_constituient('-'));
    }
    #[test]
    fn read_a_word() {
        let input = "foo bar baz";

        match word(&input) {
            Ok(("bar baz", Lexeme::Word("foo"))) => (),
            Ok(r) => panic!(
                "unexpected non-erroneous result when parsing \
                 \"{}\": {:?}",
                input, r
            ),
            Err(e) => panic!(
                "error while parsing \
                 \"{}\": {}",
                input, e
            ),
        }
    }
    #[test]
    fn read_several_words() {
        let mut input = &"foo bar baz "[..];
        let mut outputs = Vec::with_capacity(3);
        let expected = &[
            Lexeme::Word("foo"),
            Lexeme::Word("bar"),
            Lexeme::Word("baz"),
        ];

        while !(input.is_empty() || input == " ") {
            let (rem, lex) = lexeme(input).unwrap();
            input = rem;
            outputs.push(lex);
        }

        assert_eq!(outputs, expected);
    }
}
