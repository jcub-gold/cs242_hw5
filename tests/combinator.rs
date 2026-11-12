use hw::part2::combinator::{self as sol, Accumulator, Parser, TokSet};
use indexmap::IndexMap;
use smallvec::SmallVec;

fn b(s: &str) -> sol::Bytes {
    sol::Bytes::new(s.as_bytes())
}

#[test]
fn test_bytes_next() {
    let bytes0 = b("abc");
    assert!(bytes0.offset() == 0);

    let (item, bytes1) = bytes0.next().unwrap();
    assert!(item == b'a' && bytes1.offset() == 1);
    let (item, bytes2) = bytes1.next().unwrap();
    assert!(item == b'b' && bytes2.offset() == 2);
    let (item, bytes3) = bytes2.next().unwrap();
    assert!(item == b'c' && bytes3.offset() == 3);

    assert!(bytes3.next().is_none());

    let (item, bytes2) = bytes1.next().unwrap();
    assert!(item == b'b' && bytes2.offset() == 2);
    let (item, bytes3) = bytes2.next().unwrap();
    assert!(item == b'c' && bytes3.offset() == 3);

    assert!(bytes3.next().is_none());
}

#[test]
fn test_bytes_next_slice() {
    let bytes0 = b("abcdef");
    assert!(bytes0.offset() == 0);

    let (slice, bytes2) = bytes0.next_slice(2).unwrap();
    assert!(slice == b"ab" && bytes2.offset() == 2);
    let (slice, bytes6) = bytes2.next_slice(4).unwrap();
    assert!(slice == b"cdef" && bytes6.offset() == 6);

    assert!(bytes6.next_slice(1).is_none());
    assert!(bytes6.next_slice(100).is_none());
    assert!(bytes6.next().is_none());
    assert!(bytes6.next().is_none());

    assert!(bytes2.next_slice(5).is_none());
    let (item, bytes3) = bytes2.next().unwrap();
    assert!(item == b'c' && bytes3.offset() == 3);
    let (item, bytes4) = bytes3.next().unwrap();
    assert!(item == b'd' && bytes4.offset() == 4);
    let (item, bytes5) = bytes4.next().unwrap();
    assert!(item == b'e' && bytes5.offset() == 5);
    let (item, bytes6) = bytes5.next().unwrap();
    assert!(item == b'f' && bytes6.offset() == 6);

    assert!(bytes6.next_slice(1).is_none());
    assert!(bytes6.next_slice(100).is_none());
    assert!(bytes6.next().is_none());
    assert!(bytes6.next().is_none());
}

#[test]
fn test_bytes_take_while() {
    let pred = |b: u8| b.is_ascii_alphabetic();

    let bytes0 = b("abc123");
    assert!(bytes0.offset() == 0);
    let (slice, bytes3) = bytes0.take_while(pred);
    assert!(slice == b"abc" && bytes3.offset() == 3);
    let (item, bytes4) = bytes3.next().unwrap();
    assert!(item == b'1' && bytes4.offset() == 4);
    let (item, bytes5) = bytes4.next().unwrap();
    assert!(item == b'2' && bytes5.offset() == 5);
    let (item, bytes6) = bytes5.next().unwrap();
    assert!(item == b'3' && bytes6.offset() == 6);
}

#[test]
fn test_bytes_take_while_edge_cases() {
    let pred = |b: u8| b.is_ascii_alphabetic();

    let bytes = b("abc");
    assert!(bytes.offset() == 0);
    let (slice, rest) = bytes.take_while(pred);
    assert!(slice == b"abc" && rest.offset() == 3);
    assert!(rest.next().is_none());

    let bytes = b("123");
    assert!(bytes.offset() == 0);
    let (slice, rest) = bytes.take_while(pred);
    assert!(slice.is_empty() && rest.offset() == 0);
    let (item, rest) = rest.next().unwrap();
    assert!(item == b'1' && rest.offset() == 1);

    let bytes = b("");
    assert!(bytes.offset() == 0);
    let (slice, rest) = bytes.take_while(pred);
    assert!(slice.is_empty() && rest.offset() == 0);
}

