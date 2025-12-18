pub async fn register_user(
    pool: &sqlx::PgPool,
    req: &crate::models::RegisterUserRequest,
) -> crate::error::Result<crate::models::User> {
    let result: Result<crate::models::User, sqlx::Error> = sqlx::query_as!(
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
    .fetch_one(pool)
    .await;

    crate::db::error::users_db_error(result)
}
