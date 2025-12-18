ALTER TABLE messages
    ADD COLUMN deleted_at TIMESTAMPTZ,
    ADD COLUMN deleted_by UUID REFERENCES users (id);

CREATE INDEX idx_messages_deleted ON messages (deleted_at) WHERE deleted_at IS NOT NULL;
