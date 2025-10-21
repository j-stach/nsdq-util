
// For quickly defining enum variants in the crate:
macro_rules! define_enum {
    ($name:ident: $edoc:expr; $([$tag:expr] $kind:ident $kdoc:expr),*$(,)?) => {

        #[doc = $edoc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {$(
            #[doc = $kdoc]
            $kind,
        )*}

        impl $name {
            pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

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
        }


    }
}

pub(crate) use define_enum;

