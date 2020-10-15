use crate::lexer::{Token, Luthor};
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
	Val(String),
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

use Ast::*;
use Operator::*;

/// parsing TODO
impl Ast {
	fn paarse<'a>(mut lex: impl Iterator<Item=(Token, &'a str)>) -> Result<Self, SmidError> {
		let mut result = Func {
			args: None,
			out: vec![],
			debug: false,
		};

		result.add_func();

		while let Some((token, val)) = lex.next() {
			match token {
				Token::Val => result.add_deep(Val(val.to_string())),
				Token::Pipe => result.add_func(),
				Token::At => result.add_deep(Input),
				Token::Operator => match val {
					"+" => result.add_deep(Oper(Plus)),
					"-" => result.add_deep(Oper(Minus)),
					_ =>
						panic!(
							"please email dawidkubis@hitler.rocks with text: \"idiot\""
						),
				}
				Token::Sep => result.switch_for_args(),
				Token::BracketLeft => {
					let temp = vec![];

					// TODO

					result.add_deep(Self::paarse(temp.into_iter())?);
				},
				Token::Dot => result.add_deep(Dot),
				Token::Error => (), //TODO
				_ => (),
				
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

		Self::paarse(Luthor::new(lex))
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
