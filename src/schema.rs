// @generated automatically by Diesel CLI.

diesel::table! {
    choices (id) {
        id -> Integer,
        poll_id -> Integer,
        choice_text -> Text,
    }
}

diesel::table! {
    polls (id) {
        id -> Integer,
        external_id -> Text,
        question -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
    }
}

diesel::table! {
    votes (id) {
        id -> Integer,
        user_id -> Integer,
        poll_id -> Integer,
        choice_id -> Integer,
        voted_at -> Timestamp,
    }
}

diesel::joinable!(choices -> polls (poll_id));
diesel::joinable!(votes -> choices (choice_id));
diesel::joinable!(votes -> polls (poll_id));
diesel::joinable!(votes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    choices,
    polls,
    users,
    votes,
);
