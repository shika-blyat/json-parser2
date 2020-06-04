use crate::tokens::{Span, TokenKind};

#[derive(Debug)]
pub struct SyntaxErr {
    pub span: Span,
    pub reason: SyntaxErrReason,
}
#[derive(Debug)]
pub enum SyntaxErrReason {
    UnknownKeyword(String),
    UnexpectedChar(char),
    Expected(String),
    UnexpectedToken(TokenKind),
    UnexpectedEof,
}
