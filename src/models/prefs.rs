use serde::{Deserialize, Serialize};

use crate::models::db::prefs::PrefEntry;

#[derive(Debug, Deserialize, Clone)]
pub struct PrefsForm {
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PrefsPath {
    pub scope: String,
    pub key: String,
}

#[derive(Debug, Serialize)]
pub struct ScopesResponseData {
    pub scopes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PrefsResponseData {
    pub prefs: Vec<PrefEntry>,
}

#[derive(Debug, Serialize)]
pub struct PrefValueResponseData {
    pub value: Option<String>,
}
