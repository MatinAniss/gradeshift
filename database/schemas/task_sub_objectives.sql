CREATE TABLE
    IF NOT EXISTS task_sub_objectives (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        task_objective_id UUID NOT NULL,
        text TEXT NOT NULL,
        FOREIGN KEY (task_objective_id) REFERENCES task_objectives (id)
    );