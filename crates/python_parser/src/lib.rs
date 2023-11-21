//! This crate can be used to parse Python source code into an Abstract
//! Syntax Tree.
//!
//! ## Overview:
//!
//! The process by which source code is parsed into an AST can be broken down
//! into two general stages: [lexical analysis] and [parsing].
//!
//! During lexical analysis, the source code is converted into a stream of lexical
//! tokens that represent the smallest meaningful units of the language. For example,
//! the source code `print("Hello world")` would _roughly_ be converted into the following
//! stream of tokens:
//!
//! ```text
//! Name("print"), LeftParen, String("Hello world"), RightParen
//! ```
//!
//! these tokens are then consumed by the `ruff_python_parser`, which matches them against a set of
//! grammar rules to verify that the source code is syntactically valid and to construct
//! an AST that represents the source code.
//!
//! During parsing, the `ruff_python_parser` consumes the tokens generated by the lexer and constructs
//! a tree representation of the source code. The tree is made up of nodes that represent
//! the different syntactic constructs of the language. If the source code is syntactically
//! invalid, parsing fails and an error is returned. After a successful parse, the AST can
//! be used to perform further analysis on the source code. Continuing with the example
//! above, the AST generated by the `ruff_python_parser` would _roughly_ look something like this:
//!
//! ```text
//! node: Expr {
//!     value: {
//!         node: Call {
//!             func: {
//!                 node: Name {
//!                     id: "print",
//!                     ctx: Load,
//!                 },
//!             },
//!             args: [
//!                 node: Constant {
//!                     value: Str("Hello World"),
//!                     kind: None,
//!                 },
//!             ],
//!             keywords: [],
//!         },
//!     },
//! },
//!```
//!
//! Note: The Tokens/ASTs shown above are not the exact tokens/ASTs generated by the `ruff_python_parser`.
//!
//! ## Source code layout:
//!
//! The functionality of this crate is split into several modules:
//!
//! - token: This module contains the definition of the tokens that are generated by the lexer.
//! - [lexer]: This module contains the lexer and is responsible for generating the tokens.
//! - `ruff_python_parser`: This module contains an interface to the `ruff_python_parser` and is responsible for generating the AST.
//!     - Functions and strings have special parsing requirements that are handled in additional files.
//! - mode: This module contains the definition of the different modes that the `ruff_python_parser` can be in.
//!
//! # Examples
//!
//! For example, to get a stream of tokens from a given string, one could do this:
//!
//! ```
//! use ruff_python_parser::{lexer::lex, Mode};
//!
//! let python_source = r#"
//! def is_odd(i):
//!     return bool(i & 1)
//! "#;
//! let mut tokens = lex(python_source, Mode::Module);
//! assert!(tokens.all(|t| t.is_ok()));
//! ```
//!
//! These tokens can be directly fed into the `ruff_python_parser` to generate an AST:
//!
//! ```
//! use ruff_python_parser::{lexer::lex, Mode, parse_tokens};
//!
//! let python_source = r#"
//! def is_odd(i):
//!    return bool(i & 1)
//! "#;
//! let tokens = lex(python_source, Mode::Module);
//! let ast = parse_tokens(tokens, python_source, Mode::Module);
//!
//! assert!(ast.is_ok());
//! ```
//!
//! Alternatively, you can use one of the other `parse_*` functions to parse a string directly without using a specific
//! mode or tokenizing the source beforehand:
//!
//! ```
//! use ruff_python_parser::parse_suite;
//!
//! let python_source = r#"
//! def is_odd(i):
//!   return bool(i & 1)
//! "#;
//! let ast = parse_suite(python_source);
//!
//! assert!(ast.is_ok());
//! ```
//!
//! [lexical analysis]: https://en.wikipedia.org/wiki/Lexical_analysis
//! [parsing]: https://en.wikipedia.org/wiki/Parsing
//! [lexer]: crate::lexer

use itertools::Itertools;
use lexer::{lex, LexResult};
pub use parser::ParsedFile;
use python_ast::{Expr, Mod, PySourceType, Suite};
use ruff_text_size::TextSize;
pub use token::{StringKind, Tok, TokenKind};

mod error;
mod helpers;
pub mod lexer;
mod parser;
mod soft_keywords;
mod string;
mod token;
mod token_set;
pub use error::{FStringErrorType, ParseError, ParseErrorType};

