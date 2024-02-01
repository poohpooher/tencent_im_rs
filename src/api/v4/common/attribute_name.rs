use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributeName {
    #[serde(rename = "0")]
    zero: Option<String>,
    #[serde(rename = "1")]
    one: Option<String>,
    #[serde(rename = "2")]
    two: Option<String>,
    #[serde(rename = "3")]
    three: Option<String>,
    #[serde(rename = "4")]
    four: Option<String>,
    #[serde(rename = "5")]
    five: Option<String>,
    #[serde(rename = "6")]
    six: Option<String>,
    #[serde(rename = "7")]
    seven: Option<String>,
    #[serde(rename = "8")]
    eight: Option<String>,
    #[serde(rename = "9")]
    nine: Option<String>,
}
impl Default for AttributeName {
    fn default() -> Self {
        Self::new()
    }
}

impl AttributeName {
    pub fn new() -> Self {
        Self {
            zero: None,
            one: None,
            two: None,
            three: None,
            four: None,
            five: None,
            six: None,
            seven: None,
            eight: None,
            nine: None,
        }
    }

    pub fn set_zero<S: AsRef<str>>(&mut self, s: Option<S>) -> &mut Self {
        self.zero = s.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_one<S: AsRef<str>>(&mut self, s: Option<S>) -> &mut Self {
        self.one = s.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_two<S: AsRef<str>>(&mut self, s: Option<S>) -> &mut Self {
        self.two = s.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_three<S: AsRef<str>>(&mut self, s: Option<S>) -> &mut Self {
        self.three = s.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_four<S: AsRef<str>>(&mut self, s: Option<S>) -> &mut Self {
        self.four = s.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_five<S: AsRef<str>>(&mut self, s: Option<S>) -> &mut Self {
        self.five = s.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_six<S: AsRef<str>>(&mut self, s: Option<S>) -> &mut Self {
        self.six = s.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_seven<S: AsRef<str>>(&mut self, s: Option<S>) -> &mut Self {
        self.seven = s.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_eight<S: AsRef<str>>(&mut self, s: Option<S>) -> &mut Self {
        self.eight = s.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_nine<S: AsRef<str>>(&mut self, s: Option<S>) -> &mut Self {
        self.nine = s.map(|s| s.as_ref().to_string());
        self
    }
}
