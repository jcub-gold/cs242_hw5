use hw::part1::p2::*;

#[test]
fn test_shortest() {
    let data = String::from("aaa");
    let s1 = S { data: &data };
    let s2 = S { data: "aaaa" };

    let result = shortest(s1, s2);
    assert_eq!(result, "aaa");
}
