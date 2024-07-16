use non_blank_string_rs::NonBlankString;
use serde::{Deserialize, Serialize};

use crate::config::counter::Counter;

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    pub name: NonBlankString,
    pub counters: Vec<Counter>
}