use crate::parse::Parse;
use nom::IResult;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    Float(f64),
}

impl std::cmp::Eq for Value {}

impl Parse for Value {
    fn parse(input: &str) -> IResult<&str, Self> {
        use nom::{branch::alt, combinator::map};
        alt((
            map(parse_bool, Value::Boolean),
            map(parse_float, Value::Float),
            map(parse_int, Value::Integer),
        ))(input)
    }
}

fn parse_bool(input: &str) -> IResult<&str, bool> {
    use nom::{branch::alt, bytes::complete::tag, combinator::map};
    alt((map(tag("false"), |_| false), map(tag("true"), |_| true)))(input)
}
fn parse_int(input: &str) -> IResult<&str, i64> {
    use nom::{bytes::complete::take_while1, combinator::map};
    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    map(take_while1(is_digit), |s: &str| s.parse::<i64>().unwrap())(input)
}

fn parse_float(input: &str) -> IResult<&str, f64> {
    use nom::{character::complete::char, combinator::map, sequence::separated_pair};

    map(separated_pair(parse_int, char('.'), parse_int), |(a, b)| {
        format!("{}.{}", a, b).parse::<f64>().unwrap()
    })(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bool() {
        assert!(parse_int("").is_err());
        assert_eq!(parse_bool("true"), Ok(("", true)));
        assert_eq!(parse_bool("false"), Ok(("", false)));
    }

    #[test]
    fn int() {
        assert!(parse_int("").is_err());
        assert_eq!(parse_int("0"), Ok(("", 0)));
        assert_eq!(parse_int("123"), Ok(("", 123)));
        assert_eq!(parse_int("987654321"), Ok(("", 987654321)));
    }

    #[test]
    fn float() {
        assert!(parse_int("").is_err());
        assert_eq!(parse_float("0.0"), Ok(("", 0.0)));
        assert_eq!(parse_float("0.000000"), Ok(("", 0.0)));
        assert_eq!(parse_float("123.456"), Ok(("", 123.456)));
    }
}
