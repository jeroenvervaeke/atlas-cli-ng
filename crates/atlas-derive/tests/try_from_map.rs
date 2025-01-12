use std::collections::HashMap;
use atlas_derive::TryFromMap;
use atlas_derive_core::TryFromMapError;

#[derive(TryFromMap)]
struct Example {
    foo: Vec<String>,
    bar: u32,
    baz: Option<bool>,
}

#[test]
fn test_basic_conversion() {
    let mut map = HashMap::new();
    map.insert("foo".to_string(), vec!["a".to_string(), "b".to_string()]);
    map.insert("bar".to_string(), vec!["42".to_string()]);

    let example = Example::try_from(map).unwrap();
    assert_eq!(example.foo, vec!["a", "b"]);
    assert_eq!(example.bar, 42);
    assert_eq!(example.baz, None);
}

#[test]
fn test_missing_required_field() {
    let mut map = HashMap::new();
    map.insert("foo".to_string(), vec!["a".to_string(), "b".to_string()]);
    // Missing "bar" field

    let result = Example::try_from(map);
    assert!(matches!(result, Err(TryFromMapError::MissingField(field)) if field == "bar"));
}

#[test]
fn test_empty_required_field() {
    let mut map = HashMap::new();
    map.insert("foo".to_string(), vec!["a".to_string(), "b".to_string()]);
    map.insert("bar".to_string(), vec![]);

    let result = Example::try_from(map);
    assert!(matches!(result, Err(TryFromMapError::NoValuesInField(field)) if field == "bar"));
}

#[test]
fn test_parse_error() {
    let mut map = HashMap::new();
    map.insert("foo".to_string(), vec!["a".to_string(), "b".to_string()]);
    map.insert("bar".to_string(), vec!["not a number".to_string()]);

    let result = Example::try_from(map);
    assert!(matches!(result, Err(TryFromMapError::ParseError { field, value }) 
        if field == "bar" && value == "not a number"));
}

#[test]
fn test_optional_field_some() {
    let mut map = HashMap::new();
    map.insert("foo".to_string(), vec!["a".to_string(), "b".to_string()]);
    map.insert("bar".to_string(), vec!["42".to_string()]);
    map.insert("baz".to_string(), vec!["true".to_string()]);

    let example = Example::try_from(map).unwrap();
    assert_eq!(example.foo, vec!["a", "b"]);
    assert_eq!(example.bar, 42);
    assert_eq!(example.baz, Some(true));
}

#[test]
fn test_optional_field_empty() {
    let mut map = HashMap::new();
    map.insert("foo".to_string(), vec!["a".to_string(), "b".to_string()]);
    map.insert("bar".to_string(), vec!["42".to_string()]);
    map.insert("baz".to_string(), vec![]);

    let example = Example::try_from(map).unwrap();
    assert_eq!(example.foo, vec!["a", "b"]);
    assert_eq!(example.bar, 42);
    assert_eq!(example.baz, None);
}

#[derive(TryFromMap, Debug, PartialEq)]
struct AllTypes {
    // Basic types
    string: String,
    boolean: bool,
    unsigned32: u32,
    signed32: i32,
    float32: f32,

    // Vec of basic types
    vec_string: Vec<String>,
    vec_boolean: Vec<bool>,
    vec_unsigned32: Vec<u32>,
    vec_signed32: Vec<i32>,
    vec_float32: Vec<f32>,

    // Option of basic types
    opt_string: Option<String>,
    opt_boolean: Option<bool>,
    opt_unsigned32: Option<u32>,
    opt_signed32: Option<i32>,
    opt_float32: Option<f32>,

    // Option of Vec of basic types
    opt_vec_string: Option<Vec<String>>,
    opt_vec_boolean: Option<Vec<bool>>,
    opt_vec_unsigned32: Option<Vec<u32>>,
    opt_vec_signed32: Option<Vec<i32>>,
    opt_vec_float32: Option<Vec<f32>>,
}

#[test]
fn test_basic_types() {
    let mut map = HashMap::new();
    map.insert("string".to_string(), vec!["hello".to_string()]);
    map.insert("boolean".to_string(), vec!["true".to_string()]);
    map.insert("unsigned32".to_string(), vec!["42".to_string()]);
    map.insert("signed32".to_string(), vec!["-42".to_string()]);
    map.insert("float32".to_string(), vec!["3.14".to_string()]);
    map.insert("vec_string".to_string(), vec!["a".to_string(), "b".to_string()]);
    map.insert("vec_boolean".to_string(), vec!["true".to_string(), "false".to_string()]);
    map.insert("vec_unsigned32".to_string(), vec!["1".to_string(), "2".to_string()]);
    map.insert("vec_signed32".to_string(), vec!["-1".to_string(), "-2".to_string()]);
    map.insert("vec_float32".to_string(), vec!["1.1".to_string(), "2.2".to_string()]);

    let result = AllTypes::try_from(map).unwrap();
    assert_eq!(result.string, "hello");
    assert_eq!(result.boolean, true);
    assert_eq!(result.unsigned32, 42);
    assert_eq!(result.signed32, -42);
    assert_eq!(result.float32, 3.14);
    assert_eq!(result.vec_string, vec!["a", "b"]);
    assert_eq!(result.vec_boolean, vec![true, false]);
    assert_eq!(result.vec_unsigned32, vec![1, 2]);
    assert_eq!(result.vec_signed32, vec![-1, -2]);
    assert_eq!(result.vec_float32, vec![1.1, 2.2]);
    assert_eq!(result.opt_string, None);
    assert_eq!(result.opt_boolean, None);
    assert_eq!(result.opt_unsigned32, None);
    assert_eq!(result.opt_signed32, None);
    assert_eq!(result.opt_float32, None);
    assert_eq!(result.opt_vec_string, None);
    assert_eq!(result.opt_vec_boolean, None);
    assert_eq!(result.opt_vec_unsigned32, None);
    assert_eq!(result.opt_vec_signed32, None);
    assert_eq!(result.opt_vec_float32, None);
}

