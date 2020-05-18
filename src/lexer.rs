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
	Error,
}
