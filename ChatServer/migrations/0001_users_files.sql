CREATE TABLE users
(
    id              UUID PRIMARY KEY,
    username        TEXT        NOT NULL UNIQUE,
    password_hash   TEXT        NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    profile_file_id UUID
);

CREATE TABLE files
(
    id          UUID PRIMARY KEY,
    owner_id    UUID        NOT NULL,
    filename    TEXT        NOT NULL,
    mime_type   TEXT,
    size_bytes  BIGINT      NOT NULL,
    storage_key TEXT        NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

ALTER TABLE files
    ADD CONSTRAINT fk_files_owner FOREIGN KEY (owner_id)
        REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE users
    ADD CONSTRAINT fk_users_profile FOREIGN KEY (profile_file_id)
        REFERENCES files (id) ON DELETE SET NULL;
