-- Your SQL goes here
CREATE TABLE "users" (
  "id" uuid PRIMARY KEY,
  "name" varchar(100) NOT NULL,
  "email" varchar(30) NOT NULL,
  "password" varchar NOT NULL,
  "image_url" varchar(300)
);

CREATE TABLE "tasks" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid NOT NULL,
  "title" varchar NOT NULL,
  "description" text NOT NULL,
  "created_at" timestamp NOT NULL,
  "notification_at" timestamp,
  "completed_at" timestamp,
  "is_completed" boolean DEFAULT false NOT NULL
);

ALTER TABLE "tasks" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");