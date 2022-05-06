use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Preferences {}

impl Preferences {
    pub fn new() -> Self {
        Self::default()
    }
}
