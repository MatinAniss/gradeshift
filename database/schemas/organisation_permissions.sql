CREATE TABLE
    IF NOT EXISTS organisation_permissions (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        organisation_id UUID NOT NULL,
        user_id UUID NOT NULL,
        FOREIGN KEY (organisation_id) REFERENCES organisations (id),
        FOREIGN KEY (user_id) REFERENCES users (id)
    );