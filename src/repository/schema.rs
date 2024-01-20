// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Text,
        abbreviation -> Text,
        name -> Text,
        created_at -> Nullable<Text>,
        updated_at -> Nullable<Text>,
    }
}

diesel::table! {
    projects (id) {
        id -> Text,
        client_id -> Text,
        name -> Text,
        created_at -> Nullable<Text>,
        updated_at -> Nullable<Text>,
    }
}

diesel::joinable!(projects -> clients (client_id));

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    projects,
);
