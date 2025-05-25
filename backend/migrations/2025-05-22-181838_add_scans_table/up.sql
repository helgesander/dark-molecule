CREATE TABLE scans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id),
    scanner_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL,
    result_path TEXT,
    target VARCHAR(100) NOT NULL
);