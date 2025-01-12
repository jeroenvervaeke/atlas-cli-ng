use atlas_derive_core::AsUrl;
use atlas_core::AtlasURL;

#[test]
fn test_basic_url() {
    #[derive(AtlasURL)]
    #[url("/api/v2/cluster/{cluster_id}/query")]
    struct TestUrl {
        cluster_id: String,
        foo: Option<i32>,
        bar: bool,
    }

    let url = TestUrl {
        cluster_id: "123".to_string(),
        foo: Some(42),
        bar: true,
    }
    .as_url("http://jeroenvervaeke.com")
    .unwrap();

    assert_eq!(url.path(), "/api/v2/cluster/123/query");
    let query_pairs: Vec<_> = url.query_pairs().collect();
    assert_eq!(query_pairs.len(), 2);
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "foo" && v == "42"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "bar" && v == "true"));
}

#[test]
fn test_optional_query_params() {
    #[derive(AtlasURL)]
    #[url("/api/test")]
    struct TestUrl {
        optional_param: Option<String>,
    }

    // Test with Some value
    let url = TestUrl {
        optional_param: Some("test".to_string()),
    }
    .as_url("http://jeroenvervaeke.com")
    .unwrap();
    assert!(
        url.query_pairs()
            .any(|(k, v)| k == "optional_param" && v == "test")
    );

    // Test with None value
    let url = TestUrl {
        optional_param: None,
    }
    .as_url("http://jeroenvervaeke.com")
    .unwrap();
    assert_eq!(url.query(), None);
    assert_eq!(url.to_string(), "http://jeroenvervaeke.com/api/test");
}

#[test]
fn test_multiple_path_params() {
    #[derive(AtlasURL)]
    #[url("/api/{version}/users/{user_id}/posts/{post_id}")]
    struct TestUrl {
        version: String,
        user_id: u32,
        post_id: String,
    }

    let url = TestUrl {
        version: "v1".to_string(),
        user_id: 123,
        post_id: "abc".to_string(),
    }
    .as_url("http://jeroenvervaeke.com")
    .unwrap();

    assert_eq!(url.path(), "/api/v1/users/123/posts/abc");
}

#[test]
fn test_vec_path_params() {
    #[derive(AtlasURL)]
    #[url("/api/v1/users/{user_ids}")]
    struct TestUrl {
        user_ids: Vec<u32>,
    }

    let url = TestUrl {
        user_ids: vec![1, 2, 3],
    }
    .as_url("http://jeroenvervaeke.com")
    .unwrap();

    assert_eq!(url.path(), "/api/v1/users/1,2,3");
}

#[test]
fn test_vec_query_params() {
    #[derive(AtlasURL)]
    #[url("/api/{version}/users")]
    struct TestUrl {
        version: String,
        filter: Vec<String>,
    }

    let url = TestUrl {
        version: "v1".to_string(),
        filter: vec!["active".to_string(), "verified".to_string()],
    }
    .as_url("http://jeroenvervaeke.com")
    .unwrap();

    assert_eq!(url.path(), "/api/v1/users");
    let query_pairs: Vec<_> = url.query_pairs().collect();
    assert_eq!(query_pairs.len(), 2);
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "filter" && v == "active"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "filter" && v == "verified"));
}

#[test]
fn test_optional_vec_query_params() {
    #[derive(AtlasURL)]
    #[url("/api/test")]
    struct TestUrl {
        filters: Option<Vec<String>>,
    }

    // Test with Some value
    let url = TestUrl {
        filters: Some(vec!["a".to_string(), "b".to_string()]),
    }
    .as_url("http://jeroenvervaeke.com")
    .unwrap();
    let query_pairs: Vec<_> = url.query_pairs().collect();
    assert_eq!(query_pairs.len(), 2);
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "filters" && v == "a"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "filters" && v == "b"));

    // Test with None value
    let url = TestUrl { filters: None }
        .as_url("http://jeroenvervaeke.com")
        .unwrap();
    assert_eq!(url.query(), None);
    assert_eq!(url.to_string(), "http://jeroenvervaeke.com/api/test");
}

#[test]
fn test_basic_types_path_params() {
    #[derive(AtlasURL)]
    #[url("/api/{str}/{u32}/{i32}/{bool}/{float}")]
    struct TestUrl {
        str: String,
        u32: u32,
        i32: i32,
        bool: bool,
        float: f64,
    }

    let url = TestUrl {
        str: "test".to_string(),
        u32: 42,
        i32: -42,
        bool: true,
        float: 3.14,
    }
    .as_url("http://jeroenvervaeke.com")
    .unwrap();

    assert_eq!(url.path(), "/api/test/42/-42/true/3.14");
}

