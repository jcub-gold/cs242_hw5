use hw::part1::p6::*;

#[test]
fn call_choose() {
    let i = 1;
    let mut r: &i32 = &i;

    let c1 = S::<&'static i32> { x: &mut &1 };
    let c2 = S::<&i32> { x: &mut r };

    let _result = choose(true, c1, c2);
}
