-- Your SQL goes here
CREATE TABLE "clients" (
    "id"            TEXT NOT NULL PRIMARY KEY,
    "abbreviation"  TEXT NOT NULL,
    "name"          TEXT NOT NULL,
    "created_at"    TEXT,
    "updated_at"    TEXT
);