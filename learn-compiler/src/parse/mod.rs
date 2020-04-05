mod keyword;
use nom::IResult;
mod util;

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
pub struct Ident(String);

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
