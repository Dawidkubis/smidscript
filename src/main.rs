use structopt::StructOpt;

use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Cla {
	#[structopt(parse(from_os_str))]
	file: Option<PathBuf>,
}

mod lexer;
mod parser;

fn main() {
	let cla = Cla::from_args();
}
