CREATE TABLE
    IF NOT EXISTS subjects (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        organisation_id UUID NOT NULL,
        name TEXT NOT NULL,
        FOREIGN KEY (organisation_id) REFERENCES organisations (id)
    );