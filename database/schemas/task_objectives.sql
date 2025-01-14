CREATE TABLE
    IF NOT EXISTS task_objectives (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        task_id UUID NOT NULL,
        text VARCHAR(50) NOT NULL,
        FOREIGN KEY (task_id) REFERENCES tasks (id)
    );