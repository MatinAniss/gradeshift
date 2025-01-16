CREATE TABLE
    IF NOT EXISTS organisations (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name TEXT NOT NULL
    );