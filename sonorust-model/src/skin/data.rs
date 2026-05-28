use bevy::prelude::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(Deref, DerefMut, Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SkinSpriteName(pub String);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinData {
    pub width: f64,
    pub height: f64,
    pub interpolation: bool,
    pub sprites: Vec<SkinSprite>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinSprite {
    pub name: SkinSpriteName,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub transform: SkinSpriteTransform,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinSpriteTransform {
    pub x1: SkinSpriteTransformExpression,
    pub y1: SkinSpriteTransformExpression,
    pub x2: SkinSpriteTransformExpression,
    pub y2: SkinSpriteTransformExpression,
    pub x3: SkinSpriteTransformExpression,
    pub y3: SkinSpriteTransformExpression,
    pub x4: SkinSpriteTransformExpression,
    pub y4: SkinSpriteTransformExpression,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinSpriteTransformExpression {
    pub x1: Option<f32>,
    pub x2: Option<f32>,
    pub x3: Option<f32>,
    pub x4: Option<f32>,
    pub y1: Option<f32>,
    pub y2: Option<f32>,
    pub y3: Option<f32>,
    pub y4: Option<f32>,
}
