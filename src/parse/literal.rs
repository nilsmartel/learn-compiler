use crate::parse::*;
use nom::IResult;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Literal {
    pub ident: Ident,
    pub call_arguments: Option<Tuple>,
}

impl Parse for Literal {
    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::{
            combinator::{map, opt},
            multi::separated_list,
            sequence::pair,
        };
        let (rest, ident) = Ident::parse(input)?;

        let (rest, call_arguments) = opt(Tuple::parse_ws)(rest)?;

        Ok((
            rest,
            Literal {
                ident,
                call_arguments,
            },
        ))
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
                    call_arguments: Some(Tuple::new())
                }
            ))
        );
    }
}
