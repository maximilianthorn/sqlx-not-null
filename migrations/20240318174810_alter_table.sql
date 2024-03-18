-- Add migration script here
ALTER TABLE person ADD COLUMN last_name TEXT;
UPDATE person SET last_name = '';
ALTER TABLE person ALTER COLUMN last_name SET NOT NULL;
