use nessie_parse::{one_of, ParseResult, Parser};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Error {
    CantHaveLetterAfterNumber,
    IdentifierIsReservedWord(&'static str),
}

fn number<'a>() -> Parser<'a, String, Error> {
    Parser::digit()
        .repeat_1()
        .map(|digits| digits.into_iter().collect())
        .map_fail(|_| ())
        .and_then(|digits: String| {
            Parser::not(Parser::letter())
                .or_err(Error::CantHaveLetterAfterNumber)
                .map(move |_| digits.clone())
        })
        .with_name("number")
}

#[test]
fn empty_number() {
    let result = number().parse("".into());
    assert!(matches!(result, ParseResult::Fail(..)));
}

#[test]
fn number_with_letter() {
    let result = number().parse("123abc".into());
    assert!(matches!(result, ParseResult::Err(..)));
}

#[test]
fn good_number() {
    let result = number().parse("123".into());
    assert!(matches!(result, ParseResult::Ok(s, _) if s == "123"));
}

fn word<'a>() -> Parser<'a, String, Error> {
    Parser::not(Parser::digit())
        .and_then(|_| {
            one_of![
                Parser::letter().map_fail(|_| ()),
                Parser::digit().map_fail(|_| ()),
                Parser::char_eq('_'),
            ]
            .map_fail(|_| ())
            .repeat_1()
            .map(|letters| letters.into_iter().collect())
        })
        .with_name("word")
}

const RESEREVED_WORDS: &[&str] = &[
    "if", "else", "while",
];

fn identifier<'a>() -> Parser<'a, String, Error> {
    word()
        .and_then(|word| {
            if let Some(reserved) = RESEREVED_WORDS.iter().find(|&&r| r == word) {
                Parser::err(Error::IdentifierIsReservedWord(reserved))
            } else {
                Parser::ret(word)
            }
        })
}

#[test]
fn empty_identifier() {
    let result = identifier().parse("".into());
    assert!(matches!(result, ParseResult::Fail(..)));
}

#[test]
fn identifier_with_digit() {
    let result = identifier().parse("abc123de".into());
    assert!(matches!(result, ParseResult::Ok(s, _) if s == "abc123de"));
}

#[test]
fn identifier_wit_digit_at_start() {
    let result = identifier().parse("123abc".into());
    dbg!(&result);
    assert!(matches!(result, ParseResult::Fail(..)));
}

#[test]
fn identifier_that_is_resereved() {
    let result = identifier().parse("if tanin".into());
    dbg!(&result);
    assert!(matches!(result, ParseResult::Err(..)));
}

