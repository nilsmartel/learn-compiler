use crate::parse::*;
use nom::IResult;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Literal {
    pub ident: Ident,
    pub call_arguments: Option<Vec<Expression>>,
}

impl Parse for Literal {
    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::{
            character::complete::char,
            combinator::{map, opt},
            multi::separated_list,
            sequence::pair,
        };
        map(
            pair(
                Ident::parse,
                opt(util::delimited_paren(separated_list(
                    util::skip_whitespace(char(',')),
                    Expression::parse_ws,
                ))),
            ),
            |(ident, call_arguments)| Literal {
                ident,
                call_arguments,
            },
        )(input)
    }
}
