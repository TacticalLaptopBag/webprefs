use diesel::{prelude::*, sqlite::Sqlite};
use serde::Serialize;

use crate::schema;

#[derive(Clone, Debug, Queryable, Selectable, Insertable, Serialize)]
#[diesel(
    table_name = schema::prefs,
    check_for_backend(Sqlite),
)]
pub struct PrefEntry {
    #[serde(skip_serializing)]
    pub user_id: String,
    pub pref_key: String,
    pub pref_scope: String,
    pub pref_value: Option<String>,
}
