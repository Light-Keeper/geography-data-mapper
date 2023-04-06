CREATE TABLE datasets
(
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE datapoints
(
    id         INTEGER PRIMARY KEY,
    dataset_id INTEGER NOT NULL,
    lng        REAL    NOT NULL,
    lat        REAL    NOT NULL,
    FOREIGN KEY (dataset_id) REFERENCES datasets (id)
);

-- Index on datapoint table for the dataset_id column
CREATE INDEX idx_datapoints_dataset_id ON datapoints (dataset_id);

-- Index on datapoint table for the lng lat column
CREATE INDEX idx_datapoints_lng_lat ON datapoints (lng, lat);

CREATE TABLE attributes
(
    id           INTEGER PRIMARY KEY,
    dataset_id   INTEGER NOT NULL,
    datapoint_id INTEGER NOT NULL,
    name         TEXT    NOT NULL,
    value        TEXT    NOT NULL,
    FOREIGN KEY (dataset_id) REFERENCES datasets (id),
    FOREIGN KEY (datapoint_id) REFERENCES datapoints (id)
);

CREATE INDEX idx_attributes_dataset_id_name ON attributes (dataset_id, name);
CREATE INDEX idx_attributes_datapoint_id ON attributes (datapoint_id);
