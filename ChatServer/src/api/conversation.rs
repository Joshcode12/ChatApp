pub async fn create_conversation(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    axum::Json(mut payload): axum::Json<crate::models::ConversationCreateRequest>,
) -> crate::error::Result<(
    axum::http::StatusCode,
    axum::Json<crate::models::ConversationCreateResponse>,
)> {
    payload.name = payload.name.trim().to_string();
    payload.r#type = payload.r#type.trim().to_string();

    let mut ids: Vec<String> = payload
        .participants
        .iter()
        .map(|p| p.user_id.to_string())
        .collect();
    ids.sort();

    let signature: String = ids.join(",");

    let conversation: crate::models::ConversationCreateResponse =
        crate::db::conversation::create_conversation(&state.db, &payload, &signature).await?;

    Ok((axum::http::StatusCode::OK, axum::Json(conversation)))
}

pub async fn delete_conversation(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> crate::error::Result<axum::http::StatusCode> {
    crate::db::conversation::delete_conversation(&state.db, &id).await?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}
