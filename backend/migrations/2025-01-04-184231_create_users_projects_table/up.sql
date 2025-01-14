CREATE TABLE users_projects (
    user_id UUID REFERENCES users(id) NOT NULL,
    project_id UUID REFERENCES projects(id) NOT NULL,
    PRIMARY KEY (user_id, project_id)
);