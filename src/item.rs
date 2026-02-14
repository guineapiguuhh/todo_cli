use std::fmt::Display;

use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub desc: String,
    pub checked: bool,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.checked {
            write!(f, "[{}]", "X".green())?;
        } else {
            write!(f, "[ ]")?;
        }
        write!(f, " {} :\n{}", self.name, self.desc)
    }
}
