use pretty_assertions::assert_eq;
use yara_x_parser::cst::{CSTStream, SyntaxKind};

use crate::tokens::Token::*;
use crate::tokens::{Token, Tokens};
use yara_x_parser::Parser;

#[test]
fn token_generation() {
    let rule = r#"
        rule test {
            strings:
                $a = { 00 01 }
                $b = "foo" ascii wide
            condition:
                $a and $b and foo * 1 == 2
        }"#;

    let events = CSTStream::from(Parser::new(rule.as_bytes()))
        .whitespaces(false)
        .newlines(false);

    let tokens: Vec<Token> = Tokens::new(rule.as_bytes(), events).collect();

    assert_eq!(
        tokens,
        vec![
            Begin(SyntaxKind::SOURCE_FILE),
            Begin(SyntaxKind::RULE_DECL),
            Keyword(b"rule"),
            Identifier(b"test"),
            Punctuation(b"{"),
            Begin(SyntaxKind::PATTERNS_BLK),
            Keyword(b"strings"),
            Punctuation(b":"),
            Begin(SyntaxKind::PATTERN_DEF),
            Identifier(b"$a"),
            Punctuation(b"="),
            Begin(SyntaxKind::HEX_PATTERN),
            Punctuation(b"{"),
            Begin(SyntaxKind::HEX_SUB_PATTERN),
            Literal(b"00"),
            Literal(b"01"),
            End(SyntaxKind::HEX_SUB_PATTERN),
            Punctuation(b"}"),
            End(SyntaxKind::HEX_PATTERN),
            End(SyntaxKind::PATTERN_DEF),
            Begin(SyntaxKind::PATTERN_DEF),
            Identifier(b"$b"),
            Punctuation(b"="),
            Literal(b"\"foo\""),
            Begin(SyntaxKind::PATTERN_MODS),
            Begin(SyntaxKind::PATTERN_MOD),
            Keyword(b"ascii"),
            End(SyntaxKind::PATTERN_MOD),
            Begin(SyntaxKind::PATTERN_MOD),
            Keyword(b"wide"),
            End(SyntaxKind::PATTERN_MOD),
            End(SyntaxKind::PATTERN_MODS),
            End(SyntaxKind::PATTERN_DEF),
            End(SyntaxKind::PATTERNS_BLK),
            Begin(SyntaxKind::CONDITION_BLK),
            Keyword(b"condition"),
            Punctuation(b":"),
            Begin(SyntaxKind::BOOLEAN_EXPR),
            Begin(SyntaxKind::BOOLEAN_TERM),
            Identifier(b"$a"),
            End(SyntaxKind::BOOLEAN_TERM),
            Keyword(b"and"),
            Begin(SyntaxKind::BOOLEAN_TERM),
            Identifier(b"$b"),
            End(SyntaxKind::BOOLEAN_TERM),
            Keyword(b"and"),
            Begin(SyntaxKind::BOOLEAN_TERM),
            Begin(SyntaxKind::EXPR),
            Begin(SyntaxKind::TERM),
            Begin(SyntaxKind::PRIMARY_EXPR),
            Identifier(b"foo"),
            End(SyntaxKind::PRIMARY_EXPR),
            End(SyntaxKind::TERM),
            Literal(b"*"),
            Begin(SyntaxKind::TERM),
            Begin(SyntaxKind::PRIMARY_EXPR),
            Literal(b"1"),
            End(SyntaxKind::PRIMARY_EXPR),
            End(SyntaxKind::TERM),
            End(SyntaxKind::EXPR),
            Literal(b"=="),
            Begin(SyntaxKind::EXPR),
            Begin(SyntaxKind::TERM),
            Begin(SyntaxKind::PRIMARY_EXPR),
            Literal(b"2"),
            End(SyntaxKind::PRIMARY_EXPR),
            End(SyntaxKind::TERM),
            End(SyntaxKind::EXPR),
            End(SyntaxKind::BOOLEAN_TERM),
            End(SyntaxKind::BOOLEAN_EXPR),
            End(SyntaxKind::CONDITION_BLK),
            Punctuation(b"}"),
            End(SyntaxKind::RULE_DECL),
            End(SyntaxKind::SOURCE_FILE),
        ]
    )
}

#[test]
fn whitespaces() {
    let rule = r#"rule test {
        condition:
            true
    }"#;

    let events = CSTStream::from(Parser::new(rule.as_bytes()));
    let tokens: Vec<Token> = Tokens::new(rule.as_bytes(), events).collect();

    assert_eq!(
        tokens,
        vec![
            Begin(SyntaxKind::SOURCE_FILE),
            Begin(SyntaxKind::RULE_DECL),
            Keyword(b"rule"),
            Whitespace,
            Identifier(b"test"),
            Whitespace,
            Punctuation(b"{"),
            Newline,
            Whitespace,
            Whitespace,
            Whitespace,
            Whitespace,
            Whitespace,
            Whitespace,
            Whitespace,
            Whitespace,
            Begin(SyntaxKind::CONDITION_BLK),
            Keyword(b"condition"),
            Punctuation(b":"),
            Newline,
            Whitespace,
            Whitespace,
            Whitespace,
            Whitespace,
            Whitespace,
            Whitespace,
            Whitespace,
            Whitespace,
            Whitespace,
            Whitespace,
            Whitespace,
            Whitespace,
            Begin(SyntaxKind::BOOLEAN_EXPR),
            Begin(SyntaxKind::BOOLEAN_TERM),
            Keyword(b"true"),
            End(SyntaxKind::BOOLEAN_TERM),
            End(SyntaxKind::BOOLEAN_EXPR),
            End(SyntaxKind::CONDITION_BLK),
            Newline,
            Whitespace,
            Whitespace,
            Whitespace,
            Whitespace,
            Punctuation(b"}"),
            End(SyntaxKind::RULE_DECL),
            End(SyntaxKind::SOURCE_FILE),
        ]
    )
}
