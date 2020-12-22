use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct fault {
    pub name: String, 
    pub error: String,
    pub command: String,
    pub delay: i16,
}