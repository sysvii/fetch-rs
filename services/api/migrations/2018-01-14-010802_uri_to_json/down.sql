DROP TABLE info_blob;

CREATE TABLE info_uri (
    id SERIAL PRIMARY KEY,
    series_id INTEGER NOT NULL REFERENCES Series(id) ON DELETE CASCADE,
    uri VARCHAR NOT NULL,
    "primary" BOOLEAN DEFAULT FALSE NOT NULL
);

