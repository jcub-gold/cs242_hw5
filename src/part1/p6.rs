// TASK: Change ONLY THE TYPE OF `x` in the struct `S` so that `choose` and `call_choose` type check.
//
// Key concepts:
// - lifetime annotations: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-function-signatures
// - lifetime 'static: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-static-lifetime
// - lifetime subtyping: https://doc.rust-lang.org/nomicon/subtyping.html
// - lifetime elision: https://doc.rust-lang.org/nomicon/lifetime-elision.html
//
// HINT: invariance

pub struct S<'a, T> {
    pub x: &'a mut T,
}

pub fn choose<'a, 'b>(b: bool, x: S<'a, &'b i32>, y: S<'a, &'b i32>) -> S<'a, &'b i32> {
    if b {
        x
    } else {
        y
    }
}

fn call_choose() {
    let i = 1;
    let mut r: &i32 = &i;

    let c1 = S::<&'static i32> { x: &mut &1 };
    let c2 = S::<&i32> { x: &mut r };

    let _result = choose(true, c1, c2);
}
