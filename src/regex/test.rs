use super::*;

#[test]
fn lchar() {
    let regex = parse_regex(String::from("a"));
    let expected = "LChar(a)";
    assert_eq!(format!("{:?}", regex), expected);
}
#[test]
fn concat() {
    let regex = parse_regex(String::from("abc"));
    let expected = "Concat(LChar(a), Concat(LChar(b), LChar(c)))";
    assert_eq!(format!("{:?}", regex), expected);
}
#[test]
fn alter() {
    let regex = parse_regex(String::from("a|b|c"));
    let expected = "Alter(LChar(a), Alter(LChar(b), LChar(c)))";
    assert_eq!(format!("{:?}", regex), expected);
}
#[test]
fn star() {
    let regex = parse_regex(String::from("a*"));
    let expected = "Star(LChar(a))";
    assert_eq!(format!("{:?}", regex), expected);
}
#[test]
fn compose() {
    let regex = parse_regex(String::from("a(bc*d|de)f"));
    let expected = "\
        Concat(\
            LChar(a), \
            Concat(\
                Alter(\
                    Concat(\
                        LChar(b), \
                        Concat(\
                            Star(LChar(c)), \
                            LChar(d)\
                        )\
                    ), \
                    Concat(\
                        LChar(d), \
                        LChar(e)\
                    )\
                ), \
                LChar(f)\
            )\
        )";
    assert_eq!(format!("{:?}", regex), expected);
}
