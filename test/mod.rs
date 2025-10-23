
use nsdq_util::define_enum;

define_enum!{
    MyEnum:
        "This is my strong type for a character-based protocol enum.";

    ['A'] VariantA
        "This is the first variant for MyEnum.",
    ['B'] VariantB
        "This is the other variant for MyEnum."
}

#[test] fn enum_test() {


    let bytes = b"AB";

    let (bytes, parsed_1) = MyEnum::parse(bytes).unwrap();
    let (bytes, parsed_2) = MyEnum::parse(bytes).unwrap();

    assert_eq!(parsed_1, MyEnum::VariantA);
    assert_eq!(parsed_2, MyEnum::VariantB);
    assert!(bytes.is_empty());

    let mut bytes = vec![];
    bytes.extend(MyEnum::VariantA.encode());
    bytes.extend(MyEnum::VariantB.encode());

    assert_eq!(&bytes, b"AB");
}

