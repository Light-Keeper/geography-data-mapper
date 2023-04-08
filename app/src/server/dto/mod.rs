use rocket::serde::{Serialize, Deserialize};
use serde_json::value::RawValue;

#[derive(Serialize, Deserialize)]
pub struct Page<T> {
    pub limit: usize,
    pub offset: usize,
    pub more: bool,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct Datasource {
    pub id: usize,
    pub name: String,
    pub metadata: Box<RawValue>
}

#[derive(Serialize, Deserialize)]
pub struct Datapoint {
    pub tags: Box<RawValue>,
    pub lng: f32,
    pub lat: f32,
}
