use super::combinator::*;
use indexmap::IndexMap;
use smallvec::SmallVec;
use std::borrow::Cow;
use std::ops::RangeInclusive;
use std::str;

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(Cow<'a, str>),
    Array(Vec<Value<'a>>),
    Object(IndexMap<Cow<'a, str>, Value<'a>>),
}

const DIGIT: RangeInclusive<u8> = b'0'..=b'9';

/// Parses zero or more whitespace characters and ignores them (returns the empty tuple).
pub fn ws<'a>(input: Bytes<'a>) -> ParseResult<'a, ()> {
    take_while0((b' ', b'\n', b'\r', b'\t'))
        .map(|_| ())
        .parse(input)
}

/// Parses a boolean value. See the grammar in the handout.
///
/// HINT: use `alt`.
pub fn parse_bool<'a>(input: Bytes<'a>) -> ParseResult<'a, bool> {
    // ========== TODO: YOUR CODE GOES HERE ==========
    todo!()
    // ===============================================
}

fn digits_to_i64(digits: &[u8]) -> i64 {
    let s = str::from_utf8(digits).unwrap();
    s.parse::<i64>().unwrap()
}

/// Parses an integer value. See the grammar in the handout.
pub fn parse_int<'a>(input: Bytes<'a>) -> ParseResult<'a, i64> {
    (opt(one_of(b'-')), take_while1(DIGIT))
        .map(|(sign, digits)| {
            let mult = if let Some(b'-') = sign { -1 } else { 1 };
            digits_to_i64(digits) * mult
        })
        .parse(input)
}

fn digits_to_f64(i_part: &[u8], f_part: &[u8]) -> f64 {
    let mut whole = String::new();
    whole.push_str(str::from_utf8(i_part).unwrap());
    whole.push('.');
    whole.push_str(str::from_utf8(f_part).unwrap());
    whole.parse::<f64>().unwrap()
}

/// Parses a floating point value. See the grammar in the handout.
///
/// HINT: use `digits_to_f64`.
pub fn parse_float<'a>(input: Bytes<'a>) -> ParseResult<'a, f64> {
    // ========== TODO: YOUR CODE GOES HERE ==========
    todo!()
    // ===============================================
}

/// Parses an escape sequence, i.e., 'n', 'r', 't', a double quote, or a backslash. For instance, if
/// the input is the bytes b'n', the output should be the string "\n". See the grammar in the
/// handout.
///
/// HINT: a string literal like "\n" has lifetime 'static, which is the lifetime of the program and
/// therefore longer than 'a. Hence, you can return a string literal from this function.
pub fn parse_escape<'a>(input: Bytes<'a>) -> ParseResult<'a, &'a str> {
    // ========== TODO: YOUR CODE GOES HERE ==========
    todo!()
    // ===============================================
}

/// A helper function that parses characters until it sees a double quote (the end of the string) or
/// a backslash (possibly the beginning of an escape sequence).
pub fn parse_string_part<'a>(input: Bytes<'a>) -> ParseResult<'a, &'a str> {
    // ========== TODO: YOUR CODE GOES HERE ==========
    todo!()
    // ===============================================
}

/// Parses a string value. See the grammar in the handout. You might find `parse_string_part`
/// helpful here.
///
/// YOU MUST return `Cow::Borrowed` for strings that contain no escape sequences and strings that
/// are exactly an escape sequence! This is the sense in which our parser is "zero-copy".
pub fn parse_string<'a>(input: Bytes<'a>) -> ParseResult<'a, Cow<'a, str>> {
    // ========== TODO: YOUR CODE GOES HERE ==========
    // Try filling in the following skeleton:
    //
    // delimited(
    //     TODO
    // )
    // .map(|parts: SmallVec<[&'a str; 4]>| {
    //     if parts.len() == 0 {
    //         TODO
    //     } else if parts.len() == 1 {
    //         TODO
    //     } else {
    //         TODO
    //     }
    // })
    // .parse(input)
    // ===============================================
    todo!()
}

/// Parses a single JSON value. See the grammar in the handout.
///
/// CAREFUL with the order that you apply different parsers! Some orders may cause tests to fail!
pub fn parse_element<'a>(input: Bytes<'a>) -> ParseResult<'a, Value<'a>> {
    // ========== TODO: YOUR CODE GOES HERE ==========
    todo!()
    // ===============================================
}

/// Parses an array value. See the grammar in the handout.
///
/// REMEMBER to allow whitespace between the elements!
pub fn parse_array<'a>(input: Bytes<'a>) -> ParseResult<'a, Vec<Value<'a>>> {
    delimited(
        (one_of(b'['), ws),
        separated(parse_element, (ws, one_of(b','), ws)),
        (ws, one_of(b']')),
    )
    .parse(input)
}

/// Parses a single member of an object. See the grammar in the handout.
///
/// REMEMBER to allow whitespace between the key and the value!
pub fn parse_member<'a>(input: Bytes<'a>) -> ParseResult<'a, (Cow<'a, str>, Value<'a>)> {
    // ========== TODO: YOUR CODE GOES HERE ==========
    todo!()
    // ===============================================
}

/// Parses an object value. See the grammar in the handout.
///
/// REMEMBER to allow whitespace between the members!
pub fn parse_object<'a>(input: Bytes<'a>) -> ParseResult<'a, IndexMap<Cow<'a, str>, Value<'a>>> {
    // ========== TODO: YOUR CODE GOES HERE ==========
    todo!()
    // ===============================================
}
