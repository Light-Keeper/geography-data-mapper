use diesel::{AsExpression, FromSqlRow};
use diesel::prelude::*;
use diesel::backend::RawValue;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Text;
use rocket::serde::{Serialize, Deserialize};

#[derive(AsExpression, Debug, Deserialize, Serialize, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct Tags(serde_json::Value);

type DB = diesel::sqlite::Sqlite;

impl FromSql<Text, DB> for Tags {
    fn from_sql(bytes: RawValue<'_, DB>) -> diesel::deserialize::Result<Self> {
        let t = <String as FromSql<Text, DB>>::from_sql(bytes)?;
        Ok(Self(serde_json::from_str(&t)?))
    }
}

impl ToSql<Text, DB> for Tags {
    fn to_sql<'b>(&'b self, _out: &mut Output<'b, '_, DB>) -> diesel::serialize::Result {
        todo!()
    }
}

#[derive(Serialize, Queryable, Debug)]
pub struct Datasource {
    id: i32,
    name: String,
}

#[derive(Serialize, Queryable, Debug)]
pub struct Datapoint {
    id: i32,
    datasource_id: i32,
    lng: f32,
    lat: f32,
    name: String,
    tags: Tags,
}
