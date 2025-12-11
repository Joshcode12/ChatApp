CREATE TABLE messages
(
    id              UUID PRIMARY KEY,
    conversation_id UUID        NOT NULL REFERENCES conversations (id) ON DELETE CASCADE,
    sender_id       UUID        NOT NULL REFERENCES users (id) ON DELETE SET NULL,
    content         TEXT        NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    edited          BOOLEAN     NOT NULL DEFAULT FALSE,
    edited_at       TIMESTAMPTZ
);

CREATE INDEX idx_messages_conversation ON messages (conversation_id, created_at);