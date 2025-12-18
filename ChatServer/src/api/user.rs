pub async fn deregister_user(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> crate::error::Result<axum::http::StatusCode> {
    crate::db::user::delete_user(&state.db, &id).await?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

pub async fn update_user(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    axum::Json(mut payload): axum::Json<crate::models::UpdateUserRequest>,
) -> crate::error::Result<(axum::http::StatusCode, axum::Json<crate::models::User>)> {
    payload.username = payload.username.map(|s| s.trim().to_string());
    payload.password = payload.password.map(|s| s.trim().to_string());
    payload.bio = payload.bio.map(|s| s.trim().to_string());

    if let Some(ref password) = payload.password {
        let hashed: String = crate::middleware::password::hash_password(password)?;
        payload.password = Some(hashed);
    }

    let user: crate::models::User = crate::db::user::update_user(&state.db, &payload).await?;

    Ok((axum::http::StatusCode::OK, axum::Json(user)))
}
