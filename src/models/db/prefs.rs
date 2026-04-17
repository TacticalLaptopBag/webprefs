use diesel::{prelude::*, sqlite::Sqlite};

use crate::schema;

#[derive(Clone, Debug, Queryable, Selectable, Insertable)]
#[diesel(
    table_name = schema::prefs,
    check_for_backend(Sqlite),
)]
pub struct PrefEntry {
    pub user_id: String,
    pub pref_key: String,
    pub pref_value: Option<String>,
}
