CREATE TABLE issues (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    mitigation TEXT NOT NULL,
    cvss FLOAT NOT NULL DEFAULT 0.0,
    project_id UUID REFERENCES projects(id) NOT NULL
);