use diesel::prelude::*;
use rocket::serde::Serialize;

#[derive(Serialize, Queryable, Debug)]
pub struct Datasource {
    id: i32,
    name: String,
}
