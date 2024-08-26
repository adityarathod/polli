// @generated automatically by Diesel CLI.

diesel::table! {
    choices (id) {
        id -> Nullable<Integer>,
        poll_id -> Integer,
        choice_text -> Text,
    }
}

diesel::table! {
    polls (id) {
        id -> Nullable<Integer>,
        external_id -> Text,
        question -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
    }
}

diesel::table! {
    votes (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        poll_id -> Integer,
        choice_id -> Integer,
        voted_at -> Nullable<Timestamp>,
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
