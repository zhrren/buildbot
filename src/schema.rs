// @generated automatically by Diesel CLI.

diesel::table! {
    t_project (id) {
        id -> Integer,
        app_name -> Text,
        build_number -> Integer,
    }
}
