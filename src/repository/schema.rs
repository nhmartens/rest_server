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
