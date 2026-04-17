use actix_web::{HttpRequest, HttpResponse, web};
use serde_json::json;
use uuid::Uuid;

use crate::{
    api::auth::{blacklist_tokens, clear_access_cookies, create_access_cookies},
    error::{AppError, AppResult},
    models::{Claims, LoginForm, UserInfo, db::user::NewUser},
    store::{AppState, hash_password},
};

pub async fn user_get(
    id: web::Path<String>,
    state: web::Data<AppState>,
) -> AppResult<HttpResponse> {
    let user =
        web::block(move || state.get_user_by_id(&id)?.ok_or(AppError::DbObjectNotFound)).await??;
    Ok(HttpResponse::Ok().json(UserInfo::from_user(user)))
}

pub async fn user_post(
    state: web::Data<AppState>,
    form: web::Form<LoginForm>,
) -> AppResult<HttpResponse> {
    let user_id = Uuid::new_v4().to_string();

    let state_into = state.clone();
    let user_id_into = user_id.clone();
    web::block(move || {
        state_into.create_user(NewUser {
            id: &user_id_into,
            username: &form.username,
            password_hash: &hash_password(&form.password)?,
        })
    })
    .await??;

    let state_into = state.clone();
    let user = web::block(move || state_into.get_user_by_id(&user_id))
        .await??
        .ok_or(AppError::DbObjectNotFound)?;

    let cookies = create_access_cookies(&state, &user)?;
    Ok(HttpResponse::Ok()
        .cookie(cookies.access_cookie)
        .cookie(cookies.refresh_cookie)
        .json(json!({
                "message": format!("Created new user {}", user.username),
                "user": {
                    "id": user.id,
                    "username": user.username,
                },
            }
        )))
}

pub async fn user_delete(
    req: HttpRequest,
    claims: Claims,
    state: web::Data<AppState>,
) -> AppResult<HttpResponse> {
    let user_id = claims.sub;

    let state_into = state.clone();
    let user_id_into = user_id.clone();
    let user = web::block(move || state_into.delete_user_with_id(&user_id_into))
        .await??
        .ok_or(AppError::DbObjectNotFound)?;

    blacklist_tokens(&req, state.clone()).await?;
    let cleared_cookies = clear_access_cookies(&state);

    Ok(HttpResponse::Ok()
        .cookie(cleared_cookies.access_cookie)
        .cookie(cleared_cookies.refresh_cookie)
        .json(json!({
            "message": format!("Deleted user {}", user.username)
        })))
}
