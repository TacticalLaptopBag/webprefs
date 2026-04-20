use actix_web::{HttpResponse, web};

use crate::{
    error::AppResult,
    models::{
        Claims,
        db::prefs::PrefEntry,
        prefs::{
            PrefValueResponseData, PrefsForm, PrefsPath, PrefsResponseData, ScopesResponseData,
        },
    },
    store::AppState,
};

async fn get_pref_entry(
    claims: &Claims,
    scope: String,
    key: String,
    state: web::Data<AppState>,
) -> AppResult<Option<PrefEntry>> {
    let state_into = state.clone();
    let user_id_into = claims.sub.clone();
    let entry = web::block(move || state_into.get_pref(&user_id_into, &scope, &key)).await??;
    Ok(entry)
}

// ----------------------------------------------------------------------------
// ------------------------------- Endpoints ----------------------------------
// ----------------------------------------------------------------------------
pub async fn scopes_get(claims: Claims, state: web::Data<AppState>) -> AppResult<HttpResponse> {
    let scopes = state.get_pref_scopes(&claims.sub)?;
    Ok(HttpResponse::Ok().json(ScopesResponseData { scopes }))
}

pub async fn keys_get(
    claims: Claims,
    scope: web::Path<String>,
    state: web::Data<AppState>,
) -> AppResult<HttpResponse> {
    let prefs = state.get_prefs_in_scope(&claims.sub, &scope)?;
    Ok(HttpResponse::Ok().json(PrefsResponseData { prefs }))
}

pub async fn prefs_all_get(claims: Claims, state: web::Data<AppState>) -> AppResult<HttpResponse> {
    let prefs = state.get_prefs(&claims.sub)?;
    Ok(HttpResponse::Ok().json(PrefsResponseData { prefs }))
}

pub async fn prefs_get(
    claims: Claims,
    path: web::Path<PrefsPath>,
    state: web::Data<AppState>,
) -> AppResult<HttpResponse> {
    let pref_entry = get_pref_entry(&claims, path.scope.clone(), path.key.clone(), state).await?;
    if let Some(entry) = pref_entry {
        return Ok(HttpResponse::Ok().json(PrefValueResponseData {
            value: entry.pref_value,
        }));
    }

    Ok(HttpResponse::NotFound().finish())
}

pub async fn prefs_post(
    claims: Claims,
    path: web::Path<PrefsPath>,
    state: web::Data<AppState>,
    form: web::Form<PrefsForm>,
) -> AppResult<HttpResponse> {
    web::block(move || {
        state.create_pref(PrefEntry {
            user_id: claims.sub,
            pref_key: path.key.clone(),
            pref_scope: path.scope.clone(),
            pref_value: form.value.clone(),
        })
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}

pub async fn prefs_put(
    claims: Claims,
    path: web::Path<PrefsPath>,
    state: web::Data<AppState>,
    form: web::Form<PrefsForm>,
) -> AppResult<HttpResponse> {
    web::block(move || {
        state.update_pref(PrefEntry {
            user_id: claims.sub,
            pref_key: path.key.clone(),
            pref_scope: path.scope.clone(),
            pref_value: form.value.clone(),
        })
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}

pub async fn prefs_delete(
    claims: Claims,
    path: web::Path<PrefsPath>,
    state: web::Data<AppState>,
) -> AppResult<HttpResponse> {
    let pref_entry =
        get_pref_entry(&claims, path.scope.clone(), path.key.clone(), state.clone()).await?;
    if pref_entry.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    web::block(move || state.delete_pref(&claims.sub, &path.scope, &path.key)).await??;

    Ok(HttpResponse::Ok().finish())
}
