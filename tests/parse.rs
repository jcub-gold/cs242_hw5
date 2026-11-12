use hw::part2::combinator::Bytes;
use hw::part2::parse::Value::{Array, Bool, Float, Int, Object};
use hw::part2::parse::{self as sol, Value};
use indexmap::indexmap;
use std::borrow::Cow;

// https://stackoverflow.com/questions/1301402/example-invalid-utf8-string
const INVALID_UTF8: Bytes = Bytes::new(b"\xfc\xa1\xa1\xa1\xa1\xa1");

fn b(s: &str) -> Bytes {
    Bytes::new(s.as_bytes())
}

fn sborrow(s: &str) -> Value {
    Value::String(Cow::Borrowed(s))
}

macro_rules! parse_eq {
    ($left:expr, $right:expr) => {
        let (value, _rest) = $left.unwrap();
        assert_eq!(value, $right);
    };
}

macro_rules! assert_cow_ref {
    ($e:expr) => {
        assert!(matches!($e.unwrap().0, Cow::Borrowed(_)));
    };
}

#[test]
fn test_ws() {
    let ((), rest) = sol::ws(b(" \n\r\t")).unwrap();
    assert!(rest.offset() == 4);
    let ((), rest) = sol::ws(b("\n hello\t ")).unwrap();
    assert!(rest.offset() == 2);
    let ((), rest) = sol::ws(b("")).unwrap();
    assert!(rest.offset() == 0);
    let ((), rest) = sol::ws(INVALID_UTF8).unwrap();
    assert!(rest.offset() == 0);
}

#[test]
fn test_bool_true() {
    parse_eq!(sol::parse_bool(b("true")), true);
}

#[test]
fn test_bool_false() {
    parse_eq!(sol::parse_bool(b("false")), false);
}

#[test]
fn test_bool_weird() {
    let (item, rest) = sol::parse_bool(b("truefalse")).unwrap();
    assert!(item == true && rest.offset() == 4);

    let (item, rest) = sol::parse_bool(b("falsefalse")).unwrap();
    assert!(item == false && rest.offset() == 5);
}

#[test]
fn test_bool_other() {
    assert!(sol::parse_bool(b("truE")).is_err());
    assert!(sol::parse_bool(b("other")).is_err());
    assert!(sol::parse_bool(b("0")).is_err());
    assert!(sol::parse_bool(b("0.0")).is_err());
    assert!(sol::parse_bool(b("\"true\"")).is_err());
    assert!(sol::parse_bool(b("\"false\"")).is_err());
    assert!(sol::parse_bool(b("")).is_err());
    assert!(sol::parse_bool(INVALID_UTF8).is_err());
}

#[test]
fn test_int_valid() {
    parse_eq!(sol::parse_int(b("0")), 0);
    parse_eq!(sol::parse_int(b("1")), 1);
    parse_eq!(sol::parse_int(b("-678")), -678);
}

#[test]
fn test_int_valid_hidden() {
    parse_eq!(sol::parse_int(b("678")), 678);
    parse_eq!(sol::parse_int(b("-1")), -1);
    parse_eq!(sol::parse_int(b("1237890")), 1237890);
    parse_eq!(sol::parse_int(b("-1237890")), -1237890);
}

#[test]
fn test_int_weird() {
    let (item, rest) = sol::parse_int(b("101true")).unwrap();
    assert!(item == 101 && rest.offset() == 3);
}

#[test]
fn test_int_weird_hidden() {
    let (item, rest) = sol::parse_int(b("0.0")).unwrap();
    assert!(item == 0 && rest.offset() == 1);
}

#[test]
fn test_int_invalid() {
    assert!(sol::parse_int(b("other")).is_err());
    assert!(sol::parse_int(b("\"0\"")).is_err());
}

