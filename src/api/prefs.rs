use actix_web::{HttpResponse, web};
use serde_json::json;

use crate::{
    error::AppResult,
    models::{Claims, PrefsForm, PrefsQuery, db::prefs::PrefEntry},
    store::AppState,
};

fn get_full_key(key: &str, scope_query: Option<String>) -> String {
    let scope = scope_query.unwrap_or("default".into());
    format!("{}.{}", scope, key)
}

async fn get_pref_entry(
    claims: &Claims,
    key: &str,
    scope_query: Option<String>,
    state: web::Data<AppState>,
) -> AppResult<Option<PrefEntry>> {
    let full_key = get_full_key(key, scope_query);
    let state_into = state.clone();
    let user_id_into = claims.sub.clone();
    let entry = web::block(move || state_into.get_pref(&user_id_into, &full_key)).await??;
    Ok(entry)
}

pub async fn prefs_get(
    claims: Claims,
    key: web::Path<String>,
    query: web::Query<PrefsQuery>,
    state: web::Data<AppState>,
) -> AppResult<HttpResponse> {
    let pref_entry = get_pref_entry(&claims, &key, query.scope.clone(), state).await?;
    if let Some(entry) = pref_entry {
        return Ok(HttpResponse::Ok().json(json!({
            "value": entry.pref_value,
        })));
    }

    Ok(HttpResponse::NotFound().finish())
}

pub async fn prefs_post_put(
    claims: Claims,
    key: web::Path<String>,
    query: web::Query<PrefsQuery>,
    state: web::Data<AppState>,
    form: web::Form<PrefsForm>,
) -> AppResult<HttpResponse> {
    let full_key = get_full_key(&key, query.scope.clone());
    web::block(move || {
        state.set_pref(PrefEntry {
            user_id: claims.sub,
            pref_key: full_key,
            pref_value: form.value.clone(),
        })
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}

pub async fn prefs_delete(
    claims: Claims,
    key: web::Path<String>,
    query: web::Query<PrefsQuery>,
    state: web::Data<AppState>,
) -> AppResult<HttpResponse> {
    let full_key = get_full_key(&key, query.scope.clone());
    let pref_entry = get_pref_entry(&claims, &key, query.scope.clone(), state.clone()).await?;
    if pref_entry.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    web::block(move || state.delete_pref(&claims.sub, &full_key)).await??;

    Ok(HttpResponse::Ok().finish())
}
