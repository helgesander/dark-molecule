CREATE TABLE hosts (
   id SERIAL PRIMARY KEY,
   hostname VARCHAR(100),
   ip_address VARCHAR NOT NULL,
   project_id UUID REFERENCES projects(id) NOT NULL
);
