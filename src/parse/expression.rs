use crate::parse::*;
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_nonempty_list,
    sequence::preceded, IResult,
};
use util::{skip_whitespace, tag_ws};

// Note, using a Vec here isn't nice, since it's length is expected to be >2,
// the chosen type `Vec` does not reflect that. e.g. parse, don't validate
#[derive(Clone, Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod test_expression {
    use super::*;
    #[test]
    fn value() {
        assert_eq!(
            Expression::parse("true"),
            Ok(("", Expression::Value(Value::Boolean(true))),)
        );
    }
}

impl Parse for Expression {
    fn parse(input: &str) -> IResult<&str, Self> {
        or(input)
    }
}

#[inline]
fn or(i: &str) -> IResult<&str, Expression> {
    map_vec(
        separated_nonempty_list(tag_ws("|"), skip_whitespace(and)),
        Expression::Or,
    )(i)
}

#[inline]
fn and(i: &str) -> IResult<&str, Expression> {
    map_vec(
        separated_nonempty_list(tag_ws("&"), skip_whitespace(add)),
        Expression::And,
    )(i)
}

#[inline]
fn add(i: &str) -> IResult<&str, Expression> {
    map_vec(
        separated_nonempty_list(tag_ws("+"), skip_whitespace(subtract)),
        Expression::Add,
    )(i)
}

#[inline]
fn subtract(i: &str) -> IResult<&str, Expression> {
    map_vec(
        separated_nonempty_list(tag_ws("-"), skip_whitespace(multiply)),
        Expression::Subtract,
    )(i)
}

#[inline]
fn multiply(i: &str) -> IResult<&str, Expression> {
    map_vec(
        separated_nonempty_list(tag_ws("*"), skip_whitespace(divide)),
        Expression::Multiply,
    )(i)
}

#[inline]
fn divide(i: &str) -> IResult<&str, Expression> {
    map_vec(
        separated_nonempty_list(tag_ws("/"), skip_whitespace(not)),
        Expression::Divide,
    )(i)
}

#[inline]
fn not(i: &str) -> IResult<&str, Expression> {
    alt((
        map(preceded(tag("!"), literal), |e| {
            Expression::Not(Box::new(e))
        }),
        negative,
    ))(i)
}

#[inline]
fn negative(i: &str) -> IResult<&str, Expression> {
    alt((
        map(preceded(tag("-"), literal), |e| {
            Expression::Negative(Box::new(e))
        }),
        literal,
    ))(i)
}

#[inline]
fn literal(i: &str) -> IResult<&str, Expression> {
    alt((value, map(Literal::parse, Expression::Literal)))(i)
}

#[inline]
fn value(i: &str) -> IResult<&str, Expression> {
    alt((
        map(Value::parse, Expression::Value),
        util::delimited_paren(Expression::parse_ws),
    ))(i)
}

fn map_vec<'a, T>(
    f: impl Fn(&'a str) -> IResult<&'a str, Vec<T>>,
    g: impl Fn(Vec<T>) -> T,
) -> impl Fn(&'a str) -> IResult<&'a str, T> {
    move |s: &str| {
        let (rest, mut res): (&str, Vec<T>) = f(s)?;
        let len = res.len();

        if len == 1 {
            Ok((rest, res.pop().unwrap()))
        } else {
            Ok((rest, g(res)))
        }
    }
}