fn expect_next<'a>(input: sol::Bytes<'a>) -> sol::ParseResult<'a, u8> {
    match input.next() {
        Some((tok, rest)) => Ok((tok, rest)),
        None => Err(input),
    }
}

#[test]
fn test_parser_func() {
    let (item, rest) = expect_next.parse(b("abc")).unwrap();
    assert!(item == b'a' && rest.offset() == 1);
    assert!(expect_next.parse(b("")).is_err());
}

#[test]
fn test_parser_map() {
    let parser = expect_next.map(|b| b + 1);
    let (item, rest) = parser.parse(b("aaa")).unwrap();
    assert!(item == b'b' && rest.offset() == 1);
    assert!(parser.parse(b("")).is_err());
}

#[test]
fn test_parser_and_then() {
    let parser = expect_next.and_then(|b| {
        if b.is_ascii_alphabetic() {
            Some(b + 1)
        } else {
            None
        }
    });
    let (item, rest) = parser.parse(b("aaa")).unwrap();
    assert!(item == b'b' && rest.offset() == 1);
    assert!(parser.parse(b("123")).unwrap_err().offset() == 0);
    assert!(parser.parse(b("")).unwrap_err().offset() == 0);
}

#[test]
fn test_parser_seq() {
    let parser = expect_next.and_then(|b| {
        if b.is_ascii_alphabetic() {
            Some(b + 1)
        } else {
            None
        }
    });

    let ((item1, item2), rest) = (parser.clone(), parser.clone()).parse(b("ab1")).unwrap();
    assert!(item1 == b'b');
    assert!(item2 == b'c');
    assert!(rest.offset() == 2);

    let result = (
        parser.clone(),
        parser.clone(),
        parser.clone(),
        parser.clone(),
    )
        .parse(b("abc"));
    assert!(result.unwrap_err().offset() == 0)
}

#[test]
fn test_parser_alt() {
    let parser1 = expect_next.and_then(|b| {
        if b.is_ascii_alphabetic() {
            Some(b + 1)
        } else {
            None
        }
    });
    let parser2 = expect_next.and_then(|b| {
        if b.is_ascii_digit() {
            Some(b + 1)
        } else {
            None
        }
    });

    let (item, _rest) = sol::alt((parser1.clone(), parser1.clone(), parser1.clone()))
        .parse(b("abc"))
        .unwrap();
    assert!(item == b'b');

    let (item, _rest) = sol::alt((parser2.clone(), parser1.clone(), parser2.clone()))
        .parse(b("abc"))
        .unwrap();
    assert!(item == b'b');

    let result = sol::alt((parser2.clone(), parser2.clone(), parser2.clone())).parse(b("abc"));
    assert!(result.unwrap_err().offset() == 0)
}

#[test]
fn test_set_contains() {
    let set = b'a';
    assert!(set.contains(b'a'));
    assert!(!set.contains(b'b'));

    let set = b'a'..=b'z';
    assert!(set.contains(&b'a'));
    assert!(set.contains(&b'w'));
    assert!(set.contains(&b'z'));
    assert!(!set.contains(&b'A'));
    assert!(!set.contains(&b'0'));

    let set = (b'a'..=b'z', b'A'..=b'Z');
    assert!(set.contains(b'g'));
    assert!(set.contains(b'H'));
    assert!(!set.contains(b'0'));
}

#[test]
fn test_one_of() {
    let set = b'a';
    assert!(sol::one_of(set).parse(b("a")).is_ok());
    assert!(sol::one_of(set).parse(b("b")).is_err());

    let set = b'a'..=b'z';
    assert!(sol::one_of(set.clone()).parse(b("a")).is_ok());
    assert!(sol::one_of(set.clone()).parse(b("w")).is_ok());
    assert!(sol::one_of(set.clone()).parse(b("z")).is_ok());
    assert!(sol::one_of(set.clone()).parse(b("A")).is_err());
    assert!(sol::one_of(set.clone()).parse(b("0")).is_err());

    let set = (b'a'..=b'z', b'A'..=b'Z');
    assert!(sol::one_of(set.clone()).parse(b("g")).is_ok());
    assert!(sol::one_of(set.clone()).parse(b("H")).is_ok());
    assert!(sol::one_of(set.clone()).parse(b("0")).is_err());
}

