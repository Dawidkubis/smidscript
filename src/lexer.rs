use logos::Logos;

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

	#[error]
	#[regex(r"[ \t\n\f]+", logos::skip)]
	Error,
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
