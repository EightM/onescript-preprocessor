use std::collections::HashMap;

/// Перечисление возможных типов токена
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    /// Тип для токена символа "#"
    Hash,
    /// Тип для токена символа "#!"
    Shebang,
    /// Тип для токена представляющего имена областей кода и названия импортируемых библиотек
    Identifier,
    /// Тип для токена представляющего собой простой исходный текст
    Text,
    /// Тип для токенов ключевых слов инструкций препроцессора
    Keyword(Keyword),
    /// Тип для токена который является представлением пути библиотеки в директиве "#Использовать"
    Path,
    /// Тип для токена представляющего текст shebang директивы
    ShebangText,
}

/// Перечисление для ключевых слов директив препроцессора
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    /// Ключевое слово "Использовать"
    Use,
    /// Ключеовое слово "Область"
    Region,
    /// Ключеовое слово "КонецОбласти"
    EndRegion,
}

/// Представление токена разбора исходного текста
#[derive(Debug)]
pub struct Token {
    /// Тип токена
    pub token_kind: TokenKind,
    /// лексема (текст) токена
    pub lexeme: String,
    /// начальный номер строки токена (начинается с 1)
    pub start_line: u16,
    /// Конечный номер строки токена
    pub end_line: u16,
}

/// Представление для соответствия строки ключевого слово к соответствующему типу токена
pub struct KeywordTable {
    /// Таблица соответствия строки ключевого слово к соответствующему типу токена
    pub table: HashMap<String, TokenKind>,
}

impl KeywordTable {
    /// Возвращает таблицу соответствия строки ключевого слово к соответствующему типу токена.
    /// Ключом является строка ключевого слова в верхнем регистре, на русском или английском языке.
    /// Значением является соответствующий [тип](TokenKind) токена
    pub fn new() -> Self {
        let mut table = HashMap::new();
        table.insert("REGION".to_string(), TokenKind::Keyword(Keyword::Region));
        table.insert("ENDREGION".to_string(), TokenKind::Keyword(Keyword::EndRegion));
        table.insert("USE".to_string(), TokenKind::Keyword(Keyword::Use));
        table.insert("ОБЛАСТЬ".to_string(), TokenKind::Keyword(Keyword::Region));
        table.insert("КОНЕЦОБЛАСТИ".to_string(), TokenKind::Keyword(Keyword::EndRegion));
        table.insert("ИСПОЛЬЗОВАТЬ".to_string(), TokenKind::Keyword(Keyword::Use));

        return KeywordTable { table };
    }
}

impl Token {
    /// Конструктор токена
    /// 
    /// # Аргументы 
    /// 
    /// * `token_kind`: тип токена
    /// * `lexeme`: текст токена
    /// * `start_line`: начальная строка токена
    /// * `end_line`: конечная строка токена
    /// 
    /// возвращает: Token 
    ///
    /// 
    /// ```
    pub fn new(token_kind: TokenKind, lexeme: String, start_line: u16, end_line: u16) -> Token {
        Token { token_kind, lexeme, start_line, end_line }
    }
}