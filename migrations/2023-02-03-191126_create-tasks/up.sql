-- Your SQL goes here
ALTER TABLE tasks
ALTER COLUMN create_at SET DATA TYPE timestamp with time zone,
ALTER COLUMN notification_at SET DATA TYPE timestamp with time zone,
ALTER COLUMN completed_at SET DATA TYPE timestamp with time zone;