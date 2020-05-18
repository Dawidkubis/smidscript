# smidscript
esoteric programming language based on substitution

The idea is to form a functional language based on substitution.
Only data types are strings, since they can be treated as arrays or numbers.
There exist constant functions. 
Here's how they are created: `hello` is a function that takes no arguments and outputs: "hello".
Other functions are based on pattern matching. `a:s` is a function that calls another function on every occurence of "a"
that outputs "s".
This `|` is a pipe.
Therefore `aaa|a:b` outputs "bbb".
These ideas are extended further with inputs, match variables,
ranges, math operations and debug calls.
Infinite arrays are a planned feature, althought tricky to implement.

This repository will be dedicated to creating an interpreter for the language,
I might create a compiler in the future.

## Language Description

### Bottom level tokens
+ `a-zA-Z0-9`
+ `|`
+ `\`
+ `:`
+ `$`
+ `..`
+ `+-*/`
+ `(`
+ `)`
+ `@`
+ `.`

### Token explanation
+ `a-zA-Z0-9` are values
+ `|` is a pipe, it's used to pass outputs to another function.
+ `\` is a backslash, used as an escape character.
+ `:` divides the match pattern from the output.
+ `$` is used with a letter afterwards to indicate that any character shall be matched.
The character is then accessible with a call `$var`, where "var" is its name.
+ `..` is used to specify either: a range of numbers/letters or an infinite range or,
if put after `$` that multiple characters shall be matched. Note that a range of numbers
between 0 and 5 looks like this `(0)(1)(2)(3)(4)(5)`.
+ `+-*/` are arithmetic operators. They can only be used between numbers in brackets.
A `*` before a bracket unpacks it.
+ `()` are used to focus evaluation. They can also be used as values, since they are matched
as a single value.
+ `@` is used as a placeholder for user input from stdin.
+ `.` when used directly after `:` specifies that the output of that function is
to be printed.

### Parsing
Sections of the syntax are divided by `|`.
First, evaluation of the first section occurs.
Evaluation of a section is as follows: if there is a pattern, try matching it;
else a function is constant and the value is to be stored, transferring it onto the next
section. If matching is possible then for each possible match add output. No `$` variables
may be empty in a match, if that is not possible that output matcherror.
`$..` always matches as much as it possibly can.
Outputs are evaluated using values, variables, math, inputs and with respect to brackets.
If a function is marked as debug, the output is printed before it is passed.
If a function does not pass, the output is printed.

## TODO
+ lexer
+ parser
+ interactive mode
+ comments?
+ error messages

## Planned versions
+ 0.2.0 - working matching and piping + interactive mode
+ 0.3.0 - working variables and math
+ 0.4.0 - working ranges, inputs and debugs
+ 1.0.0 - hopefully everything except infinite ranges working,
they're gonna be tricky to implement.
