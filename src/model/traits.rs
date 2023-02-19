use serde::{Serialize};

pub trait ToJson {
    fn to_json(&self) -> String where Self: Serialize {
        serde_json::to_string(&self).unwrap()
    }
}