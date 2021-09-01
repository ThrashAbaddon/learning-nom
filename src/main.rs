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
*/
fn parse_separated<'i>(
    mut parse_tag1: impl Parser<&'i str, &'i str, ()>,
    mut parse_separator: impl Parser<&'i str, &'i str, ()>,
    mut parse_tag2: impl Parser<&'i str, &'i str, ()>,
) -> impl Parser<&'i str, (&'i str, &'i str), ()> {
    move |input: &'i str| {
        let (tail, value1) = parse_tag1.parse(input)?;
        let (tail, _) = parse_separator.parse(tail)?;
        let (tail, value2) = parse_tag2.parse(tail)?;
        Ok((tail, (value1, value2)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_comma_tags, parse_hello, parse_separated, parse_tag};
    use nom::{Err, IResult, Parser};

    #[test]
    fn test() {
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
    fn test_comma_tag() {
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
}
