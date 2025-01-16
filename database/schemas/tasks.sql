CREATE TABLE
    IF NOT EXISTS tasks (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        subject_id UUID NOT NULL,
        name TEXT NOT NULL,
        FOREIGN KEY (subject_id) REFERENCES subjects (id)
    );