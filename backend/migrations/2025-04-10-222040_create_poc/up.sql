CREATE TABLE IF NOT EXISTS proof_of_concepts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    issue_id UUID REFERENCES issues(id) NOT NULL,
    description TEXT NOT NULL,
    data BYTEA NOT NULL
);