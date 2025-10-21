
use nom::{ 
    Parser, 
    branch::alt, 
    combinator::map, 
    character::char 
};

/// Parse a boolean value with the standard character mapping of 'Y' and 'N'.
pub fn parse_bool(input: &[u8]) -> nom::IResult<&[u8], bool> {
    parse_bool_with_chars('Y', 'N', input)
}

/// Parse a boolean value by specifying the character mapping.
pub fn parse_bool_with_chars(
    yes: char, 
    no: char,
    input: &[u8]
) -> nom::IResult<&[u8], bool> {

    let (input, val) = alt((
        map(char(yes), |_| true),
        map(char(no), |_| false),
    )).parse(input)?;

    Ok((input, val))
}

/// Parse a ternary boolean with the standard mapping of 'Y', 'N', and ' '.
pub fn parse_ternary(input: &[u8]) -> nom::IResult<&[u8], Option<bool>> {
    parse_ternary_with_chars('Y', 'N', ' ', input)
}

/// Parse a ternary-logic boolean value by specifying the character mapping.
/// Represented as `Option<bool>`.
pub fn parse_ternary_with_chars(
    yes: char, 
    no: char,
    maybe: char,
    input: &[u8]
) -> nom::IResult<&[u8], Option<bool>> {

    let (input, val) = alt((
        map(char(yes), |_| Some(true)),
        map(char(no), |_| Some(false)),
        map(char(maybe), |_| None),
    )).parse(input)?;

    Ok((input, val))
}


