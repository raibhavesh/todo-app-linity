-- Add migration script here
ALTER TABLE todos
DROP CONSTRAINT IF EXISTS todos_user_id_fkey;

-- Re-create it with CASCADE behavior
ALTER TABLE todos
ADD CONSTRAINT todos_user_id_fkey
FOREIGN KEY (user_id) REFERENCES users(id)
ON DELETE CASCADE;