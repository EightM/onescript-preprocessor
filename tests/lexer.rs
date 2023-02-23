use onescript_preprocessor::lexer::Lexer;
use onescript_preprocessor::token::{Keyword, TokenKind};

#[test]
fn shebang() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("#!");
    assert_eq!(tokens.len(), 1);

    let token = tokens.get(0).unwrap();
    assert_eq!(token.token_kind, TokenKind::Shebang);
    assert_eq!(token.lexeme, "#!");
    assert_eq!(token.start_line, 1);
    assert_eq!(token.end_line, 1);
}

#[test]
fn hash() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("#");
    assert_eq!(tokens.len(), 1);

    let token = tokens.get(0).unwrap();
    assert_eq!(token.token_kind, TokenKind::Hash);
    assert_eq!(token.lexeme, "#");
    assert_eq!(token.start_line, 1);
    assert_eq!(token.end_line, 1);
}

#[test]
fn shebang_text_with_new_line() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("#!test/test -t\n");
    assert_eq!(tokens.len(), 3);

    let first = tokens.get(0).unwrap();
    assert_eq!(first.token_kind, TokenKind::Shebang);
    assert_eq!(first.lexeme, "#!".to_string());
    assert_eq!(first.start_line, 1);
    assert_eq!(first.end_line, 1);

    let second = tokens.get(1).unwrap();
    assert_eq!(second.token_kind, TokenKind::Text);
    assert_eq!(second.lexeme, "test/test -t");
    assert_eq!(second.start_line, 1);
    assert_eq!(second.end_line, 1);

    let third = tokens.get(2).unwrap();
    assert_eq!(third.token_kind, TokenKind::Text);
    assert_eq!(third.lexeme, "\n");
    assert_eq!(third.start_line, 1);
    assert_eq!(third.end_line, 2);
}

#[test]
fn shebang_text_without_new_line() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("#!test/test -t");
    assert_eq!(tokens.len(), 2);

    let first = tokens.get(0).unwrap();
    assert_eq!(first.token_kind, TokenKind::Shebang);
    assert_eq!(first.lexeme, "#!".to_string());
    assert_eq!(first.start_line, 1);
    assert_eq!(first.end_line, 1);

    let second = tokens.get(1).unwrap();
    assert_eq!(second.token_kind, TokenKind::Text);
    assert_eq!(second.lexeme, "test/test -t");
    assert_eq!(second.start_line, 1);
    assert_eq!(second.end_line, 1);
}

#[test]
fn region_eng() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("#region");
    assert_eq!(tokens.len(), 2);

    let first = tokens.get(0).unwrap();
    let second = tokens.get(1).unwrap();

    assert_eq!(first.token_kind, TokenKind::Hash);
    assert_eq!(first.lexeme, "#".to_string());
    assert_eq!(first.start_line, 1);
    assert_eq!(first.end_line, 1);

    assert_eq!(second.token_kind, TokenKind::Keyword(Keyword::Region));
    assert_eq!(second.lexeme, "region");
    assert_eq!(second.start_line, 1);
    assert_eq!(second.end_line, 1);
}

#[test]
fn region_ru() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("#  область");
    assert_eq!(tokens.len(), 2);

    let first = tokens.get(0).unwrap();
    let second = tokens.get(1).unwrap();

    assert_eq!(first.token_kind, TokenKind::Hash);
    assert_eq!(first.lexeme, "#".to_string());
    assert_eq!(first.start_line, 1);
    assert_eq!(first.end_line, 1);

    assert_eq!(second.token_kind, TokenKind::Keyword(Keyword::Region));
    assert_eq!(second.lexeme, "область");
    assert_eq!(second.start_line, 1);
    assert_eq!(second.end_line, 1);
}

