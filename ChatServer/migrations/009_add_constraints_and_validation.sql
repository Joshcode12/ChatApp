ALTER TABLE messages
    ADD CONSTRAINT check_message_content_not_empty
    CHECK (length(trim(content)) > 0 OR deleted_at IS NOT NULL);

ALTER TABLE users
    ADD CONSTRAINT check_username_length
    CHECK (length(username) >= 3 AND length(username) <= 30);

ALTER TABLE users
    ADD CONSTRAINT check_bio_length
    CHECK (bio IS NULL OR length(bio) <= 500);

ALTER TABLE conversations
    ADD CONSTRAINT check_group_has_name
    CHECK (type = 'direct' OR (type = 'group' AND name IS NOT NULL));

ALTER TABLE files
    ADD CONSTRAINT check_file_size_positive
    CHECK (size_bytes > 0);

ALTER TABLE refresh_tokens
    ADD CONSTRAINT check_token_expiry_future
    CHECK (expires_at > created_at);
