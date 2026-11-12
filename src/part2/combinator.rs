use super::ta::{AndThen, Map};
use colored::Colorize;
use indexmap::IndexMap;
use smallvec::{Array, SmallVec};
use std::fmt;
use std::hash::Hash;
use std::ops::RangeInclusive;
use std::str;

/// This struct represents the input to a parser.
/// - `data` is the text that we are parsing, represented as a slice of bytes.
/// - `offset` is the offset of the next byte to be parsed from the beginning of `data`.
///
/// Since `Bytes` is just a borrow plus an integer (16 bytes in size), it is very cheap to clone.
/// Therefore, we mark it as `Copy`, which means that Rust will automatically copy it whenever we
/// assign it to a new variable.
#[derive(Clone, Copy)]
pub struct Bytes<'a> {
    data: &'a [u8],
    offset: usize,
}

/// This lets us print `Bytes` to help with debugging.
impl<'a> fmt::Debug for Bytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let utf8 = String::from_utf8_lossy(self.data);
        write!(f, "Bytes {{ data: {:?}, offset: {} }}", utf8, self.offset)
    }
}

impl<'a> Bytes<'a> {
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns `None` if there is not more input. Otherwise, returns a tuple:
    /// - The first element is the next byte from the input.
    /// - The second element is the rest of the input, i.e. the input without the first byte, represented as a `Bytes`
    pub fn next(&self) -> Option<(u8, Self)> {
        // ========== TODO: YOUR CODE GOES HERE ==========
        todo!()
        // ===============================================
    }

    /// Returns `None` if there are fewer than `len` bytes of input left. Otherwise, returns a tuple:
    /// - The first element is a slice of the next `len` tokens from the input.
    /// - The second element is the rest of the input, i.e. the input without the first `len` tokens, represented as a `Bytes`
    pub fn next_slice(&self, len: usize) -> Option<(&'a [u8], Self)> {
        // ========== TODO: YOUR CODE GOES HERE ==========
        todo!()
        // ===============================================
    }

    /// Returns a tuple.
    /// - The first element is a slice of the longest prefix of tokens that satisfy the predicate.
    /// - The second element is the rest of the input, i.e. the input without the prefix, represented as a `Bytes`
    pub fn take_while(&self, pred: impl Fn(u8) -> bool) -> (&'a [u8], Self) {
        // ========== TODO: YOUR CODE GOES HERE ==========
        todo!()
        // ===============================================
    }
}

/// The output type for parsers.
///
/// If a parser succeeds, it returns a tuple:
/// - The first element is the value that was parsed.
/// - The second element is the rest of the input, i.e. the input without the tokens that were parsed.
///
/// If the parser fails, it returns the input that it was given.
pub type ParseResult<'a, O> = Result<(O, Bytes<'a>), Bytes<'a>>;

/// The trait that all parsers implement.
///
/// In the file `src/part2/ta.rs`, we implemented `Parser` for:
/// - Every function or closure that takes a `Bytes<'a>` and returns a `ParseResult<'a, O>`.
/// - Tuples of parsers. E.g. `(p1, p2)` is a parser that parses `p1` followed by `p2`.
pub trait Parser<'a>: Clone {
    type Out;

    fn parse(&self, input: Bytes<'a>) -> ParseResult<'a, Self::Out>;

    /// Returns a new parser that applies a function `f` to the output of this parser. We already
    /// implemented this for you.
    fn map<O2, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Out) -> O2,
    {
        Map::new(self, f)
    }

    /// Returns a new parser that applies a function `f` that can fail (i.e. return `None`) to the
    /// output of this parser. We already implemented this for you.
    fn and_then<O2, F>(self, f: F) -> AndThen<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Out) -> Option<O2>,
    {
        AndThen::new(self, f)
    }
}

/// Wraps `Input::next` so that it returns a `ParseResult` instead of an `Option`.
fn expect_next<'a>(input: Bytes<'a>) -> ParseResult<'a, u8> {
    match input.next() {
        Some((tok, rest)) => Ok((tok, rest)),
        None => Err(input),
    }
}

/// Wraps `Input::next_slice` so that it returns a `ParseResult` instead of an `Option`.
fn expect_next_slice<'a>(input: Bytes<'a>, len: usize) -> ParseResult<'a, &'a [u8]> {
    match input.next_slice(len) {
        Some((toks, rest)) => Ok((toks, rest)),
        None => Err(input),
    }
}

