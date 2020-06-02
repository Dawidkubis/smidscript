struct Brackets(Ast);

struct Ast(Vec<Func>);

struct Func {
	args: Option<Vec<>>,
}

