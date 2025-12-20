pub async fn create_conversation(
    pool: &sqlx::PgPool,
    req: &crate::models::ConversationCreateRequest,
    signature: &String,
) -> crate::error::Result<crate::models::ConversationCreateResponse> {
    let result: Result<crate::models::ConversationCreateResponse, sqlx::Error> = sqlx::query_as!(
        crate::models::ConversationCreateResponse,
        r#"
        INSERT INTO conversations (type, name, participant_signature)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        req.r#type,
        req.name,
        signature
    )
    .fetch_one(pool)
    .await;

    let conversation_id: uuid::Uuid = result?.id;

    for participant in &req.participants {
        sqlx::query!(
            r#"
            INSERT INTO conversation_participants (conversation_id, user_id, role)
            VALUES ($1, $2, $3)
            "#,
            conversation_id,
            participant.user_id,
            participant.role
        )
        .execute(pool)
        .await?;
    }

    Ok(crate::models::ConversationCreateResponse {
        id: conversation_id,
    })
}

pub async fn delete_conversation(pool: &sqlx::PgPool, id: &uuid::Uuid) -> crate::error::Result<()> {
    let result: sqlx::postgres::PgQueryResult = sqlx::query!(
        r#"
        DELETE FROM conversations
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(crate::error::AppError::NotFound(
            "Conversation not found".to_string(),
        ));
    }

    Ok(())
}
