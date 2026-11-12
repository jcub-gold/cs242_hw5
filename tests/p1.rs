use hw::part1::p1::*;

#[test]
fn test_shortest() {
    let s1 = String::from("aaa");
    let s2 = "aaaa";

    let result = shortest(&s1, s2);
    assert_eq!(result, "aaa");
}
