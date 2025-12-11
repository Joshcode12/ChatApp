CREATE TABLE messages
(
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    conversation_id UUID        NOT NULL REFERENCES conversations (id) ON DELETE CASCADE,
    sender_id       UUID        NOT NULL REFERENCES users (id) ON DELETE SET NULL,
    content         TEXT        NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    edited          BOOLEAN     NOT NULL DEFAULT FALSE,
    edited_at       TIMESTAMPTZ
);

CREATE INDEX idx_messages_conversation ON messages (conversation_id, created_at DESC);
CREATE INDEX idx_messages_sender ON messages(sender_id);

CREATE TABLE message_attachments
(
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    message_id UUID NOT NULL REFERENCES messages (id) ON DELETE CASCADE,
    file_id    UUID NOT NULL REFERENCES files (id) ON DELETE RESTRICT
);

CREATE INDEX idx_message_attachments_message ON message_attachments(message_id);
