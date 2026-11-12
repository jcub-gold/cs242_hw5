use hw::part1::p4::*;

#[test]
fn test_set() {
    let s = String::from("aaaa");
    let mut r1 = s.as_str();
    let r2 = "aaa";

    set(&mut r1, r2);
    assert_eq!(r1, "aaa");
}
