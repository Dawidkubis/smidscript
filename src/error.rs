use crate::lexer::Token;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SmidError {
	#[error("lexer error")]
	LexerError,
	#[error("parser error")]
	ParseError,
}
