use serde_json::Value::String;

pub async fn register_user(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    axum::Json(payload): axum::Json<crate::models::CreateUserRequest>,
) -> crate::error::Result<(axum::http::StatusCode, axum::Json<crate::models::User>)> {
    let user: crate::models::User = crate::db::users::create_user(
        &state.db,
        payload.username.trim(),               // trim username
        payload.password.trim(),               // trim password
        payload.bio.as_deref().map(str::trim), // trim bio if present
        payload.profile_file_id.as_ref(),
    )
    .await?;

    Ok((axum::http::StatusCode::CREATED, axum::Json(user)))
}

pub async fn deregister_user(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> crate::error::Result<(axum::http::StatusCode, axum::Json<()>)> {
    crate::db::users::delete_user(&state.db, &id).await?;
    Ok((axum::http::StatusCode::OK, axum::Json(())))
}
