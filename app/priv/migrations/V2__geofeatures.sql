CREATE TABLE geo_feature
(
    id       INTEGER PRIMARY KEY,
    type     TEXT    NOT NULL,
    name     TEXT    NOT NULL,
    bbox     TEXT NOT NULL,
    geometry TEXT NOT NULL,
    UNIQUE (type, name)
);

CREATE TABLE geo_feature_property
(
    id             INTEGER PRIMARY KEY,
    geo_feature_id INTEGER NOT NULL,
    key            TEXT    NOT NULL,
    value          TEXT    NOT NULL,
    FOREIGN KEY (geo_feature_id) REFERENCES geo_feature (id),
    UNIQUE (geo_feature_id, key)
);

-- Index on geo_feature_property table for the geo_feature_id column
CREATE INDEX idx_geo_feature_property_geo_feature_id ON geo_feature_property (geo_feature_id);

-- Index on geo_feature_property table for the key column
CREATE INDEX idx_geo_feature_property_key ON geo_feature_property (key);

