use nom::IResult;

pub fn skip_whitespace<'a, F, T>(f: F) -> impl Fn(&'a str) -> IResult<&'a str, T>
where
    F: Fn(&'a str) -> IResult<&str, T>,
{
    use nom::character::complete::one_of;
    use nom::multi::many0;
    let skip = many0(one_of("\n\r\t "));

    move |s: &str| {
        let (result, _) = skip(s)?;
        f(result)
    }
}
