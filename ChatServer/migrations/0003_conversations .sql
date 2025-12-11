CREATE TABLE conversations
(
    id         UUID PRIMARY KEY,
    type       TEXT        NOT NULL CHECK (type IN ('direct', 'group')),
    name       TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);