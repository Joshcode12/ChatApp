pub async fn register_user(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    axum::Json(mut payload): axum::Json<crate::models::RegisterUserRequest>,
) -> crate::error::Result<(axum::http::StatusCode, axum::Json<crate::models::User>)> {
    payload.username = payload.username.trim().to_string();
    payload.password = payload.password.trim().to_string();
    payload.bio = payload.bio.map(|s| s.trim().to_string());

    payload.password = crate::middleware::password::hash_password(payload.password.as_str())?;

    let user: crate::models::User = crate::db::auth::register_user(&state.db, &payload).await?;

    Ok((axum::http::StatusCode::OK, axum::Json(user)))
}

pub async fn login_user(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    axum::Json(mut payload): axum::Json<crate::models::LoginUserRequest>,
) -> crate::error::Result<(
    axum::http::StatusCode,
    axum::Json<crate::models::LoginUserResponse>,
)> {
    payload.username = payload.username.trim().to_string();
    payload.password = payload.password.trim().to_string();

    let user: crate::models::User = crate::db::auth::login_user(&state.db, &payload).await?;

    let result: crate::error::Result<bool> =
        crate::middleware::password::verify_password(&*payload.password, &*user.password_hash);

    if result.is_err() {
        return Err(crate::error::AppError::BadRequest(
            "Incorrect password".to_string(),
        ));
    }

    Ok((
        axum::http::StatusCode::OK,
        axum::Json(crate::models::LoginUserResponse { id: user.id }),
    ))
}
