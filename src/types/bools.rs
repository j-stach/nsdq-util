
use nom::{ 
    Parser, 
    branch::alt, 
    combinator::map, 
    character::char 
};

/// Parse a boolean value with the standard character mapping of 'Y' and 'N'.
/// ```
/// use nsdq_util::parse_bool;
///
/// let bytes = b"YN";
/// let (bytes, bool1) = parse_bool(bytes).unwrap();
/// let (bytes, bool2) = parse_bool(bytes).unwrap();
///
/// assert_eq!(bool1, true);
/// assert_eq!(bool2, false);
/// assert!(bytes.is_empty());
/// ```
pub fn parse_bool(input: &[u8]) -> nom::IResult<&[u8], bool> {
    parse_bool_with_chars('Y', 'N', input)
}

/// Parse a boolean value by specifying the character mapping.
/// ```
/// use nsdq_util::parse_bool_with_chars;
///
/// let bytes = b"RX";
/// let (bytes, bool1) = parse_bool_with_chars('R', 'X', bytes).unwrap();
/// let (bytes, bool2) = parse_bool_with_chars('R', 'X', bytes).unwrap();
///
/// assert_eq!(bool1, true);
/// assert_eq!(bool2, false);
/// assert!(bytes.is_empty());
/// ```
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
/// ("Yes", "No", "Uncertain", represented as `Option<bool>`.)
/// ```
/// use nsdq_util::parse_ternary;
///
/// let bytes = b"YN ";
/// let (bytes, bool1) = parse_ternary(bytes).unwrap();
/// let (bytes, bool2) = parse_ternary(bytes).unwrap();
/// let (bytes, bool3) = parse_ternary(bytes).unwrap();
///
/// assert_eq!(bool1, Some(true));
/// assert_eq!(bool2, Some(false));
/// assert_eq!(bool3, None);
/// assert!(bytes.is_empty());
/// ```
pub fn parse_ternary(input: &[u8]) -> nom::IResult<&[u8], Option<bool>> {
    parse_ternary_with_chars('Y', 'N', ' ', input)
}

/// Parse a ternary-logic boolean value by specifying the character mapping.
/// ("Yes", "No", "Uncertain", represented as `Option<bool>`.)
/// ```
/// use nsdq_util::parse_ternary_with_chars;
///
/// let bytes = b"RX ";
/// let (bytes, bool1) = parse_ternary_with_chars('R','X',' ', bytes).unwrap();
/// let (bytes, bool2) = parse_ternary_with_chars('R','X',' ', bytes).unwrap();
/// let (bytes, bool3) = parse_ternary_with_chars('R','X',' ', bytes).unwrap();
///
/// assert_eq!(bool1, Some(true));
/// assert_eq!(bool2, Some(false));
/// assert_eq!(bool3, None);
/// assert!(bytes.is_empty());
/// ```
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