#[test]
fn test_tag() {
    assert!(sol::tag(b"abc").parse(b("abc")).unwrap().1.offset() == 3);
    assert!(sol::tag(b"abc").parse(b("abcd")).unwrap().1.offset() == 3);
    assert!(sol::tag(b"abc").parse(b("ba")).unwrap_err().offset() == 0);
    assert!(sol::tag(b"abc").parse(b("")).unwrap_err().offset() == 0);
}

#[test]
fn test_take_while1() {
    let alpha = b'a'..=b'z';
    let (slice, rest) = sol::take_while1(alpha.clone()).parse(b("abc")).unwrap();
    assert!(slice == b"abc" && rest.offset() == 3);
    let (slice, rest) = sol::take_while1(alpha.clone()).parse(b("abc123")).unwrap();
    assert!(slice == b"abc" && rest.offset() == 3);
    assert!(
        sol::take_while1(alpha.clone())
            .parse(b("123"))
            .unwrap_err()
            .offset()
            == 0
    );
    assert!(
        sol::take_while1(alpha.clone())
            .parse(b(""))
            .unwrap_err()
            .offset()
            == 0
    );
}

#[test]
fn test_take_until1() {
    let alpha = b'a'..=b'z';
    let (slice, rest) = sol::take_until1(alpha.clone()).parse(b("123abc")).unwrap();
    assert!(slice == b"123" && rest.offset() == 3);
    let (slice, rest) = sol::take_until1(alpha.clone()).parse(b("123")).unwrap();
    assert!(slice == b"123" && rest.offset() == 3);
    assert!(
        sol::take_until1(alpha.clone())
            .parse(b("abc"))
            .unwrap_err()
            .offset()
            == 0
    );
    assert!(
        sol::take_until1(alpha.clone())
            .parse(b(""))
            .unwrap_err()
            .offset()
            == 0
    );
}

#[test]
fn test_take_while0() {
    let alpha = b'a'..=b'z';
    let (slice, rest) = sol::take_while1(alpha.clone()).parse(b("abc")).unwrap();
    assert!(slice == b"abc" && rest.offset() == 3);
    let (slice, rest) = sol::take_while1(alpha.clone()).parse(b("abc123")).unwrap();
    assert!(slice == b"abc" && rest.offset() == 3);
    let (slice, rest) = sol::take_while0(alpha.clone()).parse(b("123")).unwrap();
    assert!(slice.is_empty() && rest.offset() == 0);
    let (slice, rest) = sol::take_while0(alpha.clone()).parse(b("")).unwrap();
    assert!(slice.is_empty() && rest.offset() == 0);
}

#[test]
fn test_opt() {
    let parser = expect_next.and_then(|b| {
        if b.is_ascii_alphabetic() {
            Some(b + 1)
        } else {
            None
        }
    });

    let (item, rest) = sol::opt(parser.clone()).parse(b("abc")).unwrap();
    assert!(item == Some(b'b'));
    assert!(rest.offset() == 1);

    let (item, rest) = sol::opt(parser.clone()).parse(b("123")).unwrap();
    assert!(item == None);
    assert!(rest.offset() == 0);

    let (item, rest) = sol::opt(parser.clone()).parse(b("")).unwrap();
    assert!(item == None);
    assert!(rest.offset() == 0);
}

#[test]
fn test_delimited() {
    let inner = expect_next.and_then(|b| {
        if b.is_ascii_alphabetic() {
            Some(b + 1)
        } else {
            None
        }
    });
    let parser = sol::delimited(sol::one_of(b'('), inner, sol::one_of(b')'));
    let (item, rest) = parser.parse(b("(a)")).unwrap();
    assert!(item == b'b' && rest.offset() == 3);
    assert!(parser.parse(b("(aa)")).unwrap_err().offset() == 0);
    assert!(parser.parse(b("(0)")).unwrap_err().offset() == 0);
    assert!(parser.parse(b("(a")).unwrap_err().offset() == 0);
    assert!(parser.parse(b("a)")).unwrap_err().offset() == 0);
    assert!(parser.parse(b("")).unwrap_err().offset() == 0);
}

