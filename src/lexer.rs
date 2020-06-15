use logos::{Logos, Lexer};

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
	#[regex(r"[a-zA-Z0-9]")]
	Val,

	#[token("|")]
	Pipe,

	#[token(":")]
	Sep,

	#[token("$")]
	Dol,

	#[token("..")]
	DotDot,

	#[regex(r"[+-]")]
	Operator,

	#[token("(")]
	BracketLeft,

	#[token(")")]
	BracketRight,

	#[token("@")]
	At,

	#[token(".")]
	Dot,

	#[token(" ")]
	Space,

	#[error]
	#[regex(r"[\t\n\f]+", logos::skip)]
	Error,
}

// TODO generalise
pub struct Luthor<'a> {
	lex: Lexer<'a, Token>,
}

impl<'a> Luthor<'a> {
	pub fn new(lex: Lexer<'a, Token>) -> Self {
		Self { lex }
	}
}

impl<'a> Iterator for Luthor<'a> {
	type Item = (Token, &'a str);

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(s) = self.lex.next() {
			Some((s, self.lex.slice()))
		} else {
			None
		}
	}
}

/// tests
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn usage_of_all_tokens() {
		let mut lex = Token::lexer(
			"@aaa | a:.s | $x: $x \n| ($x:$x) | $x: (4 + 3 - 2 | $x). | $..x : $x"
			);
		assert_eq!(lex.next(), Some(Token::At));
		assert_eq!(lex.next(), Some(Token::Val));
		assert_eq!(lex.next(), Some(Token::Val));
		assert_eq!(lex.next(), Some(Token::Val));

		assert_eq!(lex.next(), Some(Token::Pipe));

		assert_eq!(lex.next(), Some(Token::Val));
		assert_eq!(lex.next(), Some(Token::Sep));
		assert_eq!(lex.next(), Some(Token::Dot));
		assert_eq!(lex.next(), Some(Token::Val));

		assert_eq!(lex.next(), Some(Token::Pipe));

		assert_eq!(lex.next(), Some(Token::Dol));
		assert_eq!(lex.next(), Some(Token::Val));
		assert_eq!(lex.next(), Some(Token::Sep));
		assert_eq!(lex.next(), Some(Token::Dol));
		assert_eq!(lex.next(), Some(Token::Val));

		assert_eq!(lex.next(), Some(Token::Pipe));

		assert_eq!(lex.next(), Some(Token::BracketLeft));
		assert_eq!(lex.next(), Some(Token::Dol));
		assert_eq!(lex.next(), Some(Token::Val));
		assert_eq!(lex.next(), Some(Token::Sep));
		assert_eq!(lex.next(), Some(Token::Dol));
		assert_eq!(lex.next(), Some(Token::Val));
		assert_eq!(lex.next(), Some(Token::BracketRight));

		assert_eq!(lex.next(), Some(Token::Pipe));

		assert_eq!(lex.next(), Some(Token::Dol));
		assert_eq!(lex.next(), Some(Token::Val));
		assert_eq!(lex.next(), Some(Token::Sep));
		assert_eq!(lex.next(), Some(Token::BracketLeft));
		assert_eq!(lex.next(), Some(Token::Val));
		assert_eq!(lex.next(), Some(Token::Operator));
		assert_eq!(lex.next(), Some(Token::Val));
		assert_eq!(lex.next(), Some(Token::Operator));
		assert_eq!(lex.next(), Some(Token::Val));
		assert_eq!(lex.next(), Some(Token::Pipe));
		assert_eq!(lex.next(), Some(Token::Dol));
		assert_eq!(lex.next(), Some(Token::Val));
		assert_eq!(lex.next(), Some(Token::BracketRight));
		assert_eq!(lex.next(), Some(Token::Dot));

		assert_eq!(lex.next(), Some(Token::Pipe));

		assert_eq!(lex.next(), Some(Token::Dol));
		assert_eq!(lex.next(), Some(Token::DotDot));
		assert_eq!(lex.next(), Some(Token::Val));
		assert_eq!(lex.next(), Some(Token::Sep));
		assert_eq!(lex.next(), Some(Token::Dol));
		assert_eq!(lex.next(), Some(Token::Val));

	}
}
