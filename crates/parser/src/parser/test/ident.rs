use super::*;

#[test]
fn ident() {
    assert_eq!(lang::ident("abc"), Ok(Ident("abc".to_string())));
    assert_eq!(lang::ident("_"), Ok(Ident("_".to_string())));
    assert_eq!(lang::ident("p1"), Ok(Ident("p1".to_string())));
}
