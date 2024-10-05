use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub public_key: String,
    pub alias: Option<String>,
    pub capacity: Option<String>,
    pub first_seen: Option<String>, 
}


#[derive(Deserialize, Serialize, Debug)]
pub struct ExternalNode {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub alias: Option<String>,
    pub channels: i32,
    pub capacity: i64, 
    #[serde(rename = "firstSeen")]
    pub first_seen: i64,
}