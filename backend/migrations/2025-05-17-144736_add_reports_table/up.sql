CREATE TABLE reports (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  file_path VARCHAR(255) NOT NULL,
  template_id INTEGER REFERENCES report_templates(id) NOT NULL,
  project_id UUID REFERENCES projects(id) NOT NULL
);