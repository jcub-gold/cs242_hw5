use hw::part1::p5::*;

#[test]
fn test_get_str() {
    let data = String::from("aaaa");
    let result = {
        let s = S { data: &data };
        s.get_str()
    };
    assert_eq!(result, "aaaa");
}
