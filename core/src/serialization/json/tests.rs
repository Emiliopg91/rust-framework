use std::{collections::HashMap, sync::LazyLock};

use serde_derive::{Deserialize, Serialize};

use crate::serialization::{Serializer, json::JsonSerializer};

#[derive(Serialize, Deserialize, Default)]
struct ConfigBean {
    age: u8,
    name: String,
    list: Vec<String>,
    map: HashMap<String, u8>,
}

static OBJ: LazyLock<ConfigBean> = LazyLock::new(|| ConfigBean {
    age: 18,
    name: "Name".to_string(),
    list: vec!["Hello".to_string(), "World".to_string()],
    map: HashMap::from([("a".to_string(), 1), ("b".to_string(), 2)]),
});

const JSON_STR: &str = r#"{
  "age": 18,
  "name": "Name",
  "list": [
    "Hello",
    "World"
  ],
  "map": {
    "b": 2,
    "a": 1
  }
}"#;

#[test]
fn test_01_json_serialization() {
    assert!(
        JsonSerializer::serialize(&*OBJ).is_ok(),
        "JSON serialization error"
    );
}

#[test]
fn test_02_json_deserialization() {
    assert!(
        JsonSerializer::deserialize::<ConfigBean>(JSON_STR).is_ok(),
        "JSON deserialization error"
    );
}
