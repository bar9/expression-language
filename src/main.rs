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

fn main() {
    let input = "((5 + 4) / 3) * 3 * (100 / 20 ) - 3";
    let parsed_string = ExpressionParser::parse(Rule::calculation, input).unwrap();
    let result:f64 = eval(parsed_string);

    println!("The result is {}", result);
}

// the code below is an example from the pest intro at https://pest.rs/book/intro.html (calculator).
// obviously this is not expression language yet, but there is much similarity
lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(power, Right)
        ])
    };
}

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
            Rule::power    => lhs.powf(rhs),
            _ => unreachable!(),
        },
    )
}
