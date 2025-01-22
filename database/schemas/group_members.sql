CREATE TABLE
    IF NOT EXISTS group_members (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        group_id UUID NOT NULL,
        user_id UUID NOT NULL,
        FOREIGN KEY (group_id) REFERENCES groups (id),
        FOREIGN KEY (user_id) REFERENCES users (id)
    );