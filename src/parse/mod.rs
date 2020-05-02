pub mod expression;
mod keyword;
mod literal;
mod tuple;
mod util;
mod value;
pub use expression::Expression;
pub use literal::Literal;
use nom::IResult;
pub use tuple::*;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_sample() {
        let input = include_str!("../../sample-code/fibonacci.abc");
        let result = Ast::parse_ws(input);

        assert!(result.is_ok());

        assert_eq!(result.unwrap().0, "");
    }

    #[test]
    fn function_simple() {
        assert_eq!(
            Function::parse("function hello() {}"),
            Ok((
                "",
                Function {
                    name: Ident("hello".to_string()),
                    args: Vec::new(),
                    return_type: None,
                    body: Body {
                        statements: Vec::new()
                    },
                }
            ))
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ast {
    pub functions: Vec<Function>,
}

impl Parse for Ast {
    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::{combinator::map, multi::many0};

        map(many0(Function::parse_ws), |functions| Ast { functions })(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub name: Ident,
    pub args: Vec<(Ident, Type)>,
    pub return_type: Option<Type>,
    pub body: Body,
}

impl Parse for Function {
    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::combinator::{map, opt};
        use nom::multi::separated_list;
        use nom::sequence::{preceded, separated_pair, tuple};
        use util::{skip_whitespace, tag_ws};
        let function_name_parser = preceded(keyword::Function::parse, Ident::parse_ws);
        let identtype_parser = separated_pair(Ident::parse_ws, tag_ws(":"), Type::parse_ws);
        let args_parser = skip_whitespace(util::delimited_paren(separated_list(
            tag_ws(","),
            identtype_parser,
        )));
        let body_parser = skip_whitespace(util::delimited_curly(Body::parse_ws));
        let return_type_parser = opt(preceded(tag_ws("->"), Type::parse_ws));

        map(
            tuple((
                function_name_parser,
                args_parser,
                return_type_parser,
                body_parser,
            )),
            |(name, args, return_type, body)| Function {
                name,
                args,
                return_type,
                body,
            },
        )(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Body {
    statements: Vec<Statement>,
}

impl Parse for Body {
    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::{combinator::map, multi::many0};

        map(many0(Statement::parse_ws), |statements| Body { statements })(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    Expression(Box<Expression>),
}

impl Statement {
    fn parse_if(i: &str) -> IResult<&str, Statement> {
        use nom::{
            combinator::{map, opt},
            sequence::preceded,
        };
        use util::{delimited_curly, skip_whitespace};
        let (rest, condition) = preceded(keyword::If::parse, Expression::parse_ws)(i)?;
        let (rest, then) = skip_whitespace(map(delimited_curly(Body::parse_ws), Box::new))(rest)?;
        let (rest, otherwise) = opt(map(
            preceded(
                keyword::Else::parse_ws,
                skip_whitespace(delimited_curly(Body::parse_ws)),
            ),
            Box::new,
        ))(rest)?;

        Ok((
            rest,
            Statement::If {
                condition,
                then,
                otherwise,
            },
        ))
    }
}

#[cfg(test)]
mod statement_tests {
    use super::*;
    #[test]
    fn test_let() {
        assert_eq!(
            Statement::parse_ws("let x"),
            Ok((
                "",
                Statement::Let {
                    name: Ident("x".to_string()),
                    assign: None
                }
            ))
        );
    }

    #[test]
    fn test_let_assign_expression() {
        assert_eq!(
            Statement::parse_ws("let x = true"),
            Ok((
                "",
                Statement::Let {
                    name: Ident("x".to_string()),
                    assign: Some(Expression::Value(value::Value::Boolean(true)))
                }
            ))
        );
    }

    #[test]
    fn test_if() {
        assert_eq!(
            Statement::parse_ws("if true { true }"),
            Ok((
                "",
                Statement::If {
                    condition: Expression::parse("true").unwrap().1,
                    then: Box::new(Body::parse("true").unwrap().1),
                    otherwise: None,
                }
            ))
        );
    }

    #[test]
    fn test_if_else() {
        assert_eq!(
            Statement::parse_ws("if true { true } else { false }"),
            Ok((
                "",
                Statement::If {
                    condition: Expression::parse("true").unwrap().1,
                    then: Box::new(Body::parse("true").unwrap().1),
                    otherwise: Some(Box::new(Body::parse("false").unwrap().1)),
                }
            ))
        );
    }

    #[test]
    fn test_while() {
        assert_eq!(
            Statement::parse_ws("while true { doStuff() }"),
            Ok((
                "",
                Statement::While {
                    condition: Expression::parse("true").unwrap().1,
                    then: Box::new(Body::parse("doStuff()").unwrap().1),
                }
            ))
        );
    }
}

impl Parse for Statement {
    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::{
            branch::alt,
            combinator::{map, opt},
            sequence::{pair, preceded},
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
            Statement::parse_if,
            map(
                preceded(keyword::Return::parse, opt(Expression::parse_ws)),
                Statement::Return,
            ),
            map(Expression::parse, |e| Statement::Expression(Box::new(e))),
        ))(input)
    }
}
