use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Condition {
    AttrsOr(HashMap<String, String>),
    AttrsAnd(HashMap<String, String>),
    TagsOr(Vec<String>),
    TagsAnd(Vec<String>),
}

impl Condition {
    pub fn attrs_or<S: AsRef<str>>(attrs: HashMap<S, S>) -> Self {
        Self::AttrsOr(
            attrs
                .iter()
                .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
                .collect(),
        )
    }

    pub fn attrs_and<S: AsRef<str>>(attrs: HashMap<S, S>) -> Self {
        Self::AttrsAnd(
            attrs
                .iter()
                .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
                .collect(),
        )
    }

    pub fn tags_or<S: AsRef<str>>(tags: Vec<S>) -> Self {
        Self::TagsOr(tags.iter().map(|s| s.as_ref().to_string()).collect())
    }

    pub fn tags_and<S: AsRef<str>>(tags: Vec<S>) -> Self {
        Self::TagsAnd(tags.iter().map(|s| s.as_ref().to_string()).collect())
    }
}
