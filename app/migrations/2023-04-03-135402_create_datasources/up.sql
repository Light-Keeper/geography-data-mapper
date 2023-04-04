-- Your SQL goes here

create table datasources (
    id INTEGER PRIMARY KEY NOT NULL,
    name varchar(255) NOT NULL UNIQUE
);

create table datapoints (
    id INTEGER PRIMARY KEY NOT NULL,
    datasource_id INTEGER NOT NULL,
    longitude REAL NOT NULL,
    latitude REAL NOT NULL,
    name varchar(255) NOT NULL,
    color varchar(255) NOT NULL,
    FOREIGN KEY (datasource_id) REFERENCES datasources(id)
)
