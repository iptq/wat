CREATE TABLE "users" (
    "id" INTEGER NOT NULL PRIMARY KEY,
    "email" VARCHAR(64) UNIQUE NOT NULL,
    "password" VARCHAR(128) NOT NULL,
    "display_name" VARCHAR(18),
    "api_key" VARCHAR(64) NOT NULL,
    "email_confirmed" BOOLEAN NOT NULL DEFAULT FALSE,

    "website" VARCHAR(64),
    "location" VARCHAR(64),

    "email_public" BOOLEAN NOT NULL DEFAULT FALSE,
    "logged_time_public" BOOLEAN NOT NULL DEFAULT FALSE,
    "languages_used_public" BOOLEAN NOT NULL DEFAULT FALSE,

    "last_heartbeat" TIMESTAMP,
    "registered_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX "users_display_name" ON "users" ("display_name");

CREATE TABLE "heartbeats" (
    "id" INTEGER NOT NULL PRIMARY KEY,
    "user_id" INTEGER NOT NULL REFERENCES "users" ("id"),

    "entity" VARCHAR(64) NOT NULL,
    "entity_type" VARCHAR(64) NOT NULL,
    "category" VARCHAR(64),
    "time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    "project" VARCHAR(64),
    "branch" VARCHAR(64),
    "language" VARCHAR(64),
    "dependencies" VARCHAR(64),

    "lines" INTEGER NOT NULL,
    "line_number" INTEGER,
    "cursor_pos" INTEGER,
    "is_write" BOOLEAN NOT NULL DEFAULT FALSE
);
