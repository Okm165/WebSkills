CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users" (
  "id" uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "firstname" CHAR(42) NOT NULL,
  "lastname" CHAR(42) NOT NULL
);