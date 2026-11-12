use super::combinator::{Alt, Bytes, ParseResult, Parser, TokSet};

////////////////////////////////////////////////////////////
/// You are not expected to understand this file.
////////////////////////////////////////////////////////////

impl<'a, O, F> Parser<'a> for F
where
    F: Fn(Bytes<'a>) -> ParseResult<'a, O> + Clone,
{
    type Out = O;

    fn parse(&self, input: Bytes<'a>) -> ParseResult<'a, O> {
        self(input)
    }
}

#[derive(Clone)]
pub struct Map<P, F> {
    parser: P,
    func: F,
}

impl<P, F> Map<P, F> {
    pub fn new(parser: P, func: F) -> Self {
        Self { parser, func }
    }
}

impl<'a, O1, O2, P, F> Parser<'a> for Map<P, F>
where
    P: Parser<'a, Out = O1>,
    F: Fn(O1) -> O2 + Clone,
{
    type Out = O2;

    // This is the actual implementation of `Parser::map`.
    fn parse(&self, input: Bytes<'a>) -> ParseResult<'a, O2> {
        self.parser
            .parse(input)
            .map(|(output, rest)| ((self.func)(output), rest))
    }
}

#[derive(Clone)]
pub struct AndThen<P, F> {
    parser: P,
    func: F,
}

impl<P, F> AndThen<P, F> {
    pub fn new(parser: P, func: F) -> Self {
        Self { parser, func }
    }
}

impl<'a, O1, O2, P, F> Parser<'a> for AndThen<P, F>
where
    P: Parser<'a, Out = O1>,
    F: Fn(O1) -> Option<O2> + Clone,
{
    type Out = O2;

    // This is the actual implementation of `Parser::and_then`.
    fn parse(&self, input: Bytes<'a>) -> ParseResult<'a, O2> {
        let (output, rest) = self.parser.parse(input)?;
        match (self.func)(output) {
            Some(output) => Ok((output, rest)),
            None => Err(input),
        }
    }
}

macro_rules! impl_seq {
    ($($p:ident, $o:ident),*) => {
        impl<'a, $($o,)* $($p: Parser<'a, Out = $o>),*> Parser<'a> for ($($p,)*) {
            type Out = ($($o,)*);

            #[allow(non_snake_case)]
            fn parse(&self, input: Bytes<'a>) -> ParseResult<'a, ($($o,)*)> {
                let ($($p,)*) = self;
                let rest = input;
                $(
                    let ($p, rest) = match $p.parse(rest) {
                        Ok((item, rest)) => {
                            (item, rest)
                        }
                        Err(_) => return Err(input),
                    };
                )*
                Ok((($($p,)*), rest))
            }
        }
    };
}

impl_seq!();
impl_seq!(P1, O1);
impl_seq!(P1, O1, P2, O2);
impl_seq!(P1, O1, P2, O2, P3, O3);
impl_seq!(P1, O1, P2, O2, P3, O3, P4, O4);
impl_seq!(P1, O1, P2, O2, P3, O3, P4, O4, P5, O5);
impl_seq!(P1, O1, P2, O2, P3, O3, P4, O4, P5, O5, P6, O6);
impl_seq!(P1, O1, P2, O2, P3, O3, P4, O4, P5, O5, P6, O6, P7, O7);
impl_seq!(P1, O1, P2, O2, P3, O3, P4, O4, P5, O5, P6, O6, P7, O7, P8, O8);

macro_rules! impl_alt {
    ($($p:ident),*) => {
        impl<'a, O, $($p: Parser<'a, Out = O>),*> Alt<'a, O> for ($($p,)*) {
            #[allow(non_snake_case)]
            fn alt(&self, input: Bytes<'a>) -> ParseResult<'a, O> {
                let ($($p,)*) = self;
                $(
                    if let Ok(($p, rest)) = $p.parse(input) {
                        return Ok(($p, rest));
                    }
                )*
                Err(input)
            }
        }
    };
}

impl_alt!();
impl_alt!(P1);
impl_alt!(P1, P2);
impl_alt!(P1, P2, P3);
impl_alt!(P1, P2, P3, P4);
impl_alt!(P1, P2, P3, P4, P5);
impl_alt!(P1, P2, P3, P4, P5, P6);
impl_alt!(P1, P2, P3, P4, P5, P6, P7);
impl_alt!(P1, P2, P3, P4, P5, P6, P7, P8);

macro_rules! impl_set {
    ($($s:ident),*) => {
        impl<$($s: TokSet,)*> TokSet for ($($s,)*) {
            #[allow(non_snake_case, unused_variables)]
            fn contains(&self, item: u8) -> bool {
                let ($($s,)*) = self;
                $(if $s.contains(item) { return true })*
                false
            }
        }
    };
}

impl_set!();
impl_set!(S1);
impl_set!(S1, S2);
impl_set!(S1, S2, S3);
impl_set!(S1, S2, S3, S4);
impl_set!(S1, S2, S3, S4, S5);
impl_set!(S1, S2, S3, S4, S5, S6);
impl_set!(S1, S2, S3, S4, S5, S6, S7);
impl_set!(S1, S2, S3, S4, S5, S6, S7, S8);
