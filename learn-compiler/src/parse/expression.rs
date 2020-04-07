use crate::parse::*;
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_nonempty_list,
    sequence::preceded, IResult,
};
use util::{skip_whitespace, tag_ws};

#[derive(Clone, Debug)]
pub enum Expression {
    Or(Vec<Expression>),
    And(Vec<Expression>),
    Add(Vec<Expression>),
    Subtract(Vec<Expression>),
    Multiply(Vec<Expression>),
    Divide(Vec<Expression>),
    Not(Box<Expression>),
    Negative(Box<Expression>),
    Literal(Literal),
    Value(value::Value),
}

impl Parse for Expression {
    fn parse(input: &str) -> IResult<&str, Self> {
        or(input)
    }
}

fn or(i: &str) -> IResult<&str, Expression> {
    map(
        separated_nonempty_list(tag_ws("|"), skip_whitespace(and)),
        Expression::Or,
    )(i)
}

fn and(i: &str) -> IResult<&str, Expression> {
    map(
        separated_nonempty_list(tag_ws("&"), skip_whitespace(add)),
        Expression::And,
    )(i)
}

fn add(i: &str) -> IResult<&str, Expression> {
    map(
        separated_nonempty_list(tag_ws("+"), skip_whitespace(subtract)),
        Expression::Add,
    )(i)
}

fn subtract(i: &str) -> IResult<&str, Expression> {
    map(
        separated_nonempty_list(tag_ws("-"), skip_whitespace(multiply)),
        Expression::Subtract,
    )(i)
}

fn multiply(i: &str) -> IResult<&str, Expression> {
    map(
        separated_nonempty_list(tag_ws("*"), skip_whitespace(divide)),
        Expression::Multiply,
    )(i)
}

fn divide(i: &str) -> IResult<&str, Expression> {
    map(
        separated_nonempty_list(tag_ws("/"), skip_whitespace(not)),
        Expression::Divide,
    )(i)
}

fn not(i: &str) -> IResult<&str, Expression> {
    alt((
        map(preceded(tag("!"), literal), |e| {
            Expression::Negative(Box::new(e))
        }),
        negative,
    ))(i)
}

fn negative(i: &str) -> IResult<&str, Expression> {
    alt((
        map(preceded(tag("-"), literal), |e| {
            Expression::Negative(Box::new(e))
        }),
        literal,
    ))(i)
}

fn literal(i: &str) -> IResult<&str, Expression> {
    alt((value, map(Literal::parse, Expression::Literal)))(i)
}

fn value(i: &str) -> IResult<&str, Expression> {
    alt((
        map(Value::parse, Expression::Value),
        util::delimited_paren(Expression::parse_ws),
    ))(i)
}
