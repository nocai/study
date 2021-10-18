use derive_more::Display;
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, Display, Type, Serialize, Deserialize, PartialEq)]
#[sqlx(rename = "ad_type")]
pub enum AdType {
    Text,
    Image,
    SmallImageText,
    BigImageText,
    ThreeImages,
    Video,
}

#[derive(Debug, Clone, Display, Type, Serialize, Deserialize, PartialEq)]
#[sqlx(rename = "os")]
pub enum OS {
    Android,
    IOS,
    WP,
}

#[derive(Debug, Clone, Type, Display, Serialize, Deserialize, PartialEq)]
#[sqlx(rename = "status")]
pub enum Status {
    Invalid,
    Valid,
}
