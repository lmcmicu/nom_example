use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alphanumeric1, char, digit1},
    combinator::recognize,
    sequence::{delimited, separated_pair},
    IResult,
};

/*
 * Functions from the original simple basic examble
 */

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

/*
 * Functions to replicate the valve grammar
 */

fn label(input: &str) -> IResult<&str, &str> {
    alt((
        alphanumeric1,
        delimited(char('"'), is_not("\""), char('"')),
        delimited(char('\''), is_not("\'"), char('\'')),
    ))(input)
}

fn main() {
    // Tests from the original simple basic example
    assert_eq!(foo_bar_or_number("foo bar"), Ok(("", "foo bar")));
    assert_eq!(foo_bar_or_number("1234567"), Ok(("", "1234567")));
    // Valve grammar tests
    let submitted_string = "\'abc123\'";
    let expected_string = "abc123";
    assert_eq!(label(submitted_string), Ok(("", expected_string)));
}
