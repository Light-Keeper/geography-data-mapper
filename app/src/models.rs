use diesel::prelude::*;
use rocket::serde::Serialize;

#[derive(Serialize, Queryable, Debug)]
pub struct Datasource {
    id: i32,
    name: String,
}

#[derive(Serialize, Queryable, Debug)]
pub struct Datapoint {
    id: i32,
    datasource_id: i32,
    longitude: f32,
    latitude: f32,
    name: String,
    color: String,
}
