use nom::{
    character::complete::char,
    sequence::{delimited, preceded},
    IResult,
};

pub fn skip_whitespace<'a, F, T>(f: F) -> impl Fn(&'a str) -> IResult<&'a str, T>
where
    F: Fn(&'a str) -> IResult<&str, T>,
{
    use nom::character::complete::one_of;
    use nom::multi::many0;
    let skip = many0(one_of("\n\r\t "));
    preceded(skip, f)
}

pub fn tag_ws<'a>(s: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    skip_whitespace(nom::bytes::complete::tag(s))
}

pub fn delimited_curly<'a, T>(
    f: impl Fn(&'a str) -> IResult<&'a str, T>,
) -> impl Fn(&'a str) -> IResult<&'a str, T> {
    delimited(char('{'), f, tag_ws("}"))
}

pub fn delimited_paren<'a, T>(
    f: impl Fn(&'a str) -> IResult<&'a str, T>,
) -> impl Fn(&'a str) -> IResult<&'a str, T> {
    delimited(char('('), f, tag_ws(")"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::bytes::complete::tag;

    #[test]
    fn delimited1() {
        let res = delimited_paren(tag("hello"))("(hello)");
        assert_eq!(res, Ok(("", "hello")));
    }
    #[test]
    fn delimited2() {
        let res = delimited_curly(tag("hello"))("{hello}");
        assert_eq!(res, Ok(("", "hello")));
    }
}