/// Wraps a parser in print statements that show the input and output of the parser. Very useful for
/// debugging your implementation.
pub fn trace<'a, O>(
    name: &'static str,
    parser: impl Parser<'a, Out = O>,
) -> impl Parser<'a, Out = O> {
    move |input: Bytes<'a>| {
        println!("{} \"{}\" (data = {:?})", "START".red(), name, input);
        let (item, rest) = parser.parse(input)?;
        println!("{} \"{}\" (data = {:?})", "END  ".blue(), name, rest);
        Ok((item, rest))
    }
}

pub trait Alt<'a, O>: Clone {
    fn alt(&self, input: Bytes<'a>) -> ParseResult<'a, O>;
}

/// Returns a new parser that tries several parsers in sequence. We have already implemented this
/// for tuples of parsers for you in `src/part2/ta.rs`.
///
/// For example, `alt((p1, p2))` tries to parse `p1` and, if that fails, tries to parse `p2`.
pub fn alt<'a, O>(parsers: impl Alt<'a, O>) -> impl Parser<'a, Out = O> {
    move |input: Bytes<'a>| parsers.alt(input)
}

/// Represents a set of tokens. In `src/part2/ta.rs`, we implemented `TokSet` for tuples of sets.
///
/// For example, since we implement `TokSet` for `u8` below, we can also use `(b'a', b'b')` (a tuple
/// of `u8`s) as a `TokSet`.
pub trait TokSet: Clone {
    fn contains(&self, tok: u8) -> bool;
}

impl TokSet for u8 {
    fn contains(&self, tok: u8) -> bool {
        *self == tok
    }
}

impl TokSet for RangeInclusive<u8> {
    fn contains(&self, item: u8) -> bool {
        RangeInclusive::<_>::contains(self, &item)
    }
}

/// Returns a new parser that parses the next byte if it is contained in `set`. If it succeeds, it
/// returns the parsed byte. Otherwise, it returns the input that it was given.
///
/// For example, `one_of((b'a', b'b')).parse(b"abc")` parses and returns the byte `b'a'`, leaving
/// remaining input b"bc". But `one_of((b'a', b'b')).parse(b"def")` returns an error.
///
/// HINT: it may be convenient to use `expect_next`.
pub fn one_of<'a>(set: impl TokSet) -> impl Parser<'a, Out = u8> {
    move |input: Bytes<'a>| {
        // ========== TODO: YOUR CODE GOES HERE ==========
        todo!()
        // ===============================================
    }
}

/// Returns a new parser tries to parse the bytes `tag` from the beginning of the input. If it
/// succeeds, it returns `tag`. Otherwise, it returns the input that it was given.
///
/// For example, `tag(b"abc").parse(b"abcdef")` parses and returns the bytes `b"abc"`, leaving
/// remaining input b"def". But `tag(b"abc").parse(b"def")` returns an error.
pub fn tag<'a, 'b: 'a>(tag: &'a [u8]) -> impl Parser<'b, Out = &'a [u8]> {
    move |input: Bytes<'b>| {
        let (head, tail) = expect_next_slice(input, tag.len())?;
        if tag == head {
            Ok((head, tail))
        } else {
            Err(input)
        }
    }
}

/// Common functionality for `take_while1` and `take_until1`.
fn take_helper1<'a>(input: Bytes<'a>, pred: impl Fn(u8) -> bool) -> ParseResult<'a, &'a [u8]> {
    let (toks, rest) = input.take_while(pred);
    if rest.offset() > input.offset() {
        Ok((toks, rest))
    } else {
        Err(input)
    }
}

/// Returns a new parser that parses tokens as long as they ARE contained in `set` and returns them
/// as a slice.
///
/// Should return an error if it can't parse at least one token!
pub fn take_while1<'a>(set: impl TokSet) -> impl Parser<'a, Out = &'a [u8]> {
    move |input: Bytes<'a>| {
        // ========== TODO: YOUR CODE GOES HERE ==========
        todo!()
        // ===============================================
    }
}

