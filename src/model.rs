use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable,Debug)]
#[diesel(table_name = crate::schema::notifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Notification {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable,Debug)]
#[diesel(table_name = crate::schema::notifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateNotification{
    pub title: String,
    pub body: String,
    pub created_at: chrono::NaiveDateTime,
    pub published: bool
}