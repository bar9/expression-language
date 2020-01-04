# expression-language
A runtime for the symfony expression language (from the PHP ecosystem), but written in Rust

## how to use
Stay tuned, this is still under initial development. There is a binary that prints a number
to the terminal from a hard-coded expression, that you can run by:
* `git clone https://github.com/bar9/expression-language.git`
* `cd expression-language && cargo run`

## motivation
The symfony expression language is a common denominator for many PHP projects and is widely
used to express business rules and other customization definitions. It provides a lexer and
parser for extendability, but PHP ist not the best fit for these tasks performance-wise.
Xml and json parsing are done by compiled binaries as well, this pattern could be applied to
the expression language as well.

## speed
At the time of this writing, the Rust ecosystem offers several interesting parser libraries,
*nom* and *pest* being the most promising options. nom is reported to be faster, pest seems
more usable to me. That's why -- for now -- I am going with pest. The goal so far is to be
significantly faster than PHP for the complete parse -> evaluate loop.

## PHP integration
With 7.4, PHP provides a C-ABI FFI. This could enable us to use expression-language from PHP,
hopefully being faster.
