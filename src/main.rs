use structopt::StructOpt;

use std::path::PathBuf;
use std::fs::read_to_string;

#[derive(Debug, StructOpt)]
struct Cla {
	#[structopt(parse(from_os_str))]
	file: PathBuf,
}

mod lexer;
mod parser;

use parser::Ast;

fn main() {
	let cla = Cla::from_args();

	let ast: Ast = read_to_string(cla.file)
		.unwrap()
		.parse()
		.unwrap();
}
