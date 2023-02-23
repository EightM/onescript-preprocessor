use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;
use crate::token::{KeywordTable, Token, TokenKind};

/// Лексер, разбирающий исходный текст на токены
pub struct Lexer {
    current_line: u16,
}

impl Lexer {
    /// Создает новый экземпляр лексера. Отсчет номеров строк токенов начинается с 1.
    pub fn new() -> Self { Lexer { current_line: 1 } }

    /// Осуществляет лексинг переданного исходного текста. Возвращает вектор токенов, включающих в себя
    /// как простой текст, так и токены инструкций препроцессора
    pub fn lex(&mut self, source: &str) -> Vec<Token> {
        let mut chars = source.chars().peekable();
        let mut tokens: Vec<Token> = Vec::new();
        let keywords_table = KeywordTable::new();

        while let Some(char) = chars.peek() {
            match char {
                '#' => {
                    chars.next();

                    if match_char('!', &mut chars) {
                        let token = Token::new(TokenKind::Shebang, "#!".to_string(), self
                            .current_line, self.current_line);
                        tokens.push(token);
                        Lexer::shebang_text(self, &mut tokens, &mut chars);
                    } else {
                        let token = Token::new(TokenKind::Hash, "#".to_string(), self
                            .current_line, self.current_line);
                        tokens.push(token);
                        Lexer::preprocessor_line(self, &mut tokens, &mut chars, &keywords_table);
                    }
                }
                _ => {
                    Lexer::text(self, &mut tokens, &mut chars);
                }
            }
        }

        tokens
    }

    fn shebang_text(&mut self, tokens: &mut Vec<Token>, chars: &mut Peekable<Chars>) {
        let mut text_chars: Vec<char> = Vec::new();
        let start_line = self.current_line;
        let end_line = self.current_line;

        while let Some(char) = chars.peek() {
            match char {
                '\n' => {
                    break;
                }
                _ => {
                    text_chars.push(*char);
                    chars.next();
                }
            }
        }

        if !text_chars.is_empty() {
            let token = Token::new(TokenKind::Text, text_chars.iter().collect(), start_line, end_line);
            tokens.push(token);
        }
    }

    fn preprocessor_line(&mut self, tokens: &mut Vec<Token>, chars: &mut Peekable<Chars>, keywords: &KeywordTable) {
        while let Some(char) = chars.peek() {
            match char {
                '\n' => {
                    break;
                }
                char if char.is_alphabetic() || *char == '_' => {
                    let token = Lexer::identifier(self, chars, &keywords.table);
                    tokens.push(token);
                }
                '"' => {
                    let mut token = Lexer::string(self, chars);
                    token.token_kind = TokenKind::Path;
                    tokens.push(token);
                }
                _ => {
                    chars.next();
                }
            }
        }
    }

    fn identifier(&mut self, chars: &mut Peekable<Chars>, keywords: &HashMap<String, TokenKind>) -> Token {
        let mut text_chars: Vec<char> = Vec::new();

        while let Some(char) = chars.peek() {
            if char.is_alphabetic() || *char == '_' {
                text_chars.push(*char);
                chars.next();
            } else {
                break;
            }
        }

        let identifier: String = String::from_iter(text_chars);
        if let Some(token_kind) = keywords.get(identifier.to_uppercase().as_str()) {
            return Token::new(*token_kind, identifier, self.current_line, self.current_line);
        }

        return Token::new(TokenKind::Identifier, identifier, self.current_line, self.current_line);
    }

    fn text(&mut self, tokens: &mut Vec<Token>, chars: &mut Peekable<Chars>) {
        let mut text_chars: Vec<char> = Vec::new();
        let mut string_or_date = false;
        let start_line = self.current_line;
        let mut end_line = self.current_line;

        while let Some(char) = chars.peek() {
            match char {
                '#' => {
                    if !string_or_date {
                        break;
                    } else {
                        text_chars.push(*char);
                        chars.next();
                    }
                }
                '"' | '\'' => {
                    if string_or_date == false {
                        string_or_date = true
                    } else {
                        string_or_date = false
                    }
                    text_chars.push(*char);
                    chars.next();
                }
                '\n' => {
                    self.current_line = self.current_line + 1;
                    end_line = end_line + 1;
                    text_chars.push(*char);
                    chars.next();
                }
                _ => {
                    text_chars.push(*char);
                    chars.next();
                }
            }
        }

        let token = Token::new(TokenKind::Text, text_chars.into_iter().collect(),
                               start_line, end_line);
        tokens.push(token);
    }

    fn string(&mut self, chars: &mut Peekable<Chars>) -> Token {
        let mut text_chars: Vec<char> = Vec::new();
        // add first quote symbol
        text_chars.push(chars.next().unwrap());
        let start_line = self.current_line;
        let mut end_line = self.current_line;

        while let Some(char) = chars.next() {
            match char {
                char if char == '"' => {
                    text_chars.push(char);
                    break;
                }
                '\n' => {
                    self.current_line = self.current_line + 1;
                    end_line = end_line + 1;
                    text_chars.push(char);
                }
                _ => { text_chars.push(char) }
            }
        }

        Token::new(TokenKind::Text, text_chars.into_iter().collect(), start_line, end_line)
    }
}

fn match_char(expected: char, chars: &mut Peekable<Chars>) -> bool {
    let next_char = chars.peek();

    if next_char.is_none() {
        return false;
    }

    if next_char.is_some() && *next_char.unwrap() != expected {
        return false;
    }

    // Consume matched character
    chars.next();
    true
}