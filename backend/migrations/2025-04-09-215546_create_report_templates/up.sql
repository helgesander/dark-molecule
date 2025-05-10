CREATE TABLE IF NOT EXISTS report_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID REFERENCES teams(id) NOT NULL,
    user_id UUID REFERENCES users(id) NOT NULL,
    name VARCHAR(100)NOT NULL,
    file_path VARCHAR(255) NOT NULL,
    extension VARCHAR(10) NOT NULL
)
