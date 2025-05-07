use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Deref)]
pub struct Colors<'a> {
    #[serde(flatten, borrow)]
    pub colors: HashMap<&'a str, &'a str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config<'a> {
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub colors: HashMap<&'a str, Colors<'a>>,
}
