use crate::parse::Parse;
use nom::{bytes::complete::tag, combinator::map, IResult};

pub struct Function {}

impl Parse for Function {
    fn parse(input: &str) -> IResult<&str, Function> {
        map(tag("function"), |_| Function {})(input)
    }
}

pub struct If {}

impl Parse for If {
    fn parse(input: &str) -> IResult<&str, If> {
        map(tag("if"), |_| If {})(input)
    }
}

pub struct While {}

impl Parse for While {
    fn parse(input: &str) -> IResult<&str, While> {
        map(tag("while"), |_| While {})(input)
    }
}

pub struct Return {}

impl Parse for Return {
    fn parse(input: &str) -> IResult<&str, Return> {
        map(tag("return"), |_| Return {})(input)
    }
}