#[test]
fn end_region_eng() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("#endRegion");
    assert_eq!(tokens.len(), 2);

    let first = tokens.get(0).unwrap();
    let second = tokens.get(1).unwrap();

    assert_eq!(first.token_kind, TokenKind::Hash);
    assert_eq!(first.lexeme, "#".to_string());
    assert_eq!(first.start_line, 1);
    assert_eq!(first.end_line, 1);

    assert_eq!(second.token_kind, TokenKind::Keyword(Keyword::EndRegion));
    assert_eq!(second.lexeme, "endRegion");
    assert_eq!(second.start_line, 1);
    assert_eq!(second.end_line, 1);
}

#[test]
fn end_region_ru() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("#  КонецОбласти");
    assert_eq!(tokens.len(), 2);

    let first = tokens.get(0).unwrap();
    let second = tokens.get(1).unwrap();

    assert_eq!(first.token_kind, TokenKind::Hash);
    assert_eq!(first.lexeme, "#".to_string());
    assert_eq!(first.start_line, 1);
    assert_eq!(first.end_line, 1);

    assert_eq!(second.token_kind, TokenKind::Keyword(Keyword::EndRegion));
    assert_eq!(second.lexeme, "КонецОбласти");
    assert_eq!(second.start_line, 1);
    assert_eq!(second.end_line, 1);
}

#[test]
fn region_identifier() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("#region TestRegion\n");
    assert_eq!(tokens.len(), 4);

    let first = tokens.get(0).unwrap();
    let second = tokens.get(1).unwrap();
    let third = tokens.get(2).unwrap();
    let forth = tokens.get(3).unwrap();

    assert_eq!(first.token_kind, TokenKind::Hash);
    assert_eq!(first.lexeme, "#".to_string());
    assert_eq!(first.start_line, 1);
    assert_eq!(first.end_line, 1);

    assert_eq!(second.token_kind, TokenKind::Keyword(Keyword::Region));
    assert_eq!(second.lexeme, "region".to_string());
    assert_eq!(second.start_line, 1);
    assert_eq!(second.end_line, 1);

    assert_eq!(third.token_kind, TokenKind::Identifier);
    assert_eq!(third.lexeme, "TestRegion");
    assert_eq!(third.start_line, 1);
    assert_eq!(third.end_line, 1);

    assert_eq!(forth.token_kind, TokenKind::Text);
    assert_eq!(forth.lexeme, "\n");
    assert_eq!(forth.start_line, 1);
    assert_eq!(forth.end_line, 2);
}

#[test]
fn text_after_region() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("#region test\r\ntext");
    assert_eq!(tokens.len(), 4);

    let text_token = tokens.get(3).unwrap();

    assert_eq!(text_token.token_kind, TokenKind::Text);
    assert_eq!(text_token.lexeme, "\ntext");
    assert_eq!(text_token.start_line, 1);
    assert_eq!(text_token.end_line, 2);
}

#[test]
fn use_with_quotes() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("#use \"../core\"");

    assert_eq!(tokens.len(), 3);
    let use_id = tokens.get(2).unwrap();

    assert_eq!(use_id.token_kind, TokenKind::Path);
    assert_eq!(use_id.lexeme, "\"../core\"");
    assert_eq!(use_id.start_line, 1);
    assert_eq!(use_id.end_line, 1);
}

#[test]
fn test_region_name_with_underscore() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("#reion test_region");

    assert_eq!(tokens.len(), 3);
    let identifier = tokens.get(2).unwrap();

    assert_eq!(identifier.token_kind, TokenKind::Identifier);
    assert_eq!(identifier.lexeme, "test_region");
    assert_eq!(identifier.start_line, 1);
    assert_eq!(identifier.end_line, 1);
}

#[test]
fn test_multiply_line_counter() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("test\ntest\ntest");

    assert_eq!(tokens.len(), 1);
    let text = tokens.get(0).unwrap();

    assert_eq!(text.token_kind, TokenKind::Text);
    assert_eq!(text.lexeme, "test\ntest\ntest");
    assert_eq!(text.start_line, 1);
    assert_eq!(text.end_line, 3);
}