use crate::lexer::Lexer;
use crate::token::TokenKind;

/// Представление препроцессора
pub struct Preprocessor {}

impl Preprocessor {
    /// Возвращает новый экземпляр препроцессора
    pub fn new() -> Self {
        return Preprocessor {};
    }

    /// Обрабатывает переданный исходный текст, разбирая его на токены. Токены инструкций препроцессора
    /// заменяются на пробелы. Остальной текст добавляется без изменений
    ///
    /// # Arguments
    ///
    /// * `source`: исходный текст для разбора
    ///
    /// returns: String исходный текст с удаленными директивами препроцессора
    ///
    /// # Examples
    ///
    /// ```
    ///     use onescript_preprocessor::preprocessor::Preprocessor;
    ///     let example = "#region Test\nProcedure Test()\nEndProcedure\n#EndRegion";
    ///     let expected = " \nProcedure Test()\nEndProcedure\n ";
    ///
    ///     let preprocessor = Preprocessor::new();
    ///     let result = preprocessor.preprocess(example);
    ///     assert_eq!(expected, result);
    /// ```
    pub fn preprocess(&self, source: &str) -> String {
        let mut lexer = Lexer::new();
        let tokens = lexer.lex(source);
        let mut result: String = String::new();

        for token in &tokens {
            match token.token_kind {
                TokenKind::Hash => { result.push(' ') }
                TokenKind::Shebang => { result.push(' ') }
                TokenKind::Text => { result.push_str(&token.lexeme) }
                TokenKind::Keyword(_) => {}
                TokenKind::Path => {}
                TokenKind::ShebangText => {}
                TokenKind::Identifier => {}
            }
        };

        result
    }
}