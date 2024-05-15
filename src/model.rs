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

#[derive(Queryable,Selectable,Debug)]
#[diesel(table_name = crate::schema::authors)]
pub struct Author{
    pub id: i32,
    pub name:String,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Queryable,Selectable,Debug)]
#[diesel(table_name = crate::schema::books)]
#[diesel(belongs_to(Author))]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author_id: i32,
    pub created_at: NaiveDateTime,
}
