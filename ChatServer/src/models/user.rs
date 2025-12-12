#[derive(Debug, serde::Deserialize)]
pub struct CreateUserRequest {
    pub password: String,
    pub username: String,
    pub bio: Option<String>,
    pub profile_file_id: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    pub bio: Option<String>,
    pub is_online: bool,
    pub last_seen: Option<time::OffsetDateTime>,
    pub profile_file_id: Option<uuid::Uuid>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateUserRequest {
    pub id: uuid::Uuid,
    pub username: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub profile_file_id: Option<uuid::Uuid>,
}

#[derive(Debug, serde::Deserialize)]
pub struct DeleteUserRequest {
    pub id: uuid::Uuid,
}

#[derive(Debug, serde::Deserialize)]
pub struct UserLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct UserLogoutRequest {
    pub id: uuid::Uuid,
}
