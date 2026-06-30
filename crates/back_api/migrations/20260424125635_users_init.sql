CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pers_id UUID NOT NULL REFERENCES persons(pers_id) ON DELETE CASCADE,
    comp_id UUID NOT NULL REFERENCES companys(comp_id) ON DELETE CASCADE,

    phone VARCHAR(15) NOT NULL, 
    password_hash TEXT NOT NULL,
    email VARCHAR NOT NULL,

    guids UUID[] NOT NULL DEFAULT '{}',
    
    sync_shard_id INTEGER DEFAULT floor(random() * 7),

    last_update TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT unique_person_company UNIQUE (pers_id, comp_id)
);


CREATE OR REPLACE FUNCTION update_user_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.guids IS NULL OR (NEW.guids = '{}' AND OLD.guids != '{}') THEN
        NEW.guids := OLD.guids;
    END IF;

    NEW.last_update := CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Исправили имя триггера
CREATE TRIGGER trg_update_user_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_user_timestamp();