#[test]
fn test_optional_types() {
    let mut map = HashMap::new();
    // Required fields
    map.insert("string".to_string(), vec!["hello".to_string()]);
    map.insert("boolean".to_string(), vec!["true".to_string()]);
    map.insert("unsigned32".to_string(), vec!["42".to_string()]);
    map.insert("signed32".to_string(), vec!["-42".to_string()]);
    map.insert("float32".to_string(), vec!["3.14".to_string()]);
    map.insert("vec_string".to_string(), vec!["a".to_string(), "b".to_string()]);
    map.insert("vec_boolean".to_string(), vec!["true".to_string(), "false".to_string()]);
    map.insert("vec_unsigned32".to_string(), vec!["1".to_string(), "2".to_string()]);
    map.insert("vec_signed32".to_string(), vec!["-1".to_string(), "-2".to_string()]);
    map.insert("vec_float32".to_string(), vec!["1.1".to_string(), "2.2".to_string()]);

    // Optional fields
    map.insert("opt_string".to_string(), vec!["optional".to_string()]);
    map.insert("opt_boolean".to_string(), vec!["false".to_string()]);
    map.insert("opt_unsigned32".to_string(), vec!["24".to_string()]);
    map.insert("opt_signed32".to_string(), vec!["-24".to_string()]);
    map.insert("opt_float32".to_string(), vec!["2.718".to_string()]);
    map.insert("opt_vec_string".to_string(), vec!["x".to_string(), "y".to_string()]);
    map.insert("opt_vec_boolean".to_string(), vec!["true".to_string(), "true".to_string()]);
    map.insert("opt_vec_unsigned32".to_string(), vec!["100".to_string(), "200".to_string()]);
    map.insert("opt_vec_signed32".to_string(), vec!["-100".to_string(), "-200".to_string()]);
    map.insert("opt_vec_float32".to_string(), vec!["0.1".to_string(), "0.2".to_string()]);

    let result = AllTypes::try_from(map).unwrap();
    assert_eq!(result.opt_string, Some("optional".to_string()));
    assert_eq!(result.opt_boolean, Some(false));
    assert_eq!(result.opt_unsigned32, Some(24));
    assert_eq!(result.opt_signed32, Some(-24));
    assert_eq!(result.opt_float32, Some(2.718));
    assert_eq!(result.opt_vec_string, Some(vec!["x".to_string(), "y".to_string()]));
    assert_eq!(result.opt_vec_boolean, Some(vec![true, true]));
    assert_eq!(result.opt_vec_unsigned32, Some(vec![100, 200]));
    assert_eq!(result.opt_vec_signed32, Some(vec![-100, -200]));
    assert_eq!(result.opt_vec_float32, Some(vec![0.1, 0.2]));
}

#[test]
fn test_parse_errors() {
    let test_cases = vec![
        ("boolean", "not_a_bool"),
        ("unsigned32", "not_a_number"),
        ("signed32", "not_a_number"),
        ("float32", "not_a_float"),
        ("vec_boolean", "invalid"),
        ("vec_unsigned32", "invalid"),
        ("vec_signed32", "invalid"),
        ("vec_float32", "invalid"),
        ("opt_boolean", "invalid"),
        ("opt_unsigned32", "invalid"),
        ("opt_signed32", "invalid"),
        ("opt_float32", "invalid"),
    ];

    for (field, invalid_value) in test_cases {
        let mut map = HashMap::new();
        // Add required fields
        map.insert("string".to_string(), vec!["hello".to_string()]);
        map.insert("boolean".to_string(), vec!["true".to_string()]);
        map.insert("unsigned32".to_string(), vec!["42".to_string()]);
        map.insert("signed32".to_string(), vec!["-42".to_string()]);
        map.insert("float32".to_string(), vec!["3.14".to_string()]);
        map.insert("vec_string".to_string(), vec!["a".to_string()]);
        map.insert("vec_boolean".to_string(), vec!["true".to_string()]);
        map.insert("vec_unsigned32".to_string(), vec!["1".to_string()]);
        map.insert("vec_signed32".to_string(), vec!["-1".to_string()]);
        map.insert("vec_float32".to_string(), vec!["1.1".to_string()]);

        // Override with invalid value
        map.insert(field.to_string(), vec![invalid_value.to_string()]);

        let result = AllTypes::try_from(map);
        assert!(matches!(result, Err(TryFromMapError::ParseError { field: f, value: v })
            if f == field && v == invalid_value));
    }
}
