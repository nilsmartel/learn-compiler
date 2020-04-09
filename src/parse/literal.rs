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
            combinator::{map, opt},
            multi::separated_list,
            sequence::pair,
        };
        map(
            pair(
                Ident::parse,
                opt(util::skip_whitespace(util::delimited_paren(
                    separated_list(util::tag_ws(","), Expression::parse_ws),
                ))),
            ),
            |(ident, call_arguments)| Literal {
                ident,
                call_arguments,
            },
        )(input)
    }
}

#[cfg(test)]
mod literal_tests {
    use super::*;
    #[test]
    fn variable() {
        assert_eq!(
            Literal::parse("hannover"),
            Ok((
                "",
                Literal {
                    ident: Ident("hannover".to_string()),
                    call_arguments: None
                }
            ))
        );
    }

    #[test]
    fn function() {
        assert_eq!(
            Literal::parse("leibniz()"),
            Ok((
                "",
                Literal {
                    ident: Ident("leibniz".to_string()),
                    call_arguments: Some(Vec::new())
                }
            ))
        );
    }
}
