use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Node {
    Literal { value: f64 },
    FunctionCall { func: String, args: Vec<usize> },
}
