CREATE TABLE users_teams (
     user_id UUID REFERENCES users(id) NOT NULL,
     team_id UUID REFERENCES teams(id) NOT NULL,
     PRIMARY KEY (user_id, team_id)
)
