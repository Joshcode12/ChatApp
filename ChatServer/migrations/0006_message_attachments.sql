CREATE TABLE message_attachments
(
    id         UUID PRIMARY KEY,
    message_id UUID NOT NULL REFERENCES messages (id) ON DELETE CASCADE,
    file_id    UUID NOT NULL REFERENCES files (id) ON DELETE RESTRICT
);