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

pub struct Else {}

impl Parse for Else {
    fn parse(input: &str) -> IResult<&str, Else> {
        map(tag("else"), |_| Else {})(input)
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

pub struct Let {}

impl Parse for Let {
    fn parse(input: &str) -> IResult<&str, Let> {
        map(tag("let"), |_| Let {})(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords() {
        assert_eq!(If::parse("if").unwrap().0, "");
        assert_eq!(Else::parse("else").unwrap().0, "");

        assert_eq!(While::parse("while").unwrap().0, "");
        assert_eq!(Return::parse("return").unwrap().0, "");
        assert_eq!(Let::parse("let").unwrap().0, "");
    }
}
