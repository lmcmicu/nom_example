use nom::{
    branch::alt,
    bytes::complete::{take, take_until},
    character::complete::{alphanumeric1},
    IResult,
    error::{ErrorKind, ParseError},
    Err::Error,
};

#[derive(Debug, PartialEq)]
pub enum CustomError<I> {
  MyError,
  Nom(I, ErrorKind),
}

impl<I> ParseError<I> for CustomError<I> {
  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    CustomError::Nom(input, kind)
  }

  fn append(_: I, _: ErrorKind, other: Self) -> Self {
    other
  }
}

fn unquoted<'a>(quote_char: &'a char, input: &'a str) -> IResult<&'a str, String, CustomError<&'a str>> {
    let quote_str = format!("{}", quote_char);

    let result: IResult<&str, &str> = take(1 as u8)(input);
    let (mut remainder, mut parsed_value) = result.as_ref().unwrap();
    if *parsed_value != quote_str {
        return Err(Error(CustomError::MyError));
    }

    let mut content = String::from("");
    let mut start = true;
    while remainder != "" {
        let result: IResult<&str, &str> = take_until(quote_str.as_str())(remainder);
        let result = result.as_ref().unwrap();
        remainder = result.0;
        parsed_value = result.1;
        if !remainder.starts_with(quote_str.as_str()) {
            return Err(Error(CustomError::MyError));
        }
        if start {
            start = false;
        } else {
            content.push_str(quote_str.as_str());
        }
        content.push_str(parsed_value);
        if remainder == format!("{}{}", quote_str, quote_str) {
            content.push_str(quote_str.as_str());
        }
        remainder = remainder.trim_start_matches(quote_str.as_str());
    }

    Ok((remainder, content))
}

fn alphanum(input: &str) -> IResult<&str, String, CustomError<&str>> {
    let foo: IResult<&str, &str> = alphanumeric1(input);
    match foo {
        Ok(boo) => {
            let remainder = boo.0;
            let content = boo.1;
            return Ok((remainder, content.to_string()));
        },
        Err(_) => {
            return Err(Error(CustomError::MyError));
        }
    };
}

fn dqstring(input: &str) -> IResult<&str, String, CustomError<&str>> {
    unquoted(&'"', &input)
}

fn sqstring(input: &str) -> IResult<&str, String, CustomError<&str>> {
    unquoted(&'\'', &input)
}

fn label(input: &str) -> IResult<&str, String, CustomError<&str>> {
    alt((alphanum, dqstring, sqstring))(input)
}

fn main() {
    let submitted_string = "\"\\\"abc123\\\"gronkiness\\\"ScoobyDoo!!\\\"joof\\\"\"";
    let expected_string = String::from("\\\"abc123\\\"gronkiness\\\"ScoobyDoo!!\\\"joof\\\"");
    assert_eq!(label(submitted_string), Ok(("", expected_string)));
    let submitted_string = r"'abc123\'gronkiness\'ScoobyDoo!!'";
    let expected_string = String::from(r"abc123\'gronkiness\'ScoobyDoo!!");
    assert_eq!(label(submitted_string), Ok(("", expected_string)));
    let submitted_string = "Belafonte";
    let expected_string = String::from("Belafonte");
    assert_eq!(label(submitted_string), Ok(("", expected_string)));
}
