use unicode_segmentation::UnicodeSegmentation;

pub struct UserPassword(String);


impl UserPassword {
    pub fn parse(name: String) -> Result<Self, String> {
        let chars = name.graphemes(true).count();
        let too_short = chars < 6;
        let too_long = chars > 32;
        let all_ascii = name.chars().any(|g| g.is_ascii_alphabetic());
        if !too_short && !too_long && all_ascii {
            Ok(Self(name))
        } else {
            Err("Parsing error".into())
        }
    }
}

impl AsRef<str> for UserPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}