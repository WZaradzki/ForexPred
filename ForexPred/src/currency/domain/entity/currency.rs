use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Currency {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub iso: String,
}