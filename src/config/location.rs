use non_blank_string_rs::NonBlankString;
use serde::Deserialize;

use crate::config::counter::Counter;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Location {
    pub name: NonBlankString,
    pub counters: Vec<Counter>
}