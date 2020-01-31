# expression-language
A runtime for the symfony expression language (from the PHP ecosystem), but faster. Written in Rust

## how to use
At the time, you can use the `eval_expression` function from PHP via the experimental _FFI_ feature.
Only a portion of the expression language is implemented so far (arithmetic expressions).
You can run the example by executing the following steps (tested for Ubuntu):

* `git clone https://github.com/bar9/expression-language.git`
* `cd expression-language && cargo build`
* `docker run -v $PWD/php:/app composer install`
* `docker build . -t php-ffi`
* `docker run -v $PWD:/app php-ffi php /app/php/expressions.php`

Stay tuned, this is still under initial development.

## motivation
The symfony expression language is a common denominator for many PHP projects and is widely
used to express business rules and other customization definitions. It provides a lexer and
parser for extendability, but PHP ist not the best fit for these tasks performance-wise.
Xml and json parsing are done by compiled binaries, this pattern could be applied to
the expression language as well.

## speed
At the time of  writing, the Rust ecosystem offers several interesting parser libraries,
*nom* and *pest* being the most promising options. nom is reported to be faster, pest seems
more usable to me. That's why -- for now -- I am going with pest. The goal so far is to be
significantly faster than PHP for the complete parse -> evaluate loop.

## PHP integration
With 7.4, PHP provides a C-ABI FFI.
Kudos to Joel Wurtz for his [CSS Filter Example](https://github.com/joelwurtz/cssfilter-php-ffi-example)

See the example provided under _php_. The Rust library is already faster than the symfony component, even
for small input. Considering the overhead of string serialization across the FFI, I am
pretty optimistic about the potential of this.
