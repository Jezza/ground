use super::*;

fn test_env<T: FromEnv>(env: &[(&'static str, &'static str)]) -> Result<T> {
    let mut ctx = Context::empty();
    for (k, v) in env {
        ctx.env.insert(String::from(*k), Ok((*v).into()));
    }
    T::from_ctx(&ctx)
}

#[test]
fn test_simple() {
    #[derive(FromEnv)]
    #[env(root = "crate")]
    struct Test {
        text: String,
        optional_text_missing: Option<String>,
        optional_text_present: Option<String>,
        number: i64,
    }

    let test = test_env::<Test>(&[
        ("TEXT", "Hello"),
        ("OPTIONAL_TEXT_PRESENT", "World"),
        ("NUMBER", "-42"),
    ])
    .unwrap();
    assert_eq!(test.text, "Hello");
    assert!(test.optional_text_missing.is_none());
    assert_eq!(test.optional_text_present.as_deref(), Some("World"));
    assert_eq!(test.number, -42);

    assert!(test_env::<Test>(&[("TEXT", "Hello"), ("OPTIONAL_TEXT_PRESENT", "World"),]).is_err());
}

#[test]
fn test_rename() {
    #[derive(FromEnv)]
    #[env(root = "crate")]
    struct Test {
        #[env(rename = "NUMBER")]
        text: String,
    }

    let test = test_env::<Test>(&[("NUMBER", "Hello")]).unwrap();
    assert_eq!(test.text, "Hello");
}

#[test]
fn test_defaults() {
    #[derive(FromEnv)]
    #[env(root = "crate")]
    struct Test {
        #[env(default_value = "Hello, World!")]
        text: String,
        #[env(default)]
        number: i64,
        #[env(default)]
        optional_text: Option<String>,
    }

    let test = test_env::<Test>(&[]).unwrap();
    assert_eq!(test.text, "Hello, World!");
    assert_eq!(test.number, 0);
    assert!(test.optional_text.is_none());
}

#[test]
fn test_flatten() {
    #[derive(Debug, FromEnv)]
    #[env(root = "crate")]
    struct TestText {
        text: String,
    }

    #[derive(Debug, FromEnv)]
    #[env(root = "crate")]
    struct TestNumber {
        number: i64,
    }

    #[derive(Debug, FromEnv)]
    #[env(root = "crate")]
    struct Test {
        #[env(flatten)]
        text: TestText,
        #[env(flatten = "TEST_")]
        number: TestNumber,
    }

    let test = test_env::<Test>(&[("TEXT", "World"), ("TEST_NUMBER", "42")]).unwrap();
    assert_eq!(test.text.text, "World");
    assert_eq!(test.number.number, 42);
}

#[test]
fn test_vec() {
    #[derive(Debug, FromEnv)]
    #[env(root = "crate")]
    struct Test {
        #[env(delimiter = " ")]
        text: Vec<String>,
        number: Vec<i64>,
        #[env(default)]
        empty: Vec<i64>,
    }

    let test = test_env::<Test>(&[("TEXT", "Hello World"), ("NUMBER", "1,2,3,4")]).unwrap();
    assert_eq!(test.text, vec!["Hello", "World"]);
    assert_eq!(test.number, vec![1, 2, 3, 4]);
    assert_eq!(test.empty, vec![]);
}

#[test]
fn test_invalid_data_type() {
    #[derive(FromEnv)]
    #[env(root = "crate")]
    #[allow(dead_code)]
    struct Test {
        number: i64,
    }

    assert!(test_env::<Test>(&[("NUMBER", "not_a_number")]).is_err());
}

#[test]
fn test_nested_structures() {
    #[derive(Debug, FromEnv)]
    #[env(root = "crate")]
    struct Inner {
        inner_text: String,
    }

    #[derive(Debug, FromEnv)]
    #[env(root = "crate")]
    struct Outer {
        outer_text: String,
        #[env(flatten)]
        inner: Inner,
    }

    let test = test_env::<Outer>(&[("OUTER_TEXT", "Hello"), ("INNER_TEXT", "World")]).unwrap();
    assert_eq!(test.outer_text, "Hello");
    assert_eq!(test.inner.inner_text, "World");
}

#[test]
fn test_missing_required_fields() {
    #[derive(FromEnv)]
    #[env(root = "crate")]
    #[allow(dead_code)]
    struct Test {
        text: String,
    }

    assert!(test_env::<Test>(&[]).is_err());
}

#[test]
fn test_different_delimiters() {
    #[derive(Debug, FromEnv)]
    #[env(root = "crate")]
    struct Test {
        #[env(delimiter = ",")]
        text: Vec<String>,
    }

    let test = test_env::<Test>(&[("TEXT", "Hello,World")]).unwrap();
    assert_eq!(test.text, vec!["Hello", "World"]);
}
