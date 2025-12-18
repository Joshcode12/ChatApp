#[derive(Debug, serde::Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub password: String,
    pub bio: Option<String>,
    pub profile_file_id: Option<uuid::Uuid>,
}