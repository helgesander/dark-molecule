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
    issue_hosts (issue_id, host_id) {
        issue_id -> Uuid,
        host_id -> Int4,
    }
}

diesel::table! {
    issues (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        mitigation -> Nullable<Text>,
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
        scope -> Nullable<Text>,
        start_date -> Date,
        end_date -> Date,
        folder -> Text,
        team_id -> Uuid,
    }
}

diesel::table! {
    proof_of_concepts (id) {
        id -> Int4,
        issue_id -> Uuid,
        description -> Text,
        data -> Bytea,
        #[max_length = 50]
        content_type -> Varchar,
        #[max_length = 50]
        host -> Varchar,
    }
}

diesel::table! {
    report_templates (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        file_path -> Varchar,
        #[max_length = 10]
        extension -> Varchar,
    }
}

diesel::table! {
    reports (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        file_path -> Varchar,
        template_id -> Int4,
        project_id -> Uuid,
    }
}

diesel::table! {
    scans (id) {
        id -> Int4,
        project_id -> Uuid,
        #[max_length = 50]
        scanner_type -> Varchar,
        #[max_length = 20]
        status -> Varchar,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        error_message -> Nullable<Text>,
        scan_config -> Nullable<Jsonb>,
        results_path -> Nullable<Text>,
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
diesel::joinable!(issue_hosts -> hosts (host_id));
diesel::joinable!(issue_hosts -> issues (issue_id));
diesel::joinable!(issues -> projects (project_id));
diesel::joinable!(projects -> teams (team_id));
diesel::joinable!(proof_of_concepts -> issues (issue_id));
diesel::joinable!(reports -> projects (project_id));
diesel::joinable!(reports -> report_templates (template_id));
diesel::joinable!(scans -> projects (project_id));
diesel::joinable!(teams -> users (admin_id));
diesel::joinable!(users_projects -> projects (project_id));
diesel::joinable!(users_projects -> users (user_id));
diesel::joinable!(users_teams -> teams (team_id));
diesel::joinable!(users_teams -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    hosts,
    issue_hosts,
    issues,
    projects,
    proof_of_concepts,
    report_templates,
    reports,
    scans,
    teams,
    users,
    users_projects,
    users_teams,
);
