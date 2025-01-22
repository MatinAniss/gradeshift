CREATE TABLE
    IF NOT EXISTS subject_assignments (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        subject_id UUID NOT NULL,
        user_id UUID NOT NULL,
        FOREIGN KEY (subject_id) REFERENCES subjects (id),
        FOREIGN KEY (user_id) REFERENCES users (id)
    );