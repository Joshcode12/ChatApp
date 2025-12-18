CREATE INDEX idx_messages_after_last_read ON messages(conversation_id, created_at)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_users_username_trgm ON users USING gin(username gin_trgm_ops);

CREATE INDEX idx_messages_conversation_latest ON messages(conversation_id, created_at DESC)
    WHERE deleted_at IS NULL;
