use nom::{Err, IResult, Parser};

fn main() {
    println!("Hello, world!");
}

/**
Did the start of the string contain "Hello"
 */
fn parse_hello(input: &str) -> IResult<&str, &str, ()> {
    match input.strip_prefix("Hello") {
        // strip_prefix returns the tail of the input, remaining thing to parse
        Some(tail) => Ok((tail, "Hello")), // our output value is again "Hello"
        None => Err(nom::Err::Error(())),
    }
}

/***
This function already exists in nom and is called `tag`
 */
fn parse_tag<'input: 'tag, 'tag>(
    tag: &'tag str,
) -> impl Parser<&'input str, &'input str, ()> + 'tag {
    move |input: &'input str| {
        match input.strip_prefix(tag) {
            // strip_prefix returns the tail of the input, remaining thing to parse
            Some(tail) => Ok((tail, &input[..tag.len()])),
            None => Err(nom::Err::Error(())),
        }
    }
}

// `'input: 'tag` means `'input` is part of `'tag`
fn parse_comma_tags<'input: 'tag, 'tag>(
    tag1: &'tag str,
    tag2: &'tag str,
) -> impl Parser<&'input str, (&'input str, &'input str), ()> + 'tag {
    move |input: &'input str| {
        let mut parse_tag1 = parse_tag(tag1);
        let mut parse_separator = parse_tag(", ");
        let mut parse_tag2 = parse_tag(tag2);

        let (tail, value1) = parse_tag1.parse(input)?;
        let (tail, _) = parse_separator.parse(tail)?;
        let (tail, value2) = parse_tag2.parse(tail)?;
        Ok((tail, (value1, value2)))
    }
}

/***
Accepts any parser we want.
This function is called `separated` in `nom`.
*/
fn parse_separated<Input, Output1, Output2, Separator, Error>(
    mut parse_tag1: impl Parser<Input, Output1, Error>,
    mut parse_separator: impl Parser<Input, Separator, Error>,
    mut parse_tag2: impl Parser<Input, Output2, Error>,
) -> impl Parser<Input, (Output1, Output2), Error> {
    move |input| {
        let (tail, value1) = parse_tag1.parse(input)?;
        let (tail, _) = parse_separator.parse(tail)?;
        let (tail, value2) = parse_tag2.parse(tail)?;
        Ok((tail, (value1, value2)))
    }
}

fn parse_bool(input: &str) -> IResult<&str, bool, ()> {
    match parse_tag("true").parse(input) {
        Ok((tail, _)) => Ok((tail, true)),
        Err(nom::Err::Error(_err)) => match parse_tag("false").parse(input) {
            Ok((tail, _)) => Ok((tail, false)),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_bool, parse_comma_tags, parse_hello, parse_separated, parse_tag};
    use nom::{Err, IResult, Parser};

    #[test]
    fn test_parse_hello() {
        assert_eq!(parse_hello("Hello, World").unwrap(), (", World", "Hello"));
    }

    #[test]
    fn test_parse_tag() {
        assert_eq!(
            parse_tag("Hello").parse("Hello, World").unwrap(),
            (", World", "Hello")
        );
    }

    #[test]
    fn test_parse_comma_tag() {
        assert_eq!(
            parse_comma_tags("Hello", "World")
                .parse("Hello, World!!")
                .unwrap(),
            ("!!", ("Hello", "World"))
        )
    }

    #[test]
    fn test_parse_separated() {
        let mut parse_hello_world =
            parse_separated(parse_tag("Hello"), parse_tag(", "), parse_tag("World"));

        assert_eq!(
            parse_hello_world.parse("Hello, World!!").unwrap(),
            ("!!", ("Hello", "World"))
        )
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse_bool("true, 1234").unwrap(), (", 1234", true));
        assert_eq!(parse_bool("false bla").unwrap(), (" bla", false));
        assert!(parse_bool("afasdlse").is_err());
    }
}