#[test]
fn test_basic_types_query_params() {
    #[derive(AtlasURL)]
    #[url("/api/test")]
    struct TestUrl {
        str: String,
        u32: u32,
        i32: i32,
        bool: bool,
        float: f64,
    }

    let url = TestUrl {
        str: "test".to_string(),
        u32: 42,
        i32: -42,
        bool: true,
        float: 3.14,
    }
    .as_url("http://jeroenvervaeke.com")
    .unwrap();

    let query_pairs: Vec<_> = url.query_pairs().collect();
    assert_eq!(query_pairs.len(), 5);
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "str" && v == "test"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "u32" && v == "42"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "i32" && v == "-42"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "bool" && v == "true"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "float" && v == "3.14"));
}

#[test]
fn test_vec_types_path_params() {
    #[derive(AtlasURL)]
    #[url("/api/{strings}/{u32s}/{i32s}/{bools}/{floats}")]
    struct TestUrl {
        strings: Vec<String>,
        u32s: Vec<u32>,
        i32s: Vec<i32>,
        bools: Vec<bool>,
        floats: Vec<f64>,
    }

    let url = TestUrl {
        strings: vec!["a".to_string(), "b".to_string()],
        u32s: vec![1, 2],
        i32s: vec![-1, -2],
        bools: vec![true, false],
        floats: vec![1.1, 2.2],
    }
    .as_url("http://jeroenvervaeke.com")
    .unwrap();

    assert_eq!(url.path(), "/api/a,b/1,2/-1,-2/true,false/1.1,2.2");
}

#[test]
fn test_vec_types_query_params() {
    #[derive(AtlasURL)]
    #[url("/api/test")]
    struct TestUrl {
        strings: Vec<String>,
        u32s: Vec<u32>,
        i32s: Vec<i32>,
        bools: Vec<bool>,
        floats: Vec<f64>,
    }

    let url = TestUrl {
        strings: vec!["a".to_string(), "b".to_string()],
        u32s: vec![1, 2],
        i32s: vec![-1, -2],
        bools: vec![true, false],
        floats: vec![1.1, 2.2],
    }
    .as_url("http://jeroenvervaeke.com")
    .unwrap();

    let query_pairs: Vec<_> = url.query_pairs().collect();
    assert_eq!(query_pairs.len(), 10);
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "strings" && v == "a"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "strings" && v == "b"));
    assert!(query_pairs.iter().any(|(k, v)| k == "u32s" && v == "1"));
    assert!(query_pairs.iter().any(|(k, v)| k == "u32s" && v == "2"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "i32s" && v == "-1"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "i32s" && v == "-2"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "bools" && v == "true"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "bools" && v == "false"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "floats" && v == "1.1"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "floats" && v == "2.2"));
}

#[test]
fn test_optional_basic_types() {
    #[derive(AtlasURL)]
    #[url("/api/test")]
    struct TestUrl {
        str: Option<String>,
        u32: Option<u32>,
        i32: Option<i32>,
        bool: Option<bool>,
        float: Option<f64>,
    }

    let url = TestUrl {
        str: Some("test".to_string()),
        u32: Some(42),
        i32: Some(-42),
        bool: Some(true),
        float: Some(3.14),
    }
    .as_url("http://jeroenvervaeke.com")
    .unwrap();

    let query_pairs: Vec<_> = url.query_pairs().collect();
    assert_eq!(query_pairs.len(), 5);
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "str" && v == "test"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "u32" && v == "42"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "i32" && v == "-42"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "bool" && v == "true"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "float" && v == "3.14"));

    let url = TestUrl {
        str: None,
        u32: None,
        i32: None,
        bool: None,
        float: None,
    }
    .as_url("http://jeroenvervaeke.com")
    .unwrap();

    assert_eq!(url.query(), None);
}

#[test]
fn test_optional_vec_types() {
    #[derive(AtlasURL)]
    #[url("/api/test")]
    struct TestUrl {
        strings: Option<Vec<String>>,
        u32s: Option<Vec<u32>>,
        i32s: Option<Vec<i32>>,
        bools: Option<Vec<bool>>,
        floats: Option<Vec<f64>>,
    }

    let url = TestUrl {
        strings: Some(vec!["a".to_string(), "b".to_string()]),
        u32s: Some(vec![1, 2]),
        i32s: Some(vec![-1, -2]),
        bools: Some(vec![true, false]),
        floats: Some(vec![1.1, 2.2]),
    }
    .as_url("http://jeroenvervaeke.com")
    .unwrap();

    let query_pairs: Vec<_> = url.query_pairs().collect();
    assert_eq!(query_pairs.len(), 10);
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "strings" && v == "a"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "strings" && v == "b"));
    assert!(query_pairs.iter().any(|(k, v)| k == "u32s" && v == "1"));
    assert!(query_pairs.iter().any(|(k, v)| k == "u32s" && v == "2"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "i32s" && v == "-1"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "i32s" && v == "-2"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "bools" && v == "true"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "bools" && v == "false"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "floats" && v == "1.1"));
    assert!(query_pairs
        .iter()
        .any(|(k, v)| k == "floats" && v == "2.2"));

    let url = TestUrl {
        strings: None,
        u32s: None,
        i32s: None,
        bools: None,
        floats: None,
    }
    .as_url("http://jeroenvervaeke.com")
    .unwrap();

    assert_eq!(url.query(), None);
}
