#[cfg(test)]
mod tests {
    use crate::parse_suite;

    // First we test, broadly, that various kinds of assignments are now
    // rejected by the parser. e.g., `5 = 3`, `5 += 3`, `(5): int = 3`.

    // Regression test: https://github.com/astral-sh/ruff/issues/6895
    #[test]
    fn err_literal_assignment() {
        let ast = parse_suite(r"5 = 3");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..5,
                        targets: [
                            NumberLiteral(
                                NumberLiteralExpr {
                                    range: 0..1,
                                    value: Int(
                                        5,
                                    ),
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 4..5,
                                value: Int(
                                    3,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..1,
                },
            ],
        )
        "###);
    }

    // This test previously passed before the assignment operator checking
    // above, but we include it here for good measure.
    #[test]
    fn err_assignment_expr() {
        let ast = parse_suite(r"(5 := 3)");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Expr(
                    ExprStmt {
                        range: 0..8,
                        value: NamedExpr(
                            NamedExpr {
                                range: 1..7,
                                target: NumberLiteral(
                                    NumberLiteralExpr {
                                        range: 1..2,
                                        value: Int(
                                            5,
                                        ),
                                    },
                                ),
                                value: NumberLiteral(
                                    NumberLiteralExpr {
                                        range: 6..7,
                                        value: Int(
                                            3,
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: NamedAssignmentError,
                    location: 1..2,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_literal_augment_assignment() {
        let ast = parse_suite(r"5 += 3");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                AugAssign(
                    AugAssignStmt {
                        range: 0..6,
                        target: NumberLiteral(
                            NumberLiteralExpr {
                                range: 0..1,
                                value: Int(
                                    5,
                                ),
                            },
                        ),
                        op: Add,
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 5..6,
                                value: Int(
                                    3,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AugAssignmentError,
                    location: 0..1,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_literal_annotation_assignment() {
        let ast = parse_suite(r"(5): int = 3");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                AnnAssign(
                    AnnAssignStmt {
                        range: 0..12,
                        target: NumberLiteral(
                            NumberLiteralExpr {
                                range: 1..2,
                                value: Int(
                                    5,
                                ),
                            },
                        ),
                        annotation: Name(
                            NameExpr {
                                range: 5..8,
                                id: "int",
                                ctx: Load,
                            },
                        ),
                        value: Some(
                            NumberLiteral(
                                NumberLiteralExpr {
                                    range: 11..12,
                                    value: Int(
                                        3,
                                    ),
                                },
                            ),
                        ),
                        simple: false,
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..3,
                },
            ],
        )
        "###);
    }

    // Now we exhaustively test all possible cases where assignment can fail.

    #[test]
    fn err_bool_op() {
        let ast = parse_suite(r"x or y = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..11,
                        targets: [
                            BoolOp(
                                BoolOpExpr {
                                    range: 0..6,
                                    op: Or,
                                    values: [
                                        Name(
                                            NameExpr {
                                                range: 0..1,
                                                id: "x",
                                                ctx: Load,
                                            },
                                        ),
                                        Name(
                                            NameExpr {
                                                range: 5..6,
                                                id: "y",
                                                ctx: Load,
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 9..11,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..6,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_named_expr() {
        let ast = parse_suite(r"(x := 5) = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..13,
                        targets: [
                            NamedExpr(
                                NamedExpr {
                                    range: 1..7,
                                    target: Name(
                                        NameExpr {
                                            range: 1..2,
                                            id: "x",
                                            ctx: Store,
                                        },
                                    ),
                                    value: NumberLiteral(
                                        NumberLiteralExpr {
                                            range: 6..7,
                                            value: Int(
                                                5,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 11..13,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 1..7,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_bin_op() {
        let ast = parse_suite(r"x + y = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..10,
                        targets: [
                            BinOp(
                                BinOpExpr {
                                    range: 0..5,
                                    left: Name(
                                        NameExpr {
                                            range: 0..1,
                                            id: "x",
                                            ctx: Load,
                                        },
                                    ),
                                    op: Add,
                                    right: Name(
                                        NameExpr {
                                            range: 4..5,
                                            id: "y",
                                            ctx: Load,
                                        },
                                    ),
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 8..10,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..5,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_unary_op() {
        let ast = parse_suite(r"-x = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..7,
                        targets: [
                            UnaryOp(
                                UnaryOpExpr {
                                    range: 0..2,
                                    op: USub,
                                    operand: Name(
                                        NameExpr {
                                            range: 1..2,
                                            id: "x",
                                            ctx: Store,
                                        },
                                    ),
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 5..7,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..2,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_lambda() {
        let ast = parse_suite(r"(lambda _: 1) = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..18,
                        targets: [
                            Lambda(
                                LambdaExpr {
                                    range: 1..12,
                                    parameters: Some(
                                        Parameters {
                                            range: 8..9,
                                            posonlyargs: [],
                                            args: [
                                                ParameterWithDefault {
                                                    range: 8..9,
                                                    parameter: Parameter {
                                                        range: 8..9,
                                                        name: Identifier {
                                                            id: "_",
                                                            range: 8..9,
                                                        },
                                                        annotation: None,
                                                    },
                                                    default: None,
                                                },
                                            ],
                                            vararg: None,
                                            kwonlyargs: [],
                                            kwarg: None,
                                        },
                                    ),
                                    body: NumberLiteral(
                                        NumberLiteralExpr {
                                            range: 11..12,
                                            value: Int(
                                                1,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 16..18,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 1..12,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_if_exp() {
        let ast = parse_suite(r"a if b else c = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..18,
                        targets: [
                            IfExp(
                                IfExpr {
                                    range: 0..13,
                                    test: Name(
                                        NameExpr {
                                            range: 5..6,
                                            id: "b",
                                            ctx: Load,
                                        },
                                    ),
                                    body: Name(
                                        NameExpr {
                                            range: 0..1,
                                            id: "a",
                                            ctx: Load,
                                        },
                                    ),
                                    orelse: Name(
                                        NameExpr {
                                            range: 12..13,
                                            id: "c",
                                            ctx: Load,
                                        },
                                    ),
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 16..18,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..13,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_dict() {
        let ast = parse_suite(r"{'a':5} = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..12,
                        targets: [
                            Dict(
                                DictExpr {
                                    range: 0..7,
                                    keys: [
                                        Some(
                                            StringLiteral(
                                                StringLiteralExpr {
                                                    range: 1..4,
                                                    value: StringLiteralValue {
                                                        inner: Single(
                                                            StringLiteral {
                                                                range: 1..4,
                                                                value: "a",
                                                                unicode: false,
                                                            },
                                                        ),
                                                    },
                                                },
                                            ),
                                        ),
                                    ],
                                    values: [
                                        NumberLiteral(
                                            NumberLiteralExpr {
                                                range: 5..6,
                                                value: Int(
                                                    5,
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 10..12,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..7,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_set() {
        let ast = parse_suite(r"{a} = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..8,
                        targets: [
                            Set(
                                SetExpr {
                                    range: 0..3,
                                    elts: [
                                        Name(
                                            NameExpr {
                                                range: 1..2,
                                                id: "a",
                                                ctx: Load,
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 6..8,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..3,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_list_comp() {
        let ast = parse_suite(r"[x for x in xs] = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..20,
                        targets: [
                            ListComp(
                                ListCompExpr {
                                    range: 0..15,
                                    elt: Name(
                                        NameExpr {
                                            range: 1..2,
                                            id: "x",
                                            ctx: Load,
                                        },
                                    ),
                                    generators: [
                                        Comprehension {
                                            range: 3..14,
                                            target: Name(
                                                NameExpr {
                                                    range: 7..8,
                                                    id: "x",
                                                    ctx: Store,
                                                },
                                            ),
                                            iter: Name(
                                                NameExpr {
                                                    range: 12..14,
                                                    id: "xs",
                                                    ctx: Load,
                                                },
                                            ),
                                            ifs: [],
                                            is_async: false,
                                        },
                                    ],
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 18..20,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..15,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_set_comp() {
        let ast = parse_suite(r"{x for x in xs} = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..20,
                        targets: [
                            SetComp(
                                SetCompExpr {
                                    range: 0..15,
                                    elt: Name(
                                        NameExpr {
                                            range: 1..2,
                                            id: "x",
                                            ctx: Load,
                                        },
                                    ),
                                    generators: [
                                        Comprehension {
                                            range: 3..14,
                                            target: Name(
                                                NameExpr {
                                                    range: 7..8,
                                                    id: "x",
                                                    ctx: Store,
                                                },
                                            ),
                                            iter: Name(
                                                NameExpr {
                                                    range: 12..14,
                                                    id: "xs",
                                                    ctx: Load,
                                                },
                                            ),
                                            ifs: [],
                                            is_async: false,
                                        },
                                    ],
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 18..20,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..15,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_dict_comp() {
        let ast = parse_suite(r"{x: x*2 for x in xs} = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..25,
                        targets: [
                            DictComp(
                                DictCompExpr {
                                    range: 0..20,
                                    key: Name(
                                        NameExpr {
                                            range: 1..2,
                                            id: "x",
                                            ctx: Load,
                                        },
                                    ),
                                    value: BinOp(
                                        BinOpExpr {
                                            range: 4..7,
                                            left: Name(
                                                NameExpr {
                                                    range: 4..5,
                                                    id: "x",
                                                    ctx: Load,
                                                },
                                            ),
                                            op: Mult,
                                            right: NumberLiteral(
                                                NumberLiteralExpr {
                                                    range: 6..7,
                                                    value: Int(
                                                        2,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    generators: [
                                        Comprehension {
                                            range: 8..19,
                                            target: Name(
                                                NameExpr {
                                                    range: 12..13,
                                                    id: "x",
                                                    ctx: Store,
                                                },
                                            ),
                                            iter: Name(
                                                NameExpr {
                                                    range: 17..19,
                                                    id: "xs",
                                                    ctx: Load,
                                                },
                                            ),
                                            ifs: [],
                                            is_async: false,
                                        },
                                    ],
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 23..25,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..20,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_generator_exp() {
        let ast = parse_suite(r"(x for x in xs) = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..20,
                        targets: [
                            GeneratorExp(
                                GeneratorExpExpr {
                                    range: 0..15,
                                    elt: Name(
                                        NameExpr {
                                            range: 1..2,
                                            id: "x",
                                            ctx: Load,
                                        },
                                    ),
                                    generators: [
                                        Comprehension {
                                            range: 3..14,
                                            target: Name(
                                                NameExpr {
                                                    range: 7..8,
                                                    id: "x",
                                                    ctx: Store,
                                                },
                                            ),
                                            iter: Name(
                                                NameExpr {
                                                    range: 12..14,
                                                    id: "xs",
                                                    ctx: Load,
                                                },
                                            ),
                                            ifs: [],
                                            is_async: false,
                                        },
                                    ],
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 18..20,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..15,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_await() {
        let ast = parse_suite(r"await x = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..12,
                        targets: [
                            Await(
                                AwaitExpr {
                                    range: 0..7,
                                    value: Name(
                                        NameExpr {
                                            range: 6..7,
                                            id: "x",
                                            ctx: Load,
                                        },
                                    ),
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 10..12,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..7,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_yield() {
        let ast = parse_suite(r"(yield x) = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..14,
                        targets: [
                            Yield(
                                YieldExpr {
                                    range: 1..8,
                                    value: Some(
                                        Name(
                                            NameExpr {
                                                range: 7..8,
                                                id: "x",
                                                ctx: Load,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 12..14,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 1..8,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_yield_from() {
        let ast = parse_suite(r"(yield from xs) = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..20,
                        targets: [
                            YieldFrom(
                                YieldFromExpr {
                                    range: 1..14,
                                    value: Name(
                                        NameExpr {
                                            range: 12..14,
                                            id: "xs",
                                            ctx: Load,
                                        },
                                    ),
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 18..20,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 1..14,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_compare() {
        let ast = parse_suite(r"a < b < c = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..14,
                        targets: [
                            Compare(
                                CompareExpr {
                                    range: 0..9,
                                    left: Name(
                                        NameExpr {
                                            range: 0..1,
                                            id: "a",
                                            ctx: Load,
                                        },
                                    ),
                                    ops: [
                                        Lt,
                                        Lt,
                                    ],
                                    comparators: [
                                        Name(
                                            NameExpr {
                                                range: 4..5,
                                                id: "b",
                                                ctx: Load,
                                            },
                                        ),
                                        Name(
                                            NameExpr {
                                                range: 8..9,
                                                id: "c",
                                                ctx: Load,
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 12..14,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..9,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_call() {
        let ast = parse_suite(r"foo() = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..10,
                        targets: [
                            Call(
                                CallExpr {
                                    range: 0..5,
                                    func: Name(
                                        NameExpr {
                                            range: 0..3,
                                            id: "foo",
                                            ctx: Load,
                                        },
                                    ),
                                    arguments: Arguments {
                                        range: 3..5,
                                        args: [],
                                        keywords: [],
                                    },
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 8..10,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..5,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_formatted_value() {
        // N.B. It looks like the parser can't generate a top-level
        // FormattedValue, where as the official Python AST permits
        // representing a single f-string containing just a variable as a
        // FormattedValue directly.
        //
        // Bottom line is that because of this, this test is (at present)
        // duplicative with the `fstring` test. That is, in theory these tests
        // could fail independently, but in practice their failure or success
        // is coupled.
        //
        // See: https://docs.python.org/3/library/ast.html#ast.FormattedValue
        let ast = parse_suite(r#"f"{quux}" = 42"#);
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..14,
                        targets: [
                            FString(
                                FStringExpr {
                                    range: 0..9,
                                    value: FStringValue {
                                        inner: Single(
                                            FString(
                                                FString {
                                                    range: 0..9,
                                                    elements: [
                                                        Expression(
                                                            FStringExpressionElement {
                                                                range: 2..8,
                                                                expression: Name(
                                                                    NameExpr {
                                                                        range: 3..7,
                                                                        id: "quux",
                                                                        ctx: Load,
                                                                    },
                                                                ),
                                                                debug_text: None,
                                                                conversion: None,
                                                                format_spec: None,
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 12..14,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..9,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_fstring() {
        let ast = parse_suite(r#"f"{foo} and {bar}" = 42"#);
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..23,
                        targets: [
                            FString(
                                FStringExpr {
                                    range: 0..18,
                                    value: FStringValue {
                                        inner: Single(
                                            FString(
                                                FString {
                                                    range: 0..18,
                                                    elements: [
                                                        Expression(
                                                            FStringExpressionElement {
                                                                range: 2..7,
                                                                expression: Name(
                                                                    NameExpr {
                                                                        range: 3..6,
                                                                        id: "foo",
                                                                        ctx: Load,
                                                                    },
                                                                ),
                                                                debug_text: None,
                                                                conversion: None,
                                                                format_spec: None,
                                                            },
                                                        ),
                                                        Literal(
                                                            FStringLiteralElement {
                                                                range: 7..12,
                                                                value: " and ",
                                                            },
                                                        ),
                                                        Expression(
                                                            FStringExpressionElement {
                                                                range: 12..17,
                                                                expression: Name(
                                                                    NameExpr {
                                                                        range: 13..16,
                                                                        id: "bar",
                                                                        ctx: Load,
                                                                    },
                                                                ),
                                                                debug_text: None,
                                                                conversion: None,
                                                                format_spec: None,
                                                            },
                                                        ),
                                                    ],
                                                },
                                            ),
                                        ),
                                    },
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 21..23,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..18,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_string_literal() {
        let ast = parse_suite(r#""foo" = 42"#);
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..10,
                        targets: [
                            StringLiteral(
                                StringLiteralExpr {
                                    range: 0..5,
                                    value: StringLiteralValue {
                                        inner: Single(
                                            StringLiteral {
                                                range: 0..5,
                                                value: "foo",
                                                unicode: false,
                                            },
                                        ),
                                    },
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 8..10,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..5,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_bytes_literal() {
        let ast = parse_suite(r#"b"foo" = 42"#);
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..11,
                        targets: [
                            BytesLiteral(
                                BytesLiteralExpr {
                                    range: 0..6,
                                    value: BytesLiteralValue {
                                        inner: Single(
                                            BytesLiteral {
                                                range: 0..6,
                                                value: [
                                                    102,
                                                    111,
                                                    111,
                                                ],
                                            },
                                        ),
                                    },
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 9..11,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..6,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_number_literal() {
        let ast = parse_suite(r"123 = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..8,
                        targets: [
                            NumberLiteral(
                                NumberLiteralExpr {
                                    range: 0..3,
                                    value: Int(
                                        123,
                                    ),
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 6..8,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..3,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_boolean_literal() {
        let ast = parse_suite(r"True = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..9,
                        targets: [
                            BooleanLiteral(
                                BooleanLiteralExpr {
                                    range: 0..4,
                                    value: true,
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 7..9,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..4,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_none_literal() {
        let ast = parse_suite(r"None = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..9,
                        targets: [
                            NoneLiteral(
                                NoneLiteralExpr {
                                    range: 0..4,
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 7..9,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..4,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_ellipsis_literal() {
        let ast = parse_suite(r"... = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..8,
                        targets: [
                            EllipsisLiteral(
                                EllipsisLiteralExpr {
                                    range: 0..3,
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 6..8,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..3,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_starred() {
        let ast = parse_suite(r"*foo() = 42");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..11,
                        targets: [
                            Starred(
                                StarredExpr {
                                    range: 0..6,
                                    value: Call(
                                        CallExpr {
                                            range: 1..6,
                                            func: Name(
                                                NameExpr {
                                                    range: 1..4,
                                                    id: "foo",
                                                    ctx: Load,
                                                },
                                            ),
                                            arguments: Arguments {
                                                range: 4..6,
                                                args: [],
                                                keywords: [],
                                            },
                                        },
                                    ),
                                    ctx: Store,
                                },
                            ),
                        ],
                        value: NumberLiteral(
                            NumberLiteralExpr {
                                range: 9..11,
                                value: Int(
                                    42,
                                ),
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..6,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_list() {
        let ast = parse_suite(r"[x, foo(), y] = [42, 42, 42]");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..28,
                        targets: [
                            List(
                                ListExpr {
                                    range: 0..13,
                                    elts: [
                                        Name(
                                            NameExpr {
                                                range: 1..2,
                                                id: "x",
                                                ctx: Store,
                                            },
                                        ),
                                        Call(
                                            CallExpr {
                                                range: 4..9,
                                                func: Name(
                                                    NameExpr {
                                                        range: 4..7,
                                                        id: "foo",
                                                        ctx: Load,
                                                    },
                                                ),
                                                arguments: Arguments {
                                                    range: 7..9,
                                                    args: [],
                                                    keywords: [],
                                                },
                                            },
                                        ),
                                        Name(
                                            NameExpr {
                                                range: 11..12,
                                                id: "y",
                                                ctx: Store,
                                            },
                                        ),
                                    ],
                                    ctx: Store,
                                },
                            ),
                        ],
                        value: List(
                            ListExpr {
                                range: 16..28,
                                elts: [
                                    NumberLiteral(
                                        NumberLiteralExpr {
                                            range: 17..19,
                                            value: Int(
                                                42,
                                            ),
                                        },
                                    ),
                                    NumberLiteral(
                                        NumberLiteralExpr {
                                            range: 21..23,
                                            value: Int(
                                                42,
                                            ),
                                        },
                                    ),
                                    NumberLiteral(
                                        NumberLiteralExpr {
                                            range: 25..27,
                                            value: Int(
                                                42,
                                            ),
                                        },
                                    ),
                                ],
                                ctx: Load,
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..13,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_list_nested() {
        let ast = parse_suite(r"[[a, b], [[42]], d] = [[1, 2], [[3]], 4]");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..40,
                        targets: [
                            List(
                                ListExpr {
                                    range: 0..19,
                                    elts: [
                                        List(
                                            ListExpr {
                                                range: 1..7,
                                                elts: [
                                                    Name(
                                                        NameExpr {
                                                            range: 2..3,
                                                            id: "a",
                                                            ctx: Store,
                                                        },
                                                    ),
                                                    Name(
                                                        NameExpr {
                                                            range: 5..6,
                                                            id: "b",
                                                            ctx: Store,
                                                        },
                                                    ),
                                                ],
                                                ctx: Store,
                                            },
                                        ),
                                        List(
                                            ListExpr {
                                                range: 9..15,
                                                elts: [
                                                    List(
                                                        ListExpr {
                                                            range: 10..14,
                                                            elts: [
                                                                NumberLiteral(
                                                                    NumberLiteralExpr {
                                                                        range: 11..13,
                                                                        value: Int(
                                                                            42,
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                            ctx: Store,
                                                        },
                                                    ),
                                                ],
                                                ctx: Store,
                                            },
                                        ),
                                        Name(
                                            NameExpr {
                                                range: 17..18,
                                                id: "d",
                                                ctx: Store,
                                            },
                                        ),
                                    ],
                                    ctx: Store,
                                },
                            ),
                        ],
                        value: List(
                            ListExpr {
                                range: 22..40,
                                elts: [
                                    List(
                                        ListExpr {
                                            range: 23..29,
                                            elts: [
                                                NumberLiteral(
                                                    NumberLiteralExpr {
                                                        range: 24..25,
                                                        value: Int(
                                                            1,
                                                        ),
                                                    },
                                                ),
                                                NumberLiteral(
                                                    NumberLiteralExpr {
                                                        range: 27..28,
                                                        value: Int(
                                                            2,
                                                        ),
                                                    },
                                                ),
                                            ],
                                            ctx: Load,
                                        },
                                    ),
                                    List(
                                        ListExpr {
                                            range: 31..36,
                                            elts: [
                                                List(
                                                    ListExpr {
                                                        range: 32..35,
                                                        elts: [
                                                            NumberLiteral(
                                                                NumberLiteralExpr {
                                                                    range: 33..34,
                                                                    value: Int(
                                                                        3,
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                        ctx: Load,
                                                    },
                                                ),
                                            ],
                                            ctx: Load,
                                        },
                                    ),
                                    NumberLiteral(
                                        NumberLiteralExpr {
                                            range: 38..39,
                                            value: Int(
                                                4,
                                            ),
                                        },
                                    ),
                                ],
                                ctx: Load,
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..19,
                },
            ],
        )
        "###);
    }

    #[test]
    fn err_tuple() {
        let ast = parse_suite(r"(x, foo(), y) = (42, 42, 42)");
        insta::assert_debug_snapshot!(ast, @r###"
        (
            [
                Assign(
                    AssignStmt {
                        range: 0..28,
                        targets: [
                            Tuple(
                                TupleExpr {
                                    range: 0..13,
                                    elts: [
                                        Name(
                                            NameExpr {
                                                range: 1..2,
                                                id: "x",
                                                ctx: Store,
                                            },
                                        ),
                                        Call(
                                            CallExpr {
                                                range: 4..9,
                                                func: Name(
                                                    NameExpr {
                                                        range: 4..7,
                                                        id: "foo",
                                                        ctx: Load,
                                                    },
                                                ),
                                                arguments: Arguments {
                                                    range: 7..9,
                                                    args: [],
                                                    keywords: [],
                                                },
                                            },
                                        ),
                                        Name(
                                            NameExpr {
                                                range: 11..12,
                                                id: "y",
                                                ctx: Store,
                                            },
                                        ),
                                    ],
                                    ctx: Store,
                                },
                            ),
                        ],
                        value: Tuple(
                            TupleExpr {
                                range: 17..27,
                                elts: [
                                    NumberLiteral(
                                        NumberLiteralExpr {
                                            range: 17..19,
                                            value: Int(
                                                42,
                                            ),
                                        },
                                    ),
                                    NumberLiteral(
                                        NumberLiteralExpr {
                                            range: 21..23,
                                            value: Int(
                                                42,
                                            ),
                                        },
                                    ),
                                    NumberLiteral(
                                        NumberLiteralExpr {
                                            range: 25..27,
                                            value: Int(
                                                42,
                                            ),
                                        },
                                    ),
                                ],
                                ctx: Load,
                            },
                        ),
                    },
                ),
            ],
            [
                ParseError {
                    error: AssignmentError,
                    location: 0..13,
                },
            ],
        )
        "###);
    }

    // This last group of tests checks that assignments we expect to be parsed
    // (including some interesting ones) continue to be parsed successfully.

    #[test]
    fn ok_starred() {
        let ast = parse_suite(r"*foo = 42");
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn ok_list() {
        let ast = parse_suite(r"[x, y, z] = [1, 2, 3]");
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn ok_tuple() {
        let ast = parse_suite(r"(x, y, z) = (1, 2, 3)");
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn ok_subscript_normal() {
        let ast = parse_suite(r"x[0] = 42");
        insta::assert_debug_snapshot!(ast);
    }

    // This is actually a type error, not a syntax error. So check that it
    // doesn't fail parsing.
    #[test]
    fn ok_subscript_weird() {
        let ast = parse_suite(r"5[0] = 42");
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn ok_slice_normal() {
        let ast = parse_suite(r"x[1:2] = [42]");
        insta::assert_debug_snapshot!(ast);
    }

    // This is actually a type error, not a syntax error. So check that it
    // doesn't fail parsing.
    #[test]
    fn ok_slice_weird() {
        let ast = parse_suite(r"5[1:2] = [42]");
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn ok_attribute_normal() {
        let ast = parse_suite(r"foo.bar = 42");
        insta::assert_debug_snapshot!(ast);
    }

    // This is actually an attribute error, not a syntax error. So check that
    // it doesn't fail parsing.
    #[test]
    fn ok_attribute_weird() {
        let ast = parse_suite(r#""foo".y = 42"#);
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn ok_name() {
        let ast = parse_suite(r"foo = 42");
        insta::assert_debug_snapshot!(ast);
    }

    // This is a sanity test for what looks like an ipython directive being
    // assigned to. Although this doesn't actually parse as an assignment
    // statement, but rather, a directive whose value is `foo = 42`.
    #[test]
    fn ok_ipy_escape_command() {
        use crate::Mode;

        let src = r"!foo = 42";
        let tokens = crate::lexer::lex(src, Mode::Ipython);
        let ast = crate::parse_tokens(tokens, src, Mode::Ipython);
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn ok_assignment_expr() {
        let ast = parse_suite(r"(x := 5)");
        insta::assert_debug_snapshot!(ast);
    }
}
