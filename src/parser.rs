use crate::lexer::Token;

use std::str::FromStr;

use anyhow::Error;
use logos::Logos;

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
}

use Ast::*;

/// implements .parse() for Ast
impl FromStr for Ast {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err>	{
		let mut result = Func {
			args: None,
			out: vec![],
			debug: false,
		};

		result.add_func();

		for i in Token::lexer(s) {
			match i {
				Token::Val |
				Token::Operator => if let Func { args, out, .. } = &mut result {
					let temp = out.last_mut().unwrap();

					if let Func { args, out, .. } = &mut **temp {
						out.push(Box::new(Val(i)))
					}
				},
				Token::Pipe => result.add_func(),
				_ => (),
			}
		}

		unimplemented!();
	}
}

/// helper functions
impl Ast {
	fn add_func(&mut self) {
		if let Func { args, out, .. } = self {
			out.push(Box::new(Func { args: None, out: vec![], debug: false }))
		}
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
