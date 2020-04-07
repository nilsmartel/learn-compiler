pub mod expression;
mod keyword;
mod literal;
mod util;
mod value;
pub use expression::Expression;
pub use literal::Literal;
use nom::IResult;
pub use value::Value;

pub trait Parse
where
    Self: Sized,
{
    fn parse(input: &str) -> IResult<&str, Self>;

    fn parse_ws(input: &str) -> IResult<&str, Self> {
        util::skip_whitespace(Self::parse)(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ident(pub String);

impl Parse for Ident {
    fn parse(i: &str) -> IResult<&str, Ident> {
        use nom::bytes::complete::take_while;
        use nom::character::complete::alpha1;
        use nom::character::is_alphanumeric;
        use nom::combinator::map;
        use nom::sequence::pair;

        map(
            pair(
                alpha1,
                take_while(|c: char| is_alphanumeric(c as u8) || c == '_'),
            ),
            |(a, b)| Ident(format!("{}{}", a, b)),
        )(i)
    }
}

#[cfg(test)]
mod ident_tests {
    use super::Ident;
    use super::Parse;

    #[test]
    fn simple() {
        assert_eq!(
            Ident::parse("hello_world12345"),
            Ok(("", Ident("hello_world12345".to_string())))
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Type(pub String);

impl Parse for Type {
    fn parse(i: &str) -> IResult<&str, Type> {
        use nom::bytes::complete::take_while;
        use nom::character::complete::alpha1;
        use nom::character::is_alphanumeric;
        use nom::combinator::map;
        use nom::sequence::pair;

        map(
            pair(
                alpha1,
                take_while(|c: char| is_alphanumeric(c as u8) || c == '_'),
            ),
            |(a, b)| Type(format!("{}{}", a, b)),
        )(i)
    }
}

#[derive(Clone, Debug)]
pub struct Ast {
    pub functions: Vec<Function>,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: Ident,
    pub args: Vec<(Ident, Type)>,
    pub body: Body,
}

#[derive(Clone, Debug)]
pub struct Body {
    statements: Vec<Statement>,
}

impl Parse for Body {
    fn parse(input: &str) -> IResult<&str, Self> {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Let {
        name: Ident,
        assign: Option<Expression>,
    },
    If {
        condition: Expression,
        then: Box<Body>,
        otherwise: Option<Box<Body>>,
    },
    While {
        condition: Expression,
        then: Box<Body>,
    },
    Return(Option<Expression>),
}

impl Parse for Statement {
    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::{
            branch::alt,
            combinator::{map, opt},
            sequence::{pair, preceded, tuple},
        };
        use util::{delimited_curly, skip_whitespace, tag_ws};

        alt((
            map(
                pair(
                    preceded(keyword::Let::parse, Ident::parse_ws),
                    opt(preceded(tag_ws("="), Expression::parse_ws)),
                ),
                |(name, assign)| Statement::Let { name, assign },
            ),
            map(
                pair(
                    preceded(keyword::While::parse, Expression::parse_ws),
                    skip_whitespace(delimited_curly(Body::parse_ws)),
                ),
                |(condition, then)| Statement::While {
                    condition,
                    then: Box::new(then),
                },
            ),
            map(
                tuple((
                    preceded(keyword::If::parse, Expression::parse_ws),
                    skip_whitespace(delimited_curly(Body::parse_ws)),
                    opt(map(
                        preceded(
                            keyword::Else::parse_ws,
                            skip_whitespace(delimited_curly(Body::parse_ws)),
                        ),
                        Box::new,
                    )),
                )),
                |(condition, then, otherwise)| Statement::If {
                    condition,
                    then: Box::new(then),
                    otherwise,
                },
            ),
            map(
                preceded(keyword::Return::parse, opt(Expression::parse_ws)),
                Statement::Return,
            ),
        ))(input)
    }
}