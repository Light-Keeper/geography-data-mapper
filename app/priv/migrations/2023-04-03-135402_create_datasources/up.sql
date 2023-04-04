-- Your SQL goes here

create table datasources (
    id INTEGER PRIMARY KEY NOT NULL,
    name varchar(255) NOT NULL UNIQUE
);

create table datapoints (
    id INTEGER PRIMARY KEY NOT NULL,
    datasource_id INTEGER NOT NULL,
    lng REAL NOT NULL,
    lat REAL NOT NULL,
    name varchar(255) NOT NULL,
    tags TEXT NOT NULL,
    FOREIGN KEY (datasource_id) REFERENCES datasources(id)
)
