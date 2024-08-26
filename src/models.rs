use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::choices)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Choice {
    pub id: Option<i32>,
    pub poll_id: i32,
    pub choice_text: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::polls)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Poll {
    pub id: Option<i32>,
    pub external_id: String,
    pub question: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::votes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Vote {
    pub id: Option<i32>,
    pub user_id: i32,
    pub poll_id: i32,
    pub choice_id: i32,
    pub voted_at: Option<chrono::NaiveDateTime>,
}

