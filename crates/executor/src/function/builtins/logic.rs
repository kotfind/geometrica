use super::*;

pub(super) fn populate(builtins: &mut FuncMap) {
    simple_builtin!(INTO builtins INSERT
        // Or
        fn "#or" (lhs: Bool, rhs: Bool) -> Bool { lhs || rhs }
        // And
        fn "#and" (lhs: Bool, rhs: Bool) -> Bool { lhs && rhs }
        // Not
        fn "#not" (v: Bool) -> Bool { !v }
    );
}

#[cfg(test)]
mod test {

    use crate::cexpr::eval::eval;

    #[test]
    fn or() {
        assert_eq!(eval("true | true"), true.into());
        assert_eq!(eval("true | false"), true.into());
        assert_eq!(eval("false | true"), true.into());
        assert_eq!(eval("false | false"), false.into());
    }

    #[test]
    fn and() {
        assert_eq!(eval("true & true"), true.into());
        assert_eq!(eval("true & false"), false.into());
        assert_eq!(eval("false & true"), false.into());
        assert_eq!(eval("false & false"), false.into());
    }

    #[test]
    fn not() {
        assert_eq!(eval("!true"), false.into());
        assert_eq!(eval("!false"), true.into());
    }
}
