use crate::{
    errors::{SyntaxErr, SyntaxErrReason},
    tokens::{Token, TokenKind},
};
use std::{convert::TryInto, str::Chars};

pub struct Lexer<'a> {
    s: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer { s: s.chars() }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        self.s.next()
    }
}

pub enum LexerState {
    InString,
    InNumber,
    InKeyword,
    None,
}
impl<'a> TryInto<Vec<Token>> for Lexer<'a> {
    type Error = SyntaxErr;
    fn try_into(self) -> Result<Vec<Token>, SyntaxErr> {
        let mut result_tokens = vec![];
        let mut lexer_state = LexerState::None;
        let mut current_tok = String::new();
        for (pos, c) in self.into_iter().enumerate() {
            match lexer_state {
                LexerState::InNumber => {
                    if c.is_ascii_digit() {
                        current_tok.push(c);
                        continue;
                    } else {
                        result_tokens.push(Token {
                            span: pos - current_tok.len()..pos,
                            kind: TokenKind::Num(current_tok.parse().unwrap()),
                        });
                        lexer_state = LexerState::None;
                        current_tok = String::new();
                    }
                }
                LexerState::InString => {
                    if c != '"' {
                        current_tok.push(c);
                        continue;
                    } else {
                        result_tokens.push(Token {
                            span: pos - current_tok.len()..pos,
                            kind: TokenKind::String(current_tok),
                        });
                        lexer_state = LexerState::None;
                        current_tok = String::new();
                        continue;
                    }
                }
                LexerState::InKeyword => {
                    if c.is_ascii_alphabetic() {
                        current_tok.push(c);
                        continue;
                    } else {
                        let kind = match current_tok.as_str() {
                            "true" => TokenKind::Bool(true),
                            "false" => TokenKind::Bool(false),
                            "null" => TokenKind::Null,
                            s => {
                                return Err(SyntaxErr {
                                    span: pos - s.len()..pos,
                                    reason: SyntaxErrReason::UnknownKeyword(current_tok),
                                })
                            }
                        };
                        result_tokens.push(Token {
                            span: pos - current_tok.len()..pos,
                            kind,
                        });
                        lexer_state = LexerState::None;
                        current_tok = String::new();
                    }
                }
                LexerState::None => (),
            }
            match c {
                '"' => {
                    lexer_state = LexerState::InString;
                }
                c if c.is_ascii_digit() => {
                    lexer_state = LexerState::InNumber;
                    current_tok.push(c);
                }
                c if c.is_ascii_alphabetic() => {
                    lexer_state = LexerState::InKeyword;
                    current_tok.push(c);
                }
                '[' => result_tokens.push(Token {
                    span: pos..pos + 1,
                    kind: TokenKind::LBracket,
                }),
                ']' => result_tokens.push(Token {
                    span: pos..pos + 1,
                    kind: TokenKind::RBracket,
                }),
                '{' => result_tokens.push(Token {
                    span: pos..pos + 1,
                    kind: TokenKind::LBrace,
                }),
                '}' => result_tokens.push(Token {
                    span: pos..pos + 1,
                    kind: TokenKind::RBrace,
                }),
                ',' => result_tokens.push(Token {
                    span: pos..pos + 1,
                    kind: TokenKind::Comma,
                }),
                ':' => result_tokens.push(Token {
                    span: pos..pos + 1,
                    kind: TokenKind::Semicolon,
                }),
                ' ' | '\t' | '\n' | '\r' => (),
                c => {
                    return Err(SyntaxErr {
                        span: pos..pos + 1,
                        reason: SyntaxErrReason::UnexpectedChar(c),
                    })
                }
            }
        }
        match lexer_state {
            LexerState::InNumber => result_tokens.push(Token {
                span: std::usize::MAX - current_tok.len()..std::usize::MAX,
                kind: TokenKind::Num(current_tok.parse().unwrap()),
            }),
            LexerState::InString => result_tokens.push(Token {
                span: std::usize::MAX - current_tok.len()..std::usize::MAX,
                kind: TokenKind::String(current_tok),
            }),
            LexerState::InKeyword => {
                let kind = match current_tok.as_str() {
                    "true" => TokenKind::Bool(true),
                    "false" => TokenKind::Bool(false),
                    "null" => TokenKind::Null,
                    s => {
                        return Err(SyntaxErr {
                            span: std::usize::MAX - s.len()..std::usize::MAX,
                            reason: SyntaxErrReason::UnknownKeyword(current_tok),
                        })
                    }
                };
                result_tokens.push(Token {
                    span: std::usize::MAX - current_tok.len()..std::usize::MAX,
                    kind,
                });
            }
            LexerState::None => (),
        }
        Ok(result_tokens)
    }
}