pub fn parse_program(source: &str) -> ParsedFile {
    parse(source, Mode::Module)
}

pub fn parse(source: &str, mode: Mode) -> ParsedFile {
    let lxr = lex(source, mode)
        .filter_ok(|(tok, _)| !matches!(tok, Tok::Comment { .. } | Tok::NonLogicalNewline));
    parse_tokens(lxr, source, mode)
}

pub fn parse_suite(source: &str) -> (Suite, Vec<ParseError>) {
    let parsed_file = parse(source, Mode::Module);
    let Mod::Module(m) = parsed_file.ast else {
        unreachable!("Mode::Module can't return other variant")
    };
    (m.body, parsed_file.parse_errors)
}

pub fn parse_tokens(
    lxr: impl IntoIterator<Item = LexResult>,
    source: &str,
    mode: Mode,
) -> ParsedFile {
    let lxr = lxr.into_iter();

    parser::Parser::new(source, mode, lxr).parse()
}

pub fn parse_expression(source: &str) -> (Expr, Vec<ParseError>) {
    let lexer = lex(source, Mode::Expression);
    let parsed_file = parse_tokens(lexer, source, Mode::Expression);
    match parsed_file.ast {
        Mod::Expression(expression) => (*expression.body, parsed_file.parse_errors),
        Mod::Module(_) => unreachable!("Mode::Expression doesn't return other variant"),
    }
}

pub fn parse_starts_at(source: &str, mode: Mode, offset: TextSize) -> ParsedFile {
    let lxr = lexer::lex_starts_at(source, mode, offset);
    parse_tokens(lxr, source, mode)
}

pub fn parse_expression_starts_at(source: &str, offset: TextSize) -> (Expr, Vec<ParseError>) {
    let parsed_file = parse_starts_at(source, Mode::Expression, offset);
    match parsed_file.ast {
        Mod::Expression(expression) => (*expression.body, parsed_file.parse_errors),
        Mod::Module(_) => unreachable!("Mode::Expression doesn't return other variant"),
    }
}

/// Control in the different modes by which a source file can be parsed.
/// The mode argument specifies in what way code must be parsed.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Mode {
    /// The code consists of a sequence of statements.
    Module,
    /// The code consists of a single expression.
    Expression,
    /// The code consists of a sequence of statements which can include the
    /// escape commands that are part of IPython syntax.
    ///
    /// ## Supported escape commands:
    ///
    /// - [Magic command system] which is limited to [line magics] and can start
    ///   with `?` or `??`.
    /// - [Dynamic object information] which can start with `?` or `??`.
    /// - [System shell access] which can start with `!` or `!!`.
    /// - [Automatic parentheses and quotes] which can start with `/`, `;`, or `,`.
    ///
    /// [Magic command system]: https://ipython.readthedocs.io/en/stable/interactive/reference.html#magic-command-system
    /// [line magics]: https://ipython.readthedocs.io/en/stable/interactive/magics.html#line-magics
    /// [Dynamic object information]: https://ipython.readthedocs.io/en/stable/interactive/reference.html#dynamic-object-information
    /// [System shell access]: https://ipython.readthedocs.io/en/stable/interactive/reference.html#system-shell-access
    /// [Automatic parentheses and quotes]: https://ipython.readthedocs.io/en/stable/interactive/reference.html#automatic-parentheses-and-quotes
    Ipython,
}

impl std::str::FromStr for Mode {
    type Err = ModeParseError;
    fn from_str(s: &str) -> Result<Self, ModeParseError> {
        match s {
            "exec" | "single" => Ok(Mode::Module),
            "eval" => Ok(Mode::Expression),
            "ipython" => Ok(Mode::Ipython),
            _ => Err(ModeParseError),
        }
    }
}

pub trait AsMode {
    fn as_mode(&self) -> Mode;
}

impl AsMode for PySourceType {
    fn as_mode(&self) -> Mode {
        match self {
            PySourceType::Python | PySourceType::Stub => Mode::Module,
            PySourceType::Ipynb => Mode::Ipython,
        }
    }
}

/// Returned when a given mode is not valid.
#[derive(Debug)]
pub struct ModeParseError;

impl std::fmt::Display for ModeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, r#"mode must be "exec", "eval", "ipython", or "single""#)
    }
}