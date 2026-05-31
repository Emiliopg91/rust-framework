use std::{collections::HashMap, sync::LazyLock};

use serde_derive::{Deserialize, Serialize};

use crate::serialization::{Serializer, toml::TomlSerializer};

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

const TOML_STR: &str = r#"age = 18
name = "Name"
list = ["Hello", "World"]

[map]
b = 2
a = 1
"#;

#[test]
fn test_01_toml_serialization() {
    assert!(
        TomlSerializer::serialize(&*OBJ).is_ok(),
        "TOML deserialization error"
    );
}

#[test]
fn test_02_toml_deserialization() {
    assert!(
        TomlSerializer::deserialize::<ConfigBean>(TOML_STR).is_ok(),
        "TOML deserialization error"
    );
}