#[test]
fn test_in_invalid_hidden() {
    assert!(sol::parse_int(b("true")).is_err());
    assert!(sol::parse_int(b("false")).is_err());
    assert!(sol::parse_int(b("inf")).is_err());
    assert!(sol::parse_int(b("")).is_err());
    assert!(sol::parse_int(INVALID_UTF8).is_err());
}

#[test]
fn test_float_valid() {
    parse_eq!(sol::parse_float(b("0.0")), 0.0);
    parse_eq!(sol::parse_float(b("1.0")), 1.0);
    parse_eq!(sol::parse_float(b("678.0")), 678.0);
    parse_eq!(sol::parse_float(b("-1.0")), -1.0);
    parse_eq!(sol::parse_float(b("-678.0")), -678.0);
    parse_eq!(sol::parse_float(b("inf")), f64::INFINITY);
    parse_eq!(sol::parse_float(b("-inf")), f64::NEG_INFINITY);
    let (value, _rest) = sol::parse_float(b("nan")).unwrap();
    assert!(value.is_nan());
}

#[test]
fn test_float_valid_hidden() {
    parse_eq!(sol::parse_float(b("1237890.0")), 1237890.0);
    parse_eq!(sol::parse_float(b("-1237890.0")), -1237890.0);
    parse_eq!(sol::parse_float(b("0.1")), 0.1);
    parse_eq!(sol::parse_float(b("1.1")), 1.1);
    parse_eq!(sol::parse_float(b("678.1")), 678.1);
    parse_eq!(sol::parse_float(b("-1.1")), -1.1);
    parse_eq!(sol::parse_float(b("-678.1")), -678.1);
    parse_eq!(sol::parse_float(b("1237890.1")), 1237890.1);
    parse_eq!(sol::parse_float(b("-1237890.1")), -1237890.1);
    parse_eq!(sol::parse_float(b("0.123")), 0.123);
    parse_eq!(sol::parse_float(b("1.123")), 1.123);
    parse_eq!(sol::parse_float(b("678.123")), 678.123);
    parse_eq!(sol::parse_float(b("-1.123")), -1.123);
    parse_eq!(sol::parse_float(b("-678.123")), -678.123);
    parse_eq!(sol::parse_float(b("1237890.123")), 1237890.123);
    parse_eq!(sol::parse_float(b("-1237890.123")), -1237890.123);
    parse_eq!(sol::parse_float(b("0.000")), 0.0);
    parse_eq!(sol::parse_float(b("1.000")), 1.0);
    parse_eq!(sol::parse_float(b("678.000")), 678.0);
    parse_eq!(sol::parse_float(b("-1.000")), -1.0);
    parse_eq!(sol::parse_float(b("-678.000")), -678.0);
    parse_eq!(sol::parse_float(b("1237890.000")), 1237890.0);
}

#[test]
fn test_float_weird() {
    let (item, rest) = sol::parse_float(b("1.2.0")).unwrap();
    assert!(item == 1.2 && rest.offset() == 3);

    let (item, rest) = sol::parse_float(b("1.22true")).unwrap();
    assert!(item == 1.22 && rest.offset() == 4);
}

#[test]
fn test_float_invalid() {
    assert!(sol::parse_float(b("other")).is_err());
    assert!(sol::parse_float(b("\"0.0\"")).is_err());
    assert!(sol::parse_float(b("0")).is_err());
    assert!(sol::parse_float(b("0.")).is_err());
}

#[test]
fn test_float_invalid_hidden() {
    assert!(sol::parse_float(b(".0")).is_err());
    assert!(sol::parse_float(b(".")).is_err());
    assert!(sol::parse_float(b("true")).is_err());
    assert!(sol::parse_float(b("false")).is_err());
    assert!(sol::parse_float(b("")).is_err());
    assert!(sol::parse_float(INVALID_UTF8).is_err());
}

