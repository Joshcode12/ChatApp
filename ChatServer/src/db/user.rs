pub async fn delete_user(pool: &sqlx::PgPool, id: &uuid::Uuid) -> crate::error::Result<()> {
    let result: sqlx::postgres::PgQueryResult = sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(crate::error::AppError::NotFound(
            "User not found".to_string(),
        ));
    }

    Ok(())
}

pub async fn update_user(
    pool: &sqlx::PgPool,
    req: &crate::models::UpdateUserRequest,
) -> crate::error::Result<crate::models::User> {
    let result: Result<crate::models::User, sqlx::Error> = sqlx::query_as!(
        crate::models::User,
        r#"
        UPDATE users
        SET
            username = COALESCE($2, username),
            password_hash = COALESCE($3, password_hash),
            bio = COALESCE($4, bio),
            profile_file_id = COALESCE($5, profile_file_id)
        WHERE id = $1
        RETURNING *
        "#,
        req.id,
        req.username,
        req.password,
        req.bio,
        req.profile_file_id
    )
    .fetch_one(pool)
    .await;

    crate::db::error::users_db_error(result)
}
