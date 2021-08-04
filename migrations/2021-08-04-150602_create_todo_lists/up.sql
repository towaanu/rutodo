CREATE TABLE todo_lists (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    label VARCHAR(255) NOT NULL CHECK(label <> ''),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE TRIGGER add_updated_at
BEFORE UPDATE ON todo_lists
FOR EACH ROW
EXECUTE PROCEDURE update_timestamp();
