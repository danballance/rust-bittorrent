use crate::bencode::decoder::Decoder;
use serde_json::{json, Value};

#[test]
fn decode_value_handles_integers() {
    let cases: Vec<(&str, Result<Value, String>)> = vec![
        ("i12e", Ok(json!(12))),
        ("i506712941e", Ok(json!(506712941))),
        ("i4294967300e", Ok(json!(4294967300i64))),
        ("i-52e", Ok(json!(-52))),
    ];
    assert_decode_values(cases);
}

#[test]
fn decode_value_handles_strings() {
    let cases: Vec<(&str, Result<Value, String>)> = vec![
        ("5:hello", Ok(json!("hello"))),
        ("4:spam", Ok(json!("spam"))),
        ("0:", Ok(json!(""))),
        (
            "55:http://bittorrent-test-tracker.codecrafters.io/announce",
            Ok(json!(
                "http://bittorrent-test-tracker.codecrafters.io/announce"
            )),
        ),
    ];
    assert_decode_values(cases);
}

#[test]
fn decode_value_handles_lists() {
    let cases: Vec<(&str, Result<Value, String>)> = vec![
        ("li52ee", Ok(json!([52]))),
        ("l5:helloi52ee", Ok(json!(["hello", 52]))),
        ("llei478e5:applee", Ok(json!([[], 478, "apple"]))),
        ("lli478e5:appleee", Ok(json!([[478, "apple"]]))),
    ];
    assert_decode_values(cases);
}
#[test]
fn decode_value_handles_dictionaries() {
    let cases: Vec<(&str, Result<Value, String>)> = vec![
        (
            "d3:foo3:bar5:helloi52ee",
            Ok(json!({"hello": 52, "foo": "bar"})),
        ),
        (
            "d10:inner_dictd4:key16:value14:key2i42e8:list_keyl5:item15:item2i3eeee",
            Ok(json!({
                "inner_dict": {
                    "key1": "value1",
                    "key2": 42,
                    "list_key": ["item1", "item2", 3]
                }
            })),
        ),
    ];
    assert_decode_values(cases);
}

#[test]
fn decode_value_handles_errors() {
    let cases = vec![
        ("", "Empty encoded value"),
        ("x", "Unhandled encoded value"),
    ];

    assert_decode_errors(cases);
}

fn assert_decode_errors(cases: Vec<(&str, &str)>) {
    for (input, expected_error) in cases {
        let decoder = Decoder::new();
        let actual = decoder.decode(input);

        match actual {
            Err(actual_error) => {
                assert_eq!(
                    actual_error, expected_error,
                    "decode_value({input:?}) returned the wrong error"
                );
            }

            Ok(actual_value) => {
                panic!("decode_value({input:?}) succeeded unexpectedly with {actual_value:?}");
            }
        }
    }
}

fn assert_decode_values(cases: Vec<(&str, Result<Value, String>)>) {
    for (input, expected) in cases {
        let decoder = Decoder::new();
        let actual = decoder.decode(input);
        assert_eq!(
            actual, expected,
            "decode_value({input:?}) returned the wrong result"
        );
    }
}
