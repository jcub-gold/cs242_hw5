// TASK: Fill in the signature of function `shortest` so that the test passes.
// It might not be necessary to fill in every TODO.
//
// Key concepts:
// - lifetimes in structs: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions

pub struct S<'a> {
    pub data: &'a str,
}

#[rustfmt::skip]
pub fn shortest< /* TODO */ >(x: S< /* TODO */ >, y: S< /* TODO */ >) -> &/* TODO */ str {
    if x.data.len() < y.data.len() {
        x.data
    } else {
        y.data
    }
}

#[test]
fn test_shortest() {
    let data = String::from("aaa");
    let s1 = S { data: &data };
    let s2 = S { data: "aaaa" };

    let result = shortest(s1, s2);
    assert_eq!(result, "aaa");
}
