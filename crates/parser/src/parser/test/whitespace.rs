use super::*;

#[test]
fn empty() {
    assert_eq!(lang::empty("/* abc */   // xy\n "), Ok(()));
    assert_eq!(lang::empty(""), Ok(()));
}

#[test]
fn comment() {
    assert_eq!(lang::comment("/* abc */"), Ok(()));
    assert_eq!(lang::comment("// abc\n"), Ok(()));
}
