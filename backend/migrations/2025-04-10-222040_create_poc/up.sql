CREATE TABLE IF NOT EXISTS proof_of_concepts (
    id SERIAL PRIMARY KEY,
    issue_id UUID REFERENCES issues(id) NOT NULL,
    description TEXT NOT NULL,
    data BYTEA NOT NULL,
    content_type VARCHAR(50) NOT NULL,
    host VARCHAR(50) NOT NULL
);

