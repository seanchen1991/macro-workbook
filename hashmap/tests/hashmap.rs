use hashmap::hashmap;
use std::collections::HashMap;

#[test]
fn test_empty() {
    let expected: HashMap<i32, i32> = HashMap::new();
    let generated: HashMap<i32, i32> = hashmap!();
    assert_eq!(generated, expected);
}

#[test]
fn test_single() {
    let mut expected = HashMap::new();
    expected.insert("foo", "bar");

    let generated = hashmap!("foo" => "bar");

    assert_eq!(generated, expected);
}

#[test]
fn test_no_trailing_comma() {
    let mut expected = HashMap::new();
    expected.insert(0, "zero");
    expected.insert(1, "one");
    expected.insert(2, "two");

    let generated = hashmap!(0 => "zero", 1 => "one", 2 => "two");

    assert_eq!(generated, expected);
}

#[test]
fn test_trailing_comma() {
    let mut expected = HashMap::new();
    expected.insert(0, "zero");
    expected.insert(1, "one");
    expected.insert(2, "two");

    let generated = hashmap!(
        0 => "zero",
        1 => "one",
        2 => "two",
    );

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

    let generated = hashmap!(
        "nested" => hashmap!(
            "foo" => "bar",
            "qux" => "paz",
        ),
        "empty" => hashmap!()
    );

    assert_eq!(generated, expected);
}

mod trybuild {
    use std::path::PathBuf;
    use std::process::Command;

    pub fn compile_fail(file_name: &str) {
        let invalid_path = ["tests", "invalid"].iter().collect::<PathBuf>();

        let mut file_path = invalid_path.clone();
        file_path.push(file_name);

        assert!(
            file_path.exists(),
            "{:?} does not exist.",
            file_path.into_os_string()
        );

        let test_name = file_name.replace(".", "-");
        let macros_dir = ["..", "..", "target", "tests", "hashmap"]
            .iter()
            .collect::<PathBuf>();

        let result = Command::new("cargo")
            .current_dir(invalid_path)
            .arg("build")
            .arg("--offline")
            .arg("--target-dir")
            .arg(macros_dir)
            .arg("--bin")
            .arg(test_name)
            .output();

        if let Ok(result) = result {
            assert!(
                !result.status.success(),
                "Expected {:?} to fail to compile, but it succeeded",
                file_path
            );
        } else {
            panic!("Running subprocess failed");
        }
    }
}
