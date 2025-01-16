CREATE TABLE
    IF NOT EXISTS task_objectives (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        task_id UUID NOT NULL,
        text TEXT NOT NULL,
        FOREIGN KEY (task_id) REFERENCES tasks (id)
    );