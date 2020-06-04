// There's plenty of error that are just handled silently,
// the error reporting architecture just sucks

use crate::ast::*;
use crate::errors::{SyntaxErr, SyntaxErrReason};
use crate::tokens::{Span, Token, TokenKind};
use std::iter::Peekable;

pub struct Parser<I: Iterator<Item = Token>> {
    pub tokens: Peekable<I>,
}
#[allow(unused_macros)]
macro_rules! tok_kind {
    ($name: ident, $pat: pat, $type: ty, $ret_val: expr) => {
        pub fn $name(&mut self) -> Option<$type> {
            if let Some(Token { kind: $pat, .. }) = self.peek() {
                if let Token { kind: $pat, .. } = self.next().unwrap() {
                    return Some($ret_val);
                }
                unreachable!()
            }
            None
        }
    };
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn json(mut self) -> Result<Value, SyntaxErr> {
        self.value()
    }
    fn value(&mut self) -> Result<Value, SyntaxErr> {
        match self
            .string()
            .or_else(|| self.num())
            .or_else(|| self.bool())
            .or_else(|| self.null())
            .or_else(|| self.array())
        {
            Some(v) => Ok(v),
            None => self.object().or_else(|err| match err.reason {
                SyntaxErrReason::UnexpectedToken(_) => Err(SyntaxErr {
                    reason: match self.peek() {
                        None => SyntaxErrReason::UnexpectedEof,
                        Some(Token { .. }) => SyntaxErrReason::Expected("a value".to_string()),
                    },
                    span: self.next_span(),
                }),
                _ => Err(err),
            }),
        }
    }
    pub fn object(&mut self) -> Result<Value, SyntaxErr> {
        self.lbrace().ok_or_else(|| SyntaxErr {
            reason: match self.peek() {
                None => SyntaxErrReason::UnexpectedEof,
                Some(Token { kind, .. }) => SyntaxErrReason::UnexpectedToken(kind.clone()),
            },
            span: self.next_span(),
        })?;
        let mut pairs = vec![];
        // FIXME raise an error when there is a leading comma
        while let Ok(p) = self.pair() {
            pairs.push(p);
            match self.comma() {
                Some(_) => (),
                None => break,
            }
        }
        self.rbrace().ok_or_else(|| SyntaxErr {
            reason: match self.peek() {
                None => SyntaxErrReason::UnexpectedEof,
                Some(Token { kind, .. }) => SyntaxErrReason::UnexpectedToken(kind.clone()),
            },
            span: self.next_span(),
        })?;
        Ok(Value::Object(Object { values: pairs }))
    }
    pub fn pair(&mut self) -> Result<PairStringValue, SyntaxErr> {
        let s = self.string().ok_or_else(|| SyntaxErr {
            reason: match self.peek() {
                None => SyntaxErrReason::UnexpectedEof,
                Some(Token { kind, .. }) => SyntaxErrReason::UnexpectedToken(kind.clone()),
            },
            span: self.next_span(),
        })?;
        self.semicolon().ok_or_else(|| SyntaxErr {
            reason: SyntaxErrReason::Expected("a semicolon".to_string()),
            span: self.next_span(),
        })?;
        let value = self.value()?;
        if let Value::String(name) = s {
            return Ok(PairStringValue { name, value });
        }
        unreachable!()
    }
    fn array(&mut self) -> Option<Value> {
        self.lbracket()?;
        let mut values = vec![];
        // FIXME raise an error when there is a leading comma
        while let Ok(v) = self.value() {
            values.push(v);
            match self.comma() {
                Some(_) => (),
                None => break,
            }
        }
        self.rbracket()?;
        Some(Value::Array(Array { values }))
    }
    fn next_span(&mut self) -> Span {
        match self.peek() {
            Some(Token { span, .. }) => span.clone(),
            None => std::usize::MAX..std::usize::MAX,
        }
    }
    tok_kind!(string, TokenKind::String(s), Value, Value::String(s));
    tok_kind!(num, TokenKind::Num(n), Value, Value::Number(n));
    tok_kind!(null, TokenKind::Null, Value, Value::Null);
    tok_kind!(bool, TokenKind::Bool(b), Value, Value::Bool(b));
    tok_kind!(lbracket, TokenKind::LBracket, (), ());
    tok_kind!(rbracket, TokenKind::RBracket, (), ());
    tok_kind!(lbrace, TokenKind::LBrace, (), ());
    tok_kind!(rbrace, TokenKind::RBrace, (), ());
    tok_kind!(semicolon, TokenKind::Semicolon, (), ());
    tok_kind!(comma, TokenKind::Comma, (), ());
    fn peek(&mut self) -> Option<&'_ Token> {
        self.tokens.peek()
    }
    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }
}
