use hw::part1::p3::*;

#[test]
fn test_second() {
    let result = {
        let s1 = String::from("aaa");
        let s2 = "aaaa";
        second(&s1, s2)
    };
    assert_eq!(result, "aaaa");
}
