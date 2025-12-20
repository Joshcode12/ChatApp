#[derive(Debug, serde::Deserialize)]
pub struct ConversationCreateRequest {
    pub r#type: String,
    pub name: String,
    pub participants: Vec<ConversationParticipantCreateRequest>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ConversationParticipantCreateRequest {
    pub user_id: uuid::Uuid,
    pub role: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ConversationCreateResponse {
    pub id: uuid::Uuid,
}
