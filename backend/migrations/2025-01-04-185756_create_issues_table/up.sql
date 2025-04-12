CREATE TABLE issues (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    mitigation TEXT,
    cvss FLOAT NOT NULL DEFAULT 0.0,
    project_id UUID REFERENCES projects(id) NOT NULL
);