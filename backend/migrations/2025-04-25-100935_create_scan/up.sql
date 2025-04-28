CREATE TABLE IF NOT EXISTS scans (
    id SERIAL PRIMARY KEY,
    project_id UUID NOT NULL REFERENCES projects(id),
    scanner_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    error_message TEXT,
    scan_config JSONB,
    results_path TEXT
);