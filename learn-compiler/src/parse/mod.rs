use nom::IResult;

pub trait Parse<T> {
    fn parse(input: &str) -> IResult<&str, T>;

    fn parse_ws(input: &str) -> IResult<&str, T> {
        util::skip_whitespace(Self::parse)(input)
    }
}
