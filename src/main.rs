use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::recognize,
    sequence::separated_pair,
    IResult,
};

fn foo(s: &str) -> IResult<&str, &str> {
    tag("foo")(s)
}

fn bar(s: &str) -> IResult<&str, &str> {
    tag("bar")(s)
}

fn foo_bar(s: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(foo, char(' '), bar)(s)
}

fn foo_bar_or_number(s: &str) -> IResult<&str, &str> {
    alt((recognize(foo_bar), recognize(digit1)))(s)
}

fn main() {
    assert_eq!(foo_bar_or_number("foo bar"), Ok(("", "foo bar")));
    assert_eq!(foo_bar_or_number("1234567"), Ok(("", "1234567")));
}
