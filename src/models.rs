use diesel::prelude::*;

#[derive(Identifiable, Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::polls)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Poll {
    pub id: i32,
    pub external_id: String,
    pub question: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::polls)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewPoll {
    pub external_id: String,
    pub question: String,
}

#[derive(Identifiable, Queryable, Selectable, Associations, Debug)]
#[diesel(table_name = crate::schema::choices)]
#[diesel(belongs_to(Poll, foreign_key = poll_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Choice {
    pub id: i32,
    pub poll_id: i32,
    pub choice_text: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::choices)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewChoice {
    pub poll_id: i32,
    pub choice_text: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::votes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Vote {
    pub id: i32,
    pub user_id: i32,
    pub poll_id: i32,
    pub choice_id: i32,
    pub voted_at: chrono::NaiveDateTime,
}

