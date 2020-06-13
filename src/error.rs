use crate::lexer::Token;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SmidError {
	#[error("lexer error - token {0:?} not understood")]
	LexerError(Token),
	#[error("parser error")]
	ParseError,
}
