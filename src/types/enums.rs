
/// For quickly defining single-character-alpha enum variants:
/// ```
/// use nsdq_util::define_enum;
///
/// define_enum!{
///     MyEnum:
///         "This is my strong type for a character-based protocol enum.";
/// 
///     ['A'] VariantA
///         "This is the first variant for MyEnum.",
///     ['B'] VariantB
///         "This is the other variant for MyEnum."
/// }
///
/// let bytes = b"AB";
///
/// let (bytes, parsed_1) = MyEnum::parse(bytes).unwrap();
/// let (bytes, parsed_2) = MyEnum::parse(bytes).unwrap();
///
/// assert_eq!(parsed_1, MyEnum::VariantA);
/// assert_eq!(parsed_2, MyEnum::VariantB);
/// assert!(bytes.is_empty());
///
/// let mut bytes = vec![];
/// bytes.extend(MyEnum::VariantA.encode());
/// bytes.extend(MyEnum::VariantB.encode());
///
/// assert_eq!(&bytes, b"AB");
///
/// ```
#[macro_export] macro_rules! define_enum {
    ($name:ident: $edoc:expr; $([$tag:expr] $kind:ident $kdoc:expr),*$(,)?) => {

        #[doc = $edoc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {$(
            #[doc = $kdoc]
            $kind,
        )*}

        impl $name {
            pub fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

                use nom::{ 
                    Parser, 
                    branch::alt, 
                    combinator::map, 
                    character::char 
                };

                let (input, kind) = alt(($(
                    map(char($tag), |_| Self::$kind),
                )*)).parse(input)?;

                Ok((input, kind))
            }

            pub fn encode(&self) -> [u8; 1] {
                match self {$(
                    $name::$kind => [$tag as u8],
                )*}
            }
        }

    }
}

/// For quickly defining single-character-alpha enum variants:
/// ```
/// use nsdq_util::define_str_enum;
///
///
/// define_str_enum!{
///
///     MyEnum2 [2usize] "Enum with fixed tag length 2";
///
///     [b"AB"] Var1 "One variant",
///     [b"XY"] Var2 "Another variant",
/// }
///
/// let bytes = b"ABXY";
/// let (bytes, var1) = MyEnum2::parse(bytes).unwrap();
/// let (bytes, var2) = MyEnum2::parse(bytes).unwrap();
///
/// assert_eq!(var1, MyEnum2::Var1);
/// assert_eq!(var2, MyEnum2::Var2);
///
///
/// define_str_enum!{
///
///     MyEnum4 [4usize] "Enum with fixed tag length 2";
///
///     [b"ABCD"] Var1 "Full-length variant",
///     [b"XY  "] Var2 "Variant with whitespace",
/// }
///
/// let bytes1 = b"ABCD";
/// let (_, var1) = MyEnum4::parse(bytes1).unwrap();
/// let bytes2 = b"XY  ";
/// let (_, var2) = MyEnum4::parse(bytes2).unwrap();
///
/// assert_eq!(var1, MyEnum4::Var1);
/// assert_eq!(var2, MyEnum4::Var2);
/// assert_eq!(&MyEnum4::Var1.encode(), bytes1);
/// assert_eq!(&MyEnum4::Var2.encode(), bytes2);
///
/// ```
#[macro_export] macro_rules! define_str_enum {
    ($name:ident [$len:expr] $edoc:expr; 
        $([$tag:expr] $kind:ident $kdoc:expr),*$(,)?
    ) => {

        #[doc = $edoc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {$(
            #[doc = $kdoc]
            $kind,
        )*}

        impl $name {
            pub fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

                use nom::{ 
                    Parser, 
                    branch::alt, 
                    combinator::map, 
                    character::char,
                    bytes::complete::take,
                };

                let (input, chars) = take($len)(input)?;
                let kind = match chars {
                    $(
                        $tag => Self::$kind,
                    )*
                    _ => return Err(nom::Err::Error(
                        nom::error::Error::new(
                            input, 
                            nom::error::ErrorKind::Tag
                        )
                    )),
                };

                Ok((input, kind))
            }

            pub fn encode(&self) -> [u8; $len] {
                match self {$(
                    $name::$kind => *$tag,
                )*}
            }
        }

    }
}

