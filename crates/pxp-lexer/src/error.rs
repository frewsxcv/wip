use std::fmt::Display;

use pxp_span::Span;

pub type SyntaxResult<T> = Result<T, SyntaxError>;

#[derive(Debug, Eq, PartialEq)]
pub enum SyntaxError {
    UnexpectedEndOfFile(Span),
    UnexpectedError(Span),
    UnexpectedCharacter(u8, Span),
    InvalidHaltCompiler(Span),
    InvalidOctalEscape(Span),
    InvalidOctalLiteral(Span),
    InvalidUnicodeEscape(Span),
    UnpredictableState(Span),
    InvalidDocIndentation(Span),
    InvalidDocBodyIndentationLevel(usize, Span),
    UnrecognisedToken(u8, Span),
}

impl SyntaxError {
    pub fn span(&self) -> Span {
        match self {
            Self::UnexpectedEndOfFile(span) => *span,
            Self::UnexpectedError(span) => *span,
            Self::UnexpectedCharacter(_, span) => *span,
            Self::InvalidHaltCompiler(span) => *span,
            Self::InvalidOctalEscape(span) => *span,
            Self::InvalidOctalLiteral(span) => *span,
            Self::InvalidUnicodeEscape(span) => *span,
            Self::UnpredictableState(span) => *span,
            Self::InvalidDocIndentation(span) => *span,
            Self::InvalidDocBodyIndentationLevel(_, span) => *span,
            Self::UnrecognisedToken(_, span) => *span,
        }
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEndOfFile(span) => write!(
                f,
                "Syntax Error: unexpected end of file on line {} column {}",
                span.start.line, span.start.column
            ),
            Self::UnexpectedError(span) => write!(
                f,
                "Syntax Error: unexpected error on line {} column {}",
                span.start.line, span.start.column
            ),
            Self::UnexpectedCharacter(char, span) => write!(
                f,
                "Syntax Error: unexpected character `{:?}` on line {} column {}",
                *char as char, span.start.line, span.start.column
            ),
            Self::InvalidHaltCompiler(span) => write!(
                f,
                "Syntax Error: invalid halt compiler on line {} column {}",
                span.start.line, span.start.column
            ),
            Self::InvalidOctalEscape(span) => write!(
                f,
                "Syntax Error: invalid octal escape on line {} column {}",
                span.start.line, span.start.column
            ),
            Self::InvalidOctalLiteral(span) => write!(
                f,
                "Syntax Error: invalid octal literal on line {} column {}",
                span.start.line, span.start.column
            ),
            Self::InvalidUnicodeEscape(span) => write!(
                f,
                "Syntax Error: invalid unicode escape on line {} column {}",
                span.start.line, span.start.column
            ),
            Self::UnpredictableState(span) => write!(
                f,
                "Syntax Error: Reached an unpredictable state on line {} column {}",
                span.start.line, span.start.column
            ),
            Self::InvalidDocIndentation(span) => write!(
                f,
                "Syntax Error: Invalid indentation - cannot use tabs and spaces on line {}",
                span.start.line
            ),
            Self::InvalidDocBodyIndentationLevel(expected, span) => write!(
                f,
                "Syntax Error: Invalid body indentation level - expecting an indentation level of at least {} on line {}",
                expected,
                span.start.line
            ),
            Self::UnrecognisedToken(token, span) => write!(
                f,
                "Syntax Error: Unrecognised token {} on line {} column {}",
                token,
                span.start.line,
                span.start.column
            )
        }
    }
}
