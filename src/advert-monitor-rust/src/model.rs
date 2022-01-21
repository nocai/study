use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[repr(i32)]
pub enum Status {
    Invalid = 1,
    Valid = 0,
}

impl Default for Status {
    fn default() -> Self {
        Status::Invalid
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[repr(i32)]
pub enum BidType {
    CPT = 1,
    CPM = 2,
    CPC = 3,
}

impl Default for BidType {
    fn default() -> Self {
        BidType::CPT
    }
}
