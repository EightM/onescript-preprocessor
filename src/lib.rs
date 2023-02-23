#![warn(missing_docs)]
//! Реализация препроцессора для языка программирования OneScript.
//! Позволяет разобрать исходный текст программ на OneScript, обработать и удалить директивы препроцессора

/// Модуль содержащий API для работы с лексером
pub mod lexer;

/// Модуль для имплементации токенов разбора исходного текста
pub mod token;

/// Модуль содержащий API для работы с препроцессором
pub mod preprocessor;