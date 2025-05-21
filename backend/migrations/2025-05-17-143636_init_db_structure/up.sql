CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    username VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    password VARCHAR(100) NOT NULL,
    created_at DATE NOT NULL DEFAULT CURRENT_DATE,
    is_admin BOOLEAN NOT NULL DEFAULT false,
    is_active BOOLEAN NOT NULL DEFAULT true
);


CREATE TABLE teams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    admin_id UUID NOT NULL REFERENCES users(id)
);


CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    scope TEXT,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    folder TEXT NOT NULL,
    team_id UUID NOT NULL REFERENCES teams(id)
);


CREATE TABLE hosts (
    id SERIAL PRIMARY KEY,
    hostname VARCHAR(100),
    ip_address VARCHAR NOT NULL,
    project_id UUID NOT NULL REFERENCES projects(id)
);

CREATE TABLE issues (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    mitigation TEXT,
    cvss DOUBLE PRECISION NOT NULL,
    project_id UUID NOT NULL REFERENCES projects(id)
);


CREATE TABLE issue_hosts (
    issue_id UUID NOT NULL REFERENCES issues(id),
    host_id INTEGER NOT NULL REFERENCES hosts(id),
    PRIMARY KEY (issue_id, host_id)
);


CREATE TABLE proof_of_concepts (
    id SERIAL PRIMARY KEY,
    issue_id UUID NOT NULL REFERENCES issues(id),
    description TEXT NOT NULL,
    data BYTEA NOT NULL,
    content_type VARCHAR(50) NOT NULL,
    host VARCHAR(50) NOT NULL
);


CREATE TABLE report_templates (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    file_path VARCHAR(255) NOT NULL,
    extension VARCHAR(10) NOT NULL
);

CREATE TABLE scans (
    id SERIAL PRIMARY KEY,
    project_id UUID NOT NULL REFERENCES projects(id),
    scanner_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL,
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    error_message TEXT,
    scan_config JSONB,
    results_path TEXT
);

CREATE TABLE users_teams (
    user_id UUID NOT NULL REFERENCES users(id),
    team_id UUID NOT NULL REFERENCES teams(id),
    PRIMARY KEY (user_id, team_id)
);

CREATE TABLE users_projects (
    user_id UUID NOT NULL REFERENCES users(id),
    project_id UUID NOT NULL REFERENCES projects(id),
    PRIMARY KEY (user_id, project_id)
);