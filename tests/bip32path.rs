use slip10::BIP32Path;

const HARDEND: u32 = 1 << 31;

#[test]
fn test_parse_path() {
    let smaples = vec![
        ("", BIP32Path(Vec::new())),
        ("m", BIP32Path(Vec::new())),
        ("m/0H", BIP32Path(vec![HARDEND + 0])),
        ("m/0H/1", BIP32Path(vec![HARDEND + 0, 1])),
        ("m/0H/1/2H", BIP32Path(vec![HARDEND + 0, 1, HARDEND + 2])),
        ("m/0H/1/2H/2", BIP32Path(vec![HARDEND + 0, 1, HARDEND + 2, 2])),
        ("m/0H/1/2H/2/1000000000", BIP32Path(vec![HARDEND + 0, 1, HARDEND + 2, 2, 1000000000])),
        ("0H", BIP32Path(vec![HARDEND + 0])),
        ("0H/1", BIP32Path(vec![HARDEND + 0, 1])),
        ("0H/1/2H", BIP32Path(vec![HARDEND + 0, 1, HARDEND + 2])),
        ("0H/1/2H/2", BIP32Path(vec![HARDEND + 0, 1, HARDEND + 2, 2])),
        ("0H/1/2H/2/1000000000", BIP32Path(vec![HARDEND + 0, 1, HARDEND + 2, 2, 1000000000])),
        ("m/0'", BIP32Path(vec![HARDEND + 0])),
        ("m/0'/1", BIP32Path(vec![HARDEND + 0, 1])),
        ("m/0'/1/2'", BIP32Path(vec![HARDEND + 0, 1, HARDEND + 2])),
        ("m/0'/1/2'/2", BIP32Path(vec![HARDEND + 0, 1, HARDEND + 2, 2])),
        ("m/0'/1/2'/2/1000000000", BIP32Path(vec![HARDEND + 0, 1, HARDEND + 2, 2, 1000000000])),
        ("0'", BIP32Path(vec![HARDEND + 0])),
        ("0'/1", BIP32Path(vec![HARDEND + 0, 1])),
        ("0'/1/2'", BIP32Path(vec![HARDEND + 0, 1, HARDEND + 2])),
        ("0'/1/2'/2", BIP32Path(vec![HARDEND + 0, 1, HARDEND + 2, 2])),
        ("0'/1/2'/2/1000000000", BIP32Path(vec![HARDEND + 0, 1, HARDEND + 2, 2, 1000000000])),
        ("0/2147483647'/1/2147483646'/2", BIP32Path(vec![0, HARDEND + 2147483647, 1, HARDEND + 2147483646, 2])),
        ("0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0", BIP32Path(vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0])),
    ];

    for (str, path) in smaples {
        assert_eq!(BIP32Path::from(str).unwrap(), path);
    }

    let errors = vec![
        "44'/2147483648",
        "44'/2147483648'",
        "44'/-1",
        "44'//0",
        "/0'/1/2'",
        "44'/'",
        "44'/'0",
        "44'/0h",
        "44'/0''",
        "44'/0H'",
        "wrong",
    ];

    for i in errors {
        assert!(BIP32Path::from(i).is_err());
    }
}
