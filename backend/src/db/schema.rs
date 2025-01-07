// @generated automatically by Diesel CLI.

diesel::table! {
    hosts (id) {
        id -> Int4,
        #[max_length = 100]
        hostname -> Nullable<Varchar>,
        ip_address -> Varchar,
        project_id -> Uuid,
    }
}

diesel::table! {
    issues (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        description -> Text,
        mitigation -> Text,
        cvss -> Float8,
        project_id -> Uuid,
    }
}

diesel::table! {
    projects (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        start_date -> Date,
        end_date -> Date,
        folder -> Text,
        team_id -> Uuid,
    }
}

diesel::table! {
    teams (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        admin_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 100]
        first_name -> Nullable<Varchar>,
        #[max_length = 100]
        last_name -> Nullable<Varchar>,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 100]
        password -> Varchar,
        created_at -> Date,
        is_admin -> Bool,
        is_active -> Bool,
    }
}

diesel::table! {
    users_projects (user_id, project_id) {
        user_id -> Uuid,
        project_id -> Uuid,
    }
}

diesel::table! {
    users_teams (user_id, team_id) {
        user_id -> Uuid,
        team_id -> Uuid,
    }
}

diesel::joinable!(hosts -> projects (project_id));
diesel::joinable!(issues -> projects (project_id));
diesel::joinable!(projects -> teams (team_id));
diesel::joinable!(teams -> users (admin_id));
diesel::joinable!(users_projects -> projects (project_id));
diesel::joinable!(users_projects -> users (user_id));
diesel::joinable!(users_teams -> teams (team_id));
diesel::joinable!(users_teams -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    hosts,
    issues,
    projects,
    teams,
    users,
    users_projects,
    users_teams,
);
