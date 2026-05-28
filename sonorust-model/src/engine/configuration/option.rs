use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum EngineOption {
    Slider {
        name: String,
        description: Option<String>,
        standard: Option<bool>,
        advanced: Option<bool>,
        scope: Option<String>,
        def: f64,
        min: f64,
        max: f64,
        step: f64,
        unit: Option<String>,
    },
    Toggle {
        name: String,
        description: Option<String>,
        standard: Option<bool>,
        advanced: Option<bool>,
        scope: Option<String>,
        def: f64,
    },
    Select {
        name: String,
        description: Option<String>,
        standard: Option<bool>,
        advanced: Option<bool>,
        scope: Option<String>,
        def: f64,
        values: Vec<String>,
    },
}
