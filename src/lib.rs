use std::intrinsics::transmute;

extern crate libc;
extern crate pest;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::{Pairs, Pair};
use pest::prec_climber::{PrecClimber, Operator, Assoc};

#[derive(Parser)]
#[grammar = "expression-language.pest"]
pub struct ExpressionParser;

//fn main() {
//    let input = "((5 + 4) / 3) * 3 * (100 / 20 ) - 3";
//    let parsed_string = ExpressionParser::parse(Rule::calculation, input).unwrap();
//    let result:f64 = eval(parsed_string);
//
//    println!("The result is {}", result);
//}

#[no_mangle]
pub extern "C" fn eval_expression(expr_cstr: *const libc::c_char) -> *const libc::c_char {
    let input = cstr_to_str(expr_cstr);
    let parsed_string = ExpressionParser::parse(Rule::calculation, input).unwrap();
    let result:String = format!("{:.2}", eval(parsed_string));

    str_to_cstr(result)
}

// the code below is an example from the pest intro at https://pest.rs/book/intro.html (calculator).
// obviously this is not expression language yet, but there is much similarity
lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use Assoc::Left as L;
        use Assoc::Right as R;
        use Operator as O;

        PrecClimber::new(vec![
            O::new(or, L),
            O::new(and, L),
            O::new(bitwiseOr, L),
            O::new(bitwiseXor, L),
            O::new(bitwiseAnd, L),
            O::new(eq, L) | O::new(identical, L)
                | O::new(neq, L) | O::new(notIdentical, L)
                | O::new(gt, L) | O::new(lt, L)
                | O::new(gte, L) | O::new(lte, L)
                | O::new(notIn, L) | O::new(isIn, L)
                | O::new(matches, L),
            O::new(range, L),
            O::new(add, L) | O::new(subtract, L),
            O::new(concat, L),
            O::new(multiply, L) | O::new(divide, L) | O::new(modulus, L),
            O::new(pow, R)
        ])
    };
}

// this evaluates the expression, considering operator precedence
fn eval(expression: Pairs<Rule>) -> f64 {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => pair.as_str().parse::<f64>().unwrap(),
            Rule::expr => eval(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: f64, op: Pair<Rule>, rhs: f64| match op.as_rule() {
            Rule::add      => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide   => lhs / rhs,
            Rule::pow      => lhs.powf(rhs),
            _ => unreachable!(),
        },
    )
}
// from https://github.com/joelwurtz/cssfilter-php-ffi-example/blob/master/src/lib.rs
fn cstr_to_str(cstr: *const libc::c_char) -> &'static str {
    unsafe {
        let cstring = std::ffi::CStr::from_ptr(cstr);
        let result = cstring.to_str();

        if result.is_err() {
            panic!(
                "Unable to create string for '{}': {}",
                String::from_utf8_lossy(cstring.to_bytes()),
                result.err().unwrap()
            );
        }

        result.unwrap()
    }
}

fn str_to_cstr(str: String) -> *const libc::c_char {
    unsafe {
        let string_result = std::ffi::CString::new(str.as_bytes());

        if string_result.is_err() {
            panic!(
                "Cannot create c string {}: {}",
                str,
                string_result.err().unwrap()
            );
        }

        let data: *const std::ffi::CString = transmute(Box::new(string_result.unwrap()));

        (&*data).as_ptr()
    }
}


