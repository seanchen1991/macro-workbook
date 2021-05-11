use hashmap::hashmap;
use std::collections::HashMap;

#[test]
fn test_empty() {
    let expected: HashMap<i32, i32> = HashMap::new();
    let generated: HashMap<i32, i32> = hashmap!{};
    assert_eq!(generated, expected);
}

#[test]
fn test_single() {
    let mut expected = HashMap::new();
    expected.insert("foo", "bar");

    let generated = hashmap!{"foo" => "bar"};

    assert_eq!(generated, expected);
}

#[test]
fn test_no_trailing_comma() {
    let mut expected = HashMap::new();
    expected.insert(0, "zero");
    expected.insert(1, "one");
    expected.insert(2, "two");

    let generated = hashmap!{0 => "zero", 1 => "one", 2 => "two"};

    assert_eq!(generated, expected);
}

#[test]
fn test_trailing_comma() {
    let mut expected = HashMap::new();
    expected.insert(0, "zero");
    expected.insert(1, "one");
    expected.insert(2, "two");

    let generated = hashmap!{
        0 => "zero",
        1 => "one",
        2 => "two",
    };

    assert_eq!(generated, expected);
}

#[test]
fn test_nesting() {
    let mut expected = HashMap::new();
    let mut nested = HashMap::new();
    nested.insert("foo", "bar");
    nested.insert("qux", "paz");
    
    expected.insert("nested", nested);
    expected.insert("empty", HashMap::new());

    let generated = hashmap!{
        "nested" => hashmap!{
            "foo" => "bar",
            "qux" => "paz",
        },
        "empty" => hashmap!{}
    };

    assert_eq!(generated, expected);
}

#[test]
fn test_compile_fails_comma_sep() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/invalid/comma-sep/comma-sep.rs");
}

#[test]
fn test_compile_fails_missing_argument() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/invalid/missing-arg/missing-arg.rs");
}

#[test]
fn test_compile_fails_no_comma() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/invalid/no-comma/no-comma.rs");
}

#[test]
fn test_compile_fails_trailing_arrow() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/invalid/trailing-arrow/trailing-arrow.rs");
}

#[test]
fn test_compile_fails_only_arrow() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/invalid/only-arrow/only-arrow.rs");
}

#[test]
fn test_compile_fails_single_arg() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/invalid/single-arg/single-arg.rs");
}