#[test]
fn test_string_simple() {
    parse_eq!(sol::parse_string(b(r#""""#)), "");
    parse_eq!(sol::parse_string(b(r#""a""#)), "a");
    parse_eq!(sol::parse_string(b(r#""hello world""#)), "hello world");
}

#[test]
fn test_string_simple_cow() {
    assert_cow_ref!(sol::parse_string(b(r#""""#)));
    assert_cow_ref!(sol::parse_string(b(r#""a""#)));
    assert_cow_ref!(sol::parse_string(b(r#""hello world""#)));
}

#[test]
fn test_escape() {
    parse_eq!(sol::parse_escape(b(r#"\n"#)), "\n");
    parse_eq!(sol::parse_escape(b(r#"\r"#)), "\r");
    parse_eq!(sol::parse_escape(b(r#"\t"#)), "\t");
    parse_eq!(sol::parse_escape(b(r#"\""#)), "\"");
    parse_eq!(sol::parse_escape(b(r#"\\"#)), "\\");
}

#[test]
fn test_escape_invalid() {
    assert!(sol::parse_escape(b(r#"\a"#)).is_err());
}

#[test]
fn test_string_escape() {
    parse_eq!(sol::parse_string(b(r#""\\""#)), "\\");
    parse_eq!(sol::parse_string(b(r#""hello\nworld""#)), "hello\nworld");
}

#[test]
fn test_string_escape_hidden() {
    parse_eq!(sol::parse_string(b(r#""\n""#)), "\n");
    parse_eq!(sol::parse_string(b(r#""\r""#)), "\r");
    parse_eq!(sol::parse_string(b(r#""\t""#)), "\t");
    parse_eq!(sol::parse_string(b(r#""\"""#)), "\"");
}

#[test]
fn test_string_escape_cow() {
    assert_cow_ref!(sol::parse_string(b(r#""\n""#)));
    assert_cow_ref!(sol::parse_string(b(r#""\r""#)));
    assert_cow_ref!(sol::parse_string(b(r#""\t""#)));
    assert_cow_ref!(sol::parse_string(b(r#""\"""#)));
    assert_cow_ref!(sol::parse_string(b(r#""\\""#)));
}

#[test]
fn test_string_complex() {
    parse_eq!(sol::parse_string(b(r#""hello\nworld""#)), "hello\nworld");
    parse_eq!(
        sol::parse_string(b(r#""\\hello\"world\t""#)),
        "\\hello\"world\t"
    );
}

#[test]
fn test_string_invalid() {
    assert!(sol::parse_string(b("other")).is_err());
    assert!(sol::parse_string(b("true")).is_err());
    assert!(sol::parse_string(b("false")).is_err());

    assert!(sol::parse_string(b(r#"hello""#)).is_err()); // no opening quote
    assert!(sol::parse_string(b(r#""hello"#)).is_err()); // no closing quote
    assert!(sol::parse_string(b(r#"he\llo"#)).is_err()); // invalid escape

    let mut bad_data = Vec::new();
    bad_data.push(b'"');
    bad_data.extend_from_slice(INVALID_UTF8.data());
    bad_data.push(b'"');
    assert!(sol::parse_string(Bytes::new(&bad_data)).is_err());
}

#[test]
fn test_string_invalid_hidden() {
    assert!(sol::parse_string(b("0")).is_err());
    assert!(sol::parse_string(b("0.0")).is_err());
    assert!(sol::parse_string(b("")).is_err());
    assert!(sol::parse_string(INVALID_UTF8).is_err());
}

#[test]
fn test_parse_element() {
    parse_eq!(sol::parse_element(b("true")), Bool(true));
    parse_eq!(sol::parse_element(b("false")), Bool(false));
    parse_eq!(sol::parse_element(b("0")), Int(0));
    parse_eq!(sol::parse_element(b("0.0")), Float(0.0));
    parse_eq!(sol::parse_element(b("-inf")), Float(f64::NEG_INFINITY));
    parse_eq!(sol::parse_element(b(r#""""#)), sborrow(""));
    parse_eq!(
        sol::parse_element(b(r#""hello world""#)),
        sborrow("hello world")
    );
    parse_eq!(sol::parse_element(b(r#""\n""#)), sborrow("\n"));
    parse_eq!(
        sol::parse_element(b(r#"[0,true,"hi"]"#)),
        Array(vec![Int(0), Bool(true), sborrow("hi")])
    );
    parse_eq!(
        sol::parse_element(b(r#"{"hi":1}"#)),
        Object(indexmap! {Cow::Borrowed("hi") => Int(1)})
    );
}

#[test]
fn test_parse_element_invalid() {
    assert!(sol::parse_element(b("other")).is_err());
    assert!(sol::parse_element(b("?")).is_err());
    assert!(sol::parse_element(b("")).is_err());
    assert!(sol::parse_element(INVALID_UTF8).is_err());
}

#[test]
fn test_parse_array() {
    parse_eq!(sol::parse_array(b("[]")), vec![]);
    parse_eq!(sol::parse_array(b("[0]")), vec![Int(0)]);
    parse_eq!(sol::parse_array(b("[0,1]")), vec![Int(0), Int(1)]);
    parse_eq!(
        sol::parse_array(b(r#"[0,"hello",2]"#)),
        vec![Int(0), sborrow("hello"), Int(2)]
    );
}

#[test]
fn test_parse_array_trailing() {
    parse_eq!(sol::parse_array(b("[,]")), vec![]);
    parse_eq!(sol::parse_array(b("[0,]")), vec![Int(0)]);
    parse_eq!(sol::parse_array(b("[0,1,]")), vec![Int(0), Int(1)]);
}

#[test]
fn test_parse_array_trailing_hidden() {
    parse_eq!(
        sol::parse_array(b(r#"[0,"hello",2,]"#)),
        vec![Int(0), sborrow("hello"), Int(2)]
    );
}

#[test]
fn test_parse_array_invalid() {
    assert!(sol::parse_array(b("other")).is_err());
    assert!(sol::parse_array(b("[0")).is_err());
    assert!(sol::parse_array(b("[0,")).is_err());
    assert!(sol::parse_array(b("[0,1")).is_err());
    assert!(sol::parse_array(b("0]")).is_err());
    assert!(sol::parse_array(b("0,]")).is_err());
    assert!(sol::parse_array(b(",0")).is_err());
    assert!(sol::parse_array(b("[,0]")).is_err());
}

#[test]
fn test_parse_array_ws() {
    parse_eq!(
        sol::parse_array(b("[        0,\"hello\",        2]")),
        vec![Int(0), sborrow("hello"), Int(2)]
    );
    parse_eq!(
        sol::parse_array(b("[0,\n\"hello\"    ,        2]")),
        vec![Int(0), sborrow("hello"), Int(2)]
    );
    parse_eq!(
        sol::parse_array(b("[        0,    \"hello\",        2,    ]")),
        vec![Int(0), sborrow("hello"), Int(2)]
    );
}

#[test]
fn test_member() {
    let input = "\"\n\":[0,true]";
    let (item, rest) = sol::parse_member(b(input)).unwrap();
    assert!(item.0 == "\n" && item.1 == Array(vec![Int(0), Bool(true)]));
    assert!(rest.offset() == input.len());
}

#[test]
fn test_member_hidden() {
    let input = "\"hello\nworld\":1";
    let (item, rest) = sol::parse_member(b(input)).unwrap();
    assert!(item.0 == "hello\nworld" && item.1 == Int(1));
    assert!(rest.offset() == input.len());
}

#[test]
fn test_member_ws() {
    let input = "\"hello\nworld\"        :1";
    let (item, rest) = sol::parse_member(b(input)).unwrap();
    assert!(item.0 == "hello\nworld" && item.1 == Int(1));
    assert!(rest.offset() == input.len());

    let input = "\"\n\"\n\n\n:\t[0,true]";
    let (item, rest) = sol::parse_member(b(input)).unwrap();
    assert!(item.0 == "\n" && item.1 == Array(vec![Int(0), Bool(true)]));
    assert!(rest.offset() == input.len());
}

#[test]
fn test_member_invalid() {
    assert!(sol::parse_member(b("other")).is_err());
    assert!(sol::parse_member(b("")).is_err());
    assert!(sol::parse_member(INVALID_UTF8).is_err());
}

#[test]
fn test_parse_object() {
    parse_eq!(sol::parse_object(b("{}")), indexmap! {});
    parse_eq!(
        sol::parse_object(b(r#"{"hi":0}"#)),
        indexmap! {Cow::Borrowed("hi") => Int(0)}
    );
    parse_eq!(
        sol::parse_object(b(r#"{"0":0,"1":1}"#)),
        indexmap! {
            Cow::Borrowed("0") => Int(0),
            Cow::Borrowed("1") => Int(1),
        }
    );
    parse_eq!(
        sol::parse_object(b(r#"{"0":0,"1":"hello","2":2}"#)),
        indexmap! {
            Cow::Borrowed("0") => Int(0),
            Cow::Borrowed("1") => sborrow("hello"),
            Cow::Borrowed("2") => Int(2),
        }
    );
}

#[test]
fn test_parse_object_trailing() {
    parse_eq!(sol::parse_object(b("{,}")), indexmap! {});
    parse_eq!(
        sol::parse_object(b(r#"{"hi":0,}"#)),
        indexmap! {Cow::Borrowed("hi") => Int(0)}
    );
    parse_eq!(
        sol::parse_object(b(r#"{"0":0,"1":1,}"#)),
        indexmap! {
            Cow::Borrowed("0") => Int(0),
            Cow::Borrowed("1") => Int(1),
        }
    );
    parse_eq!(
        sol::parse_object(b(r#"{"0":0,"1":"hello","2":2,}"#)),
        indexmap! {
            Cow::Borrowed("0") => Int(0),
            Cow::Borrowed("1") => sborrow("hello"),
            Cow::Borrowed("2") => Int(2),
        }
    );
}

#[test]
fn test_parse_object_invalid() {
    assert!(sol::parse_object(b("other")).is_err());
    assert!(sol::parse_object(b(r#"{"0":0"#)).is_err());
    assert!(sol::parse_object(b(r#"{"0":0,"#)).is_err());
    assert!(sol::parse_object(b(r#"{"0":0,"1":1"#)).is_err());
    assert!(sol::parse_object(b(r#""0":0}"#)).is_err());
    assert!(sol::parse_object(b(r#""0":0,}"#)).is_err());
    assert!(sol::parse_object(b(r#"{,"0":0}"#)).is_err());
}

#[test]
fn test_parse_object_invalid_hidden() {
    assert!(sol::parse_object(b(r#","0":0"#)).is_err());
}

#[test]
fn test_parse_object_ws() {
    parse_eq!(
        sol::parse_object(b("{        \"0\":0,\"1\":\"hello\",\"2\":2        }")),
        indexmap! {
            Cow::Borrowed("0") => Int(0),
            Cow::Borrowed("1") => sborrow("hello"),
            Cow::Borrowed("2") => Int(2),
        }
    );
    parse_eq!(
        sol::parse_object(b("{\"0\":0    ,    \"1\":\"hello\",\"2\":2\n\n}")),
        indexmap! {
            Cow::Borrowed("0") => Int(0),
            Cow::Borrowed("1") => sborrow("hello"),
            Cow::Borrowed("2") => Int(2),
        }
    );
}

#[test]
fn test_parse_object_ws_hidden() {
    parse_eq!(
        sol::parse_object(b("{\"0\":0,\"1\":\"hello\",\t\n\"2\":2}")),
        indexmap! {
            Cow::Borrowed("0") => Int(0),
            Cow::Borrowed("1") => sborrow("hello"),
            Cow::Borrowed("2") => Int(2),
        }
    );
}
