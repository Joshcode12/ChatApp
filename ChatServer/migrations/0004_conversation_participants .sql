CREATE TABLE conversation_participants
(
    id              UUID PRIMARY KEY,
    conversation_id UUID NOT NULL REFERENCES conversations (id) ON DELETE CASCADE,
    user_id         UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    UNIQUE (conversation_id, user_id)
);