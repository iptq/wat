CREATE TABLE "users" (
    "id" INTEGER PRIMARY KEY,
    "email" VARCHAR(64) NOT NULL,
    "display_name" VARCHAR(18),
    "api_key" VARCHAR(64),
    "email_confirmed" BOOLEAN NOT NULL DEFAULT FALSE,

    "website" VARCHAR(64),
    "location" VARCHAR(64),
    "last_heartbeat" TIMESTAMP,

    "email_public" BOOLEAN NOT NULL DEFAULT FALSE,
    "logged_time_public" BOOLEAN NOT NULL DEFAULT FALSE,
    "languages_used_public" BOOLEAN NOT NULL DEFAULT FALSE,

    "registered_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "last_active_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX "users_display_name" ON "users" ("display_name");

CREATE TABLE "heartbeats" (
    "id" INTEGER PRIMARY KEY,
    "entity" VARCHAR(64) NOT NULL,
    "type" VARCHAR(64) NOT NULL,
    "category" VARCHAR(64) NOT NULL,
    "time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    "project" VARCHAR(64) NOT NULL,
    "branch" VARCHAR(64) NOT NULL,
    "language" VARCHAR(64) NOT NULL,

    "lines" INTEGER NOT NULL,
    "line_number" INTEGER NOT NULL,
    "cursor_pos" INTEGER NOT NULL,
    "is_write" BOOLEAN NOT NULL DEFAULT FALSE,

    "user_id" INTEGER NOT NULL REFERENCES "users" ("id")
);
