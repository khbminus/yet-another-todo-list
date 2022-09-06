use serde::Deserialize;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Deserialize, Debug)]
pub struct ToDoListName(String);

impl ToDoListName {
    pub fn parse(name: String) -> Result<Self, String> {
        let length = name.graphemes(true).count();
        if length > 288 {
            Err("name is too long".into())
        } else {
            Ok(Self(name))
        }
    }
}

impl AsRef<str> for ToDoListName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}