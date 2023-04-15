mod char_codepoints;
mod char_stream;
pub mod span;
pub mod token;

use std::borrow::Cow;

pub use char_stream::CharStream;
use token::Token;

use crate::{
    error::{PythonError, PythonErrorType},
    valid_id_initial_chars, valid_id_noninitial_chars,
};

use self::{
    span::{Position, Span},
    token::types::{IntegerType, KeywordType, NumberType, OperatorType, SoftKeywordType, TokenType},
};

pub struct Lexer<'a> {
    cs: CharStream<'a>,
    tokens: Vec<Token<'a>>,
    indent_stack: Vec<usize>,
    /// This is used to check if we are inside a [], () or {}
    implicit_line_joining: i32,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            cs: CharStream::new(text),
            tokens: Vec::new(),
            indent_stack: vec![0],
            implicit_line_joining: 0,
        }
    }

    pub fn tokenize(&mut self) -> Option<Vec<PythonError>> {
        let mut errors: Vec<PythonError> = vec![];
        let mut is_beginning_of_line = true;

        while !self.cs.is_eof() {
            if is_beginning_of_line {
                is_beginning_of_line = false;

                let whitespace_total = self.cs.skip_whitespace();

                if self.cs.is_eof() {
                    break;
                }

                // Skip comments
                if self.cs.current_char().map_or(false, |char| char == '#') {
                    self.cs.advance_while(1, |char| char != '\n');
                }

                if self.cs.is_eof() {
                    break;
                }

                // skip lines containing only white spaces or \n, \r, \r\n
                if let Some(eol_size) = self.cs.is_at_eol() {
                    if self.implicit_line_joining == 0 {
                        is_beginning_of_line = true;
                        self.cs.advance_by(eol_size);
                        continue;
                    }
                }

                if self.implicit_line_joining == 0 {
                    if let Err(error) = self.handle_indentation(whitespace_total) {
                        errors.push(error);
                    }
                }
            }

            match self.cs.current_char().unwrap() {
                valid_id_initial_chars!() => self.lex_identifier_or_keyword(),
                '0'..='9' => {
                    if let Some(number_errors) = self.lex_number() {
                        errors.extend(number_errors);
                    }
                }
                '"' | '\'' => {
                    if let Some(string_errors) = self.lex_string() {
                        errors.extend(string_errors);
                    }
                }
                '(' => {
                    self.implicit_line_joining += 1;

                    self.lex_single_char(TokenType::OpenParenthesis);
                }
                ')' => {
                    self.implicit_line_joining -= 1;

                    self.lex_single_char(TokenType::CloseParenthesis);
                }
                '[' => {
                    self.implicit_line_joining += 1;

                    self.lex_single_char(TokenType::OpenBrackets);
                }
                ']' => {
                    self.implicit_line_joining -= 1;

                    self.lex_single_char(TokenType::CloseBrackets);
                }
                '{' => {
                    self.implicit_line_joining += 1;

                    self.lex_single_char(TokenType::OpenBrace);
                }
                '}' => {
                    self.implicit_line_joining -= 1;

                    self.lex_single_char(TokenType::CloseBrace);
                }
                '.' => {
                    if self.cs.next_char().map_or(false, |char| char.is_ascii_digit()) {
                        self.lex_number();
                        continue;
                    }

                    if matches!(
                        (self.cs.next_char(), self.cs.peek_char(self.cs.pos().index + 2)),
                        (Some('.'), Some('.'))
                    ) {
                        let start = self.cs.pos();
                        self.cs.advance_by(3);
                        let end = self.cs.pos();
                        self.tokens.push(Token {
                            kind: TokenType::Ellipsis,
                            span: self.make_span(start, end),
                        })
                    } else {
                        self.lex_single_char(TokenType::Dot)
                    }
                }
                ';' => self.lex_single_char(TokenType::SemiColon),
                ',' => self.lex_single_char(TokenType::Comma),
                ':' => {
                    if let Some('=') = self.cs.next_char() {
                        let start = self.cs.pos();
                        self.cs.advance_by(2);
                        let end = self.cs.pos();
                        self.tokens.push(Token {
                            kind: TokenType::Operator(OperatorType::ColonEqual),
                            span: self.make_span(start, end),
                        })
                    } else {
                        self.lex_single_char(TokenType::Colon)
                    }
                }
                '*' | '+' | '=' | '-' | '<' | '>' | '&' | '|' | '%' | '~' | '^' | '!' | '@' | '/' => {
                    if matches!((self.cs.current_char(), self.cs.next_char()), (Some('-'), Some('>'))) {
                        let start = self.cs.pos();
                        self.cs.advance_by(2);
                        let end = self.cs.pos();
                        self.tokens.push(Token {
                            kind: TokenType::RightArrow,
                            span: self.make_span(start, end),
                        });
                    } else {
                        self.lex_operator();
                    }
                }
                ' ' | '\t' | '\u{0c}' => {
                    self.cs.skip_whitespace();
                }
                '\n' | '\r' => {
                    is_beginning_of_line = true;
                    let eol_size = self.cs.is_at_eol().unwrap();

                    let start = self.cs.pos();

                    self.cs.advance_by(eol_size);

                    if self.implicit_line_joining > 0 {
                        continue;
                    }

                    let end = Position {
                        column: start.column + 1,
                        ..start
                    };

                    self.tokens.push(Token {
                        kind: TokenType::NewLine,
                        span: self.make_span(start, end),
                    });
                }
                '\\' => {
                    let start = self.cs.pos();
                    // consume \
                    self.cs.advance_by(1);
                    let end = self.cs.pos();

                    if self.cs.current_char().map_or(false, |char| char == '\n') {
                        self.cs.advance_by(1);
                    } else {
                        errors.push(PythonError {
                            msg: String::from("SyntaxError: unexpected characters after line continuation character"),
                            error: PythonErrorType::Syntax,
                            span: self.make_span(start, end),
                        });
                    }
                }
                '#' => {
                    self.cs.advance_while(1, |char| char != '\n');
                }
                _ => {
                    let start = self.cs.pos();
                    self.cs.advance_by(1);
                    let end = self.cs.pos();

                    errors.push(PythonError {
                        error: PythonErrorType::Syntax,
                        msg: String::from("SyntaxError: invalid syntax, character only allowed inside string literals"),
                        span: self.make_span(start, end),
                    })
                }
            }
        }

        while self.indent_stack.last().copied().unwrap() > 0 {
            self.tokens.push(Token {
                kind: TokenType::Dedent,
                span: Span {
                    column_start: 1,
                    column_end: 1,
                    ..self.make_span(self.cs.pos(), self.cs.pos())
                },
            });
            self.indent_stack.pop();
        }

        let eof_span = self.make_span(self.cs.pos(), self.cs.pos());
        self.tokens.push(Token {
            kind: TokenType::Eof,
            span: Span {
                column_end: eof_span.column_end + 1,
                ..eof_span
            },
        });

        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    // FIXME: Handle mix of tabs and spaces in indentation
    fn handle_indentation(&mut self, whitespace_total: usize) -> Result<(), PythonError> {
        let top_of_stack = self.indent_stack.last().copied().unwrap();

        match whitespace_total.cmp(&top_of_stack) {
            std::cmp::Ordering::Less => {
                while self
                    .indent_stack
                    .last()
                    .map_or(false, |&top_of_stack| whitespace_total < top_of_stack)
                {
                    self.indent_stack.pop();
                    self.tokens.push(Token {
                        kind: TokenType::Dedent,
                        span: Span {
                            column_start: 1,
                            column_end: 1,
                            ..self.make_span(self.cs.pos(), self.cs.pos())
                        },
                    });
                }

                if self
                    .indent_stack
                    .last()
                    .map_or(false, |&top_of_stack| whitespace_total != top_of_stack)
                {
                    return Err(PythonError {
                        error: PythonErrorType::Indentation,
                        msg: "IndentError: indent amount does not match previous indent".to_string(),
                        span: Span {
                            column_start: 1,
                            column_end: 1,
                            ..self.make_span(self.cs.pos(), self.cs.pos())
                        },
                    });
                }
            }
            std::cmp::Ordering::Greater => {
                self.indent_stack.push(whitespace_total);
                self.tokens.push(Token {
                    kind: TokenType::Indent,
                    span: Span {
                        column_start: 1,
                        column_end: 1,
                        ..self.make_span(self.cs.pos(), self.cs.pos())
                    },
                });
            }
            std::cmp::Ordering::Equal => (), // Do nothing!
        }

        Ok(())
    }

    fn lex_single_char(&mut self, token: TokenType<'a>) {
        let start = self.cs.pos();
        self.cs.advance_by(1);
        let end = self.cs.pos();

        self.tokens.push(Token {
            kind: token,
            span: self.make_span(start, end),
        });
    }

    fn lex_identifier_or_keyword(&mut self) {
        let start = self.cs.pos();
        while self
            .cs
            .current_char()
            .map_or(false, |char| matches!(char, valid_id_noninitial_chars!()))
        {
            self.cs.advance_by(1);
        }
        let end = self.cs.pos();

        let str = self.cs.get_slice(start.index, end.index).unwrap();

        if self.is_str_prefix(str) && self.cs.current_char().map_or(false, |char| matches!(char, '"' | '\'')) {
            self.lex_string();
            return;
        }

        let token_type = match str {
            b"and" => TokenType::Keyword(KeywordType::And),
            b"as" => TokenType::Keyword(KeywordType::As),
            b"assert" => TokenType::Keyword(KeywordType::Assert),
            b"async" => TokenType::Keyword(KeywordType::Async),
            b"await" => TokenType::Keyword(KeywordType::Await),
            b"break" => TokenType::Keyword(KeywordType::Break),
            b"case" => TokenType::SoftKeyword(SoftKeywordType::Case),
            b"class" => TokenType::Keyword(KeywordType::Class),
            b"continue" => TokenType::Keyword(KeywordType::Continue),
            b"def" => TokenType::Keyword(KeywordType::Def),
            b"del" => TokenType::Keyword(KeywordType::Del),
            b"elif" => TokenType::Keyword(KeywordType::Elif),
            b"else" => TokenType::Keyword(KeywordType::Else),
            b"except" => TokenType::Keyword(KeywordType::Except),
            b"False" => TokenType::Keyword(KeywordType::False),
            b"finally" => TokenType::Keyword(KeywordType::Finally),
            b"for" => TokenType::Keyword(KeywordType::For),
            b"from" => TokenType::Keyword(KeywordType::From),
            b"global" => TokenType::Keyword(KeywordType::Global),
            b"if" => TokenType::Keyword(KeywordType::If),
            b"import" => TokenType::Keyword(KeywordType::Import),
            b"in" => TokenType::Keyword(KeywordType::In),
            b"is" => TokenType::Keyword(KeywordType::Is),
            b"lambda" => TokenType::Keyword(KeywordType::Lambda),
            b"match" => TokenType::SoftKeyword(SoftKeywordType::Match),
            b"None" => TokenType::Keyword(KeywordType::None),
            b"nonlocal" => TokenType::Keyword(KeywordType::NonLocal),
            b"not" => TokenType::Keyword(KeywordType::Not),
            b"or" => TokenType::Keyword(KeywordType::Or),
            b"pass" => TokenType::Keyword(KeywordType::Pass),
            b"raise" => TokenType::Keyword(KeywordType::Raise),
            b"return" => TokenType::Keyword(KeywordType::Return),
            b"True" => TokenType::Keyword(KeywordType::True),
            b"try" => TokenType::Keyword(KeywordType::Try),
            b"while" => TokenType::Keyword(KeywordType::While),
            b"with" => TokenType::Keyword(KeywordType::With),
            b"yield" => TokenType::Keyword(KeywordType::Yield),
            b"_" => TokenType::SoftKeyword(SoftKeywordType::Underscore),
            _ => TokenType::Id(String::from_utf8_lossy(str)),
        };

        self.tokens.push(Token {
            kind: token_type,
            span: self.make_span(start, end),
        });
    }

    fn is_str_prefix(&self, str: &[u8]) -> bool {
        matches!(
            str,
            // string prefixes
            b"r" | b"u" | b"R" | b"U" | b"f" | b"F" | b"fr" | b"Fr" | b"fR" | b"FR" | b"rf" | b"rF" | b"Rf" | b"RF"
            // bytes prefixes
            | b"b" | b"B" | b"br" | b"Br" | b"bR" | b"BR" | b"rb" | b"rB" | b"Rb" | b"RB"
        )
    }

    // TODO: Define a different string type for string prefixes
    // TODO: try to refactor this code
    //https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals
    fn lex_string(&mut self) -> Option<Vec<PythonError>> {
        if self.implicit_line_joining > 0 {
            return self.lex_string_within_parens();
        }

        let mut errors = vec![];
        let (mut string, mut str_span, string_errors) = self.process_string();
        if let Some(string_errors) = string_errors {
            errors.extend(string_errors);
        }

        self.cs.skip_whitespace();
        // check for any string prefix
        if matches!(
            self.cs.current_char(),
            Some('r' | 'b' | 'f' | 'F' | 'R' | 'B' | 'u' | 'U')
        ) {
            while matches!(self.cs.current_char(), Some(valid_id_noninitial_chars!())) {
                self.cs.advance_by(1);
            }
        }

        while matches!(self.cs.current_char(), Some('\'' | '"')) {
            let (str, span, str_errors) = self.process_string();

            string.to_mut().push_str(&str);
            str_span.column_end = span.column_end;
            str_span.row_end = span.row_end;

            if let Some(str_errors) = str_errors {
                errors.extend(str_errors);
            }

            self.cs.skip_whitespace();

            if matches!(self.cs.current_char(), Some('#')) {
                self.cs.advance_while(1, |char| char != '\n');
            }

            // check for any string prefix
            if matches!(
                self.cs.current_char(),
                Some('r' | 'b' | 'f' | 'F' | 'R' | 'B' | 'u' | 'U')
            ) {
                while matches!(self.cs.current_char(), Some(valid_id_noninitial_chars!())) {
                    self.cs.advance_by(1);
                }
            }
        }

        // In case we find a '\' (explicit line join char) with a string after it,
        // we need to treat these strings as one Token.
        self.cs.skip_whitespace();
        while self.cs.current_char().map_or(false, |char| char == '\\') {
            self.cs.advance_by(1);

            // Skip any whitespace after the "\"
            self.cs.skip_whitespace();
            if matches!(self.cs.current_char(), Some('#')) {
                self.cs.advance_while(1, |char| char != '\n');
            }

            if matches!(self.cs.current_char(), Some('\n')) {
                self.cs.advance_by(1);
            }

            // Skip any whitespace before the string
            self.cs.skip_whitespace();

            // check for any string prefix
            if matches!(
                self.cs.current_char(),
                Some('r' | 'b' | 'f' | 'F' | 'R' | 'B' | 'u' | 'U')
            ) {
                while matches!(self.cs.current_char(), Some(valid_id_noninitial_chars!())) {
                    self.cs.advance_by(1);
                }
            }

            if matches!(self.cs.current_char(), Some('"' | '\'')) {
                let (str, span, str_errors) = self.process_string();

                string.to_mut().push_str(&str);
                str_span.column_end = span.column_end;
                str_span.row_end = span.row_end;

                if let Some(str_errors) = str_errors {
                    errors.extend(str_errors);
                }
            }

            // Skip any whitespace after the string
            self.cs.skip_whitespace();
        }

        self.tokens.push(Token {
            kind: TokenType::String(string),
            span: str_span,
        });

        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }

    fn lex_string_within_parens(&mut self) -> Option<Vec<PythonError>> {
        let mut errors = Vec::new();
        let mut out_string: Cow<str> = Cow::default();

        let start = self.cs.pos();
        while self.cs.current_char().map_or(false, |char| matches!(char, '"' | '\'')) {
            let (string, _, str_errors) = self.process_string();
            out_string.to_mut().push_str(&string);

            if let Some(str_errors) = str_errors {
                errors.extend(str_errors);
            }

            // skip comments in the same line of the string
            self.cs.skip_whitespace();
            if matches!(self.cs.current_char(), Some('#')) {
                self.cs.advance_while(1, |char| char != '\n');
            }

            if matches!(self.cs.current_char(), Some('\\')) {
                self.cs.advance_by(1);
            }

            while self.cs.is_at_eol().is_some() {
                self.cs.advance_by(1);
            }
            self.cs.skip_whitespace();

            while matches!(self.cs.current_char(), Some('#')) {
                // FIXME: add support for \r, \r\n
                self.cs.advance_while(1, |char| char != '\n');
                // skip \n
                self.cs.advance_by(1);
                self.cs.skip_whitespace();
            }

            // check for a str prefix and then consume it.
            if matches!(
                (self.cs.current_char(), self.cs.next_char()),
                (
                    Some('r' | 'b' | 'f' | 'F' | 'R' | 'B' | 'u' | 'U'),
                    Some('\'' | '"' | 'r' | 'R' | 'f' | 'F' | 'b' | 'B')
                )
            ) {
                let start = self.cs.pos();
                while matches!(self.cs.current_char(), Some(valid_id_noninitial_chars!())) {
                    self.cs.advance_by(1);
                }
                let end = self.cs.pos();

                let str_prefix = self.cs.get_slice(start.index, end.index).unwrap();
                if !self.is_str_prefix(str_prefix) {
                    errors.push(PythonError {
                        error: PythonErrorType::Syntax,
                        msg: format!(
                            "SyntaxError: Invalid str prefix \"{}\"",
                            String::from_utf8_lossy(str_prefix)
                        ),
                        span: self.make_span(start, end),
                    });
                }
            }
        }
        let end = self.cs.pos();

        self.tokens.push(Token {
            kind: TokenType::String(out_string),
            span: self.make_span(start, end),
        });

        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }

    fn process_string(&mut self) -> (Cow<'a, str>, Span, Option<Vec<PythonError>>) {
        let mut errors: Vec<PythonError> = vec![];

        let mut start_quote_total = 0;
        let mut end_quote_total = 0;

        // Can be `"` or `'`
        let quote_char = self.cs.current_char().unwrap();

        let start = self.cs.pos();

        if self.is_triple_quote_str_in_pos(start) {
            self.cs.advance_by(3);
            start_quote_total = 3;
            let mut pos = self.cs.pos();

            while !self.is_triple_quote_str_in_pos(pos) {
                if self.cs.current_char().map_or(false, |char| char == '\\')
                    && self.cs.next_char().map_or(false, |char| char == quote_char)
                {
                    // consume "\"
                    self.cs.advance_by(1);
                    if self.is_triple_quote_str_in_pos(self.cs.pos()) {
                        self.cs.advance_by(3);
                    }
                }

                self.cs.advance_by(1);

                if self.cs.current_char().map_or(false, |char| char == quote_char) {
                    pos = self.cs.pos();
                }
            }
        } else {
            self.cs.advance_by(1);
            start_quote_total = 1;

            while self.cs.current_char().map_or(false, |char| char != quote_char) {
                match (self.cs.current_char(), self.cs.next_char()) {
                    // FIXME: support \r\n
                    (Some('\\'), Some('\n' | '\r')) => {
                        self.cs.advance_by(1);
                    }
                    (Some('\\'), Some('\'' | '"' | '\\')) => self.cs.advance_by(2),
                    (Some('\''), Some('\'')) => self.cs.advance_by(2),
                    (Some('"'), Some('"')) => self.cs.advance_by(2),
                    (Some('\\'), _)
                        if self
                            .cs
                            .peek_char(self.cs.pos().index + 2)
                            .map_or(false, |char| char == '\n' || char == '\r') =>
                    {
                        self.cs.advance_by(1);
                        break;
                    }
                    _ => self.cs.advance_by(1),
                }

                if self.cs.current_char().map_or(false, |char| char == quote_char)
                    && self.cs.next_char().map_or(false, |char| char == quote_char)
                {
                    self.cs.advance_by(2);
                }
            }
        }

        // Consume `"` or `'`
        while self.cs.current_char().map_or(false, |char| char == quote_char) {
            self.cs.advance_by(1);
            end_quote_total += 1;
        }
        let end = self.cs.pos();

        if start_quote_total != end_quote_total {
            errors.push(PythonError {
                error: PythonErrorType::Syntax,
                msg: String::from("SyntaxError: unterminated string literal"),
                span: self.make_span(start, end),
            });
        }

        (
            String::from_utf8_lossy(
                self.cs
                    .get_slice(start.index + start_quote_total, end.index - end_quote_total)
                    .unwrap_or_else(|| {
                        panic!(
                            "Failed to get the string slice in pos: ({}, {}) -> ({}, {})",
                            start.row, start.column, end.row, end.column
                        )
                    }),
            ),
            self.make_span(start, end),
            if errors.is_empty() { None } else { Some(errors) },
        )
    }

    fn is_triple_quote_str_in_pos(&self, pos: Position) -> bool {
        matches!(
            (
                self.cs.peek_char(pos.index),
                self.cs.peek_char(pos.index + 1),
                self.cs.peek_char(pos.index + 2)
            ),
            (Some('"'), Some('"'), Some('"')) | (Some('\''), Some('\''), Some('\''))
        )
    }

    fn lex_operator(&mut self) {
        let start = self.cs.pos();
        let (token_type, advance_offset) = match (self.cs.current_char().unwrap(), self.cs.next_char()) {
            ('=', Some('=')) => (TokenType::Operator(OperatorType::Equals), 2),
            ('+', Some('=')) => (TokenType::Operator(OperatorType::PlusEqual), 2),
            ('*', Some('=')) => (TokenType::Operator(OperatorType::AsteriskEqual), 2),
            ('-', Some('=')) => (TokenType::Operator(OperatorType::MinusEqual), 2),
            ('<', Some('=')) => (TokenType::Operator(OperatorType::LessThanOrEqual), 2),
            ('>', Some('=')) => (TokenType::Operator(OperatorType::GreaterThanOrEqual), 2),
            ('^', Some('=')) => (TokenType::Operator(OperatorType::BitwiseXOrEqual), 2),
            ('~', Some('=')) => (TokenType::Operator(OperatorType::BitwiseNotEqual), 2),
            ('!', Some('=')) => (TokenType::Operator(OperatorType::NotEquals), 2),
            ('%', Some('=')) => (TokenType::Operator(OperatorType::ModuloEqual), 2),
            ('&', Some('=')) => (TokenType::Operator(OperatorType::BitwiseAndEqual), 2),
            ('|', Some('=')) => (TokenType::Operator(OperatorType::BitwiseOrEqual), 2),
            ('@', Some('=')) => (TokenType::Operator(OperatorType::AtEqual), 2),
            ('/', Some('=')) => (TokenType::Operator(OperatorType::DivideEqual), 2),
            ('/', Some('/')) => {
                if self
                    .cs
                    .peek_char(self.cs.pos().index + 2)
                    .map_or(false, |char| char == '=')
                {
                    (TokenType::Operator(OperatorType::FloorDivisionEqual), 3)
                } else {
                    (TokenType::Operator(OperatorType::FloorDivision), 2)
                }
            }
            ('<', Some('<')) => {
                if self
                    .cs
                    .peek_char(self.cs.pos().index + 2)
                    .map_or(false, |char| char == '=')
                {
                    (TokenType::Operator(OperatorType::BitwiseLeftShiftEqual), 3)
                } else {
                    (TokenType::Operator(OperatorType::BitwiseLeftShift), 2)
                }
            }
            ('>', Some('>')) => {
                if self
                    .cs
                    .peek_char(self.cs.pos().index + 2)
                    .map_or(false, |char| char == '=')
                {
                    (TokenType::Operator(OperatorType::BitwiseRightShiftEqual), 3)
                } else {
                    (TokenType::Operator(OperatorType::BitwiseRightShift), 2)
                }
            }
            ('*', Some('*')) => {
                if self
                    .cs
                    .peek_char(self.cs.pos().index + 2)
                    .map_or(false, |char| char == '=')
                {
                    (TokenType::Operator(OperatorType::ExponentEqual), 3)
                } else {
                    (TokenType::Operator(OperatorType::Exponent), 2)
                }
            }

            ('%', _) => (TokenType::Operator(OperatorType::Modulo), 1),
            ('&', _) => (TokenType::Operator(OperatorType::BitwiseAnd), 1),
            ('*', _) => (TokenType::Operator(OperatorType::Asterisk), 1),
            ('+', _) => (TokenType::Operator(OperatorType::Plus), 1),
            ('-', _) => (TokenType::Operator(OperatorType::Minus), 1),
            ('<', _) => (TokenType::Operator(OperatorType::LessThan), 1),
            ('=', _) => (TokenType::Operator(OperatorType::Assign), 1),
            ('>', _) => (TokenType::Operator(OperatorType::GreaterThan), 1),
            ('^', _) => (TokenType::Operator(OperatorType::BitwiseXOR), 1),
            ('|', _) => (TokenType::Operator(OperatorType::BitwiseOr), 1),
            ('~', _) => (TokenType::Operator(OperatorType::BitwiseNot), 1),
            ('@', _) => (TokenType::Operator(OperatorType::At), 1),
            ('/', _) => (TokenType::Operator(OperatorType::Divide), 1),

            (char, _) => (TokenType::Invalid(char), 1),
        };

        self.cs.advance_by(advance_offset);
        let end = self.cs.pos();
        self.tokens.push(Token {
            kind: token_type,
            span: self.make_span(start, end),
        });
    }

    // TODO: handle invalid numbers
    fn lex_number(&mut self) -> Option<Vec<PythonError>> {
        let mut errors = vec![];
        let mut number_type = NumberType::Invalid;
        let start = self.cs.pos();

        if self.cs.current_char().unwrap() == '0' && self.cs.next_char().is_some() {
            let next_char = self.cs.next_char().unwrap();
            // Try to lex binary number
            if next_char == 'b' || next_char == 'B' {
                self.cs.advance_by(2);
                self.cs.advance_while(1, |char| matches!(char, '0' | '1' | '_'));
                number_type = NumberType::Integer(IntegerType::Binary);
            }

            // Try to lex hex number
            if next_char == 'x' || next_char == 'X' {
                self.cs.advance_by(2);
                self.cs.advance_while(1, |char| char.is_ascii_hexdigit() || char == '_');
                number_type = NumberType::Integer(IntegerType::Hex);
            }

            // Try to lex octal number
            if next_char == 'o' || next_char == 'O' {
                self.cs.advance_by(2);
                // FIXME: use `is_ascii_octdigit` when stable
                self.cs.advance_while(1, |char| matches!(char, '0'..='7' | '_'));
                number_type = NumberType::Integer(IntegerType::Octal);
            }
        }

        // Try to lex decimal number
        if self.handle_decimal_number().is_some() {
            number_type = NumberType::Integer(IntegerType::Decimal);

            // Check for any character after the digits
            self.cs
                .advance_while(1, |char| matches!(char, valid_id_noninitial_chars!()));

            if let Err(error) = self.check_if_decimal_is_valid_in_pos(start, self.cs.pos()) {
                errors.push(error);
            }
        }

        // Handle float and imaginary numbers
        if self
            .cs
            .current_char()
            .map_or(false, |char| matches!(char, '.' | 'e' | 'E'))
        {
            let (float_or_imaginary, syntax_errors) = self.handle_float_or_imaginary_number(start);
            number_type = float_or_imaginary;
            errors.extend(syntax_errors);
        }

        let end = self.cs.pos();

        let number = self.cs.get_slice(start.index, end.index).unwrap();
        self.tokens.push(Token {
            kind: TokenType::Number(number_type, String::from_utf8_lossy(number)),
            span: self.make_span(start, end),
        });

        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }

    // Handle the fraction part of the float number
    fn handle_float_or_imaginary_number(&mut self, float_start: Position) -> (NumberType, Vec<PythonError>) {
        let mut errors = vec![];
        let mut allow_sign = false;

        // Here we are consuming all the valid characters that a float string can have, we don't care
        // at this point if the string is valid, the checking if the string is a valid float is done
        // later. The goal is to keep the float string in one Token, even if it is an invalid float.
        while self.cs.current_char().map_or(false, |char| {
            char.is_ascii_digit() || matches!(char, '.' | 'e' | 'E' | '_') || (allow_sign && matches!(char, '-' | '+'))
        }) {
            if self
                .cs
                .current_char()
                .map_or(false, |char| char.is_ascii_digit() || matches!(char, '-' | '+'))
            {
                let (decimal_start, decimal_end) = self.handle_decimal_number().unwrap();

                if let Err(error) = self.check_if_decimal_is_valid_in_pos(decimal_start, decimal_end) {
                    errors.push(error);
                }
            } else {
                if matches!(self.cs.current_char(), Some('e' | 'E')) {
                    allow_sign = true;
                }
                self.cs.advance_by(1);
            }
        }

        if matches!(self.cs.current_char(), Some('j' | 'J')) {
            // Consume j or J
            self.cs.advance_by(1);

            // In case we have multiple j's, consume all of them.
            if matches!(self.cs.current_char(), Some('j' | 'J')) {
                self.cs.advance_while(1, |char| matches!(char, 'j' | 'J'));

                if let Err(mut error) = self.check_if_float_number_is_valid_in_pos(float_start, self.cs.pos()) {
                    error.msg = "SyntaxError: invalid imaginary literal".to_string();
                    errors.push(error);
                }
            }

            (NumberType::Imaginary, errors)
        } else {
            if let Err(error) = self.check_if_float_number_is_valid_in_pos(float_start, self.cs.pos()) {
                errors.push(error);
            }
            (NumberType::Float, errors)
        }
    }

    fn handle_decimal_number(&mut self) -> Option<(Position, Position)> {
        let start = self.cs.pos();
        if matches!(self.cs.current_char(), Some('+' | '-')) {
            // Consumer + or -
            self.cs.advance_by(1);
        }

        self.cs.advance_while(1, |char| char.is_ascii_digit());
        let end = self.cs.pos();

        if start == end {
            None
        } else {
            Some((start, end))
        }
    }

    /// check if is a valid decimal e.g.: 123, 1_2_3, 1_2344, +1, -1
    /// invalid decimals: 1_2_, 1__2_3, 1234abc, --42
    fn check_if_decimal_is_valid_in_pos(&self, start: Position, end: Position) -> Result<(), PythonError> {
        const VALID_STATE: i8 = 2;
        const INVALID_STATE: i8 = 0;

        let mut curr_state: i8 = -1;

        for &char in self.cs.get_slice(start.index, end.index).unwrap() {
            if curr_state == INVALID_STATE && char == b'_' {
                break;
            }

            if char.is_ascii_digit() {
                curr_state = VALID_STATE;
            } else {
                curr_state = INVALID_STATE;
            }
        }

        if curr_state == INVALID_STATE {
            return Err(PythonError {
                error: PythonErrorType::Syntax,
                msg: "SyntaxError: invalid decimal literal".to_string(),
                span: self.make_span(start, end),
            });
        }

        Ok(())
    }

    /// Check if is a valid float syntax e.g.: .2, 1.3, 1_000e+40, 24., 3.14E-2, 1e5J
    /// https://docs.python.org/3/reference/lexical_analysis.html#floating-point-literals
    fn check_if_float_number_is_valid_in_pos(&self, start: Position, end: Position) -> Result<(), PythonError> {
        let mut state: u8 = 1;

        // This is a state machine that validates a float number string
        for &char in self.cs.get_slice(start.index, end.index).unwrap() {
            if (state == 4 || state == 1) && char == b'.'
                || state == 7 && matches!(char, b'j' | b'J')
                || state >= 5 && matches!(char, b'e' | b'E')
            {
                state = 0;
                break;
            }

            if state == 4 && matches!(char, b'e' | b'E') {
                state += 1;
                continue;
            }

            if (state == 2 || state == 4) && matches!(char, b'e' | b'E' | b'j' | b'J') {
                state += 3;
            }

            if matches!(state, 2 | 4 | 6) && char.is_ascii_digit() {
                continue;
            }

            if state == 5 && char.is_ascii_digit() || state == 6 && matches!(char, b'j' | b'J') || char.is_ascii_digit()
            {
                state += 1;
            }

            if char == b'.' {
                state += 2;
            }
        }

        if matches!(state, 4 | 6 | 7) {
            Ok(())
        } else {
            Err(PythonError {
                error: PythonErrorType::Syntax,
                msg: "SyntaxError: invalid float literal".to_string(),
                span: self.make_span(start, end),
            })
        }
    }

    fn make_span(&self, start: Position, end: Position) -> Span {
        Span {
            row_start: start.row,
            row_end: end.row,
            // Due to the column value starting at 0, we need to add 1 to get the correct column position
            column_start: start.column + 1,
            column_end: end.column,
        }
    }
}
