#[cfg(test)]
mod tests {
    use crate::parse_suite;

    #[test]
    fn test_assign_name() {
        let source = "x = (1, 2, 3)";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_assign_tuple() {
        let source = "(x, y) = (1, 2, 3)";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_assign_list() {
        let source = "[x, y] = (1, 2, 3)";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_assign_attribute() {
        let source = "x.y = (1, 2, 3)";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_assign_subscript() {
        let source = "x[y] = (1, 2, 3)";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_assign_starred() {
        let source = "(x, *y) = (1, 2, 3)";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_assign_for() {
        let source = "for x in (1, 2, 3): pass";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_assign_list_comp() {
        let source = "x = [y for y in (1, 2, 3)]";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_assign_set_comp() {
        let source = "x = {y for y in (1, 2, 3)}";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_assign_with() {
        let source = "with 1 as x: pass";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_assign_named_expr() {
        let source = "if x:= 1: pass";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_ann_assign_name() {
        let source = "x: int = 1";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_aug_assign_name() {
        let source = "x += 1";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_aug_assign_attribute() {
        let source = "x.y += (1, 2, 3)";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_aug_assign_subscript() {
        let source = "x[y] += (1, 2, 3)";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_del_name() {
        let source = "del x";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_del_attribute() {
        let source = "del x.y";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }

    #[test]
    fn test_del_subscript() {
        let source = "del x[y]";
        let parse_ast = parse_suite(source);
        insta::assert_debug_snapshot!(parse_ast);
    }
}
