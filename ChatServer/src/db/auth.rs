pub async fn register_user(
    pool: &sqlx::PgPool,
    req: &crate::models::RegisterUserRequest,
) -> crate::error::Result<crate::models::User> {
    let result: crate::models::User = crate::error::map_db_error(
        sqlx::query_as!(
            crate::models::User,
            r#"
            INSERT INTO users (username, password_hash, bio, profile_file_id)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            req.username,
            req.password,
            req.bio,
            req.profile_file_id
        )
            .fetch_one(pool),
        crate::error::DbErrorConfig::new()
            .unique("users_username_key", "Username already taken")
            .foreign_key("Invalid profile file ID")
            .check_violation("Username must be 3-30 characters and bio max 500 characters")
            .not_null("Username or password cannot be empty"),
    )
        .await?;

    Ok(result)
}

pub async fn login_user(
    pool: &sqlx::PgPool,
    req: &crate::models::LoginUserRequest,
) -> crate::error::Result<crate::models::User> {
    let result: Result<crate::models::User, sqlx::Error> = sqlx::query_as!(
        crate::models::User,
        r#"
        SELECT * FROM users
                  WHERE username = $1 
        "#,
        req.username
    )
        .fetch_one(pool)
        .await;

    if result.is_err() {
        return Err(crate::error::AppError::NotFound(
            "Incorrect username".to_string(),
        ));
    }

    Ok(result?)
}
