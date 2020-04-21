use crate::parse::{
    util::{delimited_paren, tag_ws},
    Expression, Parse,
};

use nom::{character::complete::char, combinator::map, multi::separated_list};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tuple(Vec<Expression>);

impl Tuple {
    pub fn new() -> Self {
        Tuple(Vec::new())
    }

    pub fn with(mut self, e: Expression) -> Self {
        self.0.push(e);
        self
    }
}

impl Parse for Tuple {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (rest, _) = dbg!(char('(')(input))?;
        let (rest, expr) = dbg!(separated_list(tag_ws(","), Expression::parse_ws)(rest))?;
        let (rest, _) = tag_ws(")")(rest)?;

        Ok((rest, Tuple(expr)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tuple() {
        let res = Tuple::parse("()");
        assert_eq!(res, Ok(("", Tuple(Vec::new()))));
    }
}
