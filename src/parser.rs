use crate::lexer::Token;

use std::str::FromStr;

use anyhow::Error;

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
}

impl FromStr for Ast {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err>	{
		let s = s.split("|");

		unimplemented!();
	}
}

impl Ast {
	fn eval(&self) {

	}
}

/// tests
#[cfg(test)]
mod tests {
	use super::*;
}
