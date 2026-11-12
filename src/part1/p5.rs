// TASK: Fill in `GetStr` and the `impl` block so that the test passes.
// It might not be necessary to fill in every TODO.
//
// Key concepts:
// - traits: https://doc.rust-lang.org/book/ch10-02-traits.html
//
// Note that traits can be parameterized by lifetimes (or types), just like functions and structs.
// For example, in part 2 of this assignment, we rely on a `Parser` trait that is parameterized by a
// lifetime `'a`, approximately defined as follows:
//
// ```
// trait Parser<'a> {
//    type Out;
//    fn parse(&self, input: &'a str) -> Self::Out;
// }
// ```
//
// A struct that implements `Parser<'a, Out = T>` can parse a string with lifetime `'a` and produce
// an output of type `T`. `Out` is known as an associated type.

pub struct S<'a> {
    pub data: &'a str,
}

#[rustfmt::skip]
pub trait GetStr< /* TODO */ > {
    fn get_str< /* TODO */ >(&self) -> &/* TODO */ str;
}

#[rustfmt::skip]
impl< /* TODO */ > GetStr< /* TODO */ > for S< /* TODO */ > {
    fn get_str< /* TODO */ >(&self) -> &/* TODO */ str {
        self.data
    }
}

#[test]
fn test_get_str() {
    let data = String::from("aaaa");
    let result = {
        let s = S { data: &data };
        s.get_str()
    };
    assert_eq!(result, "aaaa");
}
