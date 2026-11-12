// TASK: Fill in the signature of function `shortest` so that the test passes.
// It might not be necessary to fill in every TODO.
//
// Key concepts:
// - lifetime annotations: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-function-signatures
// - lifetime 'static: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-static-lifetime
// - lifetime subtyping: https://doc.rust-lang.org/nomicon/subtyping.html
// - lifetime elision: https://doc.rust-lang.org/nomicon/lifetime-elision.html

#[rustfmt::skip]
pub fn shortest< /* TODO */ >(x: &/* TODO */ str, y: &/* TODO */ str) -> &/* TODO */ str {
    if x.len() < y.len() {
        x
    } else {
        y
    }
}

#[test]
fn test_shortest() {
    let s1 = String::from("aaa");
    let s2 = "aaaa";

    let result = shortest(&s1, s2);
    assert_eq!(result, "aaa");
}
