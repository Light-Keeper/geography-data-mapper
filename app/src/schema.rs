// @generated automatically by Diesel CLI.

diesel::table! {
    datapoints (id) {
        id -> Integer,
        datasource_id -> Integer,
        longitude -> Float,
        latitude -> Float,
        name -> Text,
        color -> Text,
    }
}

diesel::table! {
    datasources (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(datapoints -> datasources (datasource_id));

diesel::allow_tables_to_appear_in_same_query!(datapoints, datasources,);
