
/// For quickly defining single-character-alpha enum variants:
/// '''
/// nsdq-util::define_enum!{
///     MyEnum:
///         "This is my strong type for a character-based protocol enum.";
///
///     ['A'] VariantA
///         "This is the first variant for MyEnum.",
///     ['B'] VariantB
///         "This is the other variant for MyEnum."
/// }
///
/// fn test() {
///
///     let bytes = b"AB";
///
///     let (bytes, parsed_1) = MyEnum::parse(bytes).unwrap();
///     let (bytes, parsed_2) = MyEnum::parse(bytes).unwrap();
///
///     assert_eq!(parsed_1, MyEnum::VariantA);
///     assert_eq!(parsed_2, MyEnum::VariantB);
///     assert!(bytes.is_empty());
///
///     let mut bytes = vec![];
///     bytes.extend(MyEnum::VariantA.encode());
///     bytes.extend(MyEnum::VariantB.encode());
///
///     assert_eq!(&bytes, b"AB");
/// }
/// '''
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
                    $name::$kind => [$tag],
                )*}
            }
        }

    }
}

