pub async fn create_user(
    pool: &sqlx::PgPool,
    username: &str,
    password_hash: &str,
    bio: Option<&str>,
    profile_file_id: Option<&uuid::Uuid>,
) -> crate::error::Result<crate::models::user::User> {
    let user: crate::models::User = sqlx::query_as::<_, crate::models::user::User>(
        r#"
        INSERT INTO users (username, password_hash, bio, profile_file_id)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(username)
    .bind(password_hash)
    .bind(bio)
    .bind(profile_file_id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn delete_user(pool: &sqlx::PgPool, id: &uuid::Uuid) -> crate::error::Result<()> {
    let result = sqlx::query(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(crate::error::AppError::NotFound("User not found".into()));
    }

    Ok(())
}