#[test]
fn test_accumulator() {
    let mut v1: SmallVec<[_; 4]> = Accumulator::empty();
    let mut v2: SmallVec<[_; 4]> = SmallVec::new();
    assert!(v1 == v2);
    Accumulator::push(&mut v1, 1);
    SmallVec::push(&mut v2, 1);
    assert!(v1 == v2);
    Accumulator::push(&mut v1, 2);
    SmallVec::push(&mut v2, 2);
    assert!(v1 == v2);

    let mut v1: Vec<_> = Accumulator::empty();
    let mut v2: Vec<_> = Vec::new();
    assert!(v1 == v2);
    Accumulator::push(&mut v1, 1);
    Vec::push(&mut v2, 1);
    assert!(v1 == v2);
    Accumulator::push(&mut v1, 2);
    Vec::push(&mut v2, 2);
    assert!(v1 == v2);

    let mut m1: IndexMap<_, _> = Accumulator::empty();
    let mut m2: IndexMap<_, _> = IndexMap::new();
    assert!(m1 == m2);
    Accumulator::push(&mut m1, (1, 1));
    IndexMap::insert(&mut m2, 1, 1);
    assert!(m1 == m2);
    Accumulator::push(&mut m1, (2, 2));
    IndexMap::insert(&mut m2, 2, 2);
    assert!(m1 == m2);
}

#[test]
fn test_repeat0() {
    let parser = expect_next.and_then(|b| {
        if b.is_ascii_alphabetic() {
            Some(b + 1)
        } else {
            None
        }
    });
    let (items, rest): (Vec<_>, _) = sol::repeat(parser.clone()).parse(b("abc")).unwrap();
    assert!(items == b"bcd" && rest.offset() == 3);
    let (items, rest): (Vec<_>, _) = sol::repeat(parser.clone()).parse(b("abc123")).unwrap();
    assert!(items == b"bcd" && rest.offset() == 3);
    let (items, rest): (Vec<_>, _) = sol::repeat(parser.clone()).parse(b("123")).unwrap();
    assert!(items.is_empty() && rest.offset() == 0);
    let (items, rest): (Vec<_>, _) = sol::repeat(parser.clone()).parse(b("")).unwrap();
    assert!(items.is_empty() && rest.offset() == 0);
}

#[test]
fn test_separated1() {
    let inner = expect_next.and_then(|b| {
        if b.is_ascii_alphabetic() {
            Some(b + 1)
        } else {
            None
        }
    });
    let parser = sol::separated(inner, sol::one_of(b','));

    let (items, rest): (Vec<_>, _) = parser.parse(b("a,b,c")).unwrap();
    assert!(items == b"bcd" && rest.offset() == 5);
    let (items, rest): (Vec<_>, _) = parser.parse(b("a,b,c,")).unwrap();
    assert!(items == b"bcd" && rest.offset() == 6);
    let (items, rest): (Vec<_>, _) = parser.parse(b("a")).unwrap();
    assert!(items == b"b" && rest.offset() == 1);
    let (items, rest): (Vec<_>, _) = parser.parse(b("a,")).unwrap();
    assert!(items == b"b" && rest.offset() == 2);
    let (items, rest) = parser.parse(b(",")).unwrap();
    assert!(items.is_empty() && rest.offset() == 1);
    let (items, rest) = parser.parse(b("")).unwrap();
    assert!(items.is_empty() && rest.offset() == 0);

    let (items, rest): (Vec<_>, _) = parser.parse(b("a,b,c,,")).unwrap();
    assert!(items == b"bcd" && rest.offset() == 6);
    let (items, rest) = parser.parse(b("a,,b")).unwrap();
    assert!(items == b"b" && rest.offset() == 2);
    let (items, rest) = parser.parse(b(",a,b")).unwrap();
    assert!(items.is_empty() && rest.offset() == 1);
    let (items, rest) = parser.parse(b("1,2,3")).unwrap();
    assert!(items.is_empty() && rest.offset() == 0);
}
