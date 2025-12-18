CREATE OR REPLACE FUNCTION prevent_duplicate_direct_conversations()
    RETURNS TRIGGER AS
$$
DECLARE
    participant_count INTEGER;
BEGIN
    IF NEW.type = 'direct' THEN
        SELECT COUNT(*)
        INTO participant_count
        FROM conversation_participants
        WHERE conversation_id = NEW.id;

        IF participant_count > 2 THEN
            RAISE EXCEPTION 'Direct conversations can only have 2 participants';
        END IF;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_direct_conversation_participants
    AFTER INSERT
    ON conversations
    FOR EACH ROW
EXECUTE FUNCTION prevent_duplicate_direct_conversations();

CREATE OR REPLACE FUNCTION update_message_edited_at()
    RETURNS TRIGGER AS
$$
BEGIN
    IF NEW.edited = TRUE AND OLD.edited = FALSE THEN
        NEW.edited_at = now();
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER message_edited_timestamp
    BEFORE UPDATE
    ON messages
    FOR EACH ROW
    WHEN (NEW.edited = TRUE AND OLD.edited = FALSE)
EXECUTE FUNCTION update_message_edited_at();
