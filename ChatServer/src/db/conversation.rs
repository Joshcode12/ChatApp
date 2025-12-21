pub async fn create_conversation(
    pool: &sqlx::PgPool,
    req: &crate::models::ConversationCreateRequest,
    signature: &String,
) -> crate::error::Result<crate::models::ConversationCreateResponse> {
    let mut tx: sqlx_core::transaction::Transaction<sqlx::Postgres> = pool.begin().await?;

    let conversation: crate::models::ConversationCreateResponse = crate::error::map_db_error(
        sqlx::query_as!(
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
            .fetch_one(&mut *tx),
        crate::error::DbErrorConfig::new()
            .unique(
                "unique_conversation_name_participants",
                "A conversation with these participants already exists",
            )
            .not_null("Conversation name is required"),
    )
        .await?;

    let conversation_id: uuid::Uuid = conversation.id;

    for participant in &req.participants {
        crate::error::map_db_error(
            sqlx::query!(
                r#"
                INSERT INTO conversation_participants (conversation_id, user_id, role)
                VALUES ($1, $2, $3)
                "#,
                conversation_id,
                participant.user_id,
                participant.role
            )
                .execute(&mut *tx),
            crate::error::DbErrorConfig::new()
                .foreign_key(format!("User {} does not exist", participant.user_id)),
        )
            .await?;
    }

    tx.commit().await?;

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
