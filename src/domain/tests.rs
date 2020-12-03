

#[test]
fn test_format_str() {
    let src = "åäö";
    let expect = "aao";
    assert_eq!(expect, unidecode::unidecode(src));

}