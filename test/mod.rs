
nsdq_util::define_enum!{
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


nsdq_util::define_str!(MyStr [4usize] "String with a fixed length of 4.");

#[test] fn str_test() {

    let bytes = b"XXXX";
    let (_, mystr) = MyStr::parse(bytes).unwrap();

    assert_eq!(mystr.encode(), *bytes);
    assert_eq!(mystr.to_str(), "XXXX");

    assert_eq!(format!("{}", mystr), String::from("XXXX"));
}

