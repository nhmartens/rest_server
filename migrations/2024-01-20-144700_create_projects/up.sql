-- Your SQL goes here
CREATE TABLE "projects" (
    "id"            TEXT NOT NULL PRIMARY KEY,
    "client_id"     TEXT NOT NULL REFERENCES clients(id),
    "name"          TEXT NOT NULL,
    "created_at"    TEXT,
    "updated_at"    TEXT
);