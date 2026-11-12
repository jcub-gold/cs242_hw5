// TASK: Fill in the signature of function `set` so that the test passes.
// It might not be necessary to fill in every TODO.
//
// Key concepts:
// - lifetime annotations: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-function-signatures
// - lifetime 'static: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-static-lifetime
// - lifetime subtyping: https://doc.rust-lang.org/nomicon/subtyping.html
// - lifetime elision: https://doc.rust-lang.org/nomicon/lifetime-elision.html

#[rustfmt::skip]
pub fn set< /* TODO */ >(x: &/* TODO */ mut &/* TODO */ str, y: &/* TODO */ str) {
    *x = y;
}

#[test]
fn test_set() {
    let s = String::from("aaaa");
    let mut r1 = s.as_str();
    let r2 = "aaa";

    set(&mut r1, r2);
    assert_eq!(r1, "aaa");
}
