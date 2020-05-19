use logos::Logos;

// perhaps lower the amount of tokens?

#[derive(Logos, Debug, PartialEq)]
enum Token {
	#[regex(r"[a-zA-Z0-9]")]
	Val,

	#[token("|")]
	Pipe,

	#[token("\\")]
	BackSlash,

	#[token(":")]
	Sep,

	#[token("$")]
	Dol,

	#[token("..")]
	DotDot,

	#[token("+")]
	Plus,

	#[token("-")]
	Minus,

	#[token("*")]
	Times,

	#[token("/")]
	Slash,

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

// tests

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn identity_func() {

		let mut lex = Token::lexer("$x : $x");
		assert_eq!(lex.next(), Some(Token::Dol));
		assert_eq!(lex.next(), Some(Token::Val));
		assert_eq!(lex.next(), Some(Token::Sep));
		assert_eq!(lex.next(), Some(Token::Dol));
		assert_eq!(lex.next(), Some(Token::Val));
		assert_eq!(lex.next(), None);
	}

	#[test]
	fn usage_of_all_tokens() {
		//TODO
	}
}
