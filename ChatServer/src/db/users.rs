fn users_db_error(
    result: Result<crate::models::User, sqlx::Error>,
) -> crate::error::Result<crate::models::User> {
    match result {
        Ok(user) => Ok(user),
        Err(e) => {
            let config = crate::error::DbErrorConfig::new()
                .unique("users_username_key", "Username already taken")
                .foreign_key("Invalid profile file ID")
                .check_violation("Username must be 3-30 characters and bio max 500 characters");

            Err(crate::error::handle_db_error(e, config))
        }
    }
}

pub async fn create_user(
    pool: &sqlx::PgPool,
    req: &crate::models::CreateUserRequest,
) -> crate::error::Result<crate::models::User> {
    let result: Result<crate::models::User, sqlx::Error> = sqlx::query_as!(
        crate::models::User,
        r#"
        INSERT INTO users (username, password_hash, bio, profile_file_id)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        req.username.trim(),
        req.password.trim(),
        req.bio.as_deref().map(str::trim),
        req.profile_file_id
    )
    .fetch_one(pool)
    .await;

    users_db_error(result)
}

pub async fn delete_user(pool: &sqlx::PgPool, id: &uuid::Uuid) -> crate::error::Result<()> {
    let result = sqlx::query!(
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
        req.username.as_deref().map(str::trim),
        req.password.as_deref().map(str::trim),
        req.bio.as_deref().map(str::trim),
        req.profile_file_id
    )
    .fetch_one(pool)
    .await;

    users_db_error(result)
}
