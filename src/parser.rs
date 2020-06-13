use crate::lexer::Token;
use crate::error::SmidError;

use std::str::FromStr;

use logos::{Logos, Lexer};

/// an abstraction over Ast
#[derive(Debug)]
pub enum Ast {
	Func {
		args: Option<Vec<Box<Ast>>>,
		out: Vec<Box<Ast>>,
		debug: bool,
	},
	Val(Token),
	Var {
		name: String,
		multiple: bool,
	},
	Input,
	Dot,
	Oper(Operator),
}

#[derive(Debug)]
pub enum Operator {
	Plus,
	Minus,
}

enum State {
	Normal,
	Bracket,
	Var,
}

use Ast::*;
use Operator::*;
use State::*;

/// parsing
impl Ast {
	fn paarse(lex: Lexer<Token>) -> Result<Self, SmidError> {
		let mut result = Func {
			args: None,
			out: vec![],
			debug: false,
		};

		let mut state = Normal;

		result.add_func();

		for i in lex {
			match state {
				Normal => match i {
					Token::Val => result.add_deep(Val(i)),
					Token::Pipe => result.add_func(),
					Token::At => result.add_deep(Input),
					Token::Operator => match lex.slice() {
						"+" => result.add_deep(Oper(Plus)),
						"-" => result.add_deep(Oper(Minus)),
						_ =>
							panic!(
								"please email dawidkubis@hitler.rocks with text: \"idiot\""
								),
						}
					Token::Sep => result.switch_for_args(),
					Token::BracketLeft => (),
					_ => (),
					}
				Bracket => match i {
					
				},
				Var => match i {

				},
			}
		}

		unimplemented!();
	}
}

/// implements .parse() for Ast
impl FromStr for Ast {
	type Err = SmidError;

	fn from_str(s: &str) -> Result<Self, Self::Err>	{
		let lex = Token::lexer(s);

		Self::paarse(lex)
	}
}

/// helper functions
impl Ast {
	/// adds a blank function to the Ast
	fn add_func(&mut self) {
		if let Func { args, out, .. } = self {
			out.push(Box::new(Func { args: None, out: vec![], debug: false }));
		}
	}

	/// adds some Ast to the Ast
	fn add(&mut self, s: Self) {
		if let Func { args, out, .. } = self {
			out.push(Box::new(s));
		}
	}

	/// adds some Ast to the child node
	fn add_deep(&mut self, s: Self) {
		if let Func { args, out, .. } = self {
			let temp = out.last_mut().unwrap();

			if let Func { args, out, .. } = &mut **temp {
				out.push(Box::new(s));
			}
		}
	}
	
	/// moves the `out` to `args`
	fn switch_for_args(&mut self) {
			
	}
}

/// evaluation functions
impl Ast {
	/// evaluation function
	pub fn eval(&self) {

	}
}

/// tests
#[cfg(test)]
mod tests {
	use super::*;

	fn parse_test() {

	}
}