/// Returns a new parser that parses tokens as long as they ARE NOT contained in `set` and returns
/// them as a slice.
///
/// Should should return an error if it can't parse at least one token.
pub fn take_until1<'a>(set: impl TokSet) -> impl Parser<'a, Out = &'a [u8]> {
    move |input: Bytes<'a>| {
        // ========== TODO: YOUR CODE GOES HERE ==========
        todo!()
        // ===============================================
    }
}

/// Similar to `take_while1`, but returns an empty slice if it can't parse at least one token
/// instead of an error.
pub fn take_while0<'a>(set: impl TokSet) -> impl Parser<'a, Out = &'a [u8]> {
    move |input: Bytes<'a>| {
        // ========== TODO: YOUR CODE GOES HERE ==========
        todo!()
        // ===============================================
    }
}

/// Returns a new parser that NEVER FAILS and returns the output of `parser` if `parser` succeeds
/// and `None` if `parser` fails.
///
/// For example, `opt(tag(b"abc")).parse(b"abcdef")` parses and returns `Some(b"abc")`. But,
/// `opt(tag(b"abc")).parse(b"def")` parses and returns `None`.
pub fn opt<'a, O>(parser: impl Parser<'a, Out = O>) -> impl Parser<'a, Out = Option<O>> {
    move |input: Bytes<'a>| {
        // ========== TODO: YOUR CODE GOES HERE ==========
        todo!()
        // ===============================================
    }
}

/// Returns a new parser that parses `open`, then `parser`, then `close` and returns the output of
/// `parser`. `delimited` should return an error if any of the three parsers fail.
pub fn delimited<'a, O1, O2, O3>(
    open: impl Parser<'a, Out = O1>,
    parser: impl Parser<'a, Out = O2>,
    close: impl Parser<'a, Out = O3>,
) -> impl Parser<'a, Out = O2> {
    move |input: Bytes<'a>| {
        // ========== TODO: YOUR CODE GOES HERE ==========
        todo!()
        // ===============================================
    }
}

/// A collection that can be used to accumulate results during parser.
pub trait Accumulator: Sized {
    type Item;
    fn empty() -> Self;
    fn push(&mut self, item: Self::Item);
}

impl<A: Array> Accumulator for SmallVec<A> {
    type Item = A::Item;

    fn empty() -> Self {
        SmallVec::new()
    }

    fn push(&mut self, item: Self::Item) {
        SmallVec::push(self, item);
    }
}

impl<T> Accumulator for Vec<T> {
    type Item = T;

    fn empty() -> Self {
        Vec::new()
    }

    fn push(&mut self, item: Self::Item) {
        Vec::push(self, item);
    }
}

/// A dictionary (`IndexMap`) is an accumulator for key-value pairs.
impl<K: Hash + Eq, V> Accumulator for IndexMap<K, V> {
    type Item = (K, V);

    fn empty() -> Self {
        IndexMap::new()
    }

    fn push(&mut self, item: Self::Item) {
        self.insert(item.0, item.1);
    }
}

/// Returns a new parser that parses `parser` ZERO OR MORE times and returns a collection of the
/// results.
///
/// The collection type is determined by the caller and can be any type that implements
/// `Acculumator`. For example, if `parser` parses a tuple of a string and an integer, then the
/// following will return a dictionary mapping strings to integers:
///
/// `repeat::<_, (&str, i32), IndexMap<&str, i32>>(parser)`
pub fn repeat<'a, O, A>(parser: impl Parser<'a, Out = O>) -> impl Parser<'a, Out = A>
where
    A: Accumulator<Item = O>,
{
    move |mut input: Bytes<'a>| {
        let mut acc = A::empty();
        while let Ok((item, rest)) = parser.parse(input) {
            acc.push(item);
            input = rest;
        }
        Ok((acc, input))
    }
}

/// Returns a new parser that parses a sequence of `parser` separated by `sep`. The input may
/// consist of only a `sep` and may having a single trailing `sep`.
///
/// For example, we can use `separated` to parse a comma-separated list of items like "1" or "1,2,3"
/// or "1,2,3,".
pub fn separated<'a, O1, O2, A>(
    item: impl Parser<'a, Out = O1>,
    sep: impl Parser<'a, Out = O2>,
) -> impl Parser<'a, Out = A>
where
    A: Accumulator<Item = O1>,
{
    move |input: Bytes<'a>| {
        // ========== TODO: YOUR CODE GOES HERE ==========
        todo!()
        // ===============================================
    }
}
