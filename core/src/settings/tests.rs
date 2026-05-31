use std::fs;

use serde_derive::{Deserialize, Serialize};

use crate::{settings::Settings, utils::file::FileUtils};
#[derive(Serialize, Deserialize, Default)]
struct ConfigBean {
    age: u8,
    name: String,
}

impl ConfigBean {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn age(&self) -> u8 {
        self.age
    }

    pub fn set_age(&mut self, age: u8) {
        self.age = age;
    }
}

#[test]
fn test_01_check_constructor() {
    Settings::<ConfigBean>::default();
}

#[tokio::test]
async fn test_02_check_load() {
    println!();
    let tmp = FileUtils::new_tmp_file().unwrap();
    let content = r#"age: 18
name: Name
"#;
    fs::write(&tmp, content).unwrap();
    let settings = Settings::<ConfigBean>::load(&tmp).await.unwrap();
    assert!(settings.age() == 18);
    assert!(settings.name() == "Name");
}

#[tokio::test]
async fn test_03_check_saves() {
    println!();
    let mut settings = Settings::<ConfigBean>::default();
    settings.set_age(18);
    settings.set_name("Name");

    let tmp = FileUtils::new_tmp_file().unwrap();
    settings.save(&tmp).await.unwrap();
}
